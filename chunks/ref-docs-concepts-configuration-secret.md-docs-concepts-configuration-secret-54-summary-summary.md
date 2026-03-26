---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#54-summary
chunk_level: summary
chunk_type: prose
heading: Types of Secret
token_count: 117
summary: ``` `kubectl create secret docker-registry secret-tiger-docker \\ --docker-email=tiger@acme.example \\ --docker-username=tiger \\ --docker-password=pass1234 \\...
---

```
`kubectl create secret docker-registry secret-tiger-docker \\
--docker-email=tiger@acme.example \\
--docker-username=tiger \\
--docker-password=pass1234 \\
--docker-server=my-registry.example:5000
`
```
This command creates a Secret of type `kubernetes.io/dockerconfigjson`.
Retrieve the `.data.dockerconfigjson` field from that new Secret and decode the
data:
```
`kubectl get secret secret-tiger-docker -o jsonpath='{.data.\*}' | base64 -d
`
```