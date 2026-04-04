---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#6-standard
chunk_level: standard
chunk_type: code
heading: Upgrading control plane nodes
token_count: 476
summary: ### Call \"kubeadm upgrade\" **For the first control plane node** 1. Upgrade kubeadm: ``` `# replace x in 1.35.x-\* with the latest patch version sudo apt-mark unhold kubeadm &amp;&amp; \\ sudo apt-get...
---

### Call "kubeadm upgrade"
**For the first control plane node**
1. Upgrade kubeadm:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo apt-mark unhold kubeadm &amp;&amp; \\
sudo apt-get update &amp;&amp; sudo apt-get install -y kubeadm='1.35.x-\*' &amp;&amp; \\
sudo apt-mark hold kubeadm
`
```
For systems with DNF:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo yum install -y kubeadm-'1.35.x-\*' --disableexcludes=kubernetes
`
```
For systems with DNF5:
```
`# replace x in 1.35.x-\* with the latest patch version
sudo yum install -y kubeadm-'1.35.x-\*' --setopt=disable\_excludes=kubernetes
`
```
2. Verify that the download works and has the expected version:
```
`kubeadm version
`
```
3. Verify the upgrade plan:
```
`sudo kubeadm upgrade plan
`
```
This command checks that your cluster can be upgraded, and fetches the versions you can upgrade to.
It also shows a table with the component config version states.
#### Note:
`kubeadm upgrade` also automatically renews the certificates that it manages on this node.
To opt-out of certificate renewal the flag `--certificate-renewal=false` can be used.
For more information see the [certificate management guide](/docs/tasks/administer-cluster/kubeadm/kubeadm-certs/).
4. Choose a version to upgrade to, and run the appropriate command. For example:
```
`# replace x with the patch version you picked for this upgrade
sudo kubeadm upgrade apply v1.35.x
`
```
Once the command finishes you should see:
```
`[upgrade/successful] SUCCESS! Your cluster was upgraded to "v1.35.x". Enjoy!
[upgrade/kubelet] Now that your control plane is upgraded, please proceed with upgrading your kubelets if you haven't already done so.
`
```