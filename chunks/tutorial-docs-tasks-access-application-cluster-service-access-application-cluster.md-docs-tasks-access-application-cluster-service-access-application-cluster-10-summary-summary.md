---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#10-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 120
summary: `Name: example-service Namespace: default Labels: run=load-balancer-example Annotations: &lt;none&gt; Selector: run=load-balancer-example Type: NodePort IP: 10.32.0.16 Port: &lt;unset&gt; 8080/TCP...
---

`Name: example-service
Namespace: default
Labels: run=load-balancer-example
Annotations: &lt;none&gt;
Selector: run=load-balancer-example
Type: NodePort
IP: 10.32.0.16
Port: &lt;unset&gt; 8080/TCP
TargetPort: 8080/TCP
NodePort: &lt;unset&gt; 31496/TCP
Endpoints: 10.200.1.4:8080,10.200.2.5:8080
Session Affinity: None