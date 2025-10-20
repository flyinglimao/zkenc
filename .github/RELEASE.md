# Release Process

## Setup

To enable automatic npm publishing, you need to add an NPM_TOKEN secret to your GitHub repository:

1. Go to npmjs.com and log in
2. Click on your profile → Access Tokens
3. Generate New Token → Choose "Automation" type
4. Copy the token
5. Go to GitHub repository → Settings → Secrets and variables → Actions
6. Click "New repository secret"
7. Name: `NPM_TOKEN`
8. Value: paste your npm token
9. Click "Add secret"

## Creating a Release

To create a new release and publish to npm:

1. Update the version in `packages/zkenc-js/package.json`
2. Commit the version change:
   ```bash
   git add packages/zkenc-js/package.json
   git commit -m "chore: bump version to x.y.z"
   ```
3. Create and push a git tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

The GitHub Action will automatically:

- Build zkenc-js
- Publish to npm
- Create a GitHub release
- Wait for npm package to be available
- Build and deploy documentation using the published package

## Workflow Details

### Release Workflow (`.github/workflows/release.yml`)

Triggers on tags matching `v*.*.*`

- Builds and publishes zkenc-js to npm
- Creates GitHub release
- Deploys documentation to GitHub Pages

### Docs Deployment Workflow (`.github/workflows/deploy-docs.yml`)

Triggers on push to main branch (docs changes)

- Builds zkenc-js locally
- Builds and deploys documentation

## Version Management

The package follows semantic versioning:

- `v0.1.0` - Initial release
- `v0.1.1` - Patch release (bug fixes)
- `v0.2.0` - Minor release (new features, backwards compatible)
- `v1.0.0` - Major release (breaking changes)
