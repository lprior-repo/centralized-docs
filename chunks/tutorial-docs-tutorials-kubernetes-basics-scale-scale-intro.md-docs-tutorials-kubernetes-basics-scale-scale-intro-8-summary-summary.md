---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#8-summary
chunk_level: summary
chunk_type: prose
heading: Scaling an application
token_count: 128
summary: If you are trying this after the [previous section](/docs/tutorials/kubernetes-basics/expose/expose-intro/), then you may have deleted the service you created, or have created a Service of `type:...
---

If you are trying this after the
[previous section](/docs/tutorials/kubernetes-basics/expose/expose-intro/), then you
may have deleted the service you created, or have created a Service of `type: NodePort`.
In this section, it is assumed that a service with `type: LoadBalancer` is created
for the kubernetes-bootcamp Deployment.
If you have *not* deleted the Service created in
[the previous section](/docs/tutorials/kubernetes-basics/expose/expose-intro/),
first delete that Service and then run the following command to create a new Service
with its