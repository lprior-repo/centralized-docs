---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#104-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 110
summary: - name: mysql image: mysql env: - name: MYSQL\_ROOT\_PASSWORD value: \"rootpasswd\" volumeMounts: - mountPath: /var/lib/mysql name: site-data subPath: mysql - name: php image: php:7.0-apache...
---

- name: mysql
image: mysql
env:
- name: MYSQL\_ROOT\_PASSWORD
value: "rootpasswd"
volumeMounts:
- mountPath: /var/lib/mysql
name: site-data
subPath: mysql
- name: php
image: php:7.0-apache
volumeMounts:
- mountPath: /var/www/html
name: site-data
subPath: html
volumes:
- name: site-data
persistentVolumeClaim:
claimName: my-lamp-site-data
`