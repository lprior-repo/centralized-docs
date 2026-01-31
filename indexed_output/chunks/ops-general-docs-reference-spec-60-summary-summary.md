---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#60-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 142
summary: {a: int, a: 1}                         {a: 1}. {a: int} & {a: 1}                      {a: 1}
---

{a: int, a: 1}                         {a: 1}
{a: int} & {a: 1}                      {a: 1}
{a: >=1 & <=7} & {a: >=5 & <=9}        {a: >=5 & <=7}
{a: >=1 & <=7, a: >=5 & <=9}           {a: >=5 & <=7}

{a: 1} & {b: 2}                        {a: 1, b: 2}
{a: 1, b: int} & {b: 2}                {a: 1, b: 2}

{a: 1} & {a: 2}                        _|_


FIELD CONSTRAINTS

A struct may declare field constraints which define values
that should be unified with a given field once it is defined.
The existence of a field constraint declares, but does not define, that field.
