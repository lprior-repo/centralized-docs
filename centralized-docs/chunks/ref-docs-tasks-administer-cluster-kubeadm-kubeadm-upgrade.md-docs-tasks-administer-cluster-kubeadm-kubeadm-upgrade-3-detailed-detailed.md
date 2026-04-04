---
doc_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade
chunk_id: ref/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade.md/docs-tasks-administer-cluster-kubeadm-kubeadm-upgrade#3-detailed
chunk_level: detailed
chunk_type: code
heading: Upgrading control plane nodes
token_count: 829
summary: ## Upgrading control plane nodes The upgrade procedure on control plane nodes should be executed one node at a time. Pick a control plane node that you wish to upgrade first. It must have the...
---

## Upgrading control plane nodes
The upgrade procedure on control plane nodes should be executed one node at a time.
Pick a control plane node that you wish to upgrade first. It must have the `/etc/kubernetes/admin.conf` file.
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
#### Note:
For versions earlier than v1.28, kubeadm defaulted to a mode that upgrades the addons (including CoreDNS and kube-proxy)
immediately during `kubeadm upgrade apply`, regardless of whether there are other control plane instances that have not
been upgraded. This may cause compatibility problems. Since v1.28, kubeadm defaults to a mode that checks whether all
the control plane instances have been upgraded before starting to upgrade the addons. You must perform control plane
instances upgrade sequentially or at least ensure that the last control plane instance upgrade is not started until all
the other control plane instances have been upgraded completely, and the addons upgrade will be performed after the last
control plane instance is upgraded.
5. Manually upgrade your CNI provider plugin.
Your Container Network Interface (CNI) provider may have its own upgrade instructions to follow.
Check the [addons](/docs/concepts/cluster-administration/addons/) page to
find your CNI provider and see whether additional upgrade steps are required.
This step is not required on additional control plane nodes if the CNI provider runs as a DaemonSet.
**For the other control plane nodes**
Same as the first control plane node but use:
```
`sudo kubeadm upgrade node
`
```
instead of:
```
`sudo kubeadm upgrade apply
`
```
Also calling `kubeadm upgrade plan` and upgrading the CNI provider plugin is no longer needed.