---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#30-summary
chunk_level: summary
chunk_type: prose
heading: Enable addons
token_count: 101
summary: service/monitoring-grafana NodePort 10.99.24.54 &lt;none&gt; 80:30002/TCP 26s service/monitoring-influxdb ClusterIP 10.111.169.94 &lt;none&gt; 8083/TCP,8086/TCP 26s ` ``` 4. Check the output from...
---

service/monitoring-grafana NodePort 10.99.24.54 &lt;none&gt; 80:30002/TCP 26s
service/monitoring-influxdb ClusterIP 10.111.169.94 &lt;none&gt; 8083/TCP,8086/TCP 26s
`
```
4. Check the output from `metrics-server`:
```
`kubectl top pods
`
```
The output is similar to: