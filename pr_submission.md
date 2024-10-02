# Pull Request Submission Process

### Before you start coding a new feature:

1. `git checkout develop` moves you to develop branch of repo.

2. `git pull` syncs your local repo with the remote repo.

3. `git checkout -b <name-of-feature> develop` moves you to a new feature branch, coming off of the develop branch

### During your coding process:

1. `git add .` stages changed files.

2. `git commit -m "Commit message."` creates a snapshot of repo with staged files (useful for going back if something gets messed up).

3. If any changes are made to develop branch while you are still working on a feature, `git merge develop` syncs your branch with develop branch. Be careful as this might delete important files you are working on. This is why it is good practice to regularly commit changes.

### After finishing a feature:

`git push --set-upstream origin feature/<name-of-feature>` pushes a copy of your current feature branch to the remote repo. At this point you may go to the github page for your feature branch and submit a pull request from there.
