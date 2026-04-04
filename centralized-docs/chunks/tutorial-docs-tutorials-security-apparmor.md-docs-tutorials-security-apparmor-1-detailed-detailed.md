---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#1-detailed
chunk_level: detailed
chunk_type: code
heading: Objectives
token_count: 807
summary: # Restrict a Container's Access to Resources with AppArmor FEATURE STATE: `Kubernetes v1.31 [stable]`(enabled by default) This page shows you how to load AppArmor profiles on your nodes and enforce...
---

# Restrict a Container's Access to Resources with AppArmor
FEATURE STATE:
`Kubernetes v1.31 [stable]`(enabled by default)
This page shows you how to load AppArmor profiles on your nodes and enforce
those profiles in Pods. To learn more about how Kubernetes can confine Pods using
AppArmor, see
[Linux kernel security constraints for Pods and containers](/docs/concepts/security/linux-kernel-security-constraints/#apparmor).
## Objectives
* See an example of how to load a profile on a Node
* Learn how to enforce the profile on a Pod
* Learn how to check that the profile is loaded
* See what happens when a profile is violated
* See what happens when a profile cannot be loaded## Before you begin
AppArmor is an optional kernel module and Kubernetes feature, so verify it is supported on your
Nodes before proceeding:
1. AppArmor kernel module is enabled -- For the Linux kernel to enforce an AppArmor profile, the
AppArmor kernel module must be installed and enabled. Several distributions enable the module by
default, such as Ubuntu and SUSE, and many others provide optional support. To check whether the
module is enabled, check the `/sys/module/apparmor/parameters/enabled` file:
```
`cat /sys/module/apparmor/parameters/enabled
Y
`
```
The kubelet verifies that AppArmor is enabled on the host before admitting a pod with AppArmor
explicitly configured.
2. Container runtime supports AppArmor -- All common Kubernetes-supported container
runtimes should support AppArmor, including [containerd](https://containerd.io/docs/) and
[CRI-O](https://cri-o.io/#what-is-cri-o). Please refer to the corresponding runtime
documentation and verify that the cluster fulfills the requirements to use AppArmor.
3. Profile is loaded -- AppArmor is applied to a Pod by specifying an AppArmor profile that each
container should be run with. If any of the specified profiles are not loaded in the
kernel, the kubelet will reject the Pod. You can view which profiles are loaded on a
node by checking the `/sys/kernel/security/apparmor/profiles` file. For example:
```
`ssh gke-test-default-pool-239f5d02-gyn2 "sudo cat /sys/kernel/security/apparmor/profiles | sort"
`
```
```
`apparmor-test-deny-write (enforce)
apparmor-test-audit-write (enforce)
docker-default (enforce)
k8s-nginx (enforce)
`
```
For more details on loading profiles on nodes, see
[Setting up nodes with profiles](#setting-up-nodes-with-profiles).
#### Note:
Prior to Kubernetes v1.30, AppArmor was specified through annotations. Use the documentation version
selector to view the documentation with this deprecated API.
AppArmor profiles can be specified at the pod level or container level. The container AppArmor
profile takes precedence over the pod profile.
```
`securityContext:
appArmorProfile:
type: &lt;profile\_type&gt;
`
```
Where `&lt;profile\_type&gt;` is one of:
* `RuntimeDefault` to use the runtime's default profile
* `Localhost` to use a profile loaded on the host (see below)
* `Unconfined` to run without AppArmor
See [Specifying AppArmor Confinement](#specifying-apparmor-confinement) for full details on the AppArmor profile API.
To verify that the profile was applied, you can check that the container's root process is
running with the correct profile by examining its proc attr:
```
`kubectl exec &lt;pod\_name&gt; -- cat /proc/1/attr/current
`
```
The output should look something like this:
```
`cri-containerd.apparmor.d (enforce)
`
```