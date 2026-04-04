---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#11-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 59
summary: the resource name is always the name of the kubelet's `Node` API object. When running in this mode, ensure the user identified by the `--kubelet-client-certificate` and `--kubelet-client-key` flags...
---

the resource name is always the name of the kubelet's `Node` API object.
When running in this mode, ensure the user identified by the `--kubelet-client-certificate` and `--kubelet-client-key`
flags passed to the apiserver is authorized for the following attributes: