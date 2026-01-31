---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#3-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 521
summary: could be represented as. \"b\": \"c\": \"foo\"
---

    "b": {
        "c": "foo"
    }
}

could be represented as


Copy code
Copied!

"a": 3
"b": "c": "foo"

All the information of the original JSON file is retained in this
representation.

CUE generalizes this notion to the following pattern:


Copy code
Copied!

<set of nodes>: <constraints>

Each field declaration in CUE defines a set of nodes to which to apply
a specific constraint.
Because order doesn’t matter, multiple constraints can be applied to the
same nodes, all of which need to apply simultaneously.
Such constraints may even be in different files.
But they may never contradict each other:
if one declaration says a field is 5, another may not override it to be 6.
Declaring a field to be both >5 and <10 is valid, though.

This approach is more restricted than full-blown inheritance;
it may not be possible to reuse existing configurations.
On the other hand, it is also a more powerful boilerplate remover.
For instance, suppose each job in a set needs to use a specific
template.
Instead of having to spell this out at each point,
one can declare this separately in a one blanket statement.

So instead of


Copy code
Copied!

jobs: {
	foo: acmeMonitoring & {...}
	bar: acmeMonitoring & {...}
	baz: acmeMonitoring & {...}
}

one can write


Copy code
Copied!

jobs: [string]: acmeMonitoring

jobs: {
	foo: {...}
	bar: {...}
	baz: {...}
}

There is no need to repeat the reference to the monitoring template for
each job, as the first already states that all jobs must use acmeMonitoring.
Such requirements can be specified across files.

This approach not only reduces the boilerplate contained in acmeMonitoring
but also removes the repetitiveness of having to specify
this template for each job in jobs.
At the same time, this statement acts as a type enforcement.
This dual function is a key aspect of CUE and
typed feature structure languages in general.

This approach breaks down, of course, if the restrictions in
acmeMonitoring are too stringent and jobs need to override them.
To this extent, CUE provides mechanisms to allow defaults, opt-out, and
