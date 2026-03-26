---
doc_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod
chunk_id: tutorial/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md/docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod#16-summary
chunk_level: summary
chunk_type: prose
heading: Migrating existing PersistentVolumes
token_count: 126
summary: `# cat-pictures-writer-deployment.yaml apiVersion: apps/v1 kind: Deployment metadata: name: cat-pictures-writer spec: replicas: 3 selector: matchLabels: app: cat-pictures-writer template: metadata:...
---

`# cat-pictures-writer-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
name: cat-pictures-writer
spec:
replicas: 3
selector:
matchLabels:
app: cat-pictures-writer
template:
metadata:
labels:
app: cat-pictures-writer
spec:
containers:
- name: nginx
image: nginx:1.14.2
ports:
- containerPort: 80
volumeMounts:
- name: cat-pictures
mountPath: /mnt
volumes:
- name: cat-pictures
persistentVolumeClaim: