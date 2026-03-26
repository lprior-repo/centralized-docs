---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#7-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of Secret
token_count: 488
summary: ### TLS Secrets The `kubernetes.io/tls` Secret type is for storing a certificate and its associated key that are typically used for TLS. One common use for TLS Secrets is to configure encryption in...
---

### TLS Secrets
The `kubernetes.io/tls` Secret type is for storing
a certificate and its associated key that are typically used for TLS.
One common use for TLS Secrets is to configure encryption in transit for
an [Ingress](/docs/concepts/services-networking/ingress/), but you can also use it
with other resources or directly in your workload.
When using this type of Secret, the `tls.key` and the `tls.crt` key must be provided
in the `data` (or `stringData`) field of the Secret configuration, although the API
server doesn't actually validate the values for each key.
As an alternative to using `stringData`, you can use the `data` field to provide
the base64 encoded certificate and private key. For details, see
[Constraints on Secret names and data](#restriction-names-data).
The following YAML contains an example config for a TLS Secret:
[`secret/tls-auth-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/tls-auth-secret.yaml)![](/images/copycode.svg "Copy secret/tls-auth-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: secret-tls
type: kubernetes.io/tls
data:
# values are base64 encoded, which obscures them but does NOT provide
# Replace the following values with your own base64-encoded certificate and key.
tls.crt: "REPLACE\_WITH\_BASE64\_CERT"
tls.key: "REPLACE\_WITH\_BASE64\_KEY"`
```
The TLS Secret type is provided only for convenience.
You can create an `Opaque` type for credentials used for TLS authentication.
However, using the defined and public Secret type (`kubernetes.io/tls`)
helps ensure the consistency of Secret format in your project. The API server
verifies if the required keys are set for a Secret of this type.
To create a TLS Secret using `kubectl`, use the `tls` subcommand:
```
`kubectl create secret tls my-tls-secret \\
--cert=path/to/cert/file \\
--key=path/to/key/file
`
```
The public/private key pair must exist before hand. The public key certificate for `--cert` must be .PEM encoded
and must match the given private key for `--key`.