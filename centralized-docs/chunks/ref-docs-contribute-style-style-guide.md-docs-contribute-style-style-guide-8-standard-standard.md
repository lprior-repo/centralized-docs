---
doc_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide
chunk_id: ref/docs-contribute-style-style-guide.md/docs-contribute-style-style-guide#8-standard
chunk_level: standard
chunk_type: table
heading: Language
token_count: 402
summary: ### Use code style for object field names and namespaces Do and Don't - Use code style for object field names|Do|Don't| |Set the value of the `replicas` field in the configuration file.|Set the value...
---

### Use code style for object field names and namespaces
Do and Don't - Use code style for object field names|Do|Don't|
|Set the value of the `replicas` field in the configuration file.|Set the value of the "replicas" field in the configuration file.|
|The value of the `exec` field is an ExecAction object.|The value of the "exec" field is an ExecAction object.|
|Run the process as a DaemonSet in the `kube-system` namespace.|Run the process as a DaemonSet in the kube-system namespace.|
### Use code style for Kubernetes command tool and component names
Do and Don't - Use code style for Kubernetes command tool and component names|Do|Don't|
|The `kubelet` preserves node stability.|The kubelet preserves node stability.|
|The `kubectl` handles locating and authenticating to the API server.|The kubectl handles locating and authenticating to the apiserver.|
|Run the process with the certificate, `kube-apiserver --client-ca-file=FILENAME`.|Run the process with the certificate, kube-apiserver --client-ca-file=FILENAME.|
### Starting a sentence with a component tool or component name
Do and Don't - Starting a sentence with a component tool or component name|Do|Don't|
|The `kubeadm` tool bootstraps and provisions machines in a cluster.|`kubeadm` tool bootstraps and provisions machines in a cluster.|
|The kube-scheduler is the default scheduler for Kubernetes.|kube-scheduler is the default scheduler for Kubernetes.|
### Use a general descriptor over a component name
Do and Don't - Use a general descriptor over a component name|Do|Don't|
|The Kubernetes API server offers an OpenAPI spec.|The apiserver offers an OpenAPI spec.|
|Aggregated APIs are subordinate API servers.|Aggregated APIs are subordinate APIServers.|