---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#92-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 120
summary: * [Use a configuration file](/docs/tasks/configmap-secret/managing-secret-using-config-file/#edit-secret) You can also edit the data in a Secret using the [Kustomize...
---

* [Use a configuration file](/docs/tasks/configmap-secret/managing-secret-using-config-file/#edit-secret)
You can also edit the data in a Secret using the [Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/#edit-secret). However, this
method creates a new `Secret` object with the edited data.
Depending on how you created the Secret, as well as how the Secret is used in
your Pods, updates to existing `Secret` objects are propagated automatically to
Pods that use the data. For more information, refer to