---
id: ref/docs-tasks-network-customize-hosts-file-for-pods.md/docs-tasks-network-customize-hosts-file-for-pods
title: Adding entries to Pod /etc/hosts with HostAliases
category: ref
tags: ["/etc/hosts", "adding", "content", "contents", "default"]
---

## Table of Contents

* [Adding entries to Pod /etc/hosts with HostAliases](#adding-entries-to-pod-etchosts-with-hostaliases)
  * [Default hosts file content](#default-hosts-file-content)
  * [Adding additional entries with hostAliases](#adding-additional-entries-with-hostaliases)
* [Entries added by HostAliases.](#entries-added-by-hostaliases)
  * [Why does the kubelet manage the hosts file?](#why-does-the-kubelet-manage-the-hosts-file)
    * [Caution:](#caution)
  * [Feedback](#feedback)

---

# Adding entries to Pod /etc/hosts with HostAliases



 > 
 > **Context**: Adding entries to a Pod's  /etc/hosts  file provides Pod-level override of hostname resolution when DNS and other options are not applicable. You can 



Adding entries to a Pod’s `/etc/hosts` file provides Pod-level override of hostname resolution when DNS and other options are not applicable. You can add these custom entries with the HostAliases field in PodSpec.
The Kubernetes project recommends modifying DNS configuration using the `hostAliases` field
(part of the `.spec` for a Pod), and not by using an init container or other means to edit `/etc/hosts`
directly.
Change made in other ways may be overwritten by the kubelet during Pod creation or restart.

## Default hosts file content

Start an Nginx Pod which is assigned a Pod IP:

````
`kubectl run nginx --image nginx
`
````

````
`pod/nginx created
`
````

Examine a Pod IP:

````
`kubectl get pods --output=wide
`
````

````
`NAME READY STATUS RESTARTS AGE IP NODE
nginx 1/1 Running 0 13s 10.200.0.4 worker0
`
````

The hosts file content would look like this:

````
`kubectl exec nginx -- cat /etc/hosts
`
````

````
`# Kubernetes-managed hosts file.
127.0.0.1 localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.200.0.4 nginx
`
````

By default, the `hosts` file only includes IPv4 and IPv6 boilerplates like
`localhost` and its own hostname.

## Adding additional entries with hostAliases

In addition to the default boilerplate, you can add additional entries to the
`hosts` file.
For example: to resolve `foo.local`, `bar.local` to `127.0.0.1` and `foo.remote`,
`bar.remote` to `10.1.2.3`, you can configure HostAliases for a Pod under
`.spec.hostAliases`:
[`service/networking/hostaliases-pod.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/service/networking/hostaliases-pod.yaml)![](/images/copycode.svg "Copy service/networking/hostaliases-pod.yaml to clipboard")

````
`apiVersion: v1
kind: Pod
metadata:
name: hostaliases-pod
spec:
restartPolicy: Never
hostAliases:
- ip: "127.0.0.1"
hostnames:
- "foo.local"
- "bar.local"
- ip: "10.1.2.3"
hostnames:
- "foo.remote"
- "bar.remote"
containers:
- name: cat-hosts
image: busybox:1.28
command:
- cat
args:
- "/etc/hosts"
`
````

You can start a Pod with that configuration by running:

````
`kubectl apply -f https://k8s.io/examples/service/networking/hostaliases-pod.yaml
`
````

````
`pod/hostaliases-pod created
`
````

Examine a Pod’s details to see its IPv4 address and its status:

````
`kubectl get pod --output=wide
`
````

````
`NAME READY STATUS RESTARTS AGE IP NODE
hostaliases-pod 0/1 Completed 0 6s 10.200.0.5 worker0
`
````

The `hosts` file content looks like this:

````
`kubectl logs hostaliases-pod
`
````

````
`# Kubernetes-managed hosts file.
127.0.0.1 localhost
::1 localhost ip6-localhost ip6-loopback
fe00::0 ip6-localnet
fe00::0 ip6-mcastprefix
fe00::1 ip6-allnodes
fe00::2 ip6-allrouters
10.200.0.5 hostaliases-pod
# Entries added by HostAliases.
127.0.0.1 foo.local bar.local
10.1.2.3 foo.remote bar.remote
`
````

with the additional entries specified at the bottom.

## Why does the kubelet manage the hosts file?

The kubelet manages the
`hosts` file for each container of the Pod to prevent the container runtime from
modifying the file after the containers have already been started.
Historically, Kubernetes always used Docker Engine as its container runtime, and Docker Engine would
then modify the `/etc/hosts` file after each container had started.
Current Kubernetes can use a variety of container runtimes; even so, the kubelet manages the
hosts file within each container so that the outcome is as intended regardless of which
container runtime you use.

### Caution:

Avoid making manual changes to the hosts file inside a container.
If you make manual changes to the hosts file,
those changes are lost when the container exits.

## Feedback

Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified October 03, 2024 at 4:50 PM PST: [Removes repeated information (05c1f011d4)](https://github.com/kubernetes/website/commit/05c1f011d49c05d985982dea3eabb7ab68049f9a)

## Related Pages

* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
* [Using RBAC Authorization](./ref-docs-reference-access-authn-authz-rbac.md-docs-reference-access-authn-authz-rbac.md)
## See Also

- [Documentation Index](./COMPASS.md)
