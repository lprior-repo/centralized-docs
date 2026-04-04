---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#15-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 88
summary: 4. If you use UDS connection, add volumes config to the kube-apiserver: ``` `spec: containers: volumeMounts: - name: konnectivity-uds mountPath: /etc/kubernetes/konnectivity-server readOnly: false...
---

4. If you use UDS connection, add volumes config to the kube-apiserver:
```
`spec:
containers:
volumeMounts:
- name: konnectivity-uds
mountPath: /etc/kubernetes/konnectivity-server
readOnly: false
volumes:
- name: konnectivity-uds
hostPath:
path: /etc/kubernetes/konnectivity-server
type: DirectoryOrCreate
`
```