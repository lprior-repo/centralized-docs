---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#3-detailed
chunk_level: detailed
chunk_type: code
heading: What's next
token_count: 998
summary: ## Enable addons The minikube tool includes a set of built-in [addons](/docs/concepts/cluster-administration/addons/) that can be enabled, disabled and opened in the local Kubernetes environment. 1....
---

## Enable addons
The minikube tool includes a set of built-in [addons](/docs/concepts/cluster-administration/addons/) that can be enabled, disabled and opened in the local Kubernetes environment.
1. List the currently supported addons:
```
`minikube addons list
`
```
The output is similar to:
```
`addon-manager: enabled
dashboard: enabled
default-storageclass: enabled
efk: disabled
freshpod: disabled
gvisor: disabled
helm-tiller: disabled
ingress: disabled
ingress-dns: disabled
logviewer: disabled
metrics-server: disabled
nvidia-driver-installer: disabled
nvidia-gpu-device-plugin: disabled
registry: disabled
registry-creds: disabled
storage-provisioner: enabled
storage-provisioner-gluster: disabled
`
```
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
```
`NAME CPU(cores) MEMORY(bytes)
hello-node-ccf4b9788-4jn97 1m 6Mi
`
```
If you see the following message, wait, and try again:
```
`error: Metrics API not available
`
```
5. Disable `metrics-server`:
```
`minikube addons disable metrics-server
`
```
The output is similar to:
```
`metrics-server was successfully disabled
`
```
## Clean up
Now you can clean up the resources you created in your cluster:
```
`kubectl delete service hello-node
kubectl delete deployment hello-node
`
```
Stop the Minikube cluster
```
`minikube stop
`
```
Optionally, delete the Minikube VM:
```
`# Optional
minikube delete
`
```
If you want to use minikube again to learn more about Kubernetes, you don't need to delete it.
## Conclusion
This page covered the basic aspects to get a minikube cluster up and running. You are now ready to deploy applications.
## What's next
* Tutorial to *[deploy your first app on Kubernetes with kubectl](/docs/tutorials/kubernetes-basics/deploy-app/deploy-intro/)*.
* Learn more about [Deployment objects](/docs/concepts/workloads/controllers/deployment/).
* Learn more about [Deploying applications](/docs/tasks/run-application/run-stateless-application-deployment/).
* Learn more about [Service objects](/docs/concepts/services-networking/service/).