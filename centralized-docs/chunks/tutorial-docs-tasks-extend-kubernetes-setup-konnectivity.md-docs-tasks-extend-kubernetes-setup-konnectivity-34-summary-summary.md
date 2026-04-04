---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#34-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 91
summary: name: konnectivity-agent-token livenessProbe: httpGet: port: 8134 path: /healthz initialDelaySeconds: 15 timeoutSeconds: 15 serviceAccountName: konnectivity-agent volumes: - name:...
---

name: konnectivity-agent-token
livenessProbe:
httpGet:
port: 8134
path: /healthz
initialDelaySeconds: 15
timeoutSeconds: 15
serviceAccountName: konnectivity-agent
volumes:
- name: konnectivity-agent-token
projected:
sources:
- serviceAccountToken:
path: konnectivity-agent-token
audience: system:konnectivity-server
`
```