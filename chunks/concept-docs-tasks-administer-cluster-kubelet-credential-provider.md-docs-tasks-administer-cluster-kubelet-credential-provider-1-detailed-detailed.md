---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Table of Contents
token_count: 1011
summary: - [Configure a kubelet image credential provider](#configure-a-kubelet-image-credential-provider)   - [Service Account Token for Image Pulls](#service-account-token-for-image-pulls)   - [Installing...
---

- [Configure a kubelet image credential provider](#configure-a-kubelet-image-credential-provider)
  - [Service Account Token for Image Pulls](#service-account-token-for-image-pulls)
  - [Installing Plugins on Nodes](#installing-plugins-on-nodes)
  - [Configuring the Kubelet](#configuring-the-kubelet)
- [providers is a list of credential provider helper plugins that will be enabled by the kubelet.](#providers-is-a-list-of-credential-provider-helper-plugins-that-will-be-enabled-by-the-kubelet)
- [Multiple providers may match against a single image, in which case credentials](#multiple-providers-may-match-against-a-single-image-in-which-case-credentials)
- [from all providers will be returned to the kubelet. If multiple providers are called](#from-all-providers-will-be-returned-to-the-kubelet-if-multiple-providers-are-called)
- [for a single image, the results are combined. If providers return overlapping](#for-a-single-image-the-results-are-combined-if-providers-return-overlapping)
- [auth keys, the value from the provider earlier in this list is used.](#auth-keys-the-value-from-the-provider-earlier-in-this-list-is-used)
- [name is the required name of the credential provider. It must match the name of the](#name-is-the-required-name-of-the-credential-provider-it-must-match-the-name-of-the)
- [provider executable as seen by the kubelet. The executable must be in the kubelet's](#provider-executable-as-seen-by-the-kubelet-the-executable-must-be-in-the-kubelets)
- [bin directory (set by the --image-credential-provider-bin-dir flag).](#bin-directory-set-by-the---image-credential-provider-bin-dir-flag)
- [matchImages is a required list of strings used to match against images in order to](#matchimages-is-a-required-list-of-strings-used-to-match-against-images-in-order-to)
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
- [- If the matchImages contains a port, then the port must match in the image as well.](#--if-the-matchimages-contains-a-port-then-the-port-must-match-in-the-image-as-well)
- [- registry.io:8080/path](#--registryio8080path)
- [defaultCacheDuration is the default duration the plugin will cache credentials in-memory](#defaultcacheduration-is-the-default-duration-the-plugin-will-cache-credentials-in-memory)