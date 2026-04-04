---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Determine which version to upgrade to
token_count: 540
summary: ``` `killall -s SIGTERM kube-apiserver # trigger a graceful kube-apiserver shutdown sleep 20 # wait a little bit to permit completing in-flight requests kubeadm upgrade ... # execute a kubeadm...
---

```
`killall -s SIGTERM kube-apiserver # trigger a graceful kube-apiserver shutdown
sleep 20 # wait a little bit to permit completing in-flight requests
kubeadm upgrade ... # execute a kubeadm upgrade command
`
```
## Changing the package repository
If you're using the community-owned package repositories (`pkgs.k8s.io`), you need to
enable the package repository for the desired Kubernetes minor release. This is explained in
[Changing the Kubernetes package repository](/docs/tasks/administer-cluster/kubeadm/change-package-repository/)
document.
**Note:** The legacy package repositories (`apt.kubernetes.io` and `yum.kubernetes.io`) have been
[deprecated and frozen starting from September 13, 2023](/blog/2023/08/31/legacy-package-repository-deprecation/).
**Using the [new package repositories hosted at `pkgs.k8s.io`](/blog/2023/08/15/pkgs-k8s-io-introduction/)
is strongly recommended and required in order to install Kubernetes versions released after September 13, 2023.**
The deprecated legacy repositories, and their contents, might be removed at any time in the future and without
a further notice period. The new package repositories provide downloads for Kubernetes versions starting with v1.24.0.
## Determine which version to upgrade to
Find the latest patch release for Kubernetes 1.35 using the OS package manager:
```
`# Find the latest 1.35 version in the list.
# It should look like 1.35.x-\*, where x is the latest patch.
sudo apt update
sudo apt-cache madison kubeadm
`
```
For systems with DNF:
```
`# Find the latest 1.35 version in the list.
# It should look like 1.35.x-\*, where x is the latest patch.
sudo yum list --showduplicates kubeadm --disableexcludes=kubernetes
`
```
For systems with DNF5:
```
`# Find the latest 1.35 version in the list.
# It should look like 1.35.x-\*, where x is the latest patch.
sudo yum list --showduplicates kubeadm --setopt=disable\_excludes=kubernetes
`
```
If you don't see the version you expect to upgrade to, [verify if the Kubernetes package repositories are used.](/docs/tasks/administer-cluster/kubeadm/change-package-repository/#verifying-if-the-kubernetes-package-repositories-are-used)