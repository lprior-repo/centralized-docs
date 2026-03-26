---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#11-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 121
summary: A Secret is an object that contains a small amount of sensitive data such as a password, a token, or a key. Such information might otherwise be put in a [Pod](/docs/concepts/workloads/pods/)...
---

A Secret is an object that contains a small amount of sensitive data such as
a password, a token, or a key. Such information might otherwise be put in a
[Pod](/docs/concepts/workloads/pods/) specification or in a
[container image](/docs/reference/glossary/?all=true#term-image). Using a
Secret means that you don't need to include confidential data in your
application code.
Because Secrets can be created independently of the Pods that use them, there
is less risk of the Secret (and its data) being exposed during the workflow of