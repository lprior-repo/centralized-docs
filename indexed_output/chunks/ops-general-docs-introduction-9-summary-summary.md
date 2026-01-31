---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#9-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: 	name:    \"Moscow\". 	pop:     11
---


Data


Copy code
Copied!

moscow: {
	name:    "Moscow"
	pop:     11.92M
	capital: true
}

Schema


Copy code
Copied!

municipality: {
	name:    string
	pop:     int
	capital: bool
}

CUE


Copy code
Copied!

largeCapital: {
	name:    string
	pop:     >5M
	capital: true
}

In general, in CUE one starts with a broad definition of a type, describing
all possible instances.
One then narrows down these definitions, possibly by combining constraints
from different sources (departments, users), until a concrete data instance
