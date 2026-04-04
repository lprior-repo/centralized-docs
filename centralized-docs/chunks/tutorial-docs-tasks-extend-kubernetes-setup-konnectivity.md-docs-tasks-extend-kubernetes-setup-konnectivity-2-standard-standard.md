---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 282
summary: - [config to secure the TCP transport.](#config-to-secure-the-tcp-transport) - [This needs to be consistent with the value set in...
---

- [config to secure the TCP transport.](#config-to-secure-the-tcp-transport)
- [This needs to be consistent with the value set in egressSelectorConfiguration.](#this-needs-to-be-consistent-with-the-value-set-in-egressselectorconfiguration)
- [The following two lines assume the Konnectivity server is](#the-following-two-lines-assume-the-konnectivity-server-is)
- [deployed on the same machine as the apiserver, and the certs and](#deployed-on-the-same-machine-as-the-apiserver-and-the-certs-and)
- [key of the API Server are at the specified location.](#key-of-the-api-server-are-at-the-specified-location)
- [This needs to be consistent with the value set in egressSelectorConfiguration.](#this-needs-to-be-consistent-with-the-value-set-in-egressselectorconfiguration)
- [Alternatively, you can deploy the agents as Deployments. It is not necessary](#alternatively-you-can-deploy-the-agents-as-deployments-it-is-not-necessary)
- [to have an agent on each node.](#to-have-an-agent-on-each-node)
- [this is the IP address of the master machine.](#this-is-the-ip-address-of-the-master-machine)
  - [Feedback](#feedback)

---