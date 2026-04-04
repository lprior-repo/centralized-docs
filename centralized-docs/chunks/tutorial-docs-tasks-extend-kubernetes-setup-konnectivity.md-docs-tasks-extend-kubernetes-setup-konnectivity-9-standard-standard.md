---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#9-standard
chunk_level: standard
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 213
summary: mountPath: /etc/kubernetes/pki readOnly: true - name: kubeconfig mountPath: /etc/kubernetes/konnectivity-server.conf readOnly: true - name: konnectivity-uds mountPath:...
---

mountPath: /etc/kubernetes/pki
readOnly: true
- name: kubeconfig
mountPath: /etc/kubernetes/konnectivity-server.conf
readOnly: true
- name: konnectivity-uds
mountPath: /etc/kubernetes/konnectivity-server
readOnly: false
volumes:
- name: k8s-certs
hostPath:
path: /etc/kubernetes/pki
- name: kubeconfig
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