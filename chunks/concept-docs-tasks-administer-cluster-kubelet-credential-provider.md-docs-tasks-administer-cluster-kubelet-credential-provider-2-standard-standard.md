---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#2-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 489
summary: - [determine if this provider should be invoked. If one of the strings matches the](#determine-if-this-provider-should-be-invoked-if-one-of-the-strings-matches-the) - [requested image from the...
---

- [determine if this provider should be invoked. If one of the strings matches the](#determine-if-this-provider-should-be-invoked-if-one-of-the-strings-matches-the)
- [requested image from the kubelet, the plugin will be invoked and given a chance](#requested-image-from-the-kubelet-the-plugin-will-be-invoked-and-given-a-chance)
- [to provide credentials. Images are expected to contain the registry domain](#to-provide-credentials-images-are-expected-to-contain-the-registry-domain)
- [Each entry in matchImages is a pattern which can optionally contain a port and a path.](#each-entry-in-matchimages-is-a-pattern-which-can-optionally-contain-a-port-and-a-path)
- [Globs can be used in the domain, but not in the port or the path. Globs are supported](#globs-can-be-used-in-the-domain-but-not-in-the-port-or-the-path-globs-are-supported)
- [as subdomains like '\*.k8s.io' or 'k8s.\*.io', and top-level-domains such as 'k8s.\*'.](#as-subdomains-like-k8sio-or-k8sio-and-top-level-domains-such-as-k8s)
- [Matching partial subdomains like 'app\*.k8s.io' is also supported. Each glob can only match](#matching-partial-subdomains-like-appk8sio-is-also-supported-each-glob-can-only-match)
- [a single subdomain segment, so `\*.io` does \*\*not\*\* match `\*.k8s.io`.](#a-single-subdomain-segment-so-io-does-not-match-k8sio)
- [A match exists between an image and a matchImage when all of the below are true:](#a-match-exists-between-an-image-and-a-matchimage-when-all-of-the-below-are-true)
- [- Both contain the same number of domain parts and each part matches.](#--both-contain-the-same-number-of-domain-parts-and-each-part-matches)
- [- The URL path of an matchImages must be a prefix of the target image URL path.](#--the-url-path-of-an-matchimages-must-be-a-prefix-of-the-target-image-url-path)