---
doc_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit
chunk_id: ref/docs-tasks-debug-debug-cluster-audit.md/docs-tasks-debug-debug-cluster-audit#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Audit policy
token_count: 993
summary: ## Audit policy Audit policy defines rules about what events should be recorded and what data they should include. The audit policy object structure is defined in the [`audit.k8s.io` API...
---

## Audit policy
Audit policy defines rules about what events should be recorded and what data
they should include. The audit policy object structure is defined in the
[`audit.k8s.io` API group](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy).
When an event is processed, it's
compared against the list of rules in order. The first matching rule sets the
*audit level* of the event. The defined audit levels are:
* `None` - don't log events that match this rule.
* `Metadata` - log events with metadata (requesting user, timestamp, resource,
verb, etc.) but not request or response body.
* `Request` - log events with request metadata and body but not response body.
This does not apply for non-resource requests.
* `RequestResponse` - log events with request metadata, request body and response body.
This does not apply for non-resource requests.
You can pass a file with the policy to `kube-apiserver`
using the `--audit-policy-file` flag. If the flag is omitted, no events are logged.
Note that the `rules` field **must** be provided in the audit policy file.
A policy with no (0) rules is treated as illegal.
Below is an example audit policy file:
[`audit/audit-policy.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/audit/audit-policy.yaml)![](/images/copycode.svg "Copy audit/audit-policy.yaml to clipboard")
```
`apiVersion: audit.k8s.io/v1 # This is required.
kind: Policy
# Don't generate audit events for all requests in RequestReceived stage.
omitStages:
- "RequestReceived"
rules:
# Log pod changes at RequestResponse level
- level: RequestResponse
resources:
- group: ""
# Resource "pods" doesn't match requests to any subresource of pods,
# which is consistent with the RBAC policy.
resources: ["pods"]
# Log "pods/log", "pods/status" at Metadata level
- level: Metadata
resources:
- group: ""
resources: ["pods/log", "pods/status"]
# Don't log requests to a configmap called "controller-leader"
- level: None
resources:
- group: ""
resources: ["configmaps"]
resourceNames: ["controller-leader"]
# Don't log watch requests by the "system:kube-proxy" on endpoints or services
- level: None
users: ["system:kube-proxy"]
verbs: ["watch"]
resources:
- group: "" # core API group
resources: ["endpoints", "services"]
# Don't log authenticated requests to certain non-resource URL paths.
- level: None
userGroups: ["system:authenticated"]
nonResourceURLs:
- "/api\*" # Wildcard matching.
- "/version"
# Log the request body of configmap changes in kube-system.
- level: Request
resources:
- group: "" # core API group
resources: ["configmaps"]
# This rule only applies to resources in the "kube-system" namespace.
# The empty string "" can be used to select non-namespaced resources.
namespaces: ["kube-system"]
# Log configmap and secret changes in all other namespaces at the Metadata level.
- level: Metadata
resources:
- group: "" # core API group
resources: ["secrets", "configmaps"]
# Log all other resources in core and extensions at the Request level.
- level: Request
resources:
- group: "" # core API group
- group: "extensions" # Version of group should NOT be included.
# A catch-all rule to log all other requests at the Metadata level.
- level: Metadata
# Long-running requests like watches that fall under this rule will not
# generate an audit event in RequestReceived.
omitStages:
- "RequestReceived"
`
```
You can use a minimal audit policy file to log all requests at the `Metadata` level:
```
`# Log all requests at the Metadata level.
apiVersion: audit.k8s.io/v1
kind: Policy
rules:
- level: Metadata
`
```
If you're crafting your own audit profile, you can use the audit profile for Google Container-Optimized OS as a starting point. You can check the
[configure-helper.sh](https://github.com/kubernetes/kubernetes/blob/master/cluster/gce/gci/configure-helper.sh)
script, which generates an audit policy file. You can see most of the audit policy file by looking directly at the script.
You can also refer to the [`Policy` configuration reference](/docs/reference/config-api/apiserver-audit.v1/#audit-k8s-io-v1-Policy)
for details about the fields defined.