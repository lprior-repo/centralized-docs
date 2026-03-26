---
doc_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers
chunk_id: tutorial/docs-tutorials-configuration-pod-sidecar-containers.md/docs-tutorials-configuration-pod-sidecar-containers#5-standard
chunk_level: standard
chunk_type: prose
heading: Adopting built-in sidecar containers
token_count: 243
summary: ### Check for 3rd party tooling and mutating webhooks If you experience issues when validating the feature, it may be an indication that one of the 3rd party tools or mutating webhooks are broken....
---

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