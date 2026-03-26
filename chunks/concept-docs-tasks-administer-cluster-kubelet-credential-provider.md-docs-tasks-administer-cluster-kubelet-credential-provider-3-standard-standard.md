---
doc_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
chunk_id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider#3-standard
chunk_level: standard
chunk_type: prose
heading: Table of Contents
token_count: 475
summary: - [- Both contain the same number of domain parts and each part matches.](#--both-contain-the-same-number-of-domain-parts-and-each-part-matches) - [- The URL path of an matchImages must be a prefix...
---

- [- Both contain the same number of domain parts and each part matches.](#--both-contain-the-same-number-of-domain-parts-and-each-part-matches)
- [- The URL path of an matchImages must be a prefix of the target image URL path.](#--the-url-path-of-an-matchimages-must-be-a-prefix-of-the-target-image-url-path)
- [- If the matchImages contains a port, then the port must match in the image as well.](#--if-the-matchimages-contains-a-port-then-the-port-must-match-in-the-image-as-well)
- [- registry.io:8080/path](#--registryio8080path)
- [defaultCacheDuration is the default duration the plugin will cache credentials in-memory](#defaultcacheduration-is-the-default-duration-the-plugin-will-cache-credentials-in-memory)
- [if a cache duration is not provided in the plugin response. This field is required.](#if-a-cache-duration-is-not-provided-in-the-plugin-response-this-field-is-required)
- [Required input version of the exec CredentialProviderRequest. The returned CredentialProviderResponse](#required-input-version-of-the-exec-credentialproviderrequest-the-returned-credentialproviderresponse)
- [MUST use the same encoding version as the input. Current supported values are:](#must-use-the-same-encoding-version-as-the-input-current-supported-values-are)
- [Arguments to pass to the command when executing it.](#arguments-to-pass-to-the-command-when-executing-it)
- [Env defines additional environment variables to expose to the process. These](#env-defines-additional-environment-variables-to-expose-to-the-process-these)
- [are unioned with the host's environment, as well as variables client-go uses](#are-unioned-with-the-hosts-environment-as-well-as-variables-client-go-uses)
- [tokenAttributes is the configuration for the service account token that will be passed to the plugin.](#tokenattributes-is-the-configuration-for-the-service-account-token-that-will-be-passed-to-the-plugin)
- [The credential provider opts in to using service account tokens for image pull by setting this field.](#the-credential-provider-opts-in-to-using-service-account-tokens-for-image-pull-by-setting-this-field)