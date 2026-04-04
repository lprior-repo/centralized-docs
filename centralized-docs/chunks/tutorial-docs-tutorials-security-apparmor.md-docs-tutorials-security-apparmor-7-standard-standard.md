---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#7-standard
chunk_level: standard
chunk_type: prose
heading: Example
token_count: 146
summary: ### Setting up Nodes with profiles Kubernetes 1.35 does not provide any built-in mechanisms for loading AppArmor profiles onto Nodes. Profiles can be loaded through custom infrastructure or tools...
---

### Setting up Nodes with profiles
Kubernetes 1.35 does not provide any built-in mechanisms for loading AppArmor profiles onto
Nodes. Profiles can be loaded through custom infrastructure or tools like the
[Kubernetes Security Profiles Operator](https://github.com/kubernetes-sigs/security-profiles-operator).
The scheduler is not aware of which profiles are loaded onto which Node, so the full set of profiles
must be loaded onto every Node. An alternative approach is to add a Node label for each profile (or
class of profiles) on the Node, and use a
[node selector](/docs/concepts/scheduling-eviction/assign-pod-node/) to ensure the Pod is run on a
Node with the required profile.