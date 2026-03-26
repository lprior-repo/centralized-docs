---
doc_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube
chunk_id: tutorial/docs-tutorials-hello-minikube.md/docs-tutorials-hello-minikube#6-standard
chunk_level: standard
chunk_type: prose
heading: Enable addons
token_count: 261
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