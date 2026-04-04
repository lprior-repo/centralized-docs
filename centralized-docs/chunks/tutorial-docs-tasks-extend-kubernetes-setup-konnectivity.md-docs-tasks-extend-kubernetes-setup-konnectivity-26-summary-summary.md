---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#26-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 126
summary: \"--mode=grpc\", \"--server-port=0\", \"--agent-port=8132\", \"--admin-port=8133\", \"--health-port=8134\", \"--agent-namespace=kube-system\", \"--agent-service-account=konnectivity-agent\",...
---

"--mode=grpc",
"--server-port=0",
"--agent-port=8132",
"--admin-port=8133",
"--health-port=8134",
"--agent-namespace=kube-system",
"--agent-service-account=konnectivity-agent",
"--kubeconfig=/etc/kubernetes/konnectivity-server.conf",
"--authentication-audience=system:konnectivity-server"
]
livenessProbe:
httpGet:
scheme: HTTP
host: 127.0.0.1
port: 8134
path: /healthz
initialDelaySeconds: 30
timeoutSeconds: 60
ports: