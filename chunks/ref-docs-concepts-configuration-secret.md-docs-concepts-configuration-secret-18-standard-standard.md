---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#18-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 378
summary: ### Creating a Secret There are several options to create a Secret: * [Use `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/) * [Use a configuration...
---

### Creating a Secret
There are several options to create a Secret:
* [Use `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/)
* [Use a configuration file](/docs/tasks/configmap-secret/managing-secret-using-config-file/)
* [Use the Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/)#### Constraints on Secret names and data
The name of a Secret object must be a valid
[DNS subdomain name](/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names).
You can specify the `data` and/or the `stringData` field when creating a
configuration file for a Secret. The `data` and the `stringData` fields are optional.
The values for all keys in the `data` field have to be base64-encoded strings.
If the conversion to base64 string is not desirable, you can choose to specify
the `stringData` field instead, which accepts arbitrary strings as values.
The keys of `data` and `stringData` must consist of alphanumeric characters,
`-`, `\_` or `.`. All key-value pairs in the `stringData` field are internally
merged into the `data` field. If a key appears in both the `data` and the
`stringData` field, the value specified in the `stringData` field takes
precedence.
#### Size limit
Individual Secrets are limited to 1MiB in size. This is to discourage creation
of very large Secrets that could exhaust the API server and kubelet memory.
However, creation of many smaller Secrets could also exhaust memory. You can
use a [resource quota](/docs/concepts/policy/resource-quotas/) to limit the
number of Secrets (or other resources) in a namespace.