---
name: create-branch-from-ticket
description: Create a Git branch from a ticket.md issue title.
argument-hint: "[path/to/ticket.md]"
---

# Create a Branch from a Ticket

1. Read the supplied or attached `ticket.md`. If neither is available, ask for
   its path. Extract the issue title from the ticket heading.
2. Check that the worktree is clean. If not, stop and report the changes.
3. Switch to `main`, then pull the latest changes:
   `git switch main` followed by `git pull`.
4. Make a branch name by converting the issue title to lowercase kebab-case.
   Create it from updated `main` with `git switch -c <title-slug>`.
5. Verify the current branch and report its name. If switching, pulling, or
   branch creation fails (including because the branch already exists), report
   the error and stop.
