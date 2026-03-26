---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#10-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 428
summary: #### Note: You should only create a ServiceAccount token Secret if you can't use the `TokenRequest` API to obtain a token, and the security exposure of persisting a non-expiring token credential in a...
---

#### Note:
You should only create a ServiceAccount token Secret
if you can't use the `TokenRequest` API to obtain a token,
and the security exposure of persisting a non-expiring token credential
in a readable API object is acceptable to you. For instructions, see
[Manually create a long-lived API token for a ServiceAccount](/docs/tasks/configure-pod-container/configure-service-account/#manually-create-an-api-token-for-a-serviceaccount).
When using this Secret type, you need to ensure that the
`kubernetes.io/service-account.name` annotation is set to an existing
ServiceAccount name. If you are creating both the ServiceAccount and
the Secret objects, you should create the ServiceAccount object first.
After the Secret is created, a Kubernetes [controller](/docs/concepts/architecture/controller/)
fills in some other fields such as the `kubernetes.io/service-account.uid` annotation, and the
`token` key in the `data` field, which is populated with an authentication token.
The following example configuration declares a ServiceAccount token Secret:
[`secret/serviceaccount-token-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/serviceaccount-token-secret.yaml)![](/images/copycode.svg "Copy secret/serviceaccount-token-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Secret
metadata:
name: secret-sa-sample
annotations:
kubernetes.io/service-account.name: "sa-name"
type: kubernetes.io/service-account-token
data:
extra: YmFyCg==`
```
After creating the Secret, wait for Kubernetes to populate the `token` key in the `data` field.
See the [ServiceAccount](/docs/concepts/security/service-accounts/)
documentation for more information on how ServiceAccounts work.
You can also check the `automountServiceAccountToken` field and the
`serviceAccountName` field of the
[`Pod`](/docs/reference/generated/kubernetes-api/v1.35/#pod-v1-core)
for information on referencing ServiceAccount credentials from within Pods.