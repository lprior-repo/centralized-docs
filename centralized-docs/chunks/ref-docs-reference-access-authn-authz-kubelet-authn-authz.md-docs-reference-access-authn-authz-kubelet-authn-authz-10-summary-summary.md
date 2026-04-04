---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#10-summary
chunk_level: summary
chunk_type: prose
heading: Kubelet authentication
token_count: 123
summary: `nodes/proxy` permission grants access to all other kubelet APIs. This includes APIs that can be used to execute commands in any container running on the node. Some of these endpoints support...
---

`nodes/proxy` permission grants access to all other kubelet APIs.
This includes APIs that can be used to execute commands in any container running on the node.
Some of these endpoints support Websocket protocols via HTTP `GET` requests, which are authorized with the **get** verb.
This means that **get** permission on `nodes/proxy` is not a read-only permission,
and authorizes executing commands in any container running on the node.
The namespace and API group attributes are always an empty string, and
the resource name is always the name of the kubelet's `Node` API object.