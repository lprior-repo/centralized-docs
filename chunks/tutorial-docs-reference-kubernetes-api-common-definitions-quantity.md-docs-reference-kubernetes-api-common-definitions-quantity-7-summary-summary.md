---
doc_id: tutorial/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity
chunk_id: tutorial/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity#7-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 127
summary: No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise...
---

No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
Before serializing, Quantity will be put in "canonical form"