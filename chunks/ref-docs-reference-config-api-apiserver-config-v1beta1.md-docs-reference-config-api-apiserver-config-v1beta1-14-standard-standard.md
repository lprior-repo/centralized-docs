---
doc_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1
chunk_id: ref/docs-reference-config-api-apiserver-config-v1beta1.md/docs-reference-config-api-apiserver-config-v1beta1#14-standard
chunk_level: standard
chunk_type: prose
heading: `Issuer`
token_count: 331
summary: 1. \"MatchAny\" when multiple audiences are specified and 2. empty (or unset) or \"MatchAny\" when a single audience is specified. * MatchAny: the \"aud\" claim in the presented JWT must match at least one...
---

1. "MatchAny" when multiple audiences are specified and
2. empty (or unset) or "MatchAny" when a single audience is specified.
* MatchAny: the "aud" claim in the presented JWT must match at least one of the entries in the "audiences" field.
For example, if "audiences" is ["foo", "bar"], the "aud" claim in the presented JWT must contain either "foo" or "bar" (and may contain both).
* "": The match policy can be empty (or unset) when a single audience is specified in the "audiences" field. The "aud" claim in the presented JWT must contain the single audience (and may contain others).
For more nuanced audience validation, use claimValidationRules.
example: claimValidationRule[].expression: 'sets.equivalent(claims.aud, ["bar", "foo", "baz"])' to require an exact match.
|
|`egressSelectorType`
[`EgressSelectorType`](#apiserver-k8s-io-v1beta1-EgressSelectorType)|
egressSelectorType is an indicator of which egress selection should be used for sending all traffic related
to this issuer (discovery, JWKS, distributed claims, etc). If unspecified, no custom dialer is used.
When specified, the valid choices are "controlplane" and "cluster". These correspond to the associated
values in the --egress-selector-config-file.
* controlplane: for traffic intended to go to the control plane.
* cluster: for traffic intended to go to the system being managed by Kubernetes.
|