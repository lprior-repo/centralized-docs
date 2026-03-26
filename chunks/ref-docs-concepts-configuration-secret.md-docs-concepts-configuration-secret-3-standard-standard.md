---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 445
summary: # Secrets A Secret is an object that contains a small amount of sensitive data such as a password, a token, or a key. Such information might otherwise be put in a...
---

# Secrets
A Secret is an object that contains a small amount of sensitive data such as
a password, a token, or a key. Such information might otherwise be put in a
[Pod](/docs/concepts/workloads/pods/) specification or in a
[container image](/docs/reference/glossary/?all=true#term-image). Using a
Secret means that you don't need to include confidential data in your
application code.
Because Secrets can be created independently of the Pods that use them, there
is less risk of the Secret (and its data) being exposed during the workflow of
creating, viewing, and editing Pods. Kubernetes, and applications that run in
your cluster, can also take additional precautions with Secrets, such as avoiding
writing sensitive data to nonvolatile storage.
Secrets are similar to [ConfigMaps](/docs/concepts/configuration/configmap/)
but are specifically intended to hold confidential data.
#### Caution:
Kubernetes Secrets are, by default, stored unencrypted in the API server's underlying data store
(etcd). Anyone with API access can retrieve or modify a Secret, and so can anyone with access to etcd.
Additionally, anyone who is authorized to create a Pod in a namespace can use that access to read
any Secret in that namespace; this includes indirect access such as the ability to create a
Deployment.
In order to safely use Secrets, take at least the following steps:
1. [Enable Encryption at Rest](/docs/tasks/administer-cluster/encrypt-data/) for Secrets.
2. [Enable or configure RBAC rules](/docs/reference/access-authn-authz/authorization/) with
least-privilege access to Secrets.
3. Restrict Secret access to specific containers.
4. [Consider using external Secret store providers](https://secrets-store-csi-driver.sigs.k8s.io/concepts.html#provider-for-the-secrets-store-csi-driver).
For more guidelines to manage and improve the security of your Secrets, refer to
[Good practices for Kubernetes Secrets](/docs/concepts/security/secrets-good-practices/).
See [Information security for Secrets](#information-security-for-secrets) for more details.