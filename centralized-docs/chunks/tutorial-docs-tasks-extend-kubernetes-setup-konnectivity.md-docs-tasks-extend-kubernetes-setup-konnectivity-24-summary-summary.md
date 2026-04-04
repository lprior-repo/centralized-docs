---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#24-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 124
summary: `apiVersion: v1 kind: Pod metadata: name: konnectivity-server namespace: kube-system spec: priorityClassName: system-cluster-critical hostNetwork: true containers: - name:...
---

`apiVersion: v1
kind: Pod
metadata:
name: konnectivity-server
namespace: kube-system
spec:
priorityClassName: system-cluster-critical
hostNetwork: true
containers:
- name: konnectivity-server-container
image: registry.k8s.io/kas-network-proxy/proxy-server:v0.0.37
command: ["/proxy-server"]
args: [
"--logtostderr=true",
# This needs to be consistent with the value set in egressSelectorConfiguration.
"--uds-name=/etc/kubernetes/konnectivity-server/konnectivity-server.socket",