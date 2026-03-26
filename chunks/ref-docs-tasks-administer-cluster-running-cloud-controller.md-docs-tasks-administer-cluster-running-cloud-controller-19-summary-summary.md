---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#19-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 128
summary: spec: selector: matchLabels: k8s-app: cloud-controller-manager template: metadata: labels: k8s-app: cloud-controller-manager spec: serviceAccountName: cloud-controller-manager containers: - name:...
---

spec:
selector:
matchLabels:
k8s-app: cloud-controller-manager
template:
metadata:
labels:
k8s-app: cloud-controller-manager
spec:
serviceAccountName: cloud-controller-manager
containers:
- name: cloud-controller-manager
# this can be replaced with any other image for out-of-tree providers
image: registry.k8s.io/cloud-controller-manager:v1.8.0
command:
- /usr/local/bin/cloud-controller-manager
- --cloud-provider=[YOUR\_CLOUD\_PROVIDER] # Add your own cloud provider here!
- --leader-elect=true
- --use-service-account-credentials