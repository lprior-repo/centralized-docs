---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#8-summary
chunk_level: summary
chunk_type: prose
heading: Kubernetes Deployments
token_count: 116
summary: Kubernetes **Deployment**. The Deployment instructs Kubernetes how to create and update instances of your application. Once you've created a Deployment, the Kubernetes control plane schedules the...
---

Kubernetes **Deployment**. The Deployment instructs Kubernetes how to create and
update instances of your application. Once you've created a Deployment, the Kubernetes
control plane schedules the application instances included in that Deployment to run
on individual Nodes in the cluster.
Once the application instances are created, a Kubernetes Deployment controller continuously
monitors those instances. If the Node hosting an instance goes down or is deleted,
the Deployment controller replaces the instance with an instance on another Node
in the cluster. **This provides a self-healing mechanism to address machine failure
or maintenance.**