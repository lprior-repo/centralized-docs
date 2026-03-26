---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#34-summary
chunk_level: summary
chunk_type: table
heading: Types of Secret
token_count: 127
summary: |Built-in Type|Usage| |`Opaque`|arbitrary user-defined data| |`kubernetes.io/service-account-token`|ServiceAccount token| |`kubernetes.io/dockercfg`|serialized `\~/.dockercfg` file|...
---

|Built-in Type|Usage|
|`Opaque`|arbitrary user-defined data|
|`kubernetes.io/service-account-token`|ServiceAccount token|
|`kubernetes.io/dockercfg`|serialized `\~/.dockercfg` file|
|`kubernetes.io/dockerconfigjson`|serialized `\~/.docker/config.json` file|
|`kubernetes.io/basic-auth`|credentials for basic authentication|
|`kubernetes.io/ssh-auth`|credentials for SSH authentication|
|`kubernetes.io/tls`|data for a TLS client or server|
|`bootstrap.kubernetes.io/token`