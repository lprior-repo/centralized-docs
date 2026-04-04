---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Core design principles
token_count: 945
summary: # Implementation details FEATURE STATE: `Kubernetes v1.10 [stable]` `kubeadm init` and `kubeadm join` together provide a nice user experience for creating a bare Kubernetes cluster from scratch, that...
---

# Implementation details
FEATURE STATE:
`Kubernetes v1.10 [stable]`
`kubeadm init` and `kubeadm join` together provide a nice user experience for creating a
bare Kubernetes cluster from scratch, that aligns with the best-practices.
However, it might not be obvious *how* kubeadm does that.
This document provides additional details on what happens under the hood, with the aim of sharing
knowledge on the best practices for a Kubernetes cluster.
## Core design principles
The cluster that `kubeadm init` and `kubeadm join` set up should be:
* **Secure**: It should adopt latest best-practices like:
* enforcing RBAC
* using the Node Authorizer
* using secure communication between the control plane components
* using secure communication between the API server and the kubelets
* lock-down the kubelet API
* locking down access to the API for system components like the kube-proxy and CoreDNS
* locking down what a Bootstrap Token can access
* **User-friendly**: The user should not have to run anything more than a couple of commands:
* `kubeadm init`
* `export KUBECONFIG=/etc/kubernetes/admin.conf`
* `kubectl apply -f &lt;network-plugin-of-choice.yaml&gt;`
* `kubeadm join --token &lt;token&gt; &lt;endpoint&gt;:&lt;port&gt;`
* **Extendable**:
* It should *not* favor any particular network provider. Configuring the cluster network is out-of-scope
* It should provide the possibility to use a config file for customizing various parameters## Constants and well-known values and paths
In order to reduce complexity and to simplify development of higher level tools that build on top of kubeadm, it uses a
limited set of constant values for well-known paths and file names.
The Kubernetes directory `/etc/kubernetes` is a constant in the application, since it is clearly the given path
in a majority of cases, and the most intuitive location; other constant paths and file names are:
* `/etc/kubernetes/manifests` as the path where the kubelet should look for static Pod manifests.
Names of static Pod manifests are:
* `etcd.yaml`
* `kube-apiserver.yaml`
* `kube-controller-manager.yaml`
* `kube-scheduler.yaml`
* `/etc/kubernetes/` as the path where kubeconfig files with identities for control plane
components are stored. Names of kubeconfig files are:
* `kubelet.conf` (`bootstrap-kubelet.conf` during TLS bootstrap)
* `controller-manager.conf`
* `scheduler.conf`
* `admin.conf` for the cluster admin and kubeadm itself
* `super-admin.conf` for the cluster super-admin that can bypass RBAC
* Names of certificates and key files:
* `ca.crt`, `ca.key` for the Kubernetes certificate authority
* `apiserver.crt`, `apiserver.key` for the API server certificate
* `apiserver-kubelet-client.crt`, `apiserver-kubelet-client.key` for the client certificate used
by the API server to connect to the kubelets securely
* `sa.pub`, `sa.key` for the key used by the controller manager when signing ServiceAccount
* `front-proxy-ca.crt`, `front-proxy-ca.key` for the front proxy certificate authority
* `front-proxy-client.crt`, `front-proxy-client.key` for the front proxy client## The kubeadm configuration file format
Most kubeadm commands support a `--config` flag which allows passing a configuration file from
disk. The configuration file format follows the common Kubernetes API `apiVersion` / `kind` scheme,
but is considered a component configuration format. Several Kubernetes components, such as the kubelet,
also support file-based configuration.
Different kubeadm subcommands require a different `kind` of configuration file.
For example, `InitConfiguration` for `kubeadm init`, `JoinConfiguration` for `kubeadm join`, `UpgradeConfiguration` for `kubeadm upgrade` and `ResetConfiguration`
for `kubeadm reset`.
The command `kubeadm config migrate` can be used to migrate an older format configuration
file to a newer (current) configuration format. The kubeadm tool only supports migrating from
deprecated configuration formats to the current format.
See the [kubeadm configuration reference](/docs/reference/config-api/kubeadm-config.v1beta4/) page for more details.