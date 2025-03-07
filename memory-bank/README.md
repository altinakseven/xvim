# Cline's Memory Bank

This directory contains Cline's Memory Bank - a structured documentation system that enables Cline to maintain continuity and project knowledge across sessions.

## Purpose

Cline's memory resets completely between sessions. The Memory Bank provides a comprehensive documentation system that allows Cline to quickly understand the project context, current status, and next steps when starting a new session.

## Core Files

The Memory Bank consists of the following core files, organized in a hierarchical structure:

1. **projectbrief.md**
   - Foundation document that shapes all other files
   - Defines core requirements and goals
   - Source of truth for project scope

2. **productContext.md**
   - Why this project exists
   - Problems it solves
   - How it should work
   - User experience goals

3. **systemPatterns.md**
   - System architecture
   - Key technical decisions
   - Design patterns in use
   - Component relationships

4. **techContext.md**
   - Technologies used
   - Development setup
   - Technical constraints
   - Dependencies

5. **activeContext.md**
   - Current work focus
   - Recent changes
   - Next steps
   - Active decisions and considerations

6. **progress.md**
   - What works
   - What's left to build
   - Current status
   - Known issues

## File Hierarchy

The files build upon each other in a clear hierarchy:

```
projectbrief.md → productContext.md, systemPatterns.md, techContext.md → activeContext.md → progress.md
```

## Additional Context

Additional files and folders may be created within the memory-bank directory as needed to organize:
- Complex feature documentation
- Integration specifications
- API documentation
- Testing strategies
- Deployment procedures

## Usage

When working with Cline, you can request updates to the Memory Bank by using the phrase **"update memory bank"**. This will trigger Cline to review all Memory Bank files and update them based on the current project state.

## Project Rules

The `.clinerules` file at the root of the project captures important patterns, preferences, and project intelligence that help Cline work more effectively. This file is updated as the project evolves to reflect emerging patterns and lessons learned.