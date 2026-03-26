---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#22-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 206
summary: ### Using Secrets as environment variables To use a Secret in an [environment variable](/docs/concepts/containers/container-environment/) in a Pod: 1. For each container in your Pod specification,...
---

### Using Secrets as environment variables
To use a Secret in an [environment variable](/docs/concepts/containers/container-environment/)
in a Pod:
1. For each container in your Pod specification, add an environment variable
for each Secret key that you want to use to the
`env[].valueFrom.secretKeyRef` field.
2. Modify your image and/or command line so that the program looks for values
in the specified environment variables.
For instructions, refer to
[Define container environment variables using Secret data](/docs/tasks/inject-data-application/distribute-credentials-secure/#define-container-environment-variables-using-secret-data).
It's important to note that the range of characters allowed for environment variable
names in pods is [restricted](/docs/tasks/inject-data-application/define-environment-variable-container/#using-environment-variables-inside-of-your-config).
If any keys do not meet the rules, those keys are not made available to your container, though
the Pod is allowed to start.