---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#2-detailed
chunk_level: detailed
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 771
summary: ## kubeadm init workflow internal design The `kubeadm init` consists of a sequence of atomic work tasks to perform, as described in the `kubeadm init` [internal...
---

## kubeadm init workflow internal design
The `kubeadm init` consists of a sequence of atomic work tasks to perform,
as described in the `kubeadm init` [internal workflow](/docs/reference/setup-tools/kubeadm/kubeadm-init/#init-workflow).
The [`kubeadm init phase`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/) command allows
users to invoke each task individually, and ultimately offers a reusable and composable
API/toolbox that can be used by other Kubernetes bootstrap tools, by any IT automation tool or by
an advanced user for creating custom clusters.
### Preflight checks
Kubeadm executes a set of preflight checks before starting the init, with the aim to verify
preconditions and avoid common cluster startup problems.
The user can skip specific preflight checks or all of them with the `--ignore-preflight-errors` option.
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
* If authorization mode is WebHook
* [Error] if webhook\_authz.conf does not exist
#### Note:
Preflight checks can be invoked individually with the
[`kubeadm init phase preflight`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-preflight)
command.