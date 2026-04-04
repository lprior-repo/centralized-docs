---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#28-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 120
summary: Note: A GPG pubkey is clearly identified by its fingerprint. Do not rely on the key's name. If you are not sure whether the presented key is authentic, ask the repository provider or check their web...
---

Note: A GPG pubkey is clearly identified by its fingerprint. Do not rely on the key's name. If
you are not sure whether the presented key is authentic, ask the repository provider or check
their web site. Many providers maintain a web page showing the fingerprints of the GPG keys they
are using.
Do you want to reject the key, trust temporarily, or trust always? [r/t/a/?] (r): a
`
```
2. Install kubectl using `zypper`:
```
`sudo zypper install -y kubectl
`
```