---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#21-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 110
summary: Next, you need to deploy the Konnectivity server and agents. [kubernetes-sigs/apiserver-network-proxy](https://github.com/kubernetes-sigs/apiserver-network-proxy) is a reference implementation....
---

Next, you need to deploy the Konnectivity server and agents.
[kubernetes-sigs/apiserver-network-proxy](https://github.com/kubernetes-sigs/apiserver-network-proxy)
is a reference implementation.
Deploy the Konnectivity server on your control plane node. The provided
`konnectivity-server.yaml` manifest assumes
that the Kubernetes components are deployed as a [static Pod](/docs/tasks/configure-pod-container/static-pod/) in your cluster. If not, you can deploy the Konnectivity
server as a DaemonSet.