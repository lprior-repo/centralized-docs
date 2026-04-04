---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#29-summary
chunk_level: summary
chunk_type: prose
heading: Types of volumes
token_count: 122
summary: ``` `apiVersion: v1 kind: Pod metadata: name: configmap-pod spec: containers: - name: test image: busybox:1.28 command: ['sh', '-c', 'echo \"The app is running!\" &amp;&amp; tail -f /dev/null']...
---

```
`apiVersion: v1
kind: Pod
metadata:
name: configmap-pod
spec:
containers:
- name: test
image: busybox:1.28
command: ['sh', '-c', 'echo "The app is running!" &amp;&amp; tail -f /dev/null']
volumeMounts:
- name: config-vol
mountPath: /etc/config
volumes:
- name: config-vol
configMap:
name: log-config
items:
- key: log\_level
path: log\_level.conf
`
```