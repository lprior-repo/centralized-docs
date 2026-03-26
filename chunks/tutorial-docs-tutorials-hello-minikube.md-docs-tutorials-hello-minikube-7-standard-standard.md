---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#7-standard
chunk_level: standard
chunk_type: prose
heading: Enable addons
token_count: 499
summary: 2. Enable an addon, for example, `metrics-server`: ``` `minikube addons enable metrics-server ` ``` The output is similar to: ``` `The 'metrics-server' addon is enabled ` ``` 3. View the Pod and...
---

2. Enable an addon, for example, `metrics-server`:
```
`minikube addons enable metrics-server
`
```
The output is similar to:
```
`The 'metrics-server' addon is enabled
`
```
3. View the Pod and Service you created by installing that addon:
```
`kubectl get pod,svc -n kube-system
`
```
The output is similar to:
```
`NAME READY STATUS RESTARTS AGE
pod/coredns-5644d7b6d9-mh9ll 1/1 Running 0 34m
pod/coredns-5644d7b6d9-pqd2t 1/1 Running 0 34m
pod/metrics-server-67fb648c5 1/1 Running 0 26s
pod/etcd-minikube 1/1 Running 0 34m
pod/influxdb-grafana-b29w8 2/2 Running 0 26s
pod/kube-addon-manager-minikube 1/1 Running 0 34m
pod/kube-apiserver-minikube 1/1 Running 0 34m
pod/kube-controller-manager-minikube 1/1 Running 0 34m
pod/kube-proxy-rnlps 1/1 Running 0 34m
pod/kube-scheduler-minikube 1/1 Running 0 34m
pod/storage-provisioner 1/1 Running 0 34m
NAME TYPE CLUSTER-IP EXTERNAL-IP PORT(S) AGE
service/metrics-server ClusterIP 10.96.241.45 &lt;none&gt; 80/TCP 26s
service/kube-dns ClusterIP 10.96.0.10 &lt;none&gt; 53/UDP,53/TCP 34m
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