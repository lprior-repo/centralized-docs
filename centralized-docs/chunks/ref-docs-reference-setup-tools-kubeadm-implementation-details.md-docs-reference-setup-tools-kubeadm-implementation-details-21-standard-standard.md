---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#21-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 393
summary: #### Set up auto approval for new bootstrap tokens Kubeadm ensures that the Bootstrap Token will get its CSR request automatically approved by the csrapprover controller. This is implemented by...
---

#### Set up auto approval for new bootstrap tokens
Kubeadm ensures that the Bootstrap Token will get its CSR request automatically approved by the
csrapprover controller.
This is implemented by creating ClusterRoleBinding named `kubeadm:node-autoapprove-bootstrap`
between the `system:bootstrappers:kubeadm:default-node-token` group and the default role
`system:certificates.k8s.io:certificatesigningrequests:nodeclient`.
The role `system:certificates.k8s.io:certificatesigningrequests:nodeclient` should be created as
well, granting POST permission to
`/apis/certificates.k8s.io/certificatesigningrequests/nodeclient`.
#### Set up nodes certificate rotation with auto approval
Kubeadm ensures that certificate rotation is enabled for nodes, and that a new certificate request
for nodes will get its CSR request automatically approved by the csrapprover controller.
This is implemented by creating ClusterRoleBinding named
`kubeadm:node-autoapprove-certificate-rotation` between the `system:nodes` group and the default
role `system:certificates.k8s.io:certificatesigningrequests:selfnodeclient`.
#### Create the public cluster-info ConfigMap
This phase creates the `cluster-info` ConfigMap in the `kube-public` namespace.
Additionally, it creates a Role and a RoleBinding granting access to the ConfigMap for
unauthenticated users (i.e. users in RBAC group `system:unauthenticated`).
#### Note:
The access to the `cluster-info` ConfigMap *is not* rate-limited. This may or may not be a
problem if you expose your cluster's API server to the internet; worst-case scenario here is a
DoS attack where an attacker uses all the in-flight requests the kube-apiserver can handle to
serve the `cluster-info` ConfigMap.