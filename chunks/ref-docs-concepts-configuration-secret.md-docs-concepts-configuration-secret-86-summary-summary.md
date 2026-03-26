---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#86-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 125
summary: * [Use the Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/)#### Constraints on Secret names and data The name of a Secret object must be a valid [DNS subdomain...
---

* [Use the Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/)#### Constraints on Secret names and data
The name of a Secret object must be a valid
[DNS subdomain name](/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names).
You can specify the `data` and/or the `stringData` field when creating a
configuration file for a Secret. The `data` and the `stringData` fields are optional.
The values for all keys in the `data` field have to be base64-encoded strings.