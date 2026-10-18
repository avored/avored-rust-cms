#!/usr/bin/env node

const fs = require('node:fs');

const API_URL = 'https://api.github.com';
const API_VERSION = '2026-03-10';
const DEFAULT_REPOSITORY = 'avored/avored-rust-cms';

function usage() {
  return `Create a GitHub issue using the GitHub REST API.

Usage:
  GITHUB_TOKEN=<token> node create_github_issue.js --title <title> --body-file <file> [options]

Options:
  --repo <owner/repo>  Repository (default: ${DEFAULT_REPOSITORY})
  --title <title>      Issue title (required)
  --body <text>        Issue body (use --body-file for multiline content)
  --body-file <file>   Read the issue body from a UTF-8 file ('-' reads stdin)
  --label <name>       Add a label; repeat this option for multiple labels
  --type <name>        Set the GitHub issue type
  --project <title>    Add the issue to an owner project by exact title
  --help               Show this help

Set GITHUB_TOKEN or GH_TOKEN to a token with permission to create issues.
`;
}

function parseArgs(args) {
  const options = {
    repo: DEFAULT_REPOSITORY,
    title: null,
    body: null,
    bodyFile: null,
    labels: [],
    type: null,
    project: null,
    help: false,
  };

  for (let index = 0; index < args.length; index += 1) {
    const argument = args[index];
    const next = args[index + 1];

    if (argument === '--help' || argument === '-h') {
      options.help = true;
      continue;
    }

    if (!next || next.startsWith('--')) {
      throw new Error(`Missing value for ${argument}`);
    }

    switch (argument) {
      case '--repo':
        options.repo = next;
        break;
      case '--title':
        options.title = next;
        break;
      case '--body':
        options.body = next;
        break;
      case '--body-file':
        options.bodyFile = next;
        break;
      case '--label':
        options.labels.push(next);
        break;
      case '--type':
        options.type = next;
        break;
      case '--project':
        options.project = next;
        break;
      default:
        throw new Error(`Unknown option: ${argument}`);
    }

    index += 1;
  }

  return options;
}

function validateOptions(options) {
  if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(options.repo)) {
    throw new Error(`Invalid repository '${options.repo}'; expected owner/repo.`);
  }
  if (!options.title || !options.title.trim()) {
    throw new Error('An issue title is required.');
  }
  if (options.body !== null && options.bodyFile !== null) {
    throw new Error('Choose either --body or --body-file, not both.');
  }
  if (options.body === null && options.bodyFile === null) {
    throw new Error('Provide an issue body with --body or --body-file.');
  }
  if (options.labels.some(label => !label.trim())) {
    throw new Error('Labels must not be empty.');
  }
}

async function requestJson(fetchImpl, url, token, options = {}) {
  const response = await fetchImpl(url, {
    method: options.method || 'GET',
    headers: {
      Accept: 'application/vnd.github+json',
      Authorization: `Bearer ${token}`,
      'X-GitHub-Api-Version': API_VERSION,
      'Content-Type': 'application/json',
    },
    ...(options.body === undefined ? {} : { body: JSON.stringify(options.body) }),
  });

  const responseText = await response.text();
  let responseBody = null;
  if (responseText) {
    try {
      responseBody = JSON.parse(responseText);
    } catch {
      throw new Error(`GitHub returned an invalid JSON response (${response.status}).`);
    }
  }

  if (!response.ok) {
    const details = responseBody?.message || response.statusText || 'Unknown API error';
    const errors = Array.isArray(responseBody?.errors)
      ? ` ${JSON.stringify(responseBody.errors)}`
      : '';
    throw new Error(`GitHub API request failed (${response.status}): ${details}${errors}`);
  }

  return responseBody;
}

async function addToProject(fetchImpl, token, owner, repo, issueNodeId, projectTitle) {
  const projectQuery = `query($owner: String!, $repo: String!, $after: String) {
    repository(owner: $owner, name: $repo) {
      owner {
        ... on Organization {
          projectsV2(first: 100, after: $after) {
            nodes { id title url }
            pageInfo { hasNextPage endCursor }
          }
        }
        ... on User {
          projectsV2(first: 100, after: $after) {
            nodes { id title url }
            pageInfo { hasNextPage endCursor }
          }
        }
      }
    }
  }`;
  const projects = [];
  let cursor = null;

  do {
    const result = await requestJson(fetchImpl, `${API_URL}/graphql`, token, {
      method: 'POST',
      body: {
        query: projectQuery,
        variables: { owner, repo, after: cursor },
      },
    });
    if (result.errors?.length) {
      throw new Error(`Could not list owner projects: ${result.errors.map(error => error.message).join('; ')}`);
    }

    const connection = result.data?.repository?.owner?.projectsV2;
    projects.push(...(connection?.nodes || []));
    const pageInfo = connection?.pageInfo;
    cursor = pageInfo?.hasNextPage ? pageInfo.endCursor : null;
  } while (cursor);

  const matches = projects.filter(project => project.title === projectTitle);
  if (matches.length !== 1) {
    const explanation = matches.length
      ? `Project title '${projectTitle}' is ambiguous for owner '${owner}'.`
      : `Project '${projectTitle}' was not found for owner '${owner}'.`;
    throw new Error(explanation);
  }

  const mutation = `mutation($projectId: ID!, $contentId: ID!) {
    addProjectV2ItemById(input: { projectId: $projectId, contentId: $contentId }) {
      item { id }
    }
  }`;
  const result = await requestJson(fetchImpl, `${API_URL}/graphql`, token, {
    method: 'POST',
    body: {
      query: mutation,
      variables: { projectId: matches[0].id, contentId: issueNodeId },
    },
  });
  if (result.errors?.length || !result.data?.addProjectV2ItemById?.item?.id) {
    const details = result.errors?.map(error => error.message).join('; ') || 'No project item was returned.';
    throw new Error(`Issue was created but could not be added to project '${projectTitle}': ${details}`);
  }

  return matches[0];
}

async function createIssue(options, { token, fetchImpl = fetch }) {
  validateOptions(options);
  if (!token) {
    throw new Error('Set GITHUB_TOKEN or GH_TOKEN to a GitHub token with issue creation permission.');
  }

  const body = options.bodyFile === null
    ? options.body
    : fs.readFileSync(options.bodyFile === '-' ? 0 : options.bodyFile, 'utf8');
  const [owner, repo] = options.repo.split('/');
  const payload = { title: options.title, body };
  if (options.labels.length) payload.labels = options.labels;
  if (options.type) payload.type = options.type;

  const issue = await requestJson(
    fetchImpl,
    `${API_URL}/repos/${encodeURIComponent(owner)}/${encodeURIComponent(repo)}/issues`,
    token,
    { method: 'POST', body: payload },
  );
  if (!Number.isInteger(issue?.number) || !issue.html_url || !issue.title) {
    throw new Error('GitHub created an issue but returned an incomplete issue record.');
  }

  const result = {
    number: issue.number,
    title: issue.title,
    url: issue.html_url,
    labels: (issue.labels || []).map(label => label.name),
    type: issue.type?.name || null,
    project: null,
  };
  const metadataWarnings = [];

  for (const requestedLabel of options.labels) {
    if (!result.labels.includes(requestedLabel)) {
      metadataWarnings.push(`Requested label '${requestedLabel}' was not applied.`);
    }
  }
  if (options.type && result.type !== options.type) {
    metadataWarnings.push(`Requested issue type '${options.type}' was not applied.`);
  }

  if (options.project) {
    try {
      const project = await addToProject(fetchImpl, token, owner, repo, issue.node_id, options.project);
      result.project = { title: project.title, url: project.url };
    } catch (error) {
      metadataWarnings.push(error.message);
    }
  }

  return { issue: result, metadataWarnings };
}

async function main() {
  let options;
  try {
    options = parseArgs(process.argv.slice(2));
    if (options.help) {
      process.stdout.write(usage());
      return;
    }

    const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;
    const result = await createIssue(options, { token });
    process.stdout.write(`${JSON.stringify(result.issue, null, 2)}\n`);
    if (result.metadataWarnings.length) {
      for (const warning of result.metadataWarnings) {
        process.stderr.write(`Warning: ${warning}\n`);
      }
      process.stderr.write('The issue was created; inspect it before retrying to avoid duplicates.\n');
      process.exitCode = 1;
    }
  } catch (error) {
    process.stderr.write(`Error: ${error.message}\n`);
    process.exitCode = 1;
  }
}

if (require.main === module) {
  main();
}

module.exports = { addToProject, createIssue, parseArgs, validateOptions };
