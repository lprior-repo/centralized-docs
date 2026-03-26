---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#12-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 396
summary: #### Note: If you do not want to perform the base64 encoding, you can choose to use the `stringData` field instead. When you create Docker config Secrets using a manifest, the API server checks...
---

#### Note:
If you do not want to perform the base64 encoding, you can choose to use the
`stringData` field instead.
When you create Docker config Secrets using a manifest, the API
server checks whether the expected key exists in the `data` field, and
it verifies if the value provided can be parsed as a valid JSON. The API
server doesn't validate if the JSON actually is a Docker config file.
You can also use `kubectl` to create a Secret for accessing a container
registry, such as when you don't have a Docker configuration file:
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
#### Caution:
The `auth` value there is base64 encoded; it is obscured but not secret.
Anyone who can read that Secret can learn the registry access bearer token.
It is suggested to use [credential providers](/docs/tasks/administer-cluster/kubelet-credential-provider/) to dynamically and securely provide pull secrets on-demand.