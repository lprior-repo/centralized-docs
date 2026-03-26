---
doc_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1
chunk_id: ref/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1.md/docs-reference-kubernetes-api-policy-resources-validating-admission-policy-v1#56-summary
chunk_level: summary
chunk_type: prose
heading: ValidatingAdmissionPolicy
token_count: 119
summary: * Expression accessing a property named \"x-prop\": {\"Expression\": \"object.x\_\_dash\_\_prop &gt; 0\"} * Expression accessing a property named \"redact\_\_d\": {\"Expression\":...
---

* Expression accessing a property named "x-prop": {"Expression": "object.x\_\_dash\_\_prop &gt; 0"}
* Expression accessing a property named "redact\_\_d": {"Expression": "object.redact\_\_underscores\_\_d &gt; 0"}
Equality on arrays with list type of 'set' or 'map' ignores element order, i.e. [1, 2] == [2, 1]. Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type: