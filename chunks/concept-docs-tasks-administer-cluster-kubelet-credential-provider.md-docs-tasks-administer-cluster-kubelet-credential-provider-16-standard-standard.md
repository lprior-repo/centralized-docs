---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#16-standard
chunk_level: standard
chunk_type: prose
heading: Configuring the Kubelet
token_count: 364
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