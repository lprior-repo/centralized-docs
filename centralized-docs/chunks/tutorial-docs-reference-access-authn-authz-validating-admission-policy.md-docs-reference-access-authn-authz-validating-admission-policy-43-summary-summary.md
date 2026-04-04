---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#43-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 68
summary: Combining the two, we can provide a way to validate optional parameters: `!has(params.optionalNumber) || (params.optionalNumber &gt;= 5 &amp;&amp; params.optionalNumber &lt;= 10)` Here, we first...
---

Combining the two, we can provide a way to validate optional parameters:
`!has(params.optionalNumber) || (params.optionalNumber &gt;= 5 &amp;&amp; params.optionalNumber &lt;= 10)`
Here, we first check that the optional parameter is present with `!has(params.optionalNumber)`.