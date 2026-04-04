---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#16-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 92
summary: ### Install with Homebrew on macOS If you are on macOS and using [Homebrew](https://brew.sh/) package manager, you can install kubectl with Homebrew. 1. Run the installation command: ``` `brew...
---

### Install with Homebrew on macOS
If you are on macOS and using [Homebrew](https://brew.sh/) package manager,
you can install kubectl with Homebrew.
1. Run the installation command:
```
`brew install kubectl
`
```
or
```
`brew install kubernetes-cli
`
```
2. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```