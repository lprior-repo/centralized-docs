---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#22-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 98
summary: . Also as a basic tutorial, we're not explaining what `Pods` are in any detail here, it will be covered in later topics. The `kubectl proxy` command can create a proxy that will forward...
---

.
Also as a basic tutorial, we're not explaining what `Pods` are in any
detail here, it will be covered in later topics.
The `kubectl proxy` command can create a proxy that will forward communications
into the cluster-wide, private network. The proxy can be terminated by pressing
control-C and won't show any output while it's running.
**You need to open a second terminal window to run the proxy.**
```
`kubectl proxy
`
```