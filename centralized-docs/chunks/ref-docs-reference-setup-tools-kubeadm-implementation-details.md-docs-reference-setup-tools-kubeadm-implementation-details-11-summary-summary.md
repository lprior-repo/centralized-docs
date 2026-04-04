---
doc_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details
chunk_id: ref/docs-reference-setup-tools-kubeadm-implementation-details.md/docs-reference-setup-tools-kubeadm-implementation-details#11-summary
chunk_level: summary
chunk_type: prose
heading: Core design principles
token_count: 93
summary: * `kubeadm init` * `export KUBECONFIG=/etc/kubernetes/admin.conf` * `kubectl apply -f &lt;network-plugin-of-choice.yaml&gt;` * `kubeadm join --token &lt;token&gt; &lt;endpoint&gt;:&lt;port&gt;` *...
---

* `kubeadm init`
* `export KUBECONFIG=/etc/kubernetes/admin.conf`
* `kubectl apply -f &lt;network-plugin-of-choice.yaml&gt;`
* `kubeadm join --token &lt;token&gt; &lt;endpoint&gt;:&lt;port&gt;`
* **Extendable**:
* It should *not* favor any particular network provider. Configuring the cluster network is out-of-scope