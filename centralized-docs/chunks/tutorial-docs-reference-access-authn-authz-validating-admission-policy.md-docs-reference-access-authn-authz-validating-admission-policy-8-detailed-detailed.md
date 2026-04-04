---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#8-detailed
chunk_level: detailed
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 755
summary: * 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and non-intersecting elements in `Y` are appended, retaining their partial order. * 'map': `X + Y`...
---

* 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
non-intersecting elements in `Y` are appended, retaining their partial order.
* 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
non-intersecting keys are appended, retaining their partial order.#### Validation expression examples
|Expression|Purpose|
|`object.minReplicas &lt;= object.replicas &amp;&amp; object.replicas &lt;= object.maxReplicas`|Validate that the three fields defining replicas are ordered appropriately|
|`'Available' in object.stateCounts`|Validate that an entry with the 'Available' key exists in a map|
|`(size(object.list1) == 0) != (size(object.list2) == 0)`|Validate that one of two lists is non-empty, but not both|
|`!('MY\_KEY' in object.map1) || object['MY\_KEY'].matches('^[a-zA-Z]\*$')`|Validate the value of a map for a specific key, if it is in the map|
|`object.envars.filter(e, e.name == 'MY\_ENV').all(e, e.value.matches('^[a-zA-Z]\*$')`|Validate the 'value' field of a listMap entry where key field 'name' is 'MY\_ENV'|
|`has(object.expired) &amp;&amp; object.created + object.ttl &lt; object.expired`|Validate that 'expired' date is after a 'create' date plus a 'ttl' duration|
|`object.health.startsWith('ok')`|Validate a 'health' string field has the prefix 'ok'|
|`object.widgets.exists(w, w.key == 'x' &amp;&amp; w.foo &lt; 10)`|Validate that the 'foo' property of a listMap item with a key 'x' is less than 10|
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