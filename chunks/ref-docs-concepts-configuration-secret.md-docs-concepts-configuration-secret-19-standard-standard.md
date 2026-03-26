---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#19-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 199
summary: ### Editing a Secret You can edit an existing Secret unless it is [immutable](#secret-immutable). To edit a Secret, use one of the following methods: * [Use...
---

### Editing a Secret
You can edit an existing Secret unless it is [immutable](#secret-immutable). To
edit a Secret, use one of the following methods:
* [Use `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/#edit-secret)
* [Use a configuration file](/docs/tasks/configmap-secret/managing-secret-using-config-file/#edit-secret)
You can also edit the data in a Secret using the [Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/#edit-secret). However, this
method creates a new `Secret` object with the edited data.
Depending on how you created the Secret, as well as how the Secret is used in
your Pods, updates to existing `Secret` objects are propagated automatically to
Pods that use the data. For more information, refer to [Using Secrets as files from a Pod](#using-secrets-as-files-from-a-pod) section.