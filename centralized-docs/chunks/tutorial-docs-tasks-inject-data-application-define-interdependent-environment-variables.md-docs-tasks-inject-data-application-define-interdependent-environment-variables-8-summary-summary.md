---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#8-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 117
summary: - sh - -c image: busybox:1.28 env: - name: SERVICE\_PORT value: \"80\" - name: SERVICE\_IP value: \"172.17.0.1\" - name: UNCHANGED\_REFERENCE value: \"$(PROTOCOL)://$(SERVICE\_IP):$(SERVICE\_PORT)\" -...
---

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