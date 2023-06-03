# Contributing Guidelines

The contribution structure for this repository is based on the [Git Etiquette](https://github.com/nergster/Git-Etiquette). Feel free to read that for more info
Also, please note that by contributing to this project, you are agreeing to the terms and conditions outlined in the [Developer Certificate of Origin](https://developercertificate.org/).

## Table of Contents
 - [Getting Started](#getting-started)
    - [Ideology](#ideology)
    - [Forking and Cloning](#forking-and-cloning)
    - [Installing Dependencies](#installing-dependencies)
    - [Compiling and Debugging](#compiling-and-debugging)
    - [Your First Contribution](#your-first-contribution)
 - [Issues and Pull Requests]()
    - [Labels](#labels)
    - [Label list](#label-list)
    - [Creating an Issue](#creating-an-issue)
    - [Creating a Pull Request](#creating-a-pull-request)
 - [Git Flow](#git-flow)
    - [Branch Structure](#branch-structure)
    - [Merge Patterns](#merge-patterns)
 - [Versioning](#versioning)
 - [Becoming a Team Member](#becoming-a-team-member)

## Getting Started

 - [Forking and Cloning](#forking-and-cloning)
 - [Installing Dependencies](#installing-dependencies)
 - [Compiling and Debugging](#compiling-and-debugging)
 - [Your First Contribution](#your-first-contribution)

<!-- ADD -->

([back to top](#contributing-guidelines))

### Ideology

<!-- ADD -->

([back to top](#contributing-guidelines))
([back to section](#getting-started))

### Forking and Cloning

<!-- ADD -->

([back to top](#contributing-guidelines))
([back to section](#getting-started))

### Installing Dependencies

<!-- ADD -->
 
([back to top](#contributing-guidelines))
([back to section](#getting-started))

### Compiling and Debugging

<!-- ADD -->

([back to top](#contributing-guidelines))
([back to section](#getting-started))

### Your First Contribution

<!-- ADD -->

([back to top](#contributing-guidelines))
([back to section](#getting-started))

## Issues and Pull Requests
 - [Labels](#labels)
    - [Descriptive labels](#descriptive-labels)
    - [Categorical labels](#categorical-labels)
    - [Flag labels](#flag-labels)
    - [Status labels](#status-labels)
 - [Label list](#label-list)
 - [Creating an Issue](#creating-an-issue)
 - [Creating a Pull Request](#creating-a-pull-request)

<!-- ADD --->

([back to top](#contributing-guidelines))

### Labels
Labels can either be assigned automatically by the automation or manually by the user. It is best to let the automation handle the assignment of labels marked as "automatically" and not add them manually to an issue or pull request.

Issues and Pull Requests should *always* have exactly one Categorical label, Status label, and Descriptive label.

#### Descriptive labels
These are labels that describe the nature of the proposed changes, and how they affect the project. These labels should be added manually.

#### Categorical labels
These are labels that describe the nature of the issue itself. These are typically added automatically.

#### Flag labels
These are labels that discribe certain attributes of issues and PRs, to make it easy for contributors to find and address them quickly. Some are automatic, and some are manual.

#### Status labels
These are labels that describe the development status of the changes. These can be added manually, but are typically updated automatically by Actions when development of the features follows the usual flow.

([back to top](#contributing-guidelines))
([back to section](#issues-and-pull-requests))

### Label list

Label Name | Label Type | Description | Manually or Automatically | Label / Status Field
--- | --- | --- | --- | :---:
[Docs 📝](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Docs+%F0%9F%93%9D%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | Changes to documentation only | Manually | Label
[Feat 🆕](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Feat+%F0%9F%86%95%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | A new feature | Manually | Label
[Style 🖌️](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Style+%F0%9F%96%8C%EF%B8%8F%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | Changes to formatting. (e.g. spreading lines, tabbing, keyword casing...) | Manually | Label
[Test ☑️](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Test+%E2%98%91%EF%B8%8F%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | Adding/correcting existing tests | Manually | Label
[Refactor 🧮](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Refactor+%F0%9F%A7%AE%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | Restructuring of the code without changing functionality | Manually | Label
[Bugfix 🩹](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Bugfix+%F0%9F%A9%B9%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | A bug fix | Manually | Label
[Chore 🧹](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Chore+%F0%9F%A7%B9%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | Routine maintenance (e.g. bump version, update changelog or deps) | Manually | Label
[Meta 👨‍💻](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Meta+%F0%9F%91%A8%E2%80%8D%F0%9F%92%BB%22+sort%3Aupdated-desc) | [Descriptive](#descriptive-labels) | Changes development files, but not product code (e.g. changes to .gitignore, workflows, readme...) | Manually | Label
[Backend ⚙️](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Backend+%E2%9A%99%EF%B8%8F%22+sort%3Aupdated-desc) | [Categorical](#categorical-labels) | Functional/mechanical changes, development changes | Automatically | Label
[Frontend 🖥️](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Frontend+%F0%9F%96%A5%EF%B8%8F%22+sort%3Aupdated-desc) | [Categorical](#categorical-labels) | Visual/graphical changes | Automatically | Label
[Design 🎨](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Design+%F0%9F%8E%A8%22+sort%3Aupdated-desc) | [Categorical](#categorical-labels) | Artistic/stylistic changes | Automatically | Label
[Unassigned ❕](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Unassigned+%E2%9D%95%22+sort%3Aupdated-desc) | [Flag](#flag-labels) | Currently nobody has been assigned to it | Automatically | Label
[Unlabeled ❗](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Unlabeled+%E2%9D%97%22+sort%3Aupdated-desc) | [Flag](#flag-labels) | No labels have been added | Automatically | Label
[Backlog 📜](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Backlog+%F0%9F%93%9C%22+sort%3Aupdated-desc) | [Status](#status-labels) | Tasks that have not yet been started or left unfinished | Both | Both
[In Progress 🛠](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22In+Progress+%F0%9F%9B%A0%22+sort%3Aupdated-desc) | [Status](#status-labels) | Currently in the process of being worked on | Both | Both
[Ready for Review 🔍](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Ready+for+Review+%F0%9F%94%8D%22+sort%3Aupdated-desc) | [Status](#status-labels) | Changes are complete and ready for review (PR only) | Both | Both
[Done ✔️](https://github.com/nergster/fossy/issues?q=is%3Aopen+label%3A%22Done+%E2%9C%94%EF%B8%8F%22+sort%3Aupdated-desc) | [Status](#status-labels) | Task has been completed or closed | Both | Both

([back to top](#contributing-guidelines))
([back to section](#issues-and-pull-requests))

### Creating an Issue

<!-- ADD -->

1. Open a new issue inside the repository.
2. Choose one of the cards or templates (Backend, Frontend, or Design). This determines the **Categorical** label.
3. Follow thew instructions to fill out the template.
4. Pick one (and *only* one) **Descriptive** label from the [label list](#label-list).
5. Submit your issue. The Backlog 📜 label will be automatically added to fill the **Status** label.
6. Once a contributor makes a pull request to address the issue, the **Status** label will be automatically updated to In Progress 🛠. See [Creating a Pull Request](#creating-a-pull-request).
7. A team member will close the issue once the linked pull request is merged. The **Status** label will be automatically updated to Done ✔️.

([back to top](#contributing-guidelines))
([back to section](#issues-and-pull-requests))

### Creating a Pull Request

<!-- ADD -->

1. Choose an issue to address.
2. Fork the repository, if needed.
3. Create a new branch. The branch name should be formated as `<desc>/<num>/<title>`. (e.g. `feat/#7/add-roadmap`)
    - `<desc>` is the **Descriptive** label from the issue. (e.g. `feat` for a Feat 🆕 issue)
    - `<num>` is the issue number. (e.g. `#7`)
    - `<title>` is the title of the branch. This should be one to three words, separated by dashes, that describe the changes. (e.g. `add-roadmap`)
4. Commit your initial changes. This can be done on the website or cloned to your local machine and pushed to the branch.
5. Open a Draft PR and link the addressed issue as soon as possible. This will allow the automation to start tracking your progress, and prevent others from duplicating your work.
    - **NOTE:** you should **always** open pull requests to the **`dev`** branch, never the `main` branch. The `main` branch is reserved for stable releases, and should only be updated by merging the `dev` branch. All pull requests to `main` will be rejected.
6. When you are ready, submit your PR for review. A team member will be assigned to review your changes and guide you through the merging process.
    - If the reviewer requests changes, make the requested changes and commit them to the branch. Once you have addressed all the requested changes, you may request another review.
8. Once the reviewer approves your changes, the reviewer will merge your changes into the `dev` branch, and the **Status** label will be automatically updated to Done ✔️. The reviewer will then close the issue.

Congradulations! You have successfully made your first contribution to this repository! Your contributions will forever be remembered in the commit history of this repository, and you will be listed as a contributor in the project's credits.

([back to top](#contributing-guidelines))
([back to section](#issues-and-pull-requests))

## Git Flow

 - [Branch Structure](#branch-structure)
    - [Base branches](#base-branches)
    - [Feature branches](#feature-branches)

This repository's workflow strategy is based on the Git Flow strategy, where every update or addition is made in temporary branches called feature branches. Every new branch created addresses an open issue, whether it be a feature request or a bug report.

The `main` branch is the production branch, which is the version that is released and is always the most stable one. No pull requests are made to the `main` branch, and the `main` branch only changes when a new release or beta is pushed from the `dev` branch. 

The `dev` branch is where all pull requests are made. It is the base of all feature branches, and is the home of active development and experimental features. New features are constantly pulled to the `dev` branch from feature branches. However, even though the `dev` branch is less stable than `main`, it should never be broken, and should always be usable for those who want to test the latest features. Nightly builds are made from the `dev` branch on a regular basis.

The actual Git Flow strategy also includes `release` and `hotfix` branches, which are not included in this repo's workflow structure. All releases should be managed using the Git tagging system, and hotfixes should be made in feature branches.

([back to top](#contributing-guidelines))

### Branch Structure

<!-- ADD -->

#### Base branches

<!-- ADD -->

Name | Description
--- | ---
main | A production branch
dev | A development branch
feature branches | Each issue is one feature branch

#### Feature branches

See [Creating a Pull Request](#creating-a-pull-request).

<!-- ADD -->

([back to top](#contributing-guidelines))
([back to section](#git-flow))

### Merge Patterns

<!-- ADD -->

* The merge order is from feature branch to development branch, and then from development branch to main branch. This order ensures that changes are tested in the development branch before they are merged in to the main branch. 
   * Simply said: the merge order is: feature > dev > main.
* Forward merges follow the order above, and are always done via pull requests. and should always be merged using a Merge Commit. This ensures features and changes are easy to keep track of.
* Backward merges are done in reverse, and are typically used to update feature branches with changes from the development branch. These merges should always be done using a Rebase Merge if possible. This ensures that the feature branch is up to date with the development branch, while preventing innecessary git spaghetti. Where a rebase merge fails, a Merge Commit is acceptable.
* **Always** open pull request from feature branch to `dev` branch (never to `main`!).
* Each issue has one and **only** one associated feature branch.
* **Never** merge changes directly to `dev` or `main`! If something should be changed, open an issue to let everyone know. Then you can make a pull request if you want to address it yourself. This ensures that changes are peer reviewed and any potential problems can be identified before the changes are applied to the branch. Even if there are problems accepted via a pull request, merging makes it easy to revert changes if needed.

<!-- TODO: Add diagram -->

([back to top](#contributing-guidelines))
([back to section](#git-flow))

## Versioning

<!-- ADD -->

([back to top](#contributing-guidelines))

## Becoming a Team Member

<!-- ADD -->

([back to top](#contributing-guidelines))