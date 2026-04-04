---
doc_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler
chunk_id: ref/docs-concepts-scheduling-eviction-kube-scheduler.md/docs-concepts-scheduling-eviction-kube-scheduler#11-summary
chunk_level: summary
chunk_type: prose
heading: kube-scheduler
token_count: 103
summary: 1. [Scheduling Policies](/docs/reference/scheduling/policies/) allow you to configure *Predicates* for filtering and *Priorities* for scoring. 2. [Scheduling...
---

1. [Scheduling Policies](/docs/reference/scheduling/policies/) allow you to configure *Predicates* for filtering and *Priorities* for scoring.
2. [Scheduling Profiles](/docs/reference/scheduling/config/#profiles) allow you to configure Plugins that implement different scheduling stages, including: `QueueSort`, `Filter`, `Score`, `Bind`, `Reserve`, `Permit`, and others. You can also configure the kube-scheduler to run different profiles.## What's next