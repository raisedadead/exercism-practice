# GitHub Actions Workflows

## Auto Merge PRs

The `auto-merge.yml` workflow automatically merges pull requests using the rebase strategy.

### How it works

1. **Trigger**: The workflow runs when a PR is opened, synchronized, reopened, or marked as ready for review, or when a review is submitted.

2. **Conditions**: The workflow only runs for PRs that are:
   - Not in draft mode
   - Created by `raisedadead` or `renovate[bot]`

3. **Action**: Enables GitHub's auto-merge feature with the rebase strategy using `gh pr merge --rebase --auto`

### Merge Strategy

The workflow uses the **rebase and merge** strategy, which:
- Rebases the PR commits onto the base branch
- Maintains a linear commit history
- Does not create merge commits

### Handling Merge Conflicts

**Important**: This workflow cannot automatically resolve merge conflicts. If a PR has conflicts:
- The auto-merge will be enabled but won't execute until conflicts are resolved
- You must manually resolve conflicts by:
  1. Pulling the latest changes from the base branch
  2. Rebasing your branch: `git rebase main`
  3. Resolving conflicts
  4. Force pushing: `git push --force-with-lease`
- Once conflicts are resolved, the auto-merge will proceed automatically

### Merge Queue

After researching GitHub's merge queue feature, we determined it's not necessary for this repository because:
- Merge queues require branch protection rules and status checks
- This repository has no CI/CD pipelines or required checks
- The primary purpose is to save Exercism solutions without extensive validation
- The simple auto-merge with rebase strategy is sufficient

### Manual Override

If you need to disable auto-merge for a specific PR:
```bash
gh pr merge --disable-auto <PR_NUMBER>
```

Or through the GitHub UI: Click "Disable auto-merge" on the PR page.
