---
doc_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos
chunk_id: concept/docs-tasks-tools-install-kubectl-macos.md/docs-tasks-tools-install-kubectl-macos#17-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on macOS
token_count: 85
summary: ### Install with Macports on macOS If you are on macOS and using [Macports](https://macports.org/) package manager, you can install kubectl with Macports. 1. Run the installation command: ``` `sudo...
---

### Install with Macports on macOS
If you are on macOS and using [Macports](https://macports.org/) package manager,
you can install kubectl with Macports.
1. Run the installation command:
```
`sudo port selfupdate
sudo port install kubectl
`
```
2. Test to ensure the version you installed is up-to-date:
```
`kubectl version --client
`
```