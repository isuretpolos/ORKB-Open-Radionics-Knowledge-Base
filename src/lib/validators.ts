import { z } from "zod";

export const entryStatusSchema = z.enum([
  "ACTIVE",
  "DISABLED",
  "DEPRECATED",
]);

export const entryTypeSchema = z.enum([
  "LEVEL",
  "DOMAIN",
  "REMEDY",
  "SUBSTANCE",
  "SYMBOL",
  "ARCHETYPE",
  "CONCEPT",
  "ORGAN",
  "SYSTEM",
  "EMOTION",
  "PATHOLOGY",
  "MODALITY",
  "RATE",
  "CUSTOM",
]);

export const relationTypeSchema = z.enum([
  "next_level",
  "contains_domain",
  "belongs_to_domain",
  "resonates_with_level",
  "supports",
  "balances",
  "aggravates",
  "opposes",
  "similar_to",
  "part_of",
  "contains",
  "extends",
  "maps_to",
  "has_rate",
  "blocked_by",
  "clarified_by",
]);

export const manifestSchema = z.object({
  format: z.literal("orkb"),
  formatVersion: z.string().min(1),
  packageId: z.string().min(1),
  name: z.string().min(1),
  version: z.string().min(1),
  author: z.object({
    id: z.string().min(1),
    name: z.string().min(1),
    contact: z.string().optional(),
    url: z.string().optional(),
  }),
  description: z.string().optional(),
  license: z.string().optional(),
  createdAt: z.string().optional(),
  updatedAt: z.string().optional(),
  dependencies: z
    .array(
      z.object({
        packageId: z.string().min(1),
        version: z.string().min(1),
      })
    )
    .optional(),
  tags: z.array(z.string()).optional(),
  compatibility: z
    .object({
      minimumEngineVersion: z.string().optional(),
    })
    .optional(),
});

export const entrySchema = z.object({
  id: z.string().min(1),
  type: entryTypeSchema,
  key: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
  status: entryStatusSchema.optional(),
  language: z.string().optional(),
  aliases: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  data: z.record(z.string(), z.unknown()).optional(),
  source: z
    .object({
      type: z.enum([
        "author",
        "book",
        "website",
        "tradition",
        "experiment",
        "unknown",
      ]),
      reference: z.string().optional(),
    })
    .optional(),
});

export const relationSchema = z.object({
  id: z.string().min(1),
  from: z.string().min(1),
  relation: relationTypeSchema,
  to: z.string().min(1),
  weight: z.number().min(0).max(1).optional(),
  status: entryStatusSchema.optional(),
  data: z.record(z.string(), z.unknown()).optional(),
});

export const analysisProfileSchema = z.object({
  id: z.string().min(1),
  name: z.string().min(1),
  description: z.string().optional(),
  status: entryStatusSchema.optional(),
  config: z.record(z.string(), z.unknown()),
});

export const packageSchema = z.object({
  manifest: manifestSchema,
  entries: z.array(entrySchema),
  relations: z.array(relationSchema),
  profiles: z.array(analysisProfileSchema).default([]),
});
