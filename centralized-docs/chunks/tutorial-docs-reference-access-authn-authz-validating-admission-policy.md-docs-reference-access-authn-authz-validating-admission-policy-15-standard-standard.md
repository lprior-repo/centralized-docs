---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#15-standard
chunk_level: standard
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 308
summary: 'x' is less than 10| |`type(object) == string ? object == '100%' : object == 1000`|Validate an int-or-string field for both the int and string cases|...
---

'x' is less than 10|
|`type(object) == string ? object == '100%' : object == 1000`|Validate an int-or-string field for both the int and string cases|
|`object.metadata.name.startsWith(object.prefix)`|Validate that an object's name has the prefix of another field value|
|`object.set1.all(e, !(e in object.set2))`|Validate that two listSets are disjoint|
|`size(object.names) == size(object.details) &amp;&amp; object.names.all(n, n in object.details)`|Validate the 'details' map is keyed by the items in the 'names' listSet|
|`size(object.clusters.filter(c, c.name == object.primary)) == 1`|Validate that the 'primary' property has one and only one occurrence in the 'clusters' listMap|
Read [Supported evaluation on CEL](https://github.com/google/cel-spec/blob/v0.6.0/doc/langdef.md#evaluation)
for more information about CEL rules.
`spec.validation[i].reason` represents a machine-readable description of why this validation failed.
If this is the first validation in the list to fail, this reason, as well as the corresponding
HTTP response code, are used in the HTTP response to the client.
The currently supported reasons are: `Unauthorized`, `Forbidden`, `Invalid`, `RequestEntityTooLarge`.
If not set, `StatusReasonInvalid` is used in the response to the client.