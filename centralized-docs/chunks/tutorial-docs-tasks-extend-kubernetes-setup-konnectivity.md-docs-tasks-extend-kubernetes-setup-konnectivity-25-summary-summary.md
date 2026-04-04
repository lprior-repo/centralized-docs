---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#25-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 124
summary: \"--uds-name=/etc/kubernetes/konnectivity-server/konnectivity-server.socket\", \"--delete-existing-uds-file\", # The following two lines assume the Konnectivity server is # deployed on the same machine...
---

"--uds-name=/etc/kubernetes/konnectivity-server/konnectivity-server.socket",
"--delete-existing-uds-file",
# The following two lines assume the Konnectivity server is
# deployed on the same machine as the apiserver, and the certs and
# key of the API Server are at the specified location.
"--cluster-cert=/etc/kubernetes/pki/apiserver.crt",
"--cluster-key=/etc/kubernetes/pki/apiserver.key",
# This needs to be consistent with the value set in egressSelectorConfiguration.
"--mode=grpc",
"--server-port=0",
"--agent-port=8132",