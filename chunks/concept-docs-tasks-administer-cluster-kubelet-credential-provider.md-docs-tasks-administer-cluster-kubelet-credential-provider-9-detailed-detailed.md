---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#9-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 700
summary: #### Configure image matching The `matchImages` field for each credential provider is used by the kubelet to determine whether a plugin should be invoked for a given image that a Pod is using. Each...
---

#### Configure image matching
The `matchImages` field for each credential provider is used by the kubelet to determine whether a plugin should be invoked
for a given image that a Pod is using. Each entry in `matchImages` is an image pattern which can optionally contain a port and a path.
Globs can be used in the domain, but not in the port or the path. Globs are supported as subdomains like `\*.k8s.io` or `k8s.\*.io`,
and top-level domains such as `k8s.\*`. Matching partial subdomains like `app\*.k8s.io` is also supported. Each glob can only match
a single subdomain segment, so `\*.io` does NOT match `\*.k8s.io`.
A match exists between an image name and a `matchImage` entry when all of the below are true:
* Both contain the same number of domain parts and each part matches.
* The URL path of match image must be a prefix of the target image URL path.
* If the matchImages contains a port, then the port must match in the image as well.
Some example values of `matchImages` patterns are:
* `123456789.dkr.ecr.us-east-1.amazonaws.com`
* `\*.azurecr.io`
* `gcr.io`
* `\*.\*.registry.io`
* `foo.registry.io:8080/path`## What's next
* Read the details about `CredentialProviderConfig` in the
[kubelet configuration API (v1) reference](/docs/reference/config-api/kubelet-config.v1/).
* Read the [kubelet credential provider API reference (v1)](/docs/reference/config-api/kubelet-credentialprovider.v1/).
## Feedback
Was this page helpful?
Yes
No
Thanks for the feedback. If you have a specific, answerable question about how to use Kubernetes, ask it on
[Stack Overflow](https://stackoverflow.com/questions/tagged/kubernetes).
Open an issue in the [GitHub Repository](https://www.github.com/kubernetes/website/) if you want to
[report a problem](<https://github.com/kubernetes/website/issues/new?title=Issue with k8s.io>)
or
[suggest an improvement](<https://github.com/kubernetes/website/issues/new?title=Improvement for k8s.io>).
Last modified June 25, 2025 at 9:19 AM PST: [Add docs for PSAT for Kubelet Image Credential Providers beta (a618b01c1a)](https://github.com/kubernetes/website/commit/a618b01c1af51236e9b35e66d5176c7d8c884166)
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)