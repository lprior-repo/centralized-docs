---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#8-summary
chunk_level: summary
chunk_type: table
heading: Kubelet authentication
token_count: 100
summary: |PUT|update| |PATCH|patch| |DELETE|delete| The resource and subresource is determined from the incoming request's path: |Kubelet API|resource|subresource| |/stats/\*|nodes|stats|...
---

|PUT|update|
|PATCH|patch|
|DELETE|delete|
The resource and subresource is determined from the incoming request's path:
|Kubelet API|resource|subresource|
|/stats/\*|nodes|stats|
|/metrics/\*|nodes|metrics|
|/logs/\*|nodes|log|
|/spec/\*|nodes|spec|
|/checkpoint/\*|nodes|checkpoint|
|*all others*|nodes|proxy|