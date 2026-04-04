---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#69-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 122
summary: |Validate the value of a map for a specific key, if it is in the map| |`object.envars.filter(e, e.name == 'MY\_ENV').all(e, e.value.matches('^[a-zA-Z]\*$')`|Validate the 'value' field of a listMap...
---

|Validate the value of a map for a specific key, if it is in the map|
|`object.envars.filter(e, e.name == 'MY\_ENV').all(e, e.value.matches('^[a-zA-Z]\*$')`|Validate the 'value' field of a listMap entry where key field 'name' is 'MY\_ENV'|
|`has(object.expired) &amp;&amp; object.created + object.ttl &lt; object.expired`|Validate that 'expired' date is after a 'create' date plus a 'ttl' duration|
|