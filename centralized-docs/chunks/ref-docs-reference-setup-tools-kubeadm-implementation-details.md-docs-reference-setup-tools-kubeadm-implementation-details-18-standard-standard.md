---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#18-standard
chunk_level: standard
chunk_type: prose
heading: kubeadm init workflow internal design
token_count: 318
summary: ### Generate static Pod manifest for local etcd If you specified an external etcd, this step will be skipped, otherwise kubeadm generates a static Pod manifest file for creating a local etcd instance...
---

### Generate static Pod manifest for local etcd
If you specified an external etcd, this step will be skipped, otherwise kubeadm generates a
static Pod manifest file for creating a local etcd instance running in a Pod with following attributes:
* listen on `localhost:2379` and use `HostNetwork=true`
* make a `hostPath` mount out from the `dataDir` to the host's filesystem
* Any extra flags specified by the user
Please note that:
1. The etcd container image will be pulled from `registry.gcr.io` by default. See
[using custom images](/docs/reference/setup-tools/kubeadm/kubeadm-init/#custom-images)
for customizing the image repository.
2. If you run kubeadm in `--dry-run` mode, the etcd static Pod manifest is written
into a temporary folder.
3. You can directly invoke static Pod manifest generation for local etcd, using the
[`kubeadm init phase etcd local`](/docs/reference/setup-tools/kubeadm/kubeadm-init-phase/#cmd-phase-etcd)
command.### Wait for the control plane to come up
On control plane nodes, kubeadm waits up to 4 minutes for the control plane components
and the kubelet to be available. It does that by performing a health check on the respective
component `/healthz` or `/livez` endpoints.
After the control plane is up, kubeadm completes the tasks described in following paragraphs.