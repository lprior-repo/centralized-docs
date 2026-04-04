---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#4-standard
chunk_level: standard
chunk_type: prose
heading: Install kubectl on Linux
token_count: 512
summary: #### Note: To upgrade kubectl to another minor release, you'll need to bump the version in `/etc/apt/sources.list.d/kubernetes.list` before running `apt-get update` and `apt-get upgrade`. This...
---

#### Note:
To upgrade kubectl to another minor release, you'll need to bump the version in `/etc/apt/sources.list.d/kubernetes.list` before running `apt-get update` and `apt-get upgrade`. This procedure is described in more detail in [Changing The Kubernetes Package Repository](/docs/tasks/administer-cluster/kubeadm/change-package-repository/).
1. Update `apt` package index, then install kubectl:
```
`sudo apt-get update
sudo apt-get install -y kubectl
`
```
1. Add the Kubernetes `yum` repository. If you want to use Kubernetes version
different than v1.35, replace v1.35 with
the desired minor version in the command below.
```
`# This overwrites any existing configuration in /etc/yum.repos.d/kubernetes.repo
cat &lt;&lt;EOF | sudo tee /etc/yum.repos.d/kubernetes.repo
[kubernetes]
name=Kubernetes
baseurl=https://pkgs.k8s.io/core:/stable:/v1.35/rpm/
enabled=1
gpgcheck=1
gpgkey=https://pkgs.k8s.io/core:/stable:/v1.35/rpm/repodata/repomd.xml.key
EOF
`
```
#### Note:
To upgrade kubectl to another minor release, you'll need to bump the version in `/etc/yum.repos.d/kubernetes.repo` before running `yum update`. This procedure is described in more detail in [Changing The Kubernetes Package Repository](/docs/tasks/administer-cluster/kubeadm/change-package-repository/).
1. Install kubectl using `yum`:
```
`sudo yum install -y kubectl
`
```
1. Add the Kubernetes `zypper` repository. If you want to use Kubernetes version
different than v1.35, replace v1.35 with
the desired minor version in the command below.
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