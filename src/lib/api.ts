import { invoke } from "@tauri-apps/api/core";
import type { OrkbEntry, OrkbRelation } from "./types";

export async function initDatabase(): Promise<void> {
  return invoke("init_database");
}

export async function listPackages(): Promise<unknown[]> {
  return invoke("list_packages");
}

export async function listEntries(): Promise<OrkbEntry[]> {
  return invoke("list_entries");
}

export async function searchEntries(query: string): Promise<OrkbEntry[]> {
  return invoke("search_entries", { query });
}

export async function listRelations(): Promise<OrkbRelation[]> {
  return invoke("list_relations");
}

export async function importPackage(path: string): Promise<void> {
  return invoke("import_package", { path });
}

export async function exportPackage(packageId: string, path: string): Promise<void> {
  return invoke("export_package", { packageId, path });
}
