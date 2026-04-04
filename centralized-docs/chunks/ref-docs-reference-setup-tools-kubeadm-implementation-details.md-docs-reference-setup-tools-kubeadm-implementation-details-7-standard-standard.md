---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#7-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 505
summary: * [Warning] if the Kubernetes version to use (specified with the `--kubernetes-version` flag) is at least one minor version higher than the kubeadm CLI version. * Kubernetes system requirements: * if...
---

* [Warning] if the Kubernetes version to use (specified with the `--kubernetes-version` flag) is
at least one minor version higher than the kubeadm CLI version.
* Kubernetes system requirements:
* if running on linux:
* [Error] if Kernel is older than the minimum required version
* [Error] if required cgroups subsystem aren't set up
* [Error] if the CRI endpoint does not answer
* [Error] if user is not root
* [Error] if the machine hostname is not a valid DNS subdomain
* [Warning] if the host name cannot be reached via network lookup
* [Error] if kubelet version is lower that the minimum kubelet version supported by kubeadm (current minor -1)
* [Error] if kubelet version is at least one minor higher than the required controlplane version (unsupported version skew)
* [Warning] if kubelet service does not exist or if it is disabled
* [Warning] if firewalld is active
* [Error] if API server bindPort or ports 10250/10251/10252 are used
* [Error] if `/etc/kubernetes/manifest` folder already exists and it is not empty
* [Error] if swap is on
* [Error] if `ip`, `iptables`, `mount`, `nsenter` commands are not present in the command path
* [Warning] if `ethtool`, `tc`, `touch` commands are not present in the command path
* [Warning] if extra arg flags for API server, controller manager, scheduler contains some invalid options
* [Warning] if connection to https://API.AdvertiseAddress:API.BindPort goes through proxy
* [Warning] if connection to services subnet goes through proxy (only first address checked)
* [Warning] if connection to Pods subnet goes through proxy (only first address checked)
* If external etcd is provided:
* [Error] if etcd version is older than the minimum required version
* [Error] if etcd certificates or keys are specified, but not provided
* If external etcd is NOT provided (and thus local etcd will be installed):
* [Error] if ports 2379 is used
* [Error] if Etcd.DataDir folder already exists and it is not empty
* If authorization mode is ABAC:
* [Error] if abac\_policy.json does not exist