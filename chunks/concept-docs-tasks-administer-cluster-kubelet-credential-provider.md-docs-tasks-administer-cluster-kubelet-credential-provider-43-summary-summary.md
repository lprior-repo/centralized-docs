---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#43-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 120
summary: \*.amazonaws.com\" - \"\*.dkr.ecr.us-iso-east-1.c2s.ic.gov\" - \"\*.dkr.ecr.us-isob-east-1.sc2s.sgov.gov\" # defaultCacheDuration is the default duration the plugin will cache credentials in-memory # if a...
---

\*.amazonaws.com"
- "\*.dkr.ecr.us-iso-east-1.c2s.ic.gov"
- "\*.dkr.ecr.us-isob-east-1.sc2s.sgov.gov"
# defaultCacheDuration is the default duration the plugin will cache credentials in-memory
# if a cache duration is not provided in the plugin response. This field is required.
defaultCacheDuration: "12h"
# Required input version of the exec CredentialProviderRequest. The returned CredentialProviderResponse
# MUST use the same encoding version as the input. Current supported values are: