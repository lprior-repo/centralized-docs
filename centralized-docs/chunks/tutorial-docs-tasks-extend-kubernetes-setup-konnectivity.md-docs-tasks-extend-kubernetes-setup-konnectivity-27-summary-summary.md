---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#27-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 118
summary: port: 8134 path: /healthz initialDelaySeconds: 30 timeoutSeconds: 60 ports: - name: agentport containerPort: 8132 hostPort: 8132 - name: adminport containerPort: 8133 hostPort: 8133 - name:...
---

port: 8134
path: /healthz
initialDelaySeconds: 30
timeoutSeconds: 60
ports:
- name: agentport
containerPort: 8132
hostPort: 8132
- name: adminport
containerPort: 8133
hostPort: 8133
- name: healthport
containerPort: 8134
hostPort: 8134
volumeMounts:
- name: k8s-certs
mountPath: /etc/kubernetes/pki
readOnly: true
- name: kubeconfig