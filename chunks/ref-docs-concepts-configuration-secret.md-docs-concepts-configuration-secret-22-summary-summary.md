---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#22-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 127
summary: `apiVersion: v1 kind: Secret metadata: name: dotfile-secret data: .secret-file: dmFsdWUtMg0KDQo= --- apiVersion: v1 kind: Pod metadata: name: secret-dotfiles-pod spec: volumes: - name: secret-volume...
---

`apiVersion: v1
kind: Secret
metadata:
name: dotfile-secret
data:
.secret-file: dmFsdWUtMg0KDQo=
---
apiVersion: v1
kind: Pod
metadata:
name: secret-dotfiles-pod
spec:
volumes:
- name: secret-volume
secret:
secretName: dotfile-secret
containers:
- name: dotfile-test-container
image: registry.k8s.io/busybox
command:
- ls
- "-l"
- "/etc/secret-volume"
volumeMounts:
- name: secret-volume
readOnly: true