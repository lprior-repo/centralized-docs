---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#21-summary
chunk_level: summary
chunk_type: prose
heading: Create a Service
token_count: 92
summary: ``` `kubectl get services ` ``` The output is similar to: ``` `NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE hello-node LoadBalancer 10.108.144.78 &lt;pending&gt; 8080:30369/TCP 21s kubernetes...
---

```
`kubectl get services
`
```
The output is similar to:
```
`NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
hello-node LoadBalancer 10.108.144.78 &lt;pending&gt; 8080:30369/TCP 21s
kubernetes ClusterIP 10.96.0.1 &lt;none&gt; 443/TCP 23m
`
```