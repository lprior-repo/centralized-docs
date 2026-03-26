---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#7-standard
chunk_level: standard
chunk_type: table
heading: Types of Secret
token_count: 364
summary: ## Types of Secret When creating a Secret, you can specify its type using the `type` field of the [Secret](/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/) resource, or certain...
---

## Types of Secret
When creating a Secret, you can specify its type using the `type` field of
the [Secret](/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/)
resource, or certain equivalent `kubectl` command line flags (if available).
The Secret type is used to facilitate programmatic handling of the Secret data.
Kubernetes provides several built-in types for some common usage scenarios.
These types vary in terms of the validations performed and the constraints
Kubernetes imposes on them.
|Built-in Type|Usage|
|`Opaque`|arbitrary user-defined data|
|`kubernetes.io/service-account-token`|ServiceAccount token|
|`kubernetes.io/dockercfg`|serialized `\~/.dockercfg` file|
|`kubernetes.io/dockerconfigjson`|serialized `\~/.docker/config.json` file|
|`kubernetes.io/basic-auth`|credentials for basic authentication|
|`kubernetes.io/ssh-auth`|credentials for SSH authentication|
|`kubernetes.io/tls`|data for a TLS client or server|
|`bootstrap.kubernetes.io/token`|bootstrap token data|
You can define and use your own Secret type by assigning a non-empty string as the
`type` value for a Secret object (an empty string is treated as an `Opaque` type).
Kubernetes doesn't impose any constraints on the type name. However, if you
are using one of the built-in types, you must meet all the requirements defined
for that type.
If you are defining a type of Secret that's for public use, follow the convention
and structure the Secret type to have your domain name before the name, separated
by a `/`. For example: `cloud-hosting.example.net/cloud-api-credentials`.