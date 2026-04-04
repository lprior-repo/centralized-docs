---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#7-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 126
summary: `apiVersion: v1 kind: Pod metadata: name: dependent-envars-demo spec: containers: - name: dependent-envars-demo args: - while true; do echo -en '\\n'; printf...
---

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