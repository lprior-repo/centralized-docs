---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver.md/docs-tasks-administer-cluster-kubeadm-configure-cgroup-driver#10-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the kubelet cgroup driver
token_count: 110
summary: ``` `# kubeadm-config.yaml kind: ClusterConfiguration apiVersion: kubeadm.k8s.io/v1beta4 kubernetesVersion: v1.21.0 --- kind: KubeletConfiguration apiVersion: kubelet.config.k8s.io/v1beta1...
---

```
`# kubeadm-config.yaml
kind: ClusterConfiguration
apiVersion: kubeadm.k8s.io/v1beta4
kubernetesVersion: v1.21.0
---
kind: KubeletConfiguration
apiVersion: kubelet.config.k8s.io/v1beta1
cgroupDriver: systemd
`
```
Such a configuration file can then be passed to the kubeadm command:
```
`kubeadm init --config kubeadm-config.yaml
`
```