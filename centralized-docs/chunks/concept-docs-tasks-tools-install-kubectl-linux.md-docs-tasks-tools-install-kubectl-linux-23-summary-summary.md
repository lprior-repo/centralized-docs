---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#23-summary
chunk_level: summary
chunk_type: prose
heading: Install kubectl on Linux
token_count: 115
summary: ``` `# This overwrites any existing configuration in /etc/zypp/repos.d/kubernetes.repo cat &lt;&lt;EOF | sudo tee /etc/zypp/repos.d/kubernetes.repo [kubernetes] name=Kubernetes...
---

```
`# This overwrites any existing configuration in /etc/zypp/repos.d/kubernetes.repo
cat &lt;&lt;EOF | sudo tee /etc/zypp/repos.d/kubernetes.repo
[kubernetes]
name=Kubernetes
baseurl=https://pkgs.k8s.io/core:/stable:/v1.35/rpm/
enabled=1
gpgcheck=1
gpgkey=https://pkgs.k8s.io/core:/stable:/v1.35/rpm/repodata/repomd.xml.key
EOF
`
```