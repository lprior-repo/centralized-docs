---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#48-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 106
summary: ``` `apiVersion: v1 kind: Pod metadata: name: server spec: containers: - image: nginx name: nginx volumeMounts: - mountPath: /mypath name: git-volume volumes: - name: git-volume gitRepo: repository:...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: server
spec:
containers:
- image: nginx
name: nginx
volumeMounts:
- mountPath: /mypath
name: git-volume
volumes:
- name: git-volume
gitRepo:
repository: "git@somewhere:me/my-git-repository.git"
revision: "22f1d8406d464b0c0874075539c1f2e96c253775"
`
```