---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#42-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 114
summary: # - The URL path of an matchImages must be a prefix of the target image URL path. # - If the matchImages contains a port, then the port must match in the image as well. # - registry.io:8080/path...
---

# - The URL path of an matchImages must be a prefix of the target image URL path.
# - If the matchImages contains a port, then the port must match in the image as well.
# - registry.io:8080/path
matchImages:
- "\*.dkr.ecr.\*.amazonaws.com"
- "\*.dkr.ecr.\*.amazonaws.com.cn"
- "\*.dkr.ecr-fips.\*.amazonaws.com"
- "\*.dkr.ecr.us-iso-east-1.c2s.ic.gov"