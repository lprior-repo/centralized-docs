---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#13-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: Instead of having to spell this out at each point,. one can declare this separately in a one blanket statement
---

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
