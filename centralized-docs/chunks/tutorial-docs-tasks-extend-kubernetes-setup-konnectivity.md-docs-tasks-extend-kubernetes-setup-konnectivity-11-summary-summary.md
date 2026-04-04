---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#11-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 128
summary: `apiVersion: apiserver.k8s.io/v1beta1 kind: EgressSelectorConfiguration egressSelections: # Since we want to control the egress traffic to the cluster, we use the # \"cluster\" as the name. Other...
---

`apiVersion: apiserver.k8s.io/v1beta1
kind: EgressSelectorConfiguration
egressSelections:
# Since we want to control the egress traffic to the cluster, we use the
# "cluster" as the name. Other supported values are "etcd", and "controlplane".
- name: cluster
connection:
# server. Supported values are "GRPC" and "HTTPConnect". There is no
# end user visible difference between the two modes. You need to set the
# Konnectivity server to work in the same mode.
proxyProtocol: GRPC
transport: