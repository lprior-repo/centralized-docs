---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#62-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 128
summary: The builtin type `kubernetes.io/ssh-auth` is provided for storing data used in SSH authentication. When using this Secret type, you will have to specify a `ssh-privatekey` key-value pair in the...
---

The builtin type `kubernetes.io/ssh-auth` is provided for storing data used in
SSH authentication. When using this Secret type, you will have to specify a
`ssh-privatekey` key-value pair in the `data` (or `stringData`) field
as the SSH credential to use.
The following manifest is an example of a Secret used for SSH public/private
key authentication:
[`secret/ssh-auth-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/ssh-auth-secret.yaml)![](/images/copycode.svg "Copy secret/ssh-auth-secret.yaml to clipboard")