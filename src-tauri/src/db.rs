use crate::db_model::{DbEntry, DbPackage, DbRelation};
use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, Row, SqliteConnection};
use sqlx::Connection;
use std::str::FromStr;

const DB_PATH: &str = "orkb.sqlite";

async fn connect() -> Result<SqliteConnection> {
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", DB_PATH))?
        .create_if_missing(true);

    let conn = SqliteConnection::connect_with(&options).await?;
    Ok(conn)
}

pub async fn init_database() -> Result<()> {
    let mut conn = connect().await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS packages (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            author_id TEXT,
            author_name TEXT NOT NULL,
            description TEXT,
            license TEXT,
            active INTEGER NOT NULL DEFAULT 1,
            manifest_json TEXT NOT NULL,
            imported_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            id TEXT PRIMARY KEY,
            package_id TEXT NOT NULL,
            type TEXT NOT NULL,
            key TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            language TEXT,
            aliases_json TEXT,
            tags_json TEXT,
            data_json TEXT NOT NULL DEFAULT '{}',
            source_json TEXT,
            FOREIGN KEY (package_id) REFERENCES packages(id)
        );
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS relations (
            id TEXT PRIMARY KEY,
            package_id TEXT NOT NULL,
            from_entry_id TEXT NOT NULL,
            relation_type TEXT NOT NULL,
            to_entry_id TEXT NOT NULL,
            weight REAL NOT NULL DEFAULT 1.0,
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            data_json TEXT NOT NULL DEFAULT '{}',
            FOREIGN KEY (package_id) REFERENCES packages(id)
        );
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS analysis_profiles (
            id TEXT PRIMARY KEY,
            package_id TEXT,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            config_json TEXT NOT NULL,
            FOREIGN KEY (package_id) REFERENCES packages(id)
        );
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS local_overrides (
            id TEXT PRIMARY KEY,
            target_id TEXT NOT NULL,
            target_kind TEXT NOT NULL,
            operation TEXT NOT NULL,
            data_json TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workspace_package_settings (
            package_id TEXT PRIMARY KEY,
            enabled INTEGER NOT NULL DEFAULT 1,
            priority INTEGER NOT NULL DEFAULT 100,
            trust_level INTEGER NOT NULL DEFAULT 50,
            notes TEXT,
            FOREIGN KEY (package_id) REFERENCES packages(id)
        );
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_package_id
        ON entries(package_id);
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_type
        ON entries(type);
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_entries_key
        ON entries(key);
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_relations_from
        ON relations(from_entry_id);
        "#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_relations_to
        ON relations(to_entry_id);
        "#,
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn list_packages() -> Result<Vec<DbPackage>> {
    let mut conn = connect().await?;

    let rows = sqlx::query(
        r#"
        SELECT id, name, version, author_id, author_name, description,
               license, active, manifest_json, imported_at
        FROM packages
        ORDER BY author_name, name, version
        "#,
    )
    .fetch_all(&mut conn)
    .await?;

    let result = rows
        .into_iter()
        .map(|row| DbPackage {
            id: row.get("id"),
            name: row.get("name"),
            version: row.get("version"),
            author_id: row.get("author_id"),
            author_name: row.get("author_name"),
            description: row.get("description"),
            license: row.get("license"),
            active: row.get::<i64, _>("active") != 0,
            manifest_json: row.get("manifest_json"),
            imported_at: row.get("imported_at"),
        })
        .collect();

    Ok(result)
}

pub async fn list_entries() -> Result<Vec<DbEntry>> {
    let mut conn = connect().await?;

    let rows = sqlx::query(
        r#"
        SELECT id, package_id, type, key, name, description, status,
               language, aliases_json, tags_json, data_json, source_json
        FROM entries
        ORDER BY type, name
        "#,
    )
    .fetch_all(&mut conn)
    .await?;

    Ok(rows.into_iter().map(row_to_entry).collect())
}

pub async fn search_entries(query_text: &str) -> Result<Vec<DbEntry>> {
    let mut conn = connect().await?;

    let like = format!("%{}%", query_text.trim());

    let rows = sqlx::query(
        r#"
        SELECT id, package_id, type, key, name, description, status,
               language, aliases_json, tags_json, data_json, source_json
        FROM entries
        WHERE name LIKE ?1
           OR key LIKE ?1
           OR description LIKE ?1
           OR tags_json LIKE ?1
        ORDER BY type, name
        LIMIT 500
        "#,
    )
    .bind(like)
    .fetch_all(&mut conn)
    .await?;

    Ok(rows.into_iter().map(row_to_entry).collect())
}

pub async fn list_relations() -> Result<Vec<DbRelation>> {
    let mut conn = connect().await?;

    let rows = sqlx::query(
        r#"
        SELECT id, package_id, from_entry_id, relation_type,
               to_entry_id, weight, status, data_json
        FROM relations
        ORDER BY relation_type, from_entry_id, to_entry_id
        "#,
    )
    .fetch_all(&mut conn)
    .await?;

    let result = rows
        .into_iter()
        .map(|row| DbRelation {
            id: row.get("id"),
            package_id: row.get("package_id"),
            from_entry_id: row.get("from_entry_id"),
            relation_type: row.get("relation_type"),
            to_entry_id: row.get("to_entry_id"),
            weight: row.get("weight"),
            status: row.get("status"),
            data_json: row.get("data_json"),
        })
        .collect();

    Ok(result)
}

fn row_to_entry(row: sqlx::sqlite::SqliteRow) -> DbEntry {
    DbEntry {
        id: row.get("id"),
        package_id: row.get("package_id"),
        entry_type: row.get("type"),
        key: row.get("key"),
        name: row.get("name"),
        description: row.get("description"),
        status: row.get("status"),
        language: row.get("language"),
        aliases_json: row.get("aliases_json"),
        tags_json: row.get("tags_json"),
        data_json: row.get("data_json"),
        source_json: row.get("source_json"),
    }
}
