#!/usr/bin/env python3
"""
Download a GitHub issue and save it as Markdown in docs/{issuetype}/{file-name-basedon-title}.md
Supports GitHub CLI (gh) if authenticated, with seamless fallback to public GitHub REST API.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request
from typing import Any, Dict, List, Optional, Tuple


def get_git_remote_repo() -> Optional[str]:
    """Extract owner/repo from local git remotes."""
    try:
        res = subprocess.run(
            ["git", "config", "--get", "remote.origin.url"],
            capture_output=True,
            text=True,
            check=False,
        )
        url = res.stdout.strip()
        if not url:
            return None
        # Match git@github.com:owner/repo.git or https://github.com/owner/repo.git
        match = re.search(r"github\.com[:/]([^/]+)/([^/\.]+)(?:\.git)?", url)
        if match:
            return f"{match.group(1)}/{match.group(2)}"
    except Exception:
        pass
    return None


def parse_issue_reference(ref: str, default_repo: Optional[str]) -> Tuple[str, int]:
    """
    Parse issue reference. Can be:
    - 356
    - #356
    - owner/repo#356
    - https://github.com/owner/repo/issues/356
    """
    url_match = re.match(r"^https?://github\.com/([^/]+)/([^/]+)/issues/(\d+)", ref.strip())
    if url_match:
        repo = f"{url_match.group(1)}/{url_match.group(2)}"
        return repo, int(url_match.group(3))

    hash_repo_match = re.match(r"^([^#]+)#(\d+)$", ref.strip())
    if hash_repo_match:
        return hash_repo_match.group(1), int(hash_repo_match.group(2))

    num_match = re.match(r"^#?(\d+)$", ref.strip())
    if num_match:
        if not default_repo:
            raise ValueError(f"Issue '{ref}' provided as a number, but repo could not be inferred.")
        return default_repo, int(num_match.group(1))

    raise ValueError(f"Unable to parse issue reference: '{ref}'")


def fetch_issue_gh(repo: str, issue_number: int) -> Optional[Dict[str, Any]]:
    """Attempt to fetch issue details using gh CLI."""
    gh_bin = None
    for path in ["gh", "/opt/homebrew/bin/gh", "/usr/local/bin/gh", "/usr/bin/gh"]:
        if os.path.exists(path) or subprocess.run(["which", path], capture_output=True).returncode == 0:
            gh_bin = path
            break

    if not gh_bin:
        return None

    try:
        cmd = [
            gh_bin,
            "issue",
            "view",
            str(issue_number),
            "--repo",
            repo,
            "--json",
            "number,title,body,state,author,labels,createdAt,updatedAt,url,assignees,comments",
        ]
        res = subprocess.run(cmd, capture_output=True, text=True, check=False)
        if res.returncode == 0 and res.stdout.strip():
            data = json.loads(res.stdout)
            # Normalize fields to match standard representation
            return {
                "number": data.get("number"),
                "title": data.get("title", ""),
                "body": data.get("body", ""),
                "state": data.get("state", "").lower(),
                "author": data.get("author", {}).get("login", "") if data.get("author") else "",
                "labels": [l.get("name") for l in data.get("labels", []) if "name" in l],
                "created_at": data.get("createdAt", ""),
                "updated_at": data.get("updatedAt", ""),
                "html_url": data.get("url", f"https://github.com/{repo}/issues/{issue_number}"),
                "assignees": [a.get("login") for a in data.get("assignees", []) if "login" in a],
                "comments_count": len(data.get("comments", [])),
            }
    except Exception:
        pass
    return None


def fetch_issue_api(repo: str, issue_number: int) -> Dict[str, Any]:
    """Fetch issue details using GitHub REST API."""
    url = f"https://api.github.com/repos/{repo}/issues/{issue_number}"
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "Antigravity-Issue-Downloader",
            "Accept": "application/vnd.github.v3+json",
        },
    )
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if token:
        req.add_header("Authorization", f"Bearer {token}")

    try:
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            return {
                "number": data.get("number"),
                "title": data.get("title", ""),
                "body": data.get("body", "") or "",
                "state": data.get("state", ""),
                "author": data.get("user", {}).get("login", "") if data.get("user") else "",
                "labels": [l.get("name") for l in data.get("labels", []) if "name" in l],
                "created_at": data.get("created_at", ""),
                "updated_at": data.get("updated_at", ""),
                "html_url": data.get("html_url", url),
                "assignees": [a.get("login") for a in data.get("assignees", []) if "login" in a],
                "comments_count": data.get("comments", 0),
            }
    except urllib.error.HTTPError as e:
        if e.code == 404:
            raise RuntimeError(f"Issue #{issue_number} not found in repository {repo}.")
        raise RuntimeError(f"GitHub API error ({e.code}): {e.reason}")
    except Exception as e:
        raise RuntimeError(f"Failed to fetch issue from GitHub API: {e}")


def determine_issue_type(labels: List[str], title: str, cli_type: Optional[str]) -> str:
    """
    Determine issuetype folder name based on explicit CLI flag, issue labels, or title conventions.
    Priority order:
    1. CLI argument if passed
    2. Exact label matches or label prefixes
    3. Title prefixes (e.g. [Feature], Bug Report:, feat:, fix:)
    4. Default fallback: 'general'
    """
    if cli_type:
        return slugify(cli_type)

    lower_labels = [l.lower() for l in labels]
    lower_title = title.lower()

    # Label checks
    if any("bug" in l for l in lower_labels):
        return "bugs"
    if any("epic" in l for l in lower_labels) or lower_title.startswith("epic:"):
        return "epics"
    if any("improvement" in l or "improvements" in l for l in lower_labels):
        return "improvements"
    if any("feature" in l or "enhancement" in l for l in lower_labels):
        return "features"
    if any("tech debt" in l or "refactor" in l for l in lower_labels):
        return "tech-debt"
    if any("documentation" in l or "doc" in l for l in lower_labels):
        return "documentation"

    # Title prefix checks
    if lower_title.startswith("bug:") or "bug report" in lower_title:
        return "bugs"
    if lower_title.startswith("[feature]") or lower_title.startswith("feat:") or lower_title.startswith("feat("):
        return "features"
    if lower_title.startswith("fix:") or lower_title.startswith("fix("):
        return "bugs"
    if lower_title.startswith("docs:") or lower_title.startswith("docs("):
        return "documentation"
    if lower_title.startswith("refactor:") or lower_title.startswith("chore:"):
        return "improvements"

    return "general"


def slugify(text: str) -> str:
    """Convert a title/text string into a clean, kebab-cased filename slug."""
    # Remove prefix tags like [Feature], [Bug], feat(admin): etc.
    cleaned = re.sub(r"^\[[^\]]+\]\s*", "", text)
    cleaned = re.sub(r"^[a-zA-Z0-9_\-\(\)]+:\s*", "", cleaned)
    # Remove non-alphanumeric characters (keep hyphens and spaces)
    cleaned = re.sub(r"[^\w\s-]", "", cleaned).strip().lower()
    # Replace spaces and consecutive hyphens with a single hyphen
    cleaned = re.sub(r"[-\s]+", "-", cleaned)
    return cleaned or "issue"


def generate_markdown(issue: Dict[str, Any], issuetype: str) -> str:
    """Format issue data into standard GitHub-flavored Markdown."""
    title = issue.get("title", "")
    num = issue.get("number", "")
    url = issue.get("html_url", "")
    state = issue.get("state", "").upper()
    author = issue.get("author", "unknown")
    created = issue.get("created_at", "")
    updated = issue.get("updated_at", "")
    labels = issue.get("labels", [])
    assignees = issue.get("assignees", [])
    body = (issue.get("body") or "").strip()

    labels_str = ", ".join(f"`{l}`" for l in labels) if labels else "None"
    assignees_str = ", ".join(f"@{a}" for a in assignees) if assignees else "None"

    lines = [
        f"# #{num}: {title}",
        "",
        "> **Issue Metadata**",
        f"> - **URL:** [{url}]({url})",
        f"> - **Type:** `{issuetype}`",
        f"> - **State:** `{state}`",
        f"> - **Author:** @{author}",
        f"> - **Assignees:** {assignees_str}",
        f"> - **Labels:** {labels_str}",
        f"> - **Created:** {created}",
        f"> - **Updated:** {updated}",
        "",
        "---",
        "",
        "## Description",
        "",
        body if body else "_No description provided._",
        "",
    ]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="Download a GitHub issue to docs/{issuetype}/{file-name-basedon-title}.md")
    parser.add_argument("issue", nargs="?", default=None, help="Issue reference: number (e.g. 356 or #356) or full URL/repo reference")
    parser.add_argument("--repo", "-r", help="Repository in 'owner/repo' format (defaults to git origin)")
    parser.add_argument("--type", "-t", help="Override issue type directory name (e.g. features, bugs, improvements)")
    parser.add_argument("--out-dir", "-o", default="docs", help="Base output directory (default: docs)")

    args = parser.parse_args()

    issue_input = args.issue
    if not issue_input:
        try:
            issue_input = input("Enter GitHub Issue number or URL: ").strip()
        except (KeyboardInterrupt, EOFError):
            print("\nAborted.")
            return 1
        if not issue_input:
            print("Error: No issue specified.")
            return 1

    default_repo = args.repo or get_git_remote_repo()
    repo, issue_number = parse_issue_reference(issue_input, default_repo)

    print(f"Fetching issue #{issue_number} from {repo}...")
    issue = fetch_issue_gh(repo, issue_number)
    if not issue:
        issue = fetch_issue_api(repo, issue_number)

    issuetype = determine_issue_type(issue.get("labels", []), issue.get("title", ""), args.type)
    slug = slugify(issue.get("title", ""))
    filename = f"{issue_number}-{slug}.md"

    target_dir = os.path.join(args.out_dir, issuetype)
    os.makedirs(target_dir, exist_ok=True)
    target_path = os.path.join(target_dir, filename)

    content = generate_markdown(issue, issuetype)
    with open(target_path, "w", encoding="utf-8") as f:
        f.write(content)

    print(f"Successfully saved issue #{issue_number} to {target_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
