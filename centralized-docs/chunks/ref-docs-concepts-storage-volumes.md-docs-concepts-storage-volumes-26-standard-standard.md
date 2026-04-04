---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#26-standard
chunk_level: standard
chunk_type: prose
heading: Using subPath
token_count: 49
summary: # The variable expansion uses round brackets (not curly brackets). subPathExpr: $(POD\_NAME) restartPolicy: Never volumes: - name: workdir1 hostPath: path: /var/log/pods `
---

# The variable expansion uses round brackets (not curly brackets).
subPathExpr: $(POD\_NAME)
restartPolicy: Never
volumes:
- name: workdir1
hostPath:
path: /var/log/pods
`