---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#10-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 343
summary: 1. If a given certificate and private key pair both exist, and their content is evaluated to be compliant with the above specs, the existing files will be used and the generation phase for the given...
---

1. If a given certificate and private key pair both exist, and their content is evaluated to be compliant with the above specs, the existing files will
be used and the generation phase for the given certificate will be skipped. This means the user can, for example, copy an existing CA to
`/etc/kubernetes/pki/ca.{crt,key}`, and then kubeadm will use those files for signing the rest of the certs.
See also [using custom certificates](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#custom-certificates)
2. For the CA, it is possible to provide the `ca.crt` file but not the `ca.key` file. If all other certificates and kubeconfig files
are already in place, kubeadm recognizes this condition and activates the ExternalCA, which also implies the `csrsigner` controller in
controller-manager won't be started
3. If kubeadm is running in [external CA mode](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/#external-ca-mode);
all the certificates must be provided by the user, because kubeadm cannot generate them by itself
4. In case kubeadm is executed in the `--dry-run` mode, certificate files are written in a temporary folder
5. Certificate generation can be invoked individually with the
[`kubeadm init phase certs all`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-certs) command### Generate kubeconfig files for control plane components
Kubeadm generates kubeconfig files with identities for control plane components: