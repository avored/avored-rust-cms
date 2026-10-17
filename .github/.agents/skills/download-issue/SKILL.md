---
name: download-issue
description: >-
  Downloads a GitHub issue by issue number or URL and generates a structured Markdown file
  in docs/{issuetype}/{short-issue-title}/ticket.md. Activate this skill whenever the user asks
  to download, fetch, import, or sync a GitHub issue or ticket into the repository docs.
---

# Skill: Download GitHub Issue to Markdown

## Description
Downloads a GitHub issue (using `gh` CLI or GitHub REST API) and creates a formatted Markdown file located at `docs/{issuetype}/{short-issue-title}/ticket.md`.

## Triggers & Intent
Activate this skill when:
- The user asks to download, fetch, or import a GitHub issue (e.g., `"download issue"`, `"download github issue"`, `"download issue #356"`, `"fetch ticket"`).
- The user wants an issue converted into documentation under `docs/{issuetype}/` for planning, implementation, or record-keeping.

---

## Instructions

### Step 1: Prompt for Issue Number If Not Provided
- Check whether the user provided an issue number, hash reference (e.g. `#356`), or full issue URL in their request.
- **If the issue number or URL is missing:**
  - **Stop immediately and ask the user directly for the issue number or URL.**
  - Example prompt:
    > "Which GitHub issue number or URL would you like to download?"
  - Do **not** proceed to run scripts or guess an issue number until the user provides one.

### Step 2: Extract Issue Identifier & Options
1. Parse the issue reference provided by the user (e.g., `356`, `#356`, or `https://github.com/owner/repo/issues/356`).
2. Check if the user specified an explicit issue type override (e.g., `features`, `bugs`, `improvements`, `epics`, `tech-debt`, `documentation`). If unspecified, the script automatically categorizes the issue type using labels and title conventions.
3. Check if a specific repository was requested. If not provided, it defaults to the local git repository's origin (`avored/avored-rust-cms`).

### Step 3: Run the Downloader Script
Execute the bundled Node.js script:

```bash
node .agents/skills/download-issue/scripts/download_issue.js <ISSUE_REF>
```

#### Supported Options:
- `--repo <owner/repo>`: Specify a different GitHub repository.
- `--type <type>`: Override the issue type directory (e.g., `features`, `bugs`, `improvements`, `epics`, `tech-debt`).
- `--out-dir <dir>`: Base documentation output folder (defaults to `docs`).

#### Examples:
```bash
# Download issue #356 in current repo
node .agents/skills/download-issue/scripts/download_issue.js 356

# Download issue with explicit category
node .agents/skills/download-issue/scripts/download_issue.js 356 --type features

# Download from full URL
node .agents/skills/download-issue/scripts/download_issue.js https://github.com/avored/avored-rust-cms/issues/356
```

### Step 4: Verify and Present Output
1. Verify that the file was created in `docs/{issuetype}/{short-issue-title}/ticket.md`.
2. Inspect the generated file using `view_file` to ensure markdown integrity and completeness.
3. Provide the user with a clickable markdown link to the generated document (e.g., `[ticket.md](file:///Users/purveshp/rust/avored-rust-cms/docs/features/add-admin-profile-page-edit-profile-change-password/ticket.md)`).
