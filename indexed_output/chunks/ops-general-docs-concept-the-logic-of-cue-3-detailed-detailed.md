---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1029
summary: In other words, an instance may never violate the constraints of its parent. This property makes it very hard to inadvertently make false conclusions in CUE
---

In other words, an instance may never violate the constraints of its parent.
This property makes it very hard to inadvertently make false conclusions in CUE.
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
causing an ever-growing pile of assertions.


SEMANTICS

CUE defaults, which are values marked with a * in disjunctions,
preserve the beneficial properties of the lattice.
In order to do so,
CUE must ensure that the order of picking defaults does not influence the outcome.
Suppose we define two fields, each with the same default value.
We also define that these fields are equal to each other.


Copy code
Copied!

a: int | *1
b: int | *1
a: b
b: a

This is fine.
The obvious answer is a: 1, b: 1.

But now suppose we change one of the default values:


Copy code
Copied!

a: int | *1
b: int | *2
a: b
b: a

What should the answer be?
Picking either 1 or 2 as the default would result in a resolution of the
constraints, but would also be highly undesirable, as the result would depend
on the mood of the implementation.
This also starts to smell like an NP-complete constraint solving problem.
(Basic graph unification itself is pseudo linear.)
CUE wants no part of these shenanigans.
So the answer in this case is that there are no concrete values
as the defaults cannot be used.

The model for this is actually quite simple.
Conceptually, CUE keeps two parallel values, one for all possible values
and one for the default, which must be an instance of the former.
Roughly speaking, for the example with the conflict,
it simultaneously evaluates:


Copy code
Copied!

// All allowed values
a: int
b: int
a: b
b: a


Copy code
Copied!

// Default
a: 1
b: 2
a: b
b: a

Equating a and b clearly results in a conflict (1 != 2) and each will
result in _|_, leaving the left values as the only viable answer.

Now consider the two values corresponding to the original example:


Copy code
Copied!

// All allowed values
a: int
b: int
a: b
b: a


Copy code
Copied!

// Default
a: 1
b: 1
a: b
b: a

Here the defaults are not in conflict and can safely be returned.
