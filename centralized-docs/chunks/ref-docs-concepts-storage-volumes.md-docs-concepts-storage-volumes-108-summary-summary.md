---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#108-summary
chunk_level: summary
chunk_type: prose
heading: Using subPath
token_count: 115
summary: `apiVersion: v1 kind: Pod metadata: name: pod1 spec: containers: - name: container1 env: - name: POD\_NAME valueFrom: fieldRef: apiVersion: v1 fieldPath: metadata.name image: busybox:1.28 command: [...
---

`apiVersion: v1
kind: Pod
metadata:
name: pod1
spec:
containers:
- name: container1
env:
- name: POD\_NAME
valueFrom:
fieldRef:
apiVersion: v1
fieldPath: metadata.name
image: busybox:1.28
command: [ "sh", "-c", "while [ true ]; do echo 'Hello'; sleep 10; done | tee -a /logs/hello.txt" ]
volumeMounts:
- name: workdir1
mountPath: /logs