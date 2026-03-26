---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#41-summary
chunk_level: summary
chunk_type: prose
heading: Configuring the Kubelet
token_count: 125
summary: \*.io', and top-level-domains such as 'k8s.\*'. # Matching partial subdomains like 'app\*.k8s.io' is also supported. Each glob can only match # a single subdomain segment, so `\*.io` does \*\*not\*\*...
---

\*.io', and top-level-domains such as 'k8s.\*'.
# Matching partial subdomains like 'app\*.k8s.io' is also supported. Each glob can only match
# a single subdomain segment, so `\*.io` does \*\*not\*\* match `\*.k8s.io`.
# A match exists between an image and a matchImage when all of the below are true:
# - Both contain the same number of domain parts and each part matches.
# - The URL path of an matchImages must be a prefix of the target image URL path.