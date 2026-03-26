---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#12-summary
chunk_level: summary
chunk_type: prose
heading: Create a Deployment
token_count: 103
summary: ## Create a Deployment A Kubernetes [*Pod*](/docs/concepts/workloads/pods/) is a group of one or more Containers, tied together for the purposes of administration and networking. The Pod in this...
---

## Create a Deployment
A Kubernetes [*Pod*](/docs/concepts/workloads/pods/) is a group of one or more Containers,
tied together for the purposes of administration and networking. The Pod in this
tutorial has only one Container. A Kubernetes
[*Deployment*](/docs/concepts/workloads/controllers/deployment/) checks on the health of your
Pod and restarts the Pod's Container if it terminates. Deployments are the
recommended way to manage the creation and scaling of Pods.