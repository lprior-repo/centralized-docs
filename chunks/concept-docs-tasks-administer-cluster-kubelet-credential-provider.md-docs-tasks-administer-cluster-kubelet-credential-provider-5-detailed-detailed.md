---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#5-detailed
chunk_level: detailed
chunk_type: prose
heading: Configuring the Kubelet
token_count: 151
summary: ## Configuring the Kubelet In order to use this feature, the kubelet expects two flags to be set: * `--image-credential-provider-config` - the path to the credential provider plugin config file. *...
---

## Configuring the Kubelet
In order to use this feature, the kubelet expects two flags to be set:
* `--image-credential-provider-config` - the path to the credential provider plugin config file.
* `--image-credential-provider-bin-dir` - the path to the directory where credential provider plugin binaries are located.### Configure a kubelet credential provider
The configuration file passed into `--image-credential-provider-config` is read by the kubelet to determine which exec plugins
should be invoked for which container images. Here's an example configuration file you may end up using if you are using the
[ECR-based plugin](https://github.com/kubernetes/cloud-provider-aws/tree/master/cmd/ecr-credential-provider):