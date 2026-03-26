---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#61-summary
chunk_level: summary
chunk_type: prose
heading: `Issuer`
token_count: 122
summary: | |`egressSelectorType` [`EgressSelectorType`](#apiserver-k8s-io-v1beta1-EgressSelectorType)| egressSelectorType is an indicator of which egress selection should be used for sending all traffic...
---

|
|`egressSelectorType`
[`EgressSelectorType`](#apiserver-k8s-io-v1beta1-EgressSelectorType)|
egressSelectorType is an indicator of which egress selection should be used for sending all traffic related
to this issuer (discovery, JWKS, distributed claims, etc). If unspecified, no custom dialer is used.
When specified, the valid choices are "controlplane" and "cluster". These correspond to the associated
values in the --egress-selector-config-file.
* controlplane: for traffic intended to go to the control plane.