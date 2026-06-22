use crate::db;
use crate::model::{OrkbAnalysisProfile, OrkbEntry, OrkbManifest, OrkbRelation};
use anyhow::{anyhow, Result};
use chrono::Utc;
use serde_json::Value;
use sqlx::{sqlite::SqliteConnectOptions, Connection, SqliteConnection};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const DB_PATH: &str = "orkb.sqlite";

async fn connect() -> Result<SqliteConnection> {
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", DB_PATH))?
        .create_if_missing(true);

    Ok(SqliteConnection::connect_with(&options).await?)
}

pub async fn import_package(path: &str) -> Result<()> {
    db::init_database().await?;

    let package_dir = PathBuf::from(path);

    if !package_dir.exists() {
        return Err(anyhow!("Import path does not exist: {}", path));
    }

    if !package_dir.is_dir() {
        return Err(anyhow!(
            "Only directory import is supported in this first version: {}",
            path
        ));
    }

    let manifest: OrkbManifest = read_json(package_dir.join("manifest.json"))?;
    let entries: Vec<OrkbEntry> = read_json_or_default(package_dir.join("entries.json"))?;
    let relations: Vec<OrkbRelation> = read_json_or_default(package_dir.join("relations.json"))?;
    let profiles: Vec<OrkbAnalysisProfile> =
        read_json_or_default(package_dir.join("profiles.json"))?;

    validate_manifest(&manifest)?;
    validate_entries(&manifest.package_id, &entries)?;
    validate_relations(&relations)?;

    let mut conn = connect().await?;
    let mut tx = conn.begin().await?;

    let manifest_json = serde_json::to_string_pretty(&manifest)?;

    sqlx::query(
        r#"
        INSERT INTO packages (
            id, name, version, author_id, author_name, description,
            license, active, manifest_json, imported_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, ?8, ?9)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            version = excluded.version,
            author_id = excluded.author_id,
            author_name = excluded.author_name,
            description = excluded.description,
            license = excluded.license,
            manifest_json = excluded.manifest_json,
            imported_at = excluded.imported_at
        "#,
    )
    .bind(&manifest.package_id)
    .bind(&manifest.name)
    .bind(&manifest.version)
    .bind(&manifest.author.id)
    .bind(&manifest.author.name)
    .bind(&manifest.description)
    .bind(&manifest.license)
    .bind(&manifest_json)
    .bind(Utc::now().to_rfc3339())
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO workspace_package_settings (
            package_id, enabled, priority, trust_level, notes
        )
        VALUES (?1, 1, 100, 50, NULL)
        ON CONFLICT(package_id) DO NOTHING
        "#,
    )
    .bind(&manifest.package_id)
    .execute(&mut *tx)
    .await?;

    for entry in entries {
        let aliases_json = serde_json::to_string(&entry.aliases.unwrap_or_default())?;
        let tags_json = serde_json::to_string(&entry.tags.unwrap_or_default())?;
        let data_json = serde_json::to_string(&entry.data.unwrap_or(Value::Object(Default::default())))?;
        let source_json = match entry.source {
            Some(source) => Some(serde_json::to_string(&source)?),
            None => None,
        };

        sqlx::query(
            r#"
            INSERT INTO entries (
                id, package_id, type, key, name, description, status,
                language, aliases_json, tags_json, data_json, source_json
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ON CONFLICT(id) DO UPDATE SET
                package_id = excluded.package_id,
                type = excluded.type,
                key = excluded.key,
                name = excluded.name,
                description = excluded.description,
                status = excluded.status,
                language = excluded.language,
                aliases_json = excluded.aliases_json,
                tags_json = excluded.tags_json,
                data_json = excluded.data_json,
                source_json = excluded.source_json
            "#,
        )
        .bind(&entry.id)
        .bind(&manifest.package_id)
        .bind(&entry.entry_type)
        .bind(&entry.key)
        .bind(&entry.name)
        .bind(&entry.description)
        .bind(entry.status.unwrap_or_else(|| "ACTIVE".to_string()))
        .bind(&entry.language)
        .bind(aliases_json)
        .bind(tags_json)
        .bind(data_json)
        .bind(source_json)
        .execute(&mut *tx)
        .await?;
    }

    for relation in relations {
        let data_json =
            serde_json::to_string(&relation.data.unwrap_or(Value::Object(Default::default())))?;

        sqlx::query(
            r#"
            INSERT INTO relations (
                id, package_id, from_entry_id, relation_type,
                to_entry_id, weight, status, data_json
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                package_id = excluded.package_id,
                from_entry_id = excluded.from_entry_id,
                relation_type = excluded.relation_type,
                to_entry_id = excluded.to_entry_id,
                weight = excluded.weight,
                status = excluded.status,
                data_json = excluded.data_json
            "#,
        )
        .bind(&relation.id)
        .bind(&manifest.package_id)
        .bind(&relation.from)
        .bind(&relation.relation)
        .bind(&relation.to)
        .bind(relation.weight.unwrap_or(1.0))
        .bind(relation.status.unwrap_or_else(|| "ACTIVE".to_string()))
        .bind(data_json)
        .execute(&mut *tx)
        .await?;
    }

    for profile in profiles {
        let config_json = serde_json::to_string_pretty(&profile.config)?;

        sqlx::query(
            r#"
            INSERT INTO analysis_profiles (
                id, package_id, name, description, status, config_json
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT(id) DO UPDATE SET
                package_id = excluded.package_id,
                name = excluded.name,
                description = excluded.description,
                status = excluded.status,
                config_json = excluded.config_json
            "#,
        )
        .bind(&profile.id)
        .bind(&manifest.package_id)
        .bind(&profile.name)
        .bind(&profile.description)
        .bind(profile.status.unwrap_or_else(|| "ACTIVE".to_string()))
        .bind(config_json)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> Result<T> {
    let text = fs::read_to_string(&path)
        .map_err(|e| anyhow!("Could not read JSON file {:?}: {}", path, e))?;

    let value = serde_json::from_str::<T>(&text)
        .map_err(|e| anyhow!("Invalid JSON in {:?}: {}", path, e))?;

    Ok(value)
}

fn read_json_or_default<T>(path: PathBuf) -> Result<T>
where
    T: serde::de::DeserializeOwned + Default,
{
    if !Path::new(&path).exists() {
        return Ok(T::default());
    }

    read_json(path)
}

fn validate_manifest(manifest: &OrkbManifest) -> Result<()> {
    if manifest.format != "orkb" {
        return Err(anyhow!("Invalid package format: {}", manifest.format));
    }

    if manifest.package_id.trim().is_empty() {
        return Err(anyhow!("Missing packageId"));
    }

    if manifest.name.trim().is_empty() {
        return Err(anyhow!("Missing package name"));
    }

    if manifest.version.trim().is_empty() {
        return Err(anyhow!("Missing package version"));
    }

    if manifest.author.id.trim().is_empty() {
        return Err(anyhow!("Missing author id"));
    }

    if manifest.author.name.trim().is_empty() {
        return Err(anyhow!("Missing author name"));
    }

    Ok(())
}

fn validate_entries(package_id: &str, entries: &[OrkbEntry]) -> Result<()> {
    for entry in entries {
        if entry.id.trim().is_empty() {
            return Err(anyhow!("Entry with empty id in package {}", package_id));
        }

        if entry.key.trim().is_empty() {
            return Err(anyhow!("Entry {} has empty key", entry.id));
        }

        if entry.name.trim().is_empty() {
            return Err(anyhow!("Entry {} has empty name", entry.id));
        }

        if entry.entry_type.trim().is_empty() {
            return Err(anyhow!("Entry {} has empty type", entry.id));
        }
    }

    Ok(())
}

fn validate_relations(relations: &[OrkbRelation]) -> Result<()> {
    for relation in relations {
        if relation.id.trim().is_empty() {
            return Err(anyhow!("Relation with empty id"));
        }

        if relation.from.trim().is_empty() {
            return Err(anyhow!("Relation {} has empty from", relation.id));
        }

        if relation.to.trim().is_empty() {
            return Err(anyhow!("Relation {} has empty to", relation.id));
        }

        if relation.relation.trim().is_empty() {
            return Err(anyhow!("Relation {} has empty relation type", relation.id));
        }

        if let Some(weight) = relation.weight {
            if !(0.0..=1.0).contains(&weight) {
                return Err(anyhow!(
                    "Relation {} has invalid weight {}. Expected 0.0..1.0",
                    relation.id,
                    weight
                ));
            }
        }
    }

    Ok(())
}
