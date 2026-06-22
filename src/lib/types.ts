export type OrkbFormat = "orkb";

export type EntryStatus = "ACTIVE" | "DISABLED" | "DEPRECATED";

export type EntryType =
  | "LEVEL"
  | "DOMAIN"
  | "REMEDY"
  | "SUBSTANCE"
  | "SYMBOL"
  | "ARCHETYPE"
  | "CONCEPT"
  | "ORGAN"
  | "SYSTEM"
  | "EMOTION"
  | "PATHOLOGY"
  | "MODALITY"
  | "RATE"
  | "CUSTOM";

export type RelationType =
  | "next_level"
  | "contains_domain"
  | "belongs_to_domain"
  | "resonates_with_level"
  | "supports"
  | "balances"
  | "aggravates"
  | "opposes"
  | "similar_to"
  | "part_of"
  | "contains"
  | "extends"
  | "maps_to"
  | "has_rate"
  | "blocked_by"
  | "clarified_by";

export interface OrkbAuthor {
  id: string;
  name: string;
  contact?: string;
  url?: string;
}

export interface OrkbDependency {
  packageId: string;
  version: string;
}

export interface OrkbManifest {
  format: OrkbFormat;
  formatVersion: string;
  packageId: string;
  name: string;
  version: string;
  author: OrkbAuthor;
  description?: string;
  license?: string;
  createdAt?: string;
  updatedAt?: string;
  dependencies?: OrkbDependency[];
  tags?: string[];
  compatibility?: {
    minimumEngineVersion?: string;
  };
}

export interface OrkbSource {
  type: "author" | "book" | "website" | "tradition" | "experiment" | "unknown";
  reference?: string;
}

export interface OrkbEntry {
  id: string;
  type: EntryType;
  key: string;
  name: string;
  description?: string;
  status?: EntryStatus;
  language?: string;
  aliases?: string[];
  tags?: string[];
  data?: Record<string, unknown>;
  source?: OrkbSource;
}

export interface OrkbRelation {
  id: string;
  from: string;
  relation: RelationType;
  to: string;
  weight?: number;
  status?: EntryStatus;
  data?: Record<string, unknown>;
}

export interface OrkbAnalysisProfile {
  id: string;
  name: string;
  description?: string;
  status?: EntryStatus;
  config: Record<string, unknown>;
}

export interface OrkbPackage {
  manifest: OrkbManifest;
  entries: OrkbEntry[];
  relations: OrkbRelation[];
  profiles: OrkbAnalysisProfile[];
}
