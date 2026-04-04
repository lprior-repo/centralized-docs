---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#17-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 99
summary: ``` `# This overwrites any existing configuration in /etc/apt/sources.list.d/kubernetes.list echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg]...
---

```
`# This overwrites any existing configuration in /etc/apt/sources.list.d/kubernetes.list
echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.35/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list
sudo chmod 644 /etc/apt/sources.list.d/kubernetes.list # helps tools such as command-not-found to work correctly
`
```