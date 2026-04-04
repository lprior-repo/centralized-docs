---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#13-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 73
summary: # The other supported transport is \"tcp\". You will need to set up TLS # config to secure the TCP transport. uds: udsName: /etc/kubernetes/konnectivity-server/konnectivity-server.socket ` ``` You need...
---

# The other supported transport is "tcp". You will need to set up TLS
# config to secure the TCP transport.
uds:
udsName: /etc/kubernetes/konnectivity-server/konnectivity-server.socket
`
```
You need to configure the API Server to use the Konnectivity service
and direct the network traffic to the cluster nodes: