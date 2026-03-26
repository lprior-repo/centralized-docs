---
doc_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret
chunk_id: ref/docs-concepts-configuration-secret.md/docs-concepts-configuration-secret#9-detailed
chunk_level: detailed
chunk_type: prose
heading: Types of Secret
token_count: 985
summary: ### Creating a Secret There are several options to create a Secret: * [Use `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/) * [Use a configuration...
---

### Creating a Secret
There are several options to create a Secret:
* [Use `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/)
* [Use a configuration file](/docs/tasks/configmap-secret/managing-secret-using-config-file/)
* [Use the Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/)#### Constraints on Secret names and data
The name of a Secret object must be a valid
[DNS subdomain name](/docs/concepts/overview/working-with-objects/names/#dns-subdomain-names).
You can specify the `data` and/or the `stringData` field when creating a
configuration file for a Secret. The `data` and the `stringData` fields are optional.
The values for all keys in the `data` field have to be base64-encoded strings.
If the conversion to base64 string is not desirable, you can choose to specify
the `stringData` field instead, which accepts arbitrary strings as values.
The keys of `data` and `stringData` must consist of alphanumeric characters,
`-`, `\_` or `.`. All key-value pairs in the `stringData` field are internally
merged into the `data` field. If a key appears in both the `data` and the
`stringData` field, the value specified in the `stringData` field takes
precedence.
#### Size limit
Individual Secrets are limited to 1MiB in size. This is to discourage creation
of very large Secrets that could exhaust the API server and kubelet memory.
However, creation of many smaller Secrets could also exhaust memory. You can
use a [resource quota](/docs/concepts/policy/resource-quotas/) to limit the
number of Secrets (or other resources) in a namespace.
### Editing a Secret
You can edit an existing Secret unless it is [immutable](#secret-immutable). To
edit a Secret, use one of the following methods:
* [Use `kubectl`](/docs/tasks/configmap-secret/managing-secret-using-kubectl/#edit-secret)
* [Use a configuration file](/docs/tasks/configmap-secret/managing-secret-using-config-file/#edit-secret)
You can also edit the data in a Secret using the [Kustomize tool](/docs/tasks/configmap-secret/managing-secret-using-kustomize/#edit-secret). However, this
method creates a new `Secret` object with the edited data.
Depending on how you created the Secret, as well as how the Secret is used in
your Pods, updates to existing `Secret` objects are propagated automatically to
Pods that use the data. For more information, refer to [Using Secrets as files from a Pod](#using-secrets-as-files-from-a-pod) section.
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