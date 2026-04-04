---
doc_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity
chunk_id: tutorial/docs-tasks-extend-kubernetes-setup-konnectivity.md/docs-tasks-extend-kubernetes-setup-konnectivity#28-summary
chunk_level: summary
chunk_type: prose
heading: Configure the Konnectivity service
token_count: 127
summary: - name: k8s-certs mountPath: /etc/kubernetes/pki readOnly: true - name: kubeconfig mountPath: /etc/kubernetes/konnectivity-server.conf readOnly: true - name: konnectivity-uds mountPath:...
---

- name: k8s-certs
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