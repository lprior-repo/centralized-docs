---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#29-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 123
summary: hostPath: path: /etc/kubernetes/konnectivity-server.conf type: FileOrCreate - name: konnectivity-uds hostPath: path: /etc/kubernetes/konnectivity-server type: DirectoryOrCreate ` ``` Then deploy the...
---

hostPath:
path: /etc/kubernetes/konnectivity-server.conf
type: FileOrCreate
- name: konnectivity-uds
hostPath:
path: /etc/kubernetes/konnectivity-server
type: DirectoryOrCreate
`
```
Then deploy the Konnectivity agents in your cluster:
[`admin/konnectivity/konnectivity-agent.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/konnectivity/konnectivity-agent.yaml)![](/images/copycode.svg "Copy admin/konnectivity/konnectivity-agent.yaml to clipboard")