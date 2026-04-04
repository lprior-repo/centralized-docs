---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#12-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 116
summary: You need to set the # Konnectivity server to work in the same mode. proxyProtocol: GRPC transport: # This controls what transport the API Server uses to communicate with the # Konnectivity server....
---

You need to set the
# Konnectivity server to work in the same mode.
proxyProtocol: GRPC
transport:
# This controls what transport the API Server uses to communicate with the
# Konnectivity server. UDS is recommended if the Konnectivity server
# locates on the same machine as the API Server. You need to configure the
# Konnectivity server to listen on the same UDS socket.
# The other supported transport is "tcp". You will need to set up TLS
# config to secure the TCP transport.
uds: