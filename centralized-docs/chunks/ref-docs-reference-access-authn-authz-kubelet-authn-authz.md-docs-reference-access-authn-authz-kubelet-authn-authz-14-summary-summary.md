---
doc_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
chunk_id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz#14-summary
chunk_level: summary
chunk_type: table
heading: Kubelet authentication
token_count: 125
summary: are determined from the incoming request's path: |Kubelet API|resource|subresource| |/stats/\*|nodes|stats| |/metrics/\*|nodes|metrics| |/logs/\*|nodes|log| |/pods|nodes|pods, proxy|...
---

are determined from the incoming request's path:
|Kubelet API|resource|subresource|
|/stats/\*|nodes|stats|
|/metrics/\*|nodes|metrics|
|/logs/\*|nodes|log|
|/pods|nodes|pods, proxy|
|/runningPods/|nodes|pods, proxy|
|/healthz|nodes|healthz, proxy|
|/configz|nodes|configz, proxy|
|*all others*|nodes|proxy|
When the feature-gate `KubeletFineGrainedAuthz`