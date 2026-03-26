---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#2-detailed
chunk_level: detailed
chunk_type: table
heading: Adopting built-in sidecar containers
token_count: 686
summary: ## Adopting built-in sidecar containers The `SidecarContainers` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/) is in beta state starting from Kubernetes version 1.29 and...
---

## Adopting built-in sidecar containers
The `SidecarContainers` [feature gate](/docs/reference/command-line-tools-reference/feature-gates/)
is in beta state starting from Kubernetes version 1.29 and is enabled by default.
Some clusters may have this feature disabled or have software installed that is incompatible with the feature.
When this happens, the Pod may be rejected or the sidecar containers may block Pod startup,
rendering the Pod useless. This condition is easy to detect as the Pod simply gets stuck on
initialization. However, it is often unclear what caused the problem.
Here are the considerations and troubleshooting steps that one can take while adopting sidecar containers for their workload.
### Ensure the feature gate is enabled
As a very first step, make sure that both API server and Nodes are at Kubernetes version v1.29 or
later. The feature will break on clusters where Nodes are running earlier versions where it is not enabled.
#### Note
The feature can be enabled on nodes with the version 1.28. The behavior of built-in sidecar
container termination was different in version 1.28, and it is not recommended to adjust
the behavior of a sidecar to that behavior. However, if the only concern is the startup order, the
above statement can be changed to Nodes running version 1.28 with the feature gate enabled.
You should ensure that the feature gate is enabled for the API server(s) within the control plane
**and** for all nodes.
One of the ways to check the feature gate enablement is to run a command like this:
* For API Server:
```
`kubectl get --raw /metrics | grep kubernetes\_feature\_enabled | grep SidecarContainers
`
```
* For the individual node:
```
`kubectl get --raw /api/v1/nodes/&lt;node-name&gt;/proxy/metrics | grep kubernetes\_feature\_enabled | grep SidecarContainers
`
```
If you see something like this:
```
`kubernetes\_feature\_enabled{name="SidecarContainers",stage="BETA"} 1
`
```
it means that the feature is enabled.
### Check for 3rd party tooling and mutating webhooks
If you experience issues when validating the feature, it may be an indication that one of the
3rd party tools or mutating webhooks are broken.
When the `SidecarContainers` feature gate is enabled, Pods gain a new field in their API.
If tools pass unknown fields as-is using various patching strategies to mutate a Pod object,
this will not be a problem. However, there are tools that will strip out unknown fields;
if you have those, they must be recompiled with the v1.28+ version of Kubernetes API client code.
The way to check this is to use the `kubectl describe pod` command with your Pod that has passed through
mutating admission. If any tools stripped out the new field (`restartPolicy:Always`),
you will not see it in the command output.
If you hit an issue like this, please advise the author of the tools or the webhooks
use one of the patching strategies for modifying objects instead of a full object update.
#### Note
Mutating webhook may update Pods based on some conditions.
Thus, sidecar containers may work for some Pods and fail for others.