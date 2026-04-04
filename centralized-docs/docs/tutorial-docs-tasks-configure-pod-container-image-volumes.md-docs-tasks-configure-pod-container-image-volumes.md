---
id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
title: Use an Image Volume With a Pod
category: tutorial
tags: ["before", "begin", "contents", "image", "table"]
---

## Table of Contents

* [Use an Image Volume With a Pod](#use-an-image-volume-with-a-pod)
  * [Before you begin](#before-you-begin)
  * [Use `subPath` (or `subPathExpr`)](#use-subpath-or-subpathexpr)
  * [Feedback](#feedback)

---

# Use an Image Volume With a Pod



 > 
 > **Context**: FEATURE STATE: Kubernetes v1.35 [beta] (enabled by default) This page shows how to configure a pod using image volumes. This allows you to mount conte



FEATURE STATE:
`Kubernetes v1.35 [beta]`(enabled by default)
This page shows how to configure a pod using image volumes. This allows you to
mount content from OCI registries inside containers.

## Before you begin

You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:

* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)Your Kubernetes server must be at or later than version v1.31.
  To check the version, enter `kubectl version`.
* The container runtime needs to support the image volumes feature
* You need to exec commands in the host
* You need to be able to exec into pods
* You need to enable the `ImageVolume` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)\## Run a Pod that uses an image volume
  An image volume for a pod is enabled by setting the `volumes[\*].image` field of `.spec`
  to a valid reference and consuming it in the `volumeMounts` of the container. For example:
  [`pods/image-volumes.yaml`
  ](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes.yaml)![](/images/copycode.svg "Copy pods/image-volumes.yaml to clipboard")

````
`apiVersion: v1
kind: Pod
metadata:
name: image-volume
spec:
containers:
- name: shell
command: ["sleep", "infinity"]
image: debian
volumeMounts:
- name: volume
mountPath: /volume
volumes:
- name: volume
image:
reference: quay.io/crio/artifact:v2
pullPolicy: IfNotPresent
`
````

1. Create the pod on your cluster:

````
`kubectl apply -f https://k8s.io/examples/pods/image-volumes.yaml
`
````

2. Attach to the container:

````
`kubectl exec image-volume -it -- bash
`
````

3. Check the content of a file in the volume:

````
`cat /volume/dir/file
`
````

The output is similar to:

````
`1
`
````

You can also check another file in a different path:

````
`cat /volume/file
`
````

The output is similar to:

````
`2
`
````

## Use `subPath` (or `subPathExpr`)

It is possible to utilize
[`subPath`](/docs/concepts/storage/volumes/#using-subpath) or
[`subPathExpr`](/docs/concepts/storage/volumes/#using-subpath-expanded-environment)
from Kubernetes v1.33 when using the image volume feature.
[`pods/image-volumes-subpath.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes-subpath.yaml)![](/images/copycode.svg "Copy pods/image-volumes-subpath.yaml to clipboard")

````
`apiVersion: v1
kind: Pod
metadata:
name: image-volume
spec:
containers:
- name: shell
command: ["sleep", "infinity"]
image: debian
volumeMounts:
- name: volume
mountPath: /volume
subPath: dir
volumes:
- name: volume
image:
reference: quay.io/crio/artifact:v2
pullPolicy: IfNotPresent
`
````

1. Create the pod on your cluster:

````
`kubectl apply -f https://k8s.io/examples/pods/image-volumes-subpath.yaml
`
````

2. Attach to the container:

````
`kubectl exec image-volume -it -- bash
`
````

3. Check the content of the file from the `dir` sub path in the volume:

````
`cat /volume/file
`
````

The output is similar to:

````
`1
`
````

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
Last modified October 24, 2025 at 3:23 PM PST: [fix typo in tasks/configure-pod-container/image-volumes.md (ca3bf06233)](https://github.com/kubernetes/website/commit/ca3bf0623374ea5950fde4145199627ad269a8e1)

## Related Pages

* [Volumes](./ref-docs-concepts-storage-volumes.md-docs-concepts-storage-volumes.md)
* [Certificates and Certificate Signing Requests](./ref-docs-reference-access-authn-authz-certificate-signing-requests.md-docs-reference-access-authn-authz-certificate-signing-requests.md)
* [Kubernetes Component SLI Metrics](./ref-docs-reference-instrumentation-slis.md-docs-reference-instrumentation-slis.md)
* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
## See Also

- [Documentation Index](./COMPASS.md)
