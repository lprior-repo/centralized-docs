---
doc_id: ops/general/docs-tour-types
chunk_id: ops/general/docs-tour-types#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: CUE doesn’t distinguish between. types and values [/docs/tour/basics/types-are-values/]
---

CUE doesn’t distinguish between
types and values [/docs/tour/basics/types-are-values/].
The term “type” merely refers to the kind of a value,
which may or may not be a concrete instance.

In the following hypothetical example, point defines an arbitrary point,
while xaxis and yaxis define any point on their respective axes.
We say that point, xaxis, and yaxis are incomplete,
as they don’t contain values that specify a specific point.
Incomplete values cannot be included when exporting to formats that can’t
