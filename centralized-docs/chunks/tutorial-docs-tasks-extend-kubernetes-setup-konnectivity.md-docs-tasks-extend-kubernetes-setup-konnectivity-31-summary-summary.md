---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#31-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 124
summary: `apiVersion: apps/v1 # Alternatively, you can deploy the agents as Deployments. It is not necessary # to have an agent on each node. kind: DaemonSet metadata: labels: addonmanager.kubernetes.io/mode:...
---

`apiVersion: apps/v1
# Alternatively, you can deploy the agents as Deployments. It is not necessary
# to have an agent on each node.
kind: DaemonSet
metadata:
labels:
addonmanager.kubernetes.io/mode: Reconcile
k8s-app: konnectivity-agent
namespace: kube-system
name: konnectivity-agent
spec:
selector:
matchLabels:
k8s-app: konnectivity-agent
template:
metadata:
labels:
k8s-app: konnectivity-agent
spec:
priorityClassName: system-cluster-critical
tolerations: