---
doc_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects
chunk_id: ref/docs-concepts-overview-working-with-objects.md/docs-concepts-overview-working-with-objects#4-standard
chunk_level: standard
chunk_type: prose
heading: Understanding Kubernetes objects
token_count: 394
summary: ### Describing a Kubernetes object When you create an object in Kubernetes, you must provide the object spec that describes its desired state, as well as some basic information about the object (such...
---

### Describing a Kubernetes object
When you create an object in Kubernetes, you must provide the object spec that describes its
desired state, as well as some basic information about the object (such as a name). When you use
the Kubernetes API to create the object (either directly or via `kubectl`), that API request must
include that information as JSON in the request body.
Most often, you provide the information to `kubectl` in a file known as a *manifest*.
By convention, manifests are YAML (you could also use JSON format).
Tools such as `kubectl` convert the information from a manifest into JSON or another supported
serialization format when making the API request over HTTP.
Here's an example manifest that shows the required fields and object spec for a Kubernetes
Deployment:
[`application/deployment.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/application/deployment.yaml)![](/images/copycode.svg "Copy application/deployment.yaml to clipboard")
```
`apiVersion: apps/v1
kind: Deployment
metadata:
name: nginx-deployment
spec:
selector:
matchLabels:
app: nginx
replicas: 2 # tells deployment to run 2 pods matching the template
template:
metadata:
labels:
app: nginx
spec:
containers:
- name: nginx
image: nginx:1.14.2
ports:
- containerPort: 80
`
```
One way to create a Deployment using a manifest file like the one above is to use the
[`kubectl apply`](/docs/reference/generated/kubectl/kubectl-commands#apply) command
in the `kubectl` command-line interface, passing the `.yaml` file as an argument. Here's an example:
```
`kubectl apply -f https://k8s.io/examples/application/deployment.yaml
`
```
The output is similar to this:
```
`deployment.apps/nginx-deployment created
`
```