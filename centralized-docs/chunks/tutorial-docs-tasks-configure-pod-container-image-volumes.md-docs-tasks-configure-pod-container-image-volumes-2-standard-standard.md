---
doc_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes
chunk_id: tutorial/docs-tasks-configure-pod-container-image-volumes.md/docs-tasks-configure-pod-container-image-volumes#2-standard
chunk_level: standard
chunk_type: prose
heading: Before you begin
token_count: 504
summary: ## Before you begin You need to have a Kubernetes cluster, and the kubectl command-line tool must be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster...
---

## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)Your Kubernetes server must be at or later than version v1.31.
To check the version, enter `kubectl version`.
* The container runtime needs to support the image volumes feature
* You need to exec commands in the host
* You need to be able to exec into pods
* You need to enable the `ImageVolume` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)## Run a Pod that uses an image volume
An image volume for a pod is enabled by setting the `volumes[\*].image` field of `.spec`
to a valid reference and consuming it in the `volumeMounts` of the container. For example:
[`pods/image-volumes.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/image-volumes.yaml)![](/images/copycode.svg "Copy pods/image-volumes.yaml to clipboard")
```
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
```
1. Create the pod on your cluster:
```
`kubectl apply -f https://k8s.io/examples/pods/image-volumes.yaml
`
```
2. Attach to the container:
```
`kubectl exec image-volume -it -- bash
`
```
3. Check the content of a file in the volume: