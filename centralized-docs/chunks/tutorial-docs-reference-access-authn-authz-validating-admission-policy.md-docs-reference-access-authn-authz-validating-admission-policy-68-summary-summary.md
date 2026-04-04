---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#68-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 125
summary: |Validate that the three fields defining replicas are ordered appropriately| |`'Available' in object.stateCounts`|Validate that an entry with the 'Available' key exists in a map|...
---

|Validate that the three fields defining replicas are ordered appropriately|
|`'Available' in object.stateCounts`|Validate that an entry with the 'Available' key exists in a map|
|`(size(object.list1) == 0) != (size(object.list2) == 0)`|Validate that one of two lists is non-empty, but not both|
|`!('MY\_KEY' in object.map1) || object['MY\_KEY'].matches('^[a-zA-Z]\*$')`|Validate the value of a map for a specific key, if it is in the map|
|