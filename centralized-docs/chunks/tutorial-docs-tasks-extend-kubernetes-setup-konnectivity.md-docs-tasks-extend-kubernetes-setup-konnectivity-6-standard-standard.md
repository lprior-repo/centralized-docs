---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#6-standard
chunk_level: standard
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 171
summary: Next, you need to deploy the Konnectivity server and agents. [kubernetes-sigs/apiserver-network-proxy](https://github.com/kubernetes-sigs/apiserver-network-proxy) is a reference implementation....
---

Next, you need to deploy the Konnectivity server and agents.
[kubernetes-sigs/apiserver-network-proxy](https://github.com/kubernetes-sigs/apiserver-network-proxy)
is a reference implementation.
Deploy the Konnectivity server on your control plane node. The provided
`konnectivity-server.yaml` manifest assumes
that the Kubernetes components are deployed as a [static Pod](/docs/tasks/configure-pod-container/static-pod/) in your cluster. If not, you can deploy the Konnectivity
server as a DaemonSet.
[`admin/konnectivity/konnectivity-server.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/konnectivity-server.yaml)![](/images/copycode.svg "Copy admin/konnectivity/konnectivity-server.yaml to clipboard")