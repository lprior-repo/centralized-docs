---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#61-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 114
summary: * Both contain the same number of domain parts and each part matches. * The URL path of match image must be a prefix of the target image URL path. * If the matchImages contains a port, then the port...
---

* Both contain the same number of domain parts and each part matches.
* The URL path of match image must be a prefix of the target image URL path.
* If the matchImages contains a port, then the port must match in the image as well.
Some example values of `matchImages` patterns are:
* `123456789.dkr.ecr.us-east-1.amazonaws.com`
* `\*.azurecr.io`
* `gcr.io`
* `\*.\*.registry.io`
* `foo.registry.io:8080/path`## What's next