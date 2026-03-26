---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#24-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 82
summary: #### Note: If port 8001 is not accessible, ensure that the `kubectl proxy` that you started above is running in the second terminal. The API server will automatically create an endpoint for each pod,...
---

#### Note:
If port 8001 is not accessible, ensure that the `kubectl proxy` that you started
above is running in the second terminal.
The API server will automatically create an endpoint for each pod, based on the
pod name, that is also accessible through the proxy.
First we need to get the Pod name, and we'll store it in the environment variable `POD\_NAME`.