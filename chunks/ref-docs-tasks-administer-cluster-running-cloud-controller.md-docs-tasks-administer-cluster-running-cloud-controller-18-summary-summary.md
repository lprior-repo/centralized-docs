---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#18-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 126
summary: kind: ClusterRoleBinding metadata: name: system:cloud-controller-manager roleRef: apiGroup: rbac.authorization.k8s.io kind: ClusterRole name: cluster-admin subjects: - kind: ServiceAccount name:...
---

kind: ClusterRoleBinding
metadata:
name: system:cloud-controller-manager
roleRef:
apiGroup: rbac.authorization.k8s.io
kind: ClusterRole
name: cluster-admin
subjects:
- kind: ServiceAccount
name: cloud-controller-manager
namespace: kube-system
---
apiVersion: apps/v1
kind: DaemonSet
metadata:
labels:
k8s-app: cloud-controller-manager
name: cloud-controller-manager
namespace: kube-system
spec:
selector:
matchLabels:
k8s-app: cloud-controller-manager
template:
metadata:
labels:
k8s-app: cloud-controller-manager