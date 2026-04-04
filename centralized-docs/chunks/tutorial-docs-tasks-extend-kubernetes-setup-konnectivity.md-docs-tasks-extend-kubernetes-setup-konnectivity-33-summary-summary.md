---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#33-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 125
summary: \"--ca-cert=/var/run/secrets/kubernetes.io/serviceaccount/ca.crt\", # this is the IP address of the master machine. \"--proxy-server-host=35.225.206.7\", \"--proxy-server-port=8132\",...
---

"--ca-cert=/var/run/secrets/kubernetes.io/serviceaccount/ca.crt",
# this is the IP address of the master machine.
"--proxy-server-host=35.225.206.7",
"--proxy-server-port=8132",
"--admin-server-port=8133",
"--health-server-port=8134",
"--service-account-token-path=/var/run/secrets/tokens/konnectivity-agent-token"
]
volumeMounts:
- mountPath: /var/run/secrets/tokens
name: konnectivity-agent-token
livenessProbe:
httpGet:
port: 8134
path: /healthz