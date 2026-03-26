---
doc_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability
chunk_id: ops/docs-setup-production-environment-tools-kubeadm-high-availability.md/docs-setup-production-environment-tools-kubeadm-high-availability#35-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 113
summary: ``` `sudo kubeadm init phase upload-certs --upload-certs ` ``` * You can also specify a custom `--certificate-key` during `init` that can later be used by `join`. To generate such a key you can use...
---

```
`sudo kubeadm init phase upload-certs --upload-certs
`
```
* You can also specify a custom `--certificate-key` during `init` that can later be used by `join`.
To generate such a key you can use the following command:
```
`kubeadm certs certificate-key
`
```
The certificate key is a hex encoded string that is an AES key of size 32 bytes.
#### Note:
The `kubeadm-certs` Secret and the decryption key expire after two hours.