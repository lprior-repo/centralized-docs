---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#11-summary
chunk_level: summary
chunk_type: prose
heading: Scaling overview
token_count: 119
summary: ![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg) ![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg) *Scaling is accomplished by changing the number...
---

![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg)
![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg)
*Scaling is accomplished by changing the number of replicas in a Deployment.*
Scaling out a Deployment will ensure new Pods are created and scheduled to Nodes
with available resources. Scaling will increase the number of Pods to the new desired
state. Kubernetes also supports [autoscaling](/docs/concepts/workloads/autoscaling/)
of Pods, but it is outside of the scope of this tutorial. Scaling to zero is also