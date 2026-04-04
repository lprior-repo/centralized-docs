---
doc_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy
chunk_id: tutorial/docs-reference-access-authn-authz-validating-admission-policy.md/docs-reference-access-authn-authz-validating-admission-policy#111-summary
chunk_level: summary
chunk_type: prose
heading: Getting Started with Validating Admission Policy
token_count: 128
summary: `# This policy enforces that all containers of a deployment has the image repo match the environment label of its namespace. # Except for \"exempt\" deployments, or any containers that do not belong to...
---

`# This policy enforces that all containers of a deployment has the image repo match the environment label of its namespace.
# Except for "exempt" deployments, or any containers that do not belong to the "example.com" organization (e.g. common sidecars).
# For example, if the namespace has a label of {"environment": "staging"}, all container images must be either staging.example.com/\*
# or do not contain "example.com" at all, unless the deployment has {"exempt": "true"} label.
apiVersion: admissionregistration.k8s.io/v1
kind: ValidatingAdmissionPolicy