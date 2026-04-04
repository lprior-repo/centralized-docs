---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#38-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 65
summary: kind: User name: system:konnectivity-server --- apiVersion: v1 kind: ServiceAccount metadata: name: konnectivity-agent namespace: kube-system labels: kubernetes.io/cluster-service: \"true\"...
---

kind: User
name: system:konnectivity-server
---
apiVersion: v1
kind: ServiceAccount
metadata:
name: konnectivity-agent
namespace: kube-system
labels:
kubernetes.io/cluster-service: "true"
addonmanager.kubernetes.io/mode: Reconcile
`
```