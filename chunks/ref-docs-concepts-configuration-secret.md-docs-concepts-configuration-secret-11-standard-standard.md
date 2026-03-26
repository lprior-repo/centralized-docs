---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#11-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 347
summary: ### Docker config Secrets If you are creating a Secret to store credentials for accessing a container image registry, you must use one of the following `type` values for that Secret: *...
---

### Docker config Secrets
If you are creating a Secret to store credentials for accessing a container image registry,
you must use one of the following `type` values for that Secret:
* `kubernetes.io/dockercfg`: store a serialized `\~/.dockercfg` which is the
legacy format for configuring Docker command line. The Secret
`data` field contains a `.dockercfg` key whose value is the content of a
base64 encoded `\~/.dockercfg` file.
* `kubernetes.io/dockerconfigjson`: store a serialized JSON that follows the
same format rules as the `\~/.docker/config.json` file, which is a new format
for `\~/.dockercfg`. The Secret `data` field must contain a
`.dockerconfigjson` key for which the value is the content of a base64
encoded `\~/.docker/config.json` file.
Below is an example for a `kubernetes.io/dockercfg` type of Secret:
[`secret/dockercfg-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/dockercfg-secret.yaml)![](/images/copycode.svg "Copy secret/dockercfg-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: secret-dockercfg
type: kubernetes.io/dockercfg
data:
.dockercfg: |
eyJhdXRocyI6eyJodHRwczovL2V4YW1wbGUvdjEvIjp7ImF1dGgiOiJvcGVuc2VzYW1lIn19fQo= `
```