---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#37-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 124
summary: `apiVersion: rbac.authorization.k8s.io/v1 kind: ClusterRoleBinding metadata: name: system:konnectivity-server labels: kubernetes.io/cluster-service: \"true\" addonmanager.kubernetes.io/mode: Reconcile...
---

`apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
name: system:konnectivity-server
labels:
kubernetes.io/cluster-service: "true"
addonmanager.kubernetes.io/mode: Reconcile
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: system:auth-delegator
subjects:
- apiGroup: rbac.authorization.k8s.io
kind: User
name: system:konnectivity-server
---
apiVersion: v1
kind: ServiceAccount
metadata: