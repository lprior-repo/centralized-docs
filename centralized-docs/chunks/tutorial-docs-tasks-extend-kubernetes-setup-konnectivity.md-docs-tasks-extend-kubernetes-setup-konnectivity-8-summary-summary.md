---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 122
summary: # Set up Konnectivity service The Konnectivity service provides a TCP level proxy for the control plane to cluster communication. ## Before you begin You need to have a Kubernetes cluster, and the...
---

# Set up Konnectivity service
The Konnectivity service provides a TCP level proxy for the control plane to cluster
communication.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this
tutorial on a cluster with at least two nodes that are not acting as control
plane hosts. If you do not already have a cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/).