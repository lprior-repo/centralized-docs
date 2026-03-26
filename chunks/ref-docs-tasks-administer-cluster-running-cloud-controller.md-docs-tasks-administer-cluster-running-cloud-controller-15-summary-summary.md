---
doc_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller
chunk_id: ref/docs-tasks-administer-cluster-running-cloud-controller.md/docs-tasks-administer-cluster-running-cloud-controller#15-summary
chunk_level: summary
chunk_type: prose
heading: Table of Contents
token_count: 90
summary: projects in repositories maintained by cloud vendors or by SIGs. For providers already in Kubernetes core, you can run the in-tree cloud controller manager as a DaemonSet in your cluster, use the...
---

projects in repositories maintained by cloud vendors or by SIGs.
For providers already in Kubernetes core, you can run the in-tree cloud controller
manager as a DaemonSet in your cluster, use the following as a guideline:
[`admin/cloud/ccm-example.yaml`
](https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/admin/cloud/ccm-example.yaml)![](/images/copycode.svg "Copy admin/cloud/ccm-example.yaml to clipboard")