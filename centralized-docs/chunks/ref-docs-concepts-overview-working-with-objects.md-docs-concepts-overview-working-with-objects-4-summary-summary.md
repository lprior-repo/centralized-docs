---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#4-summary
chunk_level: summary
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 127
summary: * The policies around how those applications behave, such as restart policies, upgrades, and fault-tolerance A Kubernetes object is a \"record of intent\"--once you create the object, the Kubernetes...
---

* The policies around how those applications behave, such as restart policies, upgrades, and fault-tolerance
A Kubernetes object is a "record of intent"--once you create the object, the Kubernetes system
will constantly work to ensure that the object exists. By creating an object, you're effectively
telling the Kubernetes system what you want your cluster's workload to look like; this is your
cluster's *desired state*.
To work with Kubernetes objects—whether to create, modify, or delete them—you'll need to use the
[Kubernetes API](/docs/concepts/overview/kubernetes-api/). When you use the