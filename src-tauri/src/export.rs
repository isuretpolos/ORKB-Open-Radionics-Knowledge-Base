use crate::model::{
    OrkbAnalysisProfile, OrkbEntry, OrkbManifest, OrkbRelation,
};
use anyhow::{anyhow, Result};
use serde_json::Value;
use sqlx::{sqlite::SqliteConnectOptions, Connection, Row, SqliteConnection};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const DB_PATH: &str = "orkb.sqlite";

async fn connect() -> Result<SqliteConnection> {
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", DB_PATH))?
        .create_if_missing(true);

    Ok(SqliteConnection::connect_with(&options).await?)
}

pub async fn export_package(package_id: &str, target_path: &str) -> Result<()> {
    let mut conn = connect().await?;

    let manifest = load_manifest(&mut conn, package_id).await?;
    let entries = load_entries(&mut conn, package_id).await?;
    let relations = load_relations(&mut conn, package_id).await?;
    let profiles = load_profiles(&mut conn, package_id).await?;

    let target_dir = PathBuf::from(target_path);

    if target_dir.exists() && !target_dir.is_dir() {
        return Err(anyhow!(
            "Export target exists but is not a directory: {}",
            target_path
        ));
    }

    fs::create_dir_all(&target_dir)?;

    write_pretty_json(target_dir.join("manifest.json"), &manifest)?;
    write_pretty_json(target_dir.join("entries.json"), &entries)?;
    write_pretty_json(target_dir.join("relations.json"), &relations)?;
    write_pretty_json(target_dir.join("profiles.json"), &profiles)?;

    Ok(())
}

async fn load_manifest(conn: &mut SqliteConnection, package_id: &str) -> Result<OrkbManifest> {
    let row = sqlx::query(
        r#"
        SELECT manifest_json
        FROM packages
        WHERE id = ?1
        "#,
    )
    .bind(package_id)
    .fetch_optional(&mut *conn)
    .await?;

    let row = row.ok_or_else(|| anyhow!("Package not found: {}", package_id))?;

    let manifest_json: String = row.get("manifest_json");
    let manifest: OrkbManifest = serde_json::from_str(&manifest_json)?;

    Ok(manifest)
}

async fn load_entries(conn: &mut SqliteConnection, package_id: &str) -> Result<Vec<OrkbEntry>> {
    let rows = sqlx::query(
        r#"
        SELECT id, type, key, name, description, status, language,
               aliases_json, tags_json, data_json, source_json
        FROM entries
        WHERE package_id = ?1
        ORDER BY type, key, name
        "#,
    )
    .bind(package_id)
    .fetch_all(&mut *conn)
    .await?;

    let mut result = Vec::new();

    for row in rows {
        let aliases_json: Option<String> = row.get("aliases_json");
        let tags_json: Option<String> = row.get("tags_json");
        let data_json: String = row.get("data_json");
        let source_json: Option<String> = row.get("source_json");

        let aliases = parse_json_optional_vec(aliases_json)?;
        let tags = parse_json_optional_vec(tags_json)?;
        let data = parse_json_value(data_json)?;
        let source = parse_json_optional_value(source_json)?;

        result.push(OrkbEntry {
            id: row.get("id"),
            entry_type: row.get("type"),
            key: row.get("key"),
            name: row.get("name"),
            description: row.get("description"),
            status: Some(row.get("status")),
            language: row.get("language"),
            aliases: Some(aliases),
            tags: Some(tags),
            data: Some(data),
            source,
        });
    }

    Ok(result)
}

async fn load_relations(
    conn: &mut SqliteConnection,
    package_id: &str,
) -> Result<Vec<OrkbRelation>> {
    let rows = sqlx::query(
        r#"
        SELECT id, from_entry_id, relation_type, to_entry_id,
               weight, status, data_json
        FROM relations
        WHERE package_id = ?1
        ORDER BY relation_type, from_entry_id, to_entry_id
        "#,
    )
    .bind(package_id)
    .fetch_all(&mut *conn)
    .await?;

    let mut result = Vec::new();

    for row in rows {
        let data_json: String = row.get("data_json");

        result.push(OrkbRelation {
            id: row.get("id"),
            from: row.get("from_entry_id"),
            relation: row.get("relation_type"),
            to: row.get("to_entry_id"),
            weight: Some(row.get("weight")),
            status: Some(row.get("status")),
            data: Some(parse_json_value(data_json)?),
        });
    }

    Ok(result)
}

async fn load_profiles(
    conn: &mut SqliteConnection,
    package_id: &str,
) -> Result<Vec<OrkbAnalysisProfile>> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, description, status, config_json
        FROM analysis_profiles
        WHERE package_id = ?1
        ORDER BY name
        "#,
    )
    .bind(package_id)
    .fetch_all(&mut *conn)
    .await?;

    let mut result = Vec::new();

    for row in rows {
        let config_json: String = row.get("config_json");

        result.push(OrkbAnalysisProfile {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            status: Some(row.get("status")),
            config: parse_json_value(config_json)?,
        });
    }

    Ok(result)
}

fn write_pretty_json<T: serde::Serialize>(path: PathBuf, value: &T) -> Result<()> {
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, text)?;
    Ok(())
}

fn parse_json_value(text: String) -> Result<Value> {
    if text.trim().is_empty() {
        return Ok(Value::Object(Default::default()));
    }

    Ok(serde_json::from_str(&text)?)
}

fn parse_json_optional_value(text: Option<String>) -> Result<Option<Value>> {
    match text {
        Some(value) if !value.trim().is_empty() => Ok(Some(serde_json::from_str(&value)?)),
        _ => Ok(None),
    }
}

fn parse_json_optional_vec(text: Option<String>) -> Result<Vec<String>> {
    match text {
        Some(value) if !value.trim().is_empty() => Ok(serde_json::from_str(&value)?),
        _ => Ok(Vec::new()),
    }
}
