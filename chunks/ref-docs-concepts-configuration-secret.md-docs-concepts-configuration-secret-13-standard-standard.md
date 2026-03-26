---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#13-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 319
summary: ### Basic authentication Secret The `kubernetes.io/basic-auth` type is provided for storing credentials needed for basic authentication. When using this Secret type, the `data` field of the Secret...
---

### Basic authentication Secret
The `kubernetes.io/basic-auth` type is provided for storing credentials needed
for basic authentication. When using this Secret type, the `data` field of the
Secret must contain one of the following two keys:
* `username`: the user name for authentication
* `password`: the password or token for authentication
Both values for the above two keys are base64 encoded strings. You can
alternatively provide the clear text content using the `stringData` field in the
Secret manifest.
The following manifest is an example of a basic authentication Secret:
[`secret/basicauth-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/basicauth-secret.yaml)![](/images/copycode.svg "Copy secret/basicauth-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: secret-basic-auth
type: kubernetes.io/basic-auth
stringData:
username: admin # required field for kubernetes.io/basic-auth
password: t0p-Secret # required field for kubernetes.io/basic-auth`
```
#### Note:
The `stringData` field for a Secret does not work well with server-side apply.
The basic authentication Secret type is provided only for convenience.
You can create an `Opaque` type for credentials used for basic authentication.
However, using the defined and public Secret type (`kubernetes.io/basic-auth`) helps other
people to understand the purpose of your Secret, and sets a convention for what key names
to expect.