// src/lib/api.ts

import { invoke } from "@tauri-apps/api/core";

export interface DbPackage {
  id: string;
  name: string;
  version: string;
  author_id?: string | null;
  author_name: string;
  description?: string | null;
  license?: string | null;
  active: boolean;
  manifest_json: string;
  imported_at: string;
}

export interface DbEntry {
  id: string;
  package_id: string;
  entry_type: string;
  key: string;
  name: string;
  description?: string | null;
  status: string;
  language?: string | null;
  aliases_json?: string | null;
  tags_json?: string | null;
  data_json: string;
  source_json?: string | null;
}

export interface DbRelation {
  id: string;
  package_id: string;
  from_entry_id: string;
  relation_type: string;
  to_entry_id: string;
  weight: number;
  status: string;
  data_json: string;
}

export async function initDatabase(): Promise<void> {
  return invoke("init_database");
}

export async function listPackages(): Promise<DbPackage[]> {
  return invoke("list_packages");
}

export async function listEntries(): Promise<DbEntry[]> {
  return invoke("list_entries");
}

export async function searchEntries(query: string): Promise<DbEntry[]> {
  return invoke("search_entries", { query });
}

export async function listRelations(): Promise<DbRelation[]> {
  return invoke("list_relations");
}

export async function importPackage(path: string): Promise<void> {
  return invoke("import_package", { path });
}

export async function exportPackage(
  packageId: string,
  path: string
): Promise<void> {
  return invoke("export_package", { packageId, path });
}
