---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#3-standard
chunk_level: standard
chunk_type: code
heading: Before you begin
token_count: 425
summary: ``` `apiVersion: v1 kind: Pod metadata: name: dependent-envars-demo spec: containers: - name: dependent-envars-demo args: - while true; do echo -en '\\n'; printf...
---

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