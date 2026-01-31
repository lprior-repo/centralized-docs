---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#6-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 524
summary: Default values do not change this property; they syntactically appear as. non-concrete values
---

Default values do not change this property; they syntactically appear as
non-concrete values.
CUE also bails out and requires explicit values if two conflicting defaults
are specified for the same field, again limiting the search space.

With approaches that allow overrides, whether it be the complex inheritance
used in languages like GCL and Jsonnet
or the much simpler file-based approaches as used in HCL and Kustomize,
finding a declaration for a concrete field value does not guarantee
a final answer,
because another concrete value that occurs elsewhere can override it.
When one needs to change a value of such a field,
it can be time-consuming and,
especially when under pressure,
very tempting to skip following complicated inheritance chains,
double-check a configuration file specifying overlay order,
or look for a file that is lexically sorted after the one under consideration.

So there is a clear benefit to having fully expanded configurations
over such override methods.
CUE simulates that benefit by guaranteeing that any observed field value
holds for the final result.

If the user makes the false assumption that no concrete value is specified to discard the default value,
CUE will catch an erroneous change to that value and report the conflicting
locations.

But there is more.
In CUE one can apply a constraint to a group of values at once,
even across files.
Once set, there is no need to look at the individual values and files to
know these constraints apply.
Such information is not readily available for
fully expanded configurations.1
But also with inheritance-based solutions
that allow arbitrary overrides, templates give little information.

The ability to enforce constraints top down is crucial for any
large-scale configuration setup.
GCL and Jsonnet address this with assertions.
Assertions, however, are typically decoupled from their fields,
making them both hard to discover and hard to reason about.
Where CUE simplifies constraints
(>=3 & <=10 and >=5 & <=20 become >=5 & <=10, >=1 & <=1 becomes 1),
GCL and Jsonnet do not (it would be quite complex),
