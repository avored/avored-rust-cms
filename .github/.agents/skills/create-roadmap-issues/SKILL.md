---
name: create-roadmap-issues
description: >-
  Create GitHub issues from an attached roadmap or the repository's
  docs/roadmap/improvement-and-tech-debt-roadmap.md. Use when asked to turn
  roadmap items into GitHub issues, especially this project's I-001–I-020
  improvement and tech-debt roadmap.
---

# Create GitHub Issues from the Improvement Roadmap

Turn each numbered roadmap item into a separate GitHub issue in
`avored/avored-rust-cms`, preserving its scope, user story, acceptance criteria,
technical notes, and task checklist. Apply only the labels, GitHub issue type,
and project the user selects.

## Workflow

1. **Choose the source.** Use the roadmap attached or named by the user. If the
   request refers to `improvement-and-tech-debt-roadmap.md` but no attachment is
   available, use
   `docs/roadmap/improvement-and-tech-debt-roadmap.md`. If the source cannot be
   read, stop and report that clearly.
2. **Load the issue payloads.** For the repository roadmap, use
   `./roadmap-issues.json` as the saved payload set and compare its source path,
   SHA-256, and issue IDs with the Markdown roadmap. If the JSON is missing or
   stale, rebuild it from the complete Markdown sections before publishing. For
   another attachment, extract each numbered section into the same JSON shape
   and save it beside the source unless the user requests a different location.
   Each payload contains the issue ID, title, body, roadmap category, priority,
   phase, dependencies, `labels`, `githubIssueType`, and `project`. Cross-check
   IDs against the roadmap index and dependency graph. For the current roadmap,
   expect I-001 through I-020; report missing, duplicate, or conflicting items
   rather than silently omitting or inventing content.
3. **Verify the destination.** Default to `avored/avored-rust-cms`, as
   specified by this roadmap, unless the user explicitly names another
   repository. The issue-creation script uses the GitHub REST API and requires
   `GITHUB_TOKEN` or `GH_TOKEN` with permission to create issues. Confirm that
   a token is available and the repository is accessible before preparing
   publication; never ask the user to paste a token into chat.
4. **Load and choose metadata.** Read
   [GitHub metadata](./github-metadata.json) for the latest saved label, issue
   type, and project lists. Before publishing, refresh the lists from GitHub
   when authentication and network access are available, and update the JSON
   snapshot. The current snapshot's labels are complete; its issue types are
   only those observed on public issues, and its project list could not be
   retrieved without authentication. Treat incomplete lists as suggestions,
   not authoritative: verify selected values against GitHub and ask the user
   to authenticate if necessary. Ask which exact labels, GitHub issue type,
   and project to apply. They may select multiple labels and leave fields
   unset. Ask whether selections apply to every new issue or vary by roadmap
   ID. Store selections in each corresponding issue payload and verify all
   selected values before publishing. Never infer GitHub metadata from
   roadmap categories, priorities, or phases.
5. **Check for duplicates.** Search all issue states in the destination for
   each roadmap ID in the title, using the stable title marker `[I-001]` (and
   corresponding marker for each item). Reuse existing matching issue numbers
   when resolving dependencies, and do not create a second issue for an ID
   already represented. If a search result is ambiguous, show it to the user
   and ask before proceeding.
6. **Prepare the issue set.** Create one issue per roadmap item not already
   present. Prefix each source title with its stable ID, for example:
   `[I-001] [Tech Debt]: Remove all unwrap() calls from production code`.
   Use the roadmap section as the issue body, preserving its Markdown and
   requirements. Keep the priority, phase, and dependency fields. Resolve every
   dependency ID to an existing or newly created GitHub issue number, replacing
   `#I-NNN` references in the issue body with the corresponding `#<number>`.
   Keep `None` unchanged. Include each selected label with a separate
   `--label <name>` flag, the selected issue type with `--type <name>`, and the
   selected project with `--project <title>`. Omit flags for fields the user
   left unset.
7. **Get approval before publishing.** Present the destination, the complete
   list of proposed titles, the labels/type/project that will be applied (and
   any per-issue overrides), existing issues that will be reused, and the
   number of new issues to create. Ask the user to approve the batch. Do not
   create issues until the user explicitly approves.
8. **Create in dependency order.** After approval, create each new issue with
   the [GitHub REST API script](./scripts/create_github_issue.js), one issue at
   a time. Pass `--repo`, `--title`, `--body-file`, repeat `--label` for each
   selected label, and pass `--type` or `--project` when selected. Supply the
   Markdown body through a temporary UTF-8 file or standard input using
   `--body-file -`. Create prerequisites before dependents and replace
   dependency IDs in the dependent issue body with real GitHub issue numbers.
   Reuse existing issues from the duplicate check. The script verifies whether
   GitHub applied requested labels and issue type; if metadata or project
   association fails after creation, it prints the created issue and an error.
   Record that issue before retrying so it is not duplicated. Project
   association uses GraphQL and may require additional token permissions.
9. **Report the outcome.** For every roadmap ID, provide its GitHub issue URL
   and number, or state that it was already present. If creation stops or fails
   partway through, report exactly which issues were created and which remain;
   never report the batch as complete unless every requested issue is accounted
   for. Save the final selected metadata and issue URLs/numbers back into the
   JSON file. On a later retry, repeat the duplicate check and continue without
   duplicating already-created issues.

## Roadmap section boundaries

For the current Markdown roadmap, each issue body starts at its `### I-NNN:`
heading and ends immediately before the next `### I-NNN:` heading or the
roadmap's `## Dependency Graph`, whichever comes first. Use the summary index
and dependency graph to validate extraction, not as substitutes for the full
issue sections.

## JSON payload format

Keep the roadmap export as valid JSON with this shape:

```json
{
  "repository": "avored/avored-rust-cms",
  "source": "docs/roadmap/improvement-and-tech-debt-roadmap.md",
  "sourceSha256": "SHA-256 of the source Markdown",
  "issues": [
    {
      "id": "I-001",
      "title": "[I-001] [Tech Debt]: Example title",
      "roadmapCategory": "Tech Debt",
      "priority": "High",
      "phase": "Phase 1 (Code Quality)",
      "dependencies": [],
      "body": "Full Markdown issue body",
      "labels": [],
      "githubIssueType": null,
      "project": null,
      "githubIssueNumber": null,
      "githubIssueUrl": null
    }
  ]
}
```

Keep `body` as the full extracted Markdown section, excluding its `### I-NNN:`
heading. Use JSON `null` for unset single-value metadata and `[]` for no
selected labels or dependencies. Store completed issue numbers and URLs on
their matching payloads.
