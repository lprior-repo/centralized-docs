---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#1-detailed
chunk_level: detailed
chunk_type: code
heading: Before you begin
token_count: 997
summary: # Define Dependent Environment Variables This page shows how to define dependent environment variables for a container in a Kubernetes Pod. ## Before you begin You need to have a Kubernetes cluster,...
---

# Define Dependent Environment Variables
This page shows how to define dependent environment variables for a container
in a Kubernetes Pod.
## Before you begin
You need to have a Kubernetes cluster, and the kubectl command-line tool must
be configured to communicate with your cluster. It is recommended to run this tutorial on a cluster with at least two nodes that are not acting as control plane hosts. If you do not already have a
cluster, you can create one by using
[minikube](https://minikube.sigs.k8s.io/docs/tutorials/multi_node/)
or you can use one of these Kubernetes playgrounds:
* [iximiuz Labs](https://labs.iximiuz.com/playgrounds?category=kubernetes&amp;filter=all)
* [Killercoda](https://killercoda.com/playgrounds/scenario/kubernetes)
* [KodeKloud](https://kodekloud.com/public-playgrounds)## Define an environment dependent variable for a container
When you create a Pod, you can set dependent environment variables for the containers that run in the Pod. To set dependent environment variables, you can use $(VAR\_NAME) in the `value` of `env` in the configuration file.
In this exercise, you create a Pod that runs one container. The configuration
file for the Pod defines a dependent environment variable with common usage defined. Here is the configuration manifest for the
Pod:
[`pods/inject/dependent-envars.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/inject/dependent-envars.yaml)![](/images/copycode.svg "Copy pods/inject/dependent-envars.yaml to clipboard")
```
`apiVersion: v1
kind: Pod
metadata:
name: dependent-envars-demo
spec:
containers:
- name: dependent-envars-demo
args:
- while true; do echo -en '\\n'; printf UNCHANGED\_REFERENCE=$UNCHANGED\_REFERENCE'\\n'; printf SERVICE\_ADDRESS=$SERVICE\_ADDRESS'\\n';printf ESCAPED\_REFERENCE=$ESCAPED\_REFERENCE'\\n'; sleep 30; done;
command:
- sh
- -c
image: busybox:1.28
env:
- name: SERVICE\_PORT
value: "80"
- name: SERVICE\_IP
value: "172.17.0.1"
- name: UNCHANGED\_REFERENCE
value: "$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)"
- name: PROTOCOL
value: "https"
- name: SERVICE\_ADDRESS
value: "$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)"
- name: ESCAPED\_REFERENCE
value: "$$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)"
`
```
1. Create a Pod based on that manifest:
```
`kubectl apply -f https://k8s.io/examples/pods/inject/dependent-envars.yaml
`
```
```
`pod/dependent-envars-demo created
`
```
2. List the running Pods:
```
`kubectl get pods dependent-envars-demo
`
```
```
`NAME READY STATUS RESTARTS AGE
dependent-envars-demo 1/1 Running 0 9s
`
```
3. Check the logs for the container running in your Pod:
```
`kubectl logs pod/dependent-envars-demo
`
```
```
`
UNCHANGED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80
SERVICE\_ADDRESS=https://172.17.0.1:80
ESCAPED\_REFERENCE=$(PROTOCOL)://172.17.0.1:80
`
```
As shown above, you have defined the correct dependency reference of `SERVICE\_ADDRESS`, bad dependency reference of `UNCHANGED\_REFERENCE` and skip dependent references of `ESCAPED\_REFERENCE`.
When an environment variable is already defined when being referenced,
the reference can be correctly resolved, such as in the `SERVICE\_ADDRESS` case.
Note that order matters in the `env` list. An environment variable is not considered
"defined" if it is specified further down the list. That is why `UNCHANGED\_REFERENCE`
fails to resolve `$(PROTOCOL)` in the example above.
When the environment variable is undefined or only includes some variables, the undefined environment variable is treated as a normal string, such as `UNCHANGED\_REFERENCE`. Note that incorrectly parsed environment variables, in general, will not block the container from starting.
The `$(VAR\_NAME)` syntax can be escaped with a double `$`, ie: `$$(VAR\_NAME)`.
Escaped references are never expanded, regardless of whether the referenced variable
is defined or not. This can be seen from the `ESCAPED\_REFERENCE` case above.