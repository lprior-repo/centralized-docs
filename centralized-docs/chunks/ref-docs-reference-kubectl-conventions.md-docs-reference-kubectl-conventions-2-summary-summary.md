---
doc_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions
chunk_id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions#2-summary
chunk_level: summary
chunk_type: prose
heading: Using `kubectl` in Reusable Scripts
token_count: 89
summary: * Request one of the machine-oriented output forms, such as `-o name`, `-o json`, `-o yaml`, `-o go-template`, or `-o jsonpath`. * Fully-qualify the version. For example, `jobs.v1.batch/myjob`. This...
---

* Request one of the machine-oriented output forms, such as `-o name`, `-o json`, `-o yaml`, `-o go-template`, or `-o jsonpath`.
* Fully-qualify the version. For example, `jobs.v1.batch/myjob`. This will ensure that kubectl does not use its default version that can change over time.
* Don't rely on context, preferences, or other implicit states.## Subresources