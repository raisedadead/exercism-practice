# GitHub Actions Workflows

## Auto Merge PRs

The `auto-merge.yml` workflow automatically merges pull requests using the rebase strategy.

## Keep PR Branches Updated

The `keep-prs-updated.yml` workflow runs twice daily to automatically update PR branches that are behind the base branch. This helps prevent merge conflicts by keeping branches current.

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

---

## Keep PR Branches Updated

The `keep-prs-updated.yml` workflow helps prevent merge conflicts by proactively updating PR branches.

### How it works

1. **Schedule**: Runs automatically twice daily (midnight and noon UTC)
2. **Manual trigger**: Can also be triggered manually via GitHub Actions UI
3. **Process**:
   - Finds all open PRs from trusted users (raisedadead, renovate[bot])
   - Checks if each PR branch is behind the base branch
   - Attempts to rebase branches that are behind but still mergeable
   - Pushes updated branches automatically

### Benefits

- **Prevents conflicts**: By keeping branches up to date, reduces the likelihood of merge conflicts
- **Automatic**: Runs without manual intervention
- **Safe**: Only updates branches that can be cleanly rebased (no conflicts)

### When it doesn't update

The workflow will skip updating a PR if:
- The branch has merge conflicts
- The PR is in draft mode
- The PR is not from a trusted user
- The branch is already up to date

### Manual update

You can also manually trigger this workflow:
1. Go to Actions tab in GitHub
2. Select "Keep PR Branches Updated"
3. Click "Run workflow"

This is useful if you want to update branches before the scheduled run.
