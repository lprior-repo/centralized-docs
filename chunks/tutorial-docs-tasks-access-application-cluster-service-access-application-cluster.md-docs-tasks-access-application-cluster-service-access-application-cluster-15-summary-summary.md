---
doc_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster
chunk_id: tutorial/docs-tasks-access-application-cluster-service-access-application-cluster.md/docs-tasks-access-application-cluster-service-access-application-cluster#15-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 108
summary: ``` `curl http://&lt;public-node-ip&gt;:&lt;node-port&gt; ` ``` where `&lt;public-node-ip&gt;` is the public IP address of your node, and `&lt;node-port&gt;` is the NodePort value for your service....
---

```
`curl http://&lt;public-node-ip&gt;:&lt;node-port&gt;
`
```
where `&lt;public-node-ip&gt;` is the public IP address of your node,
and `&lt;node-port&gt;` is the NodePort value for your service. The
response to a successful request is a hello message:
```
`Hello, world!
Version: 2.0.0
Hostname: hello-world-cdd4458f4-m47c8
`
```