---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#58-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 119
summary: * `username`: the user name for authentication * `password`: the password or token for authentication Both values for the above two keys are base64 encoded strings. You can alternatively provide the...
---

* `username`: the user name for authentication
* `password`: the password or token for authentication
Both values for the above two keys are base64 encoded strings. You can
alternatively provide the clear text content using the `stringData` field in the
Secret manifest.
The following manifest is an example of a basic authentication Secret:
[`secret/basicauth-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/basicauth-secret.yaml)![](/images/copycode.svg "Copy secret/basicauth-secret.yaml to clipboard")