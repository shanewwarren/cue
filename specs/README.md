# Cue Specifications

Design documentation for a CLI tool that plays categorized audio cues from a sound library.

## Overview

This directory contains specifications for the project's features and systems. Each spec describes the design intent, architecture, and implementation guidance for a specific concern.

**Status Legend:**
- **Planned** - Design complete, not yet implemented
- **In Progress** - Currently being implemented
- **Implemented** - Feature complete and in production

---

## Core Features

| Spec | Status | Purpose |
|------|--------|---------|
| [sound-archive.md](./sound-archive.md) | Implemented | Discovering and indexing the sounds directory structure |
| [playback.md](./playback.md) | Implemented | Playing sound files with volume control |
| [cli-interface.md](./cli-interface.md) | Implemented | Commands: play, list, preview specific sounds |
| [configuration.md](./configuration.md) | Implemented | Config file with environment variable override |

## Playback Control

| Spec | Status | Purpose |
|------|--------|---------|
| [playback-suppression.md](./playback-suppression.md) | Implemented | Suppress sounds when meeting apps are running |

---

## Using These Specs

### For Implementers

1. **Read the spec first** before writing code
2. **Check existing code** - specs describe intent, code describes reality
3. **Follow the patterns** outlined in each spec's Architecture section
4. **Update status** when implementation begins/completes

### Updating Specs

Specs are living documents. Update them when:
- Implementation reveals a better approach
- Requirements change
- New edge cases are discovered

---

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - Project-level AI guidance
