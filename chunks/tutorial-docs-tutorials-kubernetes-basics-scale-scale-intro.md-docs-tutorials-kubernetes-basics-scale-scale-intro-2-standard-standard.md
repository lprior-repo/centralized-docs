---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#2-standard
chunk_level: standard
chunk_type: prose
heading: Scaling overview
token_count: 248
summary: ## Scaling overview ![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg) ![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg) *Scaling is accomplished by...
---

## Scaling overview
![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg)
![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg)
*Scaling is accomplished by changing the number of replicas in a Deployment.*
Scaling out a Deployment will ensure new Pods are created and scheduled to Nodes
with available resources. Scaling will increase the number of Pods to the new desired
state. Kubernetes also supports [autoscaling](/docs/concepts/workloads/autoscaling/)
of Pods, but it is outside of the scope of this tutorial. Scaling to zero is also
possible, and it will terminate all Pods of the specified Deployment.
Running multiple instances of an application will require a way to distribute the
traffic to all of them. Services have an integrated load-balancer that will distribute
network traffic to all Pods of an exposed Deployment. Services will monitor continuously
the running Pods using endpoints, to ensure the traffic is sent only to available Pods.
Once you have multiple instances of an application running, you would be able to
do Rolling updates without downtime. We'll cover that in the next section of the
tutorial. Now, let's go to the terminal and scale our application.