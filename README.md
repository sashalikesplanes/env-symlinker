# env-symlinker
This CLI tool assists managing untracked .env files when using git worktrees with monorepos.

## Assumptions
Due to the infinite configuration options, this tool makes some assumptions about your workspace.

1. All the .env files are kept in a directory external to the root of the worktrees.
2. All the .env files are named `app-name.env` where the app name corresponds to an application in 
   the monorepo.

## Results
It will create a symlink with the name `.env` in each application within each worktree
pointing to the matching `app-name.env` in the external env dir.

## WARNING
This tool WILL DELETE existing .env files in the locations it tries to create a symlink.
