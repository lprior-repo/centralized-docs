---
doc_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux
chunk_id: concept/docs-tasks-tools-install-kubectl-linux.md/docs-tasks-tools-install-kubectl-linux#2-detailed
chunk_level: detailed
chunk_type: code
heading: Install kubectl on Linux
token_count: 934
summary: ### Install using native package management 1. Update the `apt` package index and install packages needed to use the Kubernetes `apt` repository: ``` `sudo apt-get update # apt-transport-https may be...
---

### Install using native package management
1. Update the `apt` package index and install packages needed to use the Kubernetes `apt` repository:
```
`sudo apt-get update
# apt-transport-https may be a dummy package; if so, you can skip that package
sudo apt-get install -y apt-transport-https ca-certificates curl gnupg
`
```
2. Download the public signing key for the Kubernetes package repositories. The same signing key is used for all repositories so you can disregard the version in the URL:
```
`# If the folder `/etc/apt/keyrings` does not exist, it should be created before the curl command, read the note below.
# sudo mkdir -p -m 755 /etc/apt/keyrings
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.35/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
sudo chmod 644 /etc/apt/keyrings/kubernetes-apt-keyring.gpg # allow unprivileged APT programs to read this keyring
`
```
#### Note:
In releases older than Debian 12 and Ubuntu 22.04, folder `/etc/apt/keyrings` does not exist by default, and it should be created before the curl command.
1. Add the appropriate Kubernetes `apt` repository. If you want to use Kubernetes version different than v1.35,
replace v1.35 with the desired minor version in the command below:
```
`# This overwrites any existing configuration in /etc/apt/sources.list.d/kubernetes.list
echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.35/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list
sudo chmod 644 /etc/apt/sources.list.d/kubernetes.list # helps tools such as command-not-found to work correctly
`
```
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