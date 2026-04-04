---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#70-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 116
summary: |Validate that 'expired' date is after a 'create' date plus a 'ttl' duration| |`object.health.startsWith('ok')`|Validate a 'health' string field has the prefix 'ok'| |`object.widgets.exists(w, w.key...
---

|Validate that 'expired' date is after a 'create' date plus a 'ttl' duration|
|`object.health.startsWith('ok')`|Validate a 'health' string field has the prefix 'ok'|
|`object.widgets.exists(w, w.key == 'x' &amp;&amp; w.foo &lt; 10)`|Validate that the 'foo' property of a listMap item with a key 'x' is less than 10|
|`type(object) == string ? object == '100%' : object == 1000`