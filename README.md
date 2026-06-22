# ORKB Editor

ORKB Editor is a local-first editor for modular radionics knowledge databases.

ORKB stands for **Open Radionics Knowledge Base**. The goal of the project is to make radionics-related knowledge portable, extensible, author-attributed, and reusable across different applications, including AetherOnePi or other independent radionics tools.

The project is currently in an early design and prototype phase.

## Core Idea

ORKB is not meant to be one single central database. Instead, it is designed as a package-based knowledge system.

Each author can create their own ORKB package. Other users can import these packages into their local workspace, enable or disable them, assign trust levels, and use them selectively during analysis.

Example:

```text
Use:
    Isuret Core Levels
    Fritz Mineral Additions
    Laura Bach Flower Mapping

Do not use:
    Unknown Experimental Database
```

Imported packages keep their identity. Entries are not merged blindly into one anonymous database. Every entry remains connected to its source package and author.

## Main Concepts

### Package

A package is an importable and exportable knowledge unit.

A package has:

```text
packageId
name
version
author
license
description
dependencies
entries
relations
profiles
```

Example package IDs:

```text
isuret.core.levels
isuret.core.domains
fritz.minerals
Laura.bach_flowers
```

### Entry

Everything is an entry.

Levels, domains, vitamins, minerals, remedies, symbols, organs, emotions, archetypes, rates, and concepts are all stored as entries.

There are no hardcoded special tables for “levels” or “vitamins”. The meaning of an entry comes from its type and its relations.

Example entry types:

```text
LEVEL
DOMAIN
REMEDY
SUBSTANCE
SYMBOL
ARCHETYPE
CONCEPT
ORGAN
SYSTEM
EMOTION
PATHOLOGY
MODALITY
RATE
CUSTOM
```

### Relation

Relations connect entries.

The hierarchy and analysis logic are not hardcoded in the application. They are also stored as data.

Example:

```text
ASTRAL -> contains_domain -> BACH_FLOWERS
PHYSICAL -> contains_domain -> VITAMINS
VITAMIN_B12 -> belongs_to_domain -> VITAMINS
MIMULUS -> resonates_with_level -> ASTRAL
```

Important relation types:

```text
next_level
contains_domain
belongs_to_domain
resonates_with_level
supports
balances
aggravates
opposes
similar_to
part_of
contains
extends
maps_to
has_rate
blocked_by
clarified_by
```

### Analysis Profile

An analysis profile defines how an analysis engine should traverse the knowledge base.

A simple default strategy is:

```text
1. Scan LEVEL entries.
2. Identify the strongest level.
3. Follow contains_domain relations from that level.
4. Scan entries belonging to these domains.
5. Return ranked results with source package and author.
```

Example:

```text
If ASTRAL is identified:
    scan Bach Flowers, emotions, symbols, homeopathic remedies

If PHYSICAL is identified:
    scan vitamins, minerals, organs, tissues
```

The analysis path is data-driven and can be changed by packages or user profiles.

## ORKB Package Format

An ORKB package is currently represented as a directory:

```text
package-name/
    manifest.json
    entries.json
    relations.json
    profiles.json
```

Later, this directory can be zipped into a portable `.orkb` file.

### manifest.json

Contains package metadata.

```json
{
  "format": "orkb",
  "formatVersion": "1.0",
  "packageId": "isuret.core.levels",
  "name": "Isuret Core Levels",
  "version": "1.0.0",
  "author": {
    "id": "isuret",
    "name": "Isuret"
  },
  "description": "Core metaphysical levels used as the primary diagnostic hierarchy in radionics analysis.",
  "license": "CC-BY-SA-4.0",
  "dependencies": []
}
```

### entries.json

Contains all entries owned by the package.

```json
[
  {
    "id": "isuret.level.astral",
    "type": "LEVEL",
    "key": "ASTRAL",
    "name": "Astral",
    "description": "The imaginal and emotional level of dreams, desires, fears, impressions, symbols, and reactive patterns.",
    "status": "ACTIVE",
    "language": "en",
    "tags": ["emotion", "dream", "imaginal"],
    "data": {
      "rank": 90,
      "radionicInfluence": true
    }
  }
]
```

### relations.json

Contains all relations owned by the package.

```json
[
  {
    "id": "rel.isuret.level.astral.contains.bach_flowers",
    "from": "isuret.level.astral",
    "relation": "contains_domain",
    "to": "isuret.domain.bach_flowers",
    "weight": 0.95,
    "status": "ACTIVE",
    "data": {}
  }
]
```

### profiles.json

Contains optional analysis profiles.

```json
[
  {
    "id": "isuret.profile.standard_level_drilldown",
    "name": "Standard Level Drilldown",
    "description": "Identify the strongest level, follow its domains, then scan entries inside those domains.",
    "status": "ACTIVE",
    "config": {}
  }
]
```

## Local Workspace

The local application stores imported packages in SQLite.

The local workspace must distinguish between:

```text
Imported package data
Local user additions
Local user overrides
```

Foreign packages should not be edited destructively. If a user changes an imported entry, the change should be stored as a local override. This allows package updates without losing local adjustments.

## Trust and Package Selection

Different packages can have different trust levels.

Example:

```text
Isuret Core Levels       trust_level 90
Fritz Additions          trust_level 70
Unknown Internet DB      trust_level 30
```

An analysis engine may later use this value during scoring:

```text
finalScore = radionicScore * relationWeight * packageTrust
```

This allows users to import many knowledge packages while still controlling which authors and sources influence the analysis more strongly.

## Planned Technology Stack

The planned stack for the first version is:

```text
Tauri 2
React
TypeScript
SQLite
Rust backend
Zod validation
```

The application is intended to be local-first and offline-capable.

## Current Prototype Features

The first prototype should support:

```text
Initialize local SQLite database
Import ORKB package directories
List imported packages
List entries
Search entries
List relations
Export packages
```

The first prototype does not need a full analysis engine yet. Import and export must be stable before analysis logic is added.

## Planned Project Structure

```text
orkb-editor/
    src-tauri/
        src/
            main.rs
            model.rs
            db_model.rs
            db.rs
            import.rs
            export.rs
            analysis.rs

    src/
        App.tsx
        main.tsx
        style.css
        lib/
            api.ts
            types.ts
            validators.ts
        pages/
            PackagesPage.tsx
            EntriesPage.tsx
            RelationsPage.tsx
            ImportPage.tsx
            ExportPage.tsx
            AnalysisProfilesPage.tsx

    packages/
        isuret.core.levels/
            manifest.json
            entries.json
            relations.json
            profiles.json

        isuret.core.domains/
            manifest.json
            entries.json
            relations.json
            profiles.json
```

## First Core Packages

The initial core packages are:

```text
isuret.core.levels
isuret.core.domains
```

### isuret.core.levels

Defines the primary operational levels:

```text
CAPUT_MORTUM
ENTROPY
PHYSICAL
VITAL
BIOENERGETIC
ETHERIC
QUANTUM
AETHER
ASTRAL
PSYCHIC
MENTAL
ARCHETYPAL
CAUSAL
NOETIC
```

### isuret.core.domains

Defines common analysis domains:

```text
VITAMINS
MINERALS
TRACE_ELEMENTS
ORGANS
TISSUES
HOMEOPATHY
BACH_FLOWERS
EMOTIONS
BELIEFS
ARCHETYPES
SYMBOLS
CHAKRAS
ELEMENTS
PLANETS
HERBS
```

## Development Setup

Install Node.js, Rust, and the Tauri prerequisites for your operating system.

Create the project:

```bash
npm create tauri-app@latest orkb-editor
cd orkb-editor
npm install
```

Install frontend dependencies:

```bash
npm install @tauri-apps/api zod
```

Run in development mode:

```bash
npm run tauri dev
```

Build the application:

```bash
npm run tauri build
```

## Import Test

After starting the application, import the packages in this order:

```text
packages/isuret.core.levels
packages/isuret.core.domains
```

Then verify:

```text
Packages page shows both packages.
Entries page shows levels and domains.
Relations page shows next_level and contains_domain relations.
```

## Design Rules

The project follows these basic rules:

```text
Everything is an entry.
Meaning comes from relations.
Packages keep their author identity.
Imported data should not be destructively edited.
Local changes should be stored as overrides.
Analysis should follow relation paths, not hardcoded categories.
The exchange format should remain independent from the local SQL schema.
```

## Status

Early prototype.

The data model, package format, and import/export structure are still subject to change.
