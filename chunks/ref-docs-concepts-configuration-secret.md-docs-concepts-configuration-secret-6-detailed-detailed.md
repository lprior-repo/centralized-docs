---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#6-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of Secret
token_count: 669
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
### SSH authentication Secrets
The builtin type `kubernetes.io/ssh-auth` is provided for storing data used in
SSH authentication. When using this Secret type, you will have to specify a
`ssh-privatekey` key-value pair in the `data` (or `stringData`) field
as the SSH credential to use.
The following manifest is an example of a Secret used for SSH public/private
key authentication:
[`secret/ssh-auth-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/ssh-auth-secret.yaml)![](/images/copycode.svg "Copy secret/ssh-auth-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: secret-ssh-auth
type: kubernetes.io/ssh-auth
data:
# the data is abbreviated in this example
ssh-privatekey: |
UG91cmluZzYlRW1vdGljb24lU2N1YmE= `
```
The SSH authentication Secret type is provided only for convenience.
You can create an `Opaque` type for credentials used for SSH authentication.
However, using the defined and public Secret type (`kubernetes.io/ssh-auth`) helps other
people to understand the purpose of your Secret, and sets a convention for what key names
to expect.
The Kubernetes API verifies that the required keys are set for a Secret of this type.
#### Caution:
SSH private keys do not establish trusted communication between an SSH client and
host server on their own. A secondary means of establishing trust is needed to
mitigate "man in the middle" attacks, such as a `known\_hosts` file added to a ConfigMap.