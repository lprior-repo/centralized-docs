---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#20-standard
chunk_level: standard
chunk_type: prose
heading: Types of Secret
token_count: 408
summary: ### Using a Secret Secrets can be mounted as data volumes or exposed as [environment variables](/docs/concepts/containers/container-environment/) to be used by a container in a Pod. Secrets can also...
---

### Using a Secret
Secrets can be mounted as data volumes or exposed as
[environment variables](/docs/concepts/containers/container-environment/)
to be used by a container in a Pod. Secrets can also be used by other parts of the
system, without being directly exposed to the Pod. For example, Secrets can hold
credentials that other parts of the system should use to interact with external
systems on your behalf.
Secret volume sources are validated to ensure that the specified object
reference actually points to an object of type Secret. Therefore, a Secret
needs to be created before any Pods that depend on it.
If the Secret cannot be fetched (perhaps because it does not exist, or
due to a temporary lack of connection to the API server) the kubelet
periodically retries running that Pod. The kubelet also reports an Event
for that Pod, including details of the problem fetching the Secret.
#### Optional Secrets
When you reference a Secret in a Pod, you can mark the Secret as *optional*,
such as in the following example. If an optional Secret doesn't exist,
Kubernetes ignores it.
[`secret/optional-secret.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/secret/optional-secret.yaml)![](/images/copycode.svg "Copy secret/optional-secret.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: mypod
spec:
containers:
- name: mypod
image: redis
volumeMounts:
- name: foo
mountPath: "/etc/foo"
readOnly: true
volumes:
- name: foo
secret:
secretName: mysecret
optional: true`
```
By default, Secrets are required. None of a Pod's containers will start until
all non-optional Secrets are available.
If a Pod references a specific key in a non-optional Secret and that Secret
does exist, but is missing the named key, the Pod fails during startup.