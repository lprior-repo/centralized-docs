---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#79-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 126
summary: matchConditions: - name: 'exclude-leases' # Each match condition must have a unique name expression: '!(request.resource.group == \"coordination.k8s.io\" &amp;&amp; request.resource.resource ==...
---

matchConditions:
- name: 'exclude-leases' # Each match condition must have a unique name
expression: '!(request.resource.group == "coordination.k8s.io" &amp;&amp; request.resource.resource == "leases")' # Match non-lease resources.
- name: 'exclude-kubelet-requests'
expression: '!("system:nodes" in request.userInfo.groups)' # Match requests made by non-node users.
- name: 'rbac' # Skip RBAC requests.
expression: 'request.resource.group != "rbac.authorization.k8s.io"'
validations:
- expression: "!