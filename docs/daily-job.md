# Daily job

A scheduled Claude Code task that works on one Jira item per day and opens one pull request for review. Adrik reviews and merges; the job never merges.

## Facts the job needs

| What | Value |
|---|---|
| Repo | `github.com/dotbatch/crypto-garage`, local checkout `E:\Development\Adrik\crypto-planning` |
| Jira site | `pixsol.atlassian.net`, project `FAM` (Crypto Garage) |
| Jira transitions | `11` = Start Progress (→ In Progress), `21` = Done |
| Branch name | `fam-<number>-<short-slug>`, e.g. `fam-24-tech-stack` |
| Skip label | `needs-human` |

## Procedure

Stop at the first step that fails and report why. Never work around a failure.

### 1. Preflight

1. `git status` must be clean and on `main`. If not, stop: Adrik has local work in progress.
2. `git pull --ff-only origin main`.
3. `gh auth status` must succeed, and the toolchain for the work must be installed (`cargo`, `rustup` for Rust work). If missing, stop and say what to install.

### 2. Reconcile yesterday's work

1. Find FAM Sub-tasks in **In Progress**.
2. For each, look up its PR with `gh pr list --state all --head <branch>`:
   - **Merged** → apply transition `21` (Done) and comment "Merged in <PR link>".
   - **Open** → stop for today. Only one job PR is open at a time; report that it is waiting for review.
   - **Closed without merging** → comment on the Jira item asking Adrik what to change, leave it In Progress, and stop for today.

### 3. Pick today's item

Query:

```
project = FAM AND issuetype = Sub-task AND status = "To Do"
AND (labels IS EMPTY OR labels NOT IN (needs-human))
ORDER BY key ASC
```

Keys were created in build order, so the first result is next. Read its description, its parent Task, and the earlier Sub-tasks under the same parent.

If the item can't be finished because something it depends on isn't done (for example a `needs-human` step such as DNS), comment on it saying what it is waiting for, and move to the next result. If nothing is eligible, stop and report.

### 4. Do the work

1. `git switch -c fam-<number>-<slug>` from the updated `main`.
2. Follow `CLAUDE.md`: cryptography rules first, small focused changes, no dead code.
3. Meet the item's **Done when** line. If it is too big for one day, do a coherent first part, say so in the PR, and leave the item In Progress.
4. Run the formatter, linter and tests for everything touched. All must pass. If they can't be made to pass, stop: don't open a PR with failing checks.

### 5. Commit and open the PR

1. Commits use `type: brief description` (see `CLAUDE.md`), one logical change each, ending with:

   ```
   Refs FAM-<number>

   Co-Authored-By: Claude <noreply@anthropic.com>
   ```

2. `git push -u origin <branch>`.
3. `gh pr create --base main` with:
   - **Title:** `type: summary (FAM-<number>)`
   - **Body:** what changed and why; how it was verified (commands run and results); anything left for later; link to `https://pixsol.atlassian.net/browse/FAM-<number>`; ending with `🤖 Generated with [Claude Code](https://claude.com/claude-code)`.
4. Jira: apply transition `11` (In Progress) and comment with the PR link.
5. `git switch main`.

### 6. Report

One short summary: the item, the PR link, test results, and anything Adrik needs to do.

## Never

- Push to `main`, merge a PR, force-push, or rewrite history
- Backdate commits or change author/committer dates
- Work on `needs-human` items, or change GitHub settings, DNS, the VPS, or Jira items other than the one being worked on
- Commit secrets, real keys, or anything from an employer
- Hand-edit files in `vectors/`
- Open a second PR while one job PR is still open
