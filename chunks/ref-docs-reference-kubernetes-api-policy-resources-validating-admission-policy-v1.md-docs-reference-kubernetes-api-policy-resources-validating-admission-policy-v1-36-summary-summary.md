---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#36-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 128
summary: \"runlevel\" of \"0\" or \"1\"; you will set the selector as follows: \"namespaceSelector\": { \"matchExpressions\": [ { \"key\": \"runlevel\", \"operator\": \"NotIn\", \"values\": [ \"0\", \"1\" ] } ] } If instead you want...
---

"runlevel" of "0" or "1"; you will set the selector as follows: "namespaceSelector": {
"matchExpressions": [
{
"key": "runlevel",
"operator": "NotIn",
"values": [
"0",
"1"
]
}
]
}
If instead you want to only run the policy on any objects whose namespace is associated with the "environment" of "prod" or "staging"; you will set the selector as follows: "namespaceSelector": {
"matchExpressions": [
{
"key": "environment",
"operator": "In",
"values": [
"prod",
"