---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#15-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 128
summary: ``` `# If the folder `/etc/apt/keyrings` does not exist, it should be created before the curl command, read the note below. # sudo mkdir -p -m 755 /etc/apt/keyrings curl -fsSL...
---

```
`# If the folder `/etc/apt/keyrings` does not exist, it should be created before the curl command, read the note below.
# sudo mkdir -p -m 755 /etc/apt/keyrings
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.35/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
sudo chmod 644 /etc/apt/keyrings/kubernetes-apt-keyring.gpg # allow unprivileged APT programs to read this keyring
`
```