---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#69-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 68
summary: - name: test-webserver image: registry.k8s.io/test-webserver:latest volumeMounts: - mountPath: /var/local/aaa name: mydir - mountPath: /var/local/aaa/1.txt name: myfile volumes: - name: mydir...
---

- name: test-webserver
image: registry.k8s.io/test-webserver:latest
volumeMounts:
- mountPath: /var/local/aaa
name: mydir
- mountPath: /var/local/aaa/1.txt
name: myfile
volumes:
- name: mydir
hostPath: