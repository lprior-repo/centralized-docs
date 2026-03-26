---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#55-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 120
summary: ``` `kubectl get secret secret-tiger-docker -o jsonpath='{.data.\*}' | base64 -d ` ``` The output is equivalent to the following JSON document (which is also a valid Docker configuration file): ```...
---

```
`kubectl get secret secret-tiger-docker -o jsonpath='{.data.\*}' | base64 -d
`
```
The output is equivalent to the following JSON document (which is also a valid
Docker configuration file):
```
`{
"auths": {
"my-registry.example:5000": {
"username": "tiger",
"password": "pass1234",
"email": "tiger@acme.example",
"auth": "dGlnZXI6cGFzczEyMzQ="
}
}
}
`
```