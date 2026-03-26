---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#30-summary
chunk_level: summary
chunk_type: prose
heading: Uses for Secrets
token_count: 117
summary: * You can use a [device plugin](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/) to expose node-local encryption hardware to a specific Pod. For example, you can schedule trusted...
---

* You can use a [device plugin](/docs/concepts/extend-kubernetes/compute-storage-net/device-plugins/)
to expose node-local encryption hardware to a specific Pod. For example, you can schedule
trusted Pods onto nodes that provide a Trusted Platform Module, configured out-of-band.
You can also combine two or more of those options, including the option to use Secret objects themselves.
For example: implement (or deploy) an [operator](/docs/concepts/extend-kubernetes/operator/)
that fetches short-lived session tokens from an external service, and then creates Secrets based