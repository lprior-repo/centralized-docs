---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#18-summary
chunk_level: summary
chunk_type: prose
heading: Migrating to the `systemd` driver
token_count: 72
summary: ### Modify the kubelet ConfigMap * Call `kubectl edit cm kubelet-config -n kube-system`. * Either modify the existing `cgroupDriver` value or add a new field that looks like this: ``` `cgroupDriver:...
---

### Modify the kubelet ConfigMap
* Call `kubectl edit cm kubelet-config -n kube-system`.
* Either modify the existing `cgroupDriver` value or add a new field that looks like this:
```
`cgroupDriver: systemd
`
```
This field must be present under the `kubelet:` section of the ConfigMap.