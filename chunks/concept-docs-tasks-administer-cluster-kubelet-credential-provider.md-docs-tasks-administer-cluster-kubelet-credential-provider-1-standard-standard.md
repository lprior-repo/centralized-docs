---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#1-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 505
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