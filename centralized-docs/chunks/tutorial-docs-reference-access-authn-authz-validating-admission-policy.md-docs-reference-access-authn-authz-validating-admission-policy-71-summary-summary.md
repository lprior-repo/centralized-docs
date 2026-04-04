---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#71-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 125
summary: 'x' is less than 10| |`type(object) == string ? object == '100%' : object == 1000`|Validate an int-or-string field for both the int and string cases|...
---

'x' is less than 10|
|`type(object) == string ? object == '100%' : object == 1000`|Validate an int-or-string field for both the int and string cases|
|`object.metadata.name.startsWith(object.prefix)`|Validate that an object's name has the prefix of another field value|
|`object.set1.all(e, !(e in object.set2))`|Validate that two listSets are disjoint|
|`size(object.names) == size(object.details) &amp;&amp; object.names.all(n, n in object.details)`|Validate the 'details'