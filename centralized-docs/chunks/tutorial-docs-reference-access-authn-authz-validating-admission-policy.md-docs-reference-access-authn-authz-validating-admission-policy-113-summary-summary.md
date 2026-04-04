---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#113-summary
chunk_level: summary
chunk_type: table
heading: Getting Started with Validating Admission Policy
token_count: 122
summary: expression: \"'environment' in namespaceObject.metadata.labels ? namespaceObject.metadata.labels['environment'] : 'prod'\" - name: exempt expression: \"'exempt' in object.metadata.labels &amp;&amp;...
---

expression: "'environment' in namespaceObject.metadata.labels ? namespaceObject.metadata.labels['environment'] : 'prod'"
- name: exempt
expression: "'exempt' in object.metadata.labels &amp;&amp; object.metadata.labels['exempt'] == 'true'"
- name: containers
expression: "object.spec.template.spec.containers"
- name: containersToCheck
expression: "variables.containers.filter(c, c.image.contains('example.com/'))"
validations:
- expression: "variables.exempt || variables.containersToCheck.all(c, c.image.startsWith(variables.environment + '.'))"