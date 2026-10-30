#!/usr/bin/env node
/**
 * Download a GitHub issue and save it as Markdown in docs/{issuetype}/{short-issue-title}/ticket.md
 * Supports GitHub CLI (gh) if authenticated, with seamless fallback to public GitHub REST API.
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const readline = require('readline');

function getGitRemoteRepo() {
  try {
    const url = execSync('git config --get remote.origin.url', { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'ignore'] }).trim();
    if (!url) return null;
    const match = url.match(/github\.com[:/]([^/]+)\/([^/.]+)(?:\.git)?/);
    if (match) {
      return `${match[1]}/${match[2]}`;
    }
  } catch {
    // Ignore error
  }
  return null;
}

function parseIssueReference(ref, defaultRepo) {
  const trimmed = ref.trim();

  // Full URL
  const urlMatch = trimmed.match(/^https?:\/\/github\.com\/([^/]+)\/([^/]+)\/issues\/(\d+)/);
  if (urlMatch) {
    return { repo: `${urlMatch[1]}/${urlMatch[2]}`, issueNumber: parseInt(urlMatch[3], 10) };
  }

  // owner/repo#123
  const hashRepoMatch = trimmed.match(/^([^#]+)#(\d+)$/);
  if (hashRepoMatch) {
    return { repo: hashRepoMatch[1], issueNumber: parseInt(hashRepoMatch[2], 10) };
  }

  // #123 or 123
  const numMatch = trimmed.match(/^#?(\d+)$/);
  if (numMatch) {
    if (!defaultRepo) {
      throw new Error(`Issue '${ref}' provided as a number, but repo could not be inferred.`);
    }
    return { repo: defaultRepo, issueNumber: parseInt(numMatch[1], 10) };
  }

  throw new Error(`Unable to parse issue reference: '${ref}'`);
}

function fetchIssueGh(repo, issueNumber) {
  let ghBin = null;
  const candidates = ['gh', '/opt/homebrew/bin/gh', '/usr/local/bin/gh', '/usr/bin/gh'];

  for (const candidate of candidates) {
    try {
      if (fs.existsSync(candidate)) {
        ghBin = candidate;
        break;
      }
      execSync(`which ${candidate}`, { stdio: 'ignore' });
      ghBin = candidate;
      break;
    } catch {
      // continue
    }
  }

  if (!ghBin) return null;

  try {
    const stdout = execSync(
      `"${ghBin}" issue view "${issueNumber}" --repo "${repo}" --json number,title,body,state,author,labels,createdAt,updatedAt,url,assignees,comments`,
      { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'ignore'] }
    );
    if (!stdout.trim()) return null;

    const data = JSON.parse(stdout);
    return {
      number: data.number,
      title: data.title || '',
      body: data.body || '',
      state: (data.state || '').toLowerCase(),
      author: data.author ? data.author.login : '',
      labels: (data.labels || []).map((l) => (typeof l === 'string' ? l : l.name)).filter(Boolean),
      createdAt: data.createdAt || '',
      updatedAt: data.updatedAt || '',
      htmlUrl: data.url || `https://github.com/${repo}/issues/${issueNumber}`,
      assignees: (data.assignees || []).map((a) => (typeof a === 'string' ? a : a.login)).filter(Boolean),
      commentsCount: (data.comments || []).length,
    };
  } catch {
    return null;
  }
}

async function fetchIssueApi(repo, issueNumber) {
  const url = `https://api.github.com/repos/{repo}/issues/${issueNumber}`.replace('{repo}', repo);
  const headers = {
    'User-Agent': 'Antigravity-Issue-Downloader-Node',
    Accept: 'application/vnd.github.v3+json',
  };
  const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }

  const response = await fetch(url, { headers });
  if (response.status === 404) {
    throw new Error(`Issue #${issueNumber} not found in repository ${repo}.`);
  }
  if (!response.ok) {
    throw new Error(`GitHub API error (${response.status}): ${response.statusText}`);
  }

  const data = await response.json();
  return {
    number: data.number,
    title: data.title || '',
    body: data.body || '',
    state: data.state || '',
    author: data.user ? data.user.login : '',
    labels: (data.labels || []).map((l) => (typeof l === 'string' ? l : l.name)).filter(Boolean),
    createdAt: data.created_at || '',
    updatedAt: data.updated_at || '',
    htmlUrl: data.html_url || url,
    assignees: (data.assignees || []).map((a) => (typeof a === 'string' ? a : a.login)).filter(Boolean),
    commentsCount: data.comments || 0,
  };
}

function slugify(text) {
  // Remove prefix tags like [Feature], [Bug], feat(admin): etc.
  let cleaned = text.replace(/^\[[^\]]+\]\s*/, '');
  cleaned = cleaned.replace(/^[a-zA-Z0-9_\-()]+\s*:\s*/, '');
  // Remove non-alphanumeric characters (keep hyphens and spaces)
  cleaned = cleaned.replace(/[^\w\s-]/g, '').trim().toLowerCase();
  // Replace spaces and consecutive hyphens with a single hyphen
  cleaned = cleaned.replace(/[-\s]+/g, '-');
  return cleaned || 'issue';
}

function determineIssueType(labels, title, cliType) {
  if (cliType) {
    return slugify(cliType);
  }

  const lowerLabels = labels.map((l) => l.toLowerCase());
  const lowerTitle = title.toLowerCase();

  // Label checks
  if (lowerLabels.some((l) => l.includes('feature') || l.includes('enhancement'))) return 'features';
  if (lowerLabels.some((l) => l.includes('bug'))) return 'bugs';
  if (lowerLabels.some((l) => l.includes('epic')) || lowerTitle.startsWith('epic:')) return 'epics';
  if (lowerLabels.some((l) => l.includes('improvement') || l.includes('improvements'))) return 'improvements';
  if (lowerLabels.some((l) => l.includes('tech debt') || l.includes('refactor'))) return 'tech-debt';
  if (lowerLabels.some((l) => l.includes('documentation') || l.includes('doc'))) return 'documentation';

  // Title prefix checks
  if (lowerTitle.startsWith('bug:') || lowerTitle.includes('bug report')) return 'bugs';
  if (lowerTitle.startsWith('[feature]') || lowerTitle.startsWith('feat:') || lowerTitle.startsWith('feat(')) return 'features';
  if (lowerTitle.startsWith('fix:') || lowerTitle.startsWith('fix(')) return 'bugs';
  if (lowerTitle.startsWith('docs:') || lowerTitle.startsWith('docs(')) return 'documentation';
  if (lowerTitle.startsWith('refactor:') || lowerTitle.startsWith('chore:')) return 'improvements';

  return 'features';
}

function generateMarkdown(issue, issuetype) {
  const title = issue.title || '';
  const num = issue.number || '';
  const url = issue.htmlUrl || '';
  const state = (issue.state || '').toUpperCase();
  const author = issue.author || 'unknown';
  const created = issue.createdAt || '';
  const updated = issue.updatedAt || '';
  const labels = issue.labels || [];
  const assignees = issue.assignees || [];
  const body = (issue.body || '').trim();

  const labelsStr = labels.length > 0 ? labels.map((l) => `\`${l}\``).join(', ') : 'None';
  const assigneesStr = assignees.length > 0 ? assignees.map((a) => `@${a}`).join(', ') : 'None';
  const templatePath = path.join(__dirname, '..', 'templates', 'ticket-template.md');
  const template = fs.readFileSync(templatePath, 'utf-8');
  const values = {
    number: num,
    title,
    url,
    issuetype,
    state,
    author,
    assignees: assigneesStr,
    labels: labelsStr,
    created,
    updated,
    description: body || '_No description provided._',
  };

  return template
    .replace(/\{\{(\w+)\}\}/g, (placeholder, key) => {
      if (!Object.prototype.hasOwnProperty.call(values, key)) {
        throw new Error(`Unknown ticket template placeholder: ${placeholder}`);
      }
      return values[key];
    })
    .trimEnd() + '\n';
}

function promptInput(query) {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });
  return new Promise((resolve) => {
    rl.question(query, (ans) => {
      rl.close();
      resolve(ans.trim());
    });
  });
}

function parseCliArgs() {
  const args = process.argv.slice(2);
  const options = {
    issue: null,
    repo: null,
    type: null,
    outDir: 'docs',
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === '--repo' || arg === '-r') {
      options.repo = args[++i];
    } else if (arg === '--type' || arg === '-t') {
      options.type = args[++i];
    } else if (arg === '--out-dir' || arg === '-o') {
      options.outDir = args[++i];
    } else if (!arg.startsWith('-') && !options.issue) {
      options.issue = arg;
    }
  }

  return options;
}

async function main() {
  try {
    const options = parseCliArgs();

    let issueInput = options.issue;
    if (!issueInput) {
      issueInput = await promptInput('Enter GitHub Issue number or URL: ');
      if (!issueInput) {
        console.error('Error: No issue specified.');
        process.exit(1);
      }
    }

    const defaultRepo = options.repo || getGitRemoteRepo();
    const { repo, issueNumber } = parseIssueReference(issueInput, defaultRepo);

    console.log(`Fetching issue #${issueNumber} from ${repo}...`);
    let issue = fetchIssueGh(repo, issueNumber);
    if (!issue) {
      issue = await fetchIssueApi(repo, issueNumber);
    }

    const issuetype = determineIssueType(issue.labels, issue.title, options.type);
    const slug = slugify(issue.title);
    const folderName = slug;
    const filename = 'ticket.md';

    const targetDir = path.join(process.cwd(), options.outDir, issuetype, folderName);
    fs.mkdirSync(targetDir, { recursive: true });
    const targetPath = path.join(targetDir, filename);

    const content = generateMarkdown(issue, issuetype);
    fs.writeFileSync(targetPath, content, 'utf-8');

    console.log(`Successfully saved issue #${issueNumber} to ${targetPath}`);
  } catch (err) {
    console.error(`Error: ${err.message}`);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

module.exports = { generateMarkdown };
