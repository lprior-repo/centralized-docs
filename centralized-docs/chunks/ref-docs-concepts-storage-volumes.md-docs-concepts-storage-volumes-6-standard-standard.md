---
doc_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes
chunk_id: ref/docs-concepts-storage-volumes.md/docs-concepts-storage-volumes#6-standard
chunk_level: standard
chunk_type: prose
heading: Types of volumes
token_count: 439
summary: ## Types of volumes Kubernetes supports several types of volumes. ### configMap A [ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/) provides a way to inject configuration data...
---

## Types of volumes
Kubernetes supports several types of volumes.
### configMap
A [ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/)
provides a way to inject configuration data into Pods.
The data stored in a ConfigMap can be referenced in a volume of type
`configMap` and then consumed by containerized applications running in a Pod.
When referencing a ConfigMap, you provide the name of the ConfigMap in the
volume. You can customize the path to use for a specific
entry in the ConfigMap. The following configuration shows how to mount
the `log-config` ConfigMap onto a Pod called `configmap-pod`:
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
The `log-config` ConfigMap is mounted as a volume, and all contents stored in
its `log\_level` entry are mounted into the Pod at path `/etc/config/log\_level.conf`.
Note that this path is derived from the volume's `mountPath` and the `path`
keyed with `log\_level`.
#### Note:
* You must [create a ConfigMap](/docs/tasks/configure-pod-container/configure-pod-configmap/#create-a-configmap)
before you can use it.
* A ConfigMap is always mounted as `readOnly`.
* A container using a ConfigMap as a [`subPath`](#using-subpath) volume mount will not
receive updates when the ConfigMap changes.
* Text data is exposed as files using the UTF-8 character encoding.
For other character encodings, use `binaryData`.