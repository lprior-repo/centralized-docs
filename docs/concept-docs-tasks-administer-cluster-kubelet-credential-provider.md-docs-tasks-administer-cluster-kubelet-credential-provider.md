---
id: concept/docs-tasks-administer-cluster-kubelet-credential-provider.md/docs-tasks-administer-cluster-kubelet-credential-provider
title: Configure a kubelet image credential provider
category: concept
tags: ["account", "concept", "configure", "contents", "credential"]
---

## Table of Contents

* [Configure a kubelet image credential provider](#configure-a-kubelet-image-credential-provider)
  * [Service Account Token for Image Pulls](#service-account-token-for-image-pulls)
  * [Installing Plugins on Nodes](#installing-plugins-on-nodes)
  * [Configuring the Kubelet](#configuring-the-kubelet)
* [providers is a list of credential provider helper plugins that will be enabled by the kubelet.](#providers-is-a-list-of-credential-provider-helper-plugins-that-will-be-enabled-by-the-kubelet)
* [Multiple providers may match against a single image, in which case credentials](#multiple-providers-may-match-against-a-single-image-in-which-case-credentials)
* [from all providers will be returned to the kubelet. If multiple providers are called](#from-all-providers-will-be-returned-to-the-kubelet-if-multiple-providers-are-called)
* [for a single image, the results are combined. If providers return overlapping](#for-a-single-image-the-results-are-combined-if-providers-return-overlapping)
* [auth keys, the value from the provider earlier in this list is used.](#auth-keys-the-value-from-the-provider-earlier-in-this-list-is-used)
* [name is the required name of the credential provider. It must match the name of the](#name-is-the-required-name-of-the-credential-provider-it-must-match-the-name-of-the)
* [provider executable as seen by the kubelet. The executable must be in the kubelet’s](#provider-executable-as-seen-by-the-kubelet-the-executable-must-be-in-the-kubelets)
* [bin directory (set by the –image-credential-provider-bin-dir flag).](#bin-directory-set-by-the---image-credential-provider-bin-dir-flag)
* [matchImages is a required list of strings used to match against images in order to](#matchimages-is-a-required-list-of-strings-used-to-match-against-images-in-order-to)
* [determine if this provider should be invoked. If one of the strings matches the](#determine-if-this-provider-should-be-invoked-if-one-of-the-strings-matches-the)
* [requested image from the kubelet, the plugin will be invoked and given a chance](#requested-image-from-the-kubelet-the-plugin-will-be-invoked-and-given-a-chance)
* [to provide credentials. Images are expected to contain the registry domain](#to-provide-credentials-images-are-expected-to-contain-the-registry-domain)
* [Each entry in matchImages is a pattern which can optionally contain a port and a path.](#each-entry-in-matchimages-is-a-pattern-which-can-optionally-contain-a-port-and-a-path)
* [Globs can be used in the domain, but not in the port or the path. Globs are supported](#globs-can-be-used-in-the-domain-but-not-in-the-port-or-the-path-globs-are-supported)
* [as subdomains like ‘\*.k8s.io’ or ‘k8s.\*.io’, and top-level-domains such as ‘k8s.\*’.](#as-subdomains-like-k8sio-or-k8sio-and-top-level-domains-such-as-k8s)
* [Matching partial subdomains like ‘app\*.k8s.io’ is also supported. Each glob can only match](#matching-partial-subdomains-like-appk8sio-is-also-supported-each-glob-can-only-match)
* [a single subdomain segment, so `\*.io` does \*\*not\*\* match `\*.k8s.io`.](#a-single-subdomain-segment-so-io-does-not-match-k8sio)
* [A match exists between an image and a matchImage when all of the below are true:](#a-match-exists-between-an-image-and-a-matchimage-when-all-of-the-below-are-true)
* [- Both contain the same number of domain parts and each part matches.](#--both-contain-the-same-number-of-domain-parts-and-each-part-matches)
* [- The URL path of an matchImages must be a prefix of the target image URL path.](#--the-url-path-of-an-matchimages-must-be-a-prefix-of-the-target-image-url-path)
* [- If the matchImages contains a port, then the port must match in the image as well.](#--if-the-matchimages-contains-a-port-then-the-port-must-match-in-the-image-as-well)
* [- registry.io:8080/path](#--registryio8080path)
* [defaultCacheDuration is the default duration the plugin will cache credentials in-memory](#defaultcacheduration-is-the-default-duration-the-plugin-will-cache-credentials-in-memory)
* [if a cache duration is not provided in the plugin response. This field is required.](#if-a-cache-duration-is-not-provided-in-the-plugin-response-this-field-is-required)
* [Required input version of the exec CredentialProviderRequest. The returned CredentialProviderResponse](#required-input-version-of-the-exec-credentialproviderrequest-the-returned-credentialproviderresponse)
* [MUST use the same encoding version as the input. Current supported values are:](#must-use-the-same-encoding-version-as-the-input-current-supported-values-are)
* [Arguments to pass to the command when executing it.](#arguments-to-pass-to-the-command-when-executing-it)
* [Env defines additional environment variables to expose to the process. These](#env-defines-additional-environment-variables-to-expose-to-the-process-these)
* [are unioned with the host’s environment, as well as variables client-go uses](#are-unioned-with-the-hosts-environment-as-well-as-variables-client-go-uses)
* [tokenAttributes is the configuration for the service account token that will be passed to the plugin.](#tokenattributes-is-the-configuration-for-the-service-account-token-that-will-be-passed-to-the-plugin)
* [The credential provider opts in to using service account tokens for image pull by setting this field.](#the-credential-provider-opts-in-to-using-service-account-tokens-for-image-pull-by-setting-this-field)
* [if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled,](#if-this-field-is-set-without-the-kubeletserviceaccounttokenforcredentialproviders-feature-gate-enabled)
* [kubelet will fail to start with invalid configuration error.](#kubelet-will-fail-to-start-with-invalid-configuration-error)
* [serviceAccountTokenAudience is the intended audience for the projected service account token.](#serviceaccounttokenaudience-is-the-intended-audience-for-the-projected-service-account-token)
* [cacheType indicates the type of cache key use for caching the credentials returned by the plugin](#cachetype-indicates-the-type-of-cache-key-use-for-caching-the-credentials-returned-by-the-plugin)
* [The most conservative option is to set this to “Token”, which means the kubelet will cache](#the-most-conservative-option-is-to-set-this-to-token-which-means-the-kubelet-will-cache)
* [returned credentials on a per-token basis. This should be set if the returned credential’s](#returned-credentials-on-a-per-token-basis-this-should-be-set-if-the-returned-credentials)
* [lifetime is limited to the service account token’s lifetime.](#lifetime-is-limited-to-the-service-account-tokens-lifetime)
* [If the plugin’s credential retrieval logic depends only on the service account and not on](#if-the-plugins-credential-retrieval-logic-depends-only-on-the-service-account-and-not-on)
* [pod-specific claims, then the plugin can set this to “ServiceAccount”. In this case, the](#pod-specific-claims-then-the-plugin-can-set-this-to-serviceaccount-in-this-case-the)
* [kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the](#kubelet-will-cache-returned-credentials-on-a-per-serviceaccount-basis-use-this-when-the)
* [returned credential is valid for all pods using the same service account.](#returned-credential-is-valid-for-all-pods-using-the-same-service-account)
* [requireServiceAccount indicates whether the plugin requires the pod to have a service account.](#requireserviceaccount-indicates-whether-the-plugin-requires-the-pod-to-have-a-service-account)
* [If set to true, kubelet will only invoke the plugin if the pod has a service account.](#if-set-to-true-kubelet-will-only-invoke-the-plugin-if-the-pod-has-a-service-account)
* [If set to false, kubelet will invoke the plugin even if the pod does not have a service account](#if-set-to-false-kubelet-will-invoke-the-plugin-even-if-the-pod-does-not-have-a-service-account)
* [and will not include a token in the CredentialProviderRequest. This is useful for plugins](#and-will-not-include-a-token-in-the-credentialproviderrequest-this-is-useful-for-plugins)
* [that are used to pull images for pods without service accounts (e.g., static pods).](#that-are-used-to-pull-images-for-pods-without-service-accounts-eg-static-pods)
* [requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in](#requiredserviceaccountannotationkeys-is-the-list-of-annotation-keys-that-the-plugin-is-interested-in)
* [and that are required to be present in the service account.](#and-that-are-required-to-be-present-in-the-service-account)
* [The keys defined in this list will be extracted from the corresponding service account and passed](#the-keys-defined-in-this-list-will-be-extracted-from-the-corresponding-service-account-and-passed)
* [to the plugin as part of the CredentialProviderRequest. If any of the keys defined in this list](#to-the-plugin-as-part-of-the-credentialproviderrequest-if-any-of-the-keys-defined-in-this-list)
* [are not present in the service account, kubelet will not invoke the plugin and will return an error.](#are-not-present-in-the-service-account-kubelet-will-not-invoke-the-plugin-and-will-return-an-error)
* [This field is optional and may be empty. Plugins may use this field to extract additional information](#this-field-is-optional-and-may-be-empty-plugins-may-use-this-field-to-extract-additional-information)
* [required to fetch credentials or allow workloads to opt in to using service account tokens for image pull.](#required-to-fetch-credentials-or-allow-workloads-to-opt-in-to-using-service-account-tokens-for-image-pull)
* [The keys defined in this list must be unique and not overlap with the keys defined in the](#the-keys-defined-in-this-list-must-be-unique-and-not-overlap-with-the-keys-defined-in-the)
* [optionalServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in](#optionalserviceaccountannotationkeys-is-the-list-of-annotation-keys-that-the-plugin-is-interested-in)
* [and that are optional to be present in the service account.](#and-that-are-optional-to-be-present-in-the-service-account)
* [The keys defined in this list will be extracted from the corresponding service account and passed](#the-keys-defined-in-this-list-will-be-extracted-from-the-corresponding-service-account-and-passed)
* [to the plugin as part of the CredentialProviderRequest. The plugin is responsible for validating the](#to-the-plugin-as-part-of-the-credentialproviderrequest-the-plugin-is-responsible-for-validating-the)
* [existence of annotations and their values. This field is optional and may be empty.](#existence-of-annotations-and-their-values-this-field-is-optional-and-may-be-empty)
* [Plugins may use this field to extract additional information required to fetch credentials.](#plugins-may-use-this-field-to-extract-additional-information-required-to-fetch-credentials)
* [The keys defined in this list must be unique and not overlap with the keys defined in the](#the-keys-defined-in-this-list-must-be-unique-and-not-overlap-with-the-keys-defined-in-the)
* [+optional](#optional)
  * [Configure image matching](#configure-image-matching)
  * [Feedback](#feedback)

---

# Configure a kubelet image credential provider



 > 
 > **Context**: FEATURE STATE: Kubernetes v1.26 [stable] Starting from Kubernetes v1.20, the kubelet can dynamically retrieve credentials for a container image regist



FEATURE STATE:
`Kubernetes v1.26 [stable]`
Starting from Kubernetes v1.20, the kubelet can dynamically retrieve credentials for a container image registry
using exec plugins. The kubelet and the exec plugin communicate through stdio (stdin, stdout, and stderr) using
Kubernetes versioned APIs. These plugins allow the kubelet to request credentials for a container registry dynamically
as opposed to storing static credentials on disk. For example, the plugin may talk to a local metadata server to retrieve
short-lived credentials for an image that is being pulled by the kubelet.
You may be interested in using this capability if any of the below are true:

* API calls to a cloud provider service are required to retrieve authentication information for a registry.
* Credentials have short expiration times and requesting new credentials frequently is required.
* Storing registry credentials on disk or in imagePullSecrets is not acceptable.
  This guide demonstrates how to configure the kubelet’s image credential provider plugin mechanism.

## Service Account Token for Image Pulls

FEATURE STATE:
`Kubernetes v1.34 [beta]`(enabled by default)
Starting from Kubernetes v1.33,
the kubelet can be configured to send a service account token
bound to the pod for which the image pull is being performed
to the credential provider plugin.
This allows the plugin to exchange the token for credentials
to access the image registry.
To enable this feature,
the `KubeletServiceAccountTokenForCredentialProviders` feature gate
must be enabled on the kubelet,
and the `tokenAttributes` field must be set
in the `CredentialProviderConfig` file for the plugin.
The `tokenAttributes` field contains information
about the service account token that will be passed to the plugin,
including the intended audience for the token
and whether the plugin requires the pod to have a service account.
Using service account token credentials can enable the following use-cases:

* Avoid needing a kubelet/node-based identity to pull images from a registry.
* Allow workloads to pull images based on their own runtime identity
  without long-lived/persisted secrets.## Before you begin
* You need a Kubernetes cluster with nodes that support kubelet credential
  provider plugins. This support is available in Kubernetes 1.35;
  Kubernetes v1.24 and v1.25 included this as a beta feature, enabled by default.
* If you are configuring a credential provider plugin
  that requires the service account token,
  you need a Kubernetes cluster with nodes running Kubernetes v1.33 or later
  and the `KubeletServiceAccountTokenForCredentialProviders` feature gate
  enabled on the kubelet.
* A working implementation of a credential provider exec plugin. You can build your own plugin or use one provided by cloud providers.Your Kubernetes server must be at or later than version v1.26.
  To check the version, enter `kubectl version`.

## Installing Plugins on Nodes

A credential provider plugin is an executable binary that will be run by the kubelet. Ensure that the plugin binary exists on
every node in your cluster and stored in a known directory. The directory will be required later when configuring kubelet flags.

## Configuring the Kubelet

In order to use this feature, the kubelet expects two flags to be set:

* `--image-credential-provider-config` - the path to the credential provider plugin config file.
* `--image-credential-provider-bin-dir` - the path to the directory where credential provider plugin binaries are located.### Configure a kubelet credential provider
  The configuration file passed into `--image-credential-provider-config` is read by the kubelet to determine which exec plugins
  should be invoked for which container images. Here’s an example configuration file you may end up using if you are using the
  [ECR-based plugin](https://github.com/kubernetes/cloud-provider-aws/tree/master/cmd/ecr-credential-provider):

````
`apiVersion: kubelet.config.k8s.io/v1
kind: CredentialProviderConfig
# providers is a list of credential provider helper plugins that will be enabled by the kubelet.
# Multiple providers may match against a single image, in which case credentials
# from all providers will be returned to the kubelet. If multiple providers are called
# for a single image, the results are combined. If providers return overlapping
# auth keys, the value from the provider earlier in this list is used.
providers:
# name is the required name of the credential provider. It must match the name of the
# provider executable as seen by the kubelet. The executable must be in the kubelet's
# bin directory (set by the --image-credential-provider-bin-dir flag).
- name: ecr-credential-provider
# matchImages is a required list of strings used to match against images in order to
# determine if this provider should be invoked. If one of the strings matches the
# requested image from the kubelet, the plugin will be invoked and given a chance
# to provide credentials. Images are expected to contain the registry domain
# Each entry in matchImages is a pattern which can optionally contain a port and a path.
# Globs can be used in the domain, but not in the port or the path. Globs are supported
# as subdomains like '\*.k8s.io' or 'k8s.\*.io', and top-level-domains such as 'k8s.\*'.
# Matching partial subdomains like 'app\*.k8s.io' is also supported. Each glob can only match
# a single subdomain segment, so `\*.io` does \*\*not\*\* match `\*.k8s.io`.
# A match exists between an image and a matchImage when all of the below are true:
# - Both contain the same number of domain parts and each part matches.
# - The URL path of an matchImages must be a prefix of the target image URL path.
# - If the matchImages contains a port, then the port must match in the image as well.
# - registry.io:8080/path
matchImages:
- "\*.dkr.ecr.\*.amazonaws.com"
- "\*.dkr.ecr.\*.amazonaws.com.cn"
- "\*.dkr.ecr-fips.\*.amazonaws.com"
- "\*.dkr.ecr.us-iso-east-1.c2s.ic.gov"
- "\*.dkr.ecr.us-isob-east-1.sc2s.sgov.gov"
# defaultCacheDuration is the default duration the plugin will cache credentials in-memory
# if a cache duration is not provided in the plugin response. This field is required.
defaultCacheDuration: "12h"
# Required input version of the exec CredentialProviderRequest. The returned CredentialProviderResponse
# MUST use the same encoding version as the input. Current supported values are:
# Arguments to pass to the command when executing it.
# Env defines additional environment variables to expose to the process. These
# are unioned with the host's environment, as well as variables client-go uses
# tokenAttributes is the configuration for the service account token that will be passed to the plugin.
# The credential provider opts in to using service account tokens for image pull by setting this field.
# if this field is set without the `KubeletServiceAccountTokenForCredentialProviders` feature gate enabled,
# kubelet will fail to start with invalid configuration error.
# serviceAccountTokenAudience is the intended audience for the projected service account token.
# cacheType indicates the type of cache key use for caching the credentials returned by the plugin
# The most conservative option is to set this to "Token", which means the kubelet will cache
# returned credentials on a per-token basis. This should be set if the returned credential's
# lifetime is limited to the service account token's lifetime.
# If the plugin's credential retrieval logic depends only on the service account and not on
# pod-specific claims, then the plugin can set this to "ServiceAccount". In this case, the
# kubelet will cache returned credentials on a per-serviceaccount basis. Use this when the
# returned credential is valid for all pods using the same service account.
# requireServiceAccount indicates whether the plugin requires the pod to have a service account.
# If set to true, kubelet will only invoke the plugin if the pod has a service account.
# If set to false, kubelet will invoke the plugin even if the pod does not have a service account
# and will not include a token in the CredentialProviderRequest. This is useful for plugins
# that are used to pull images for pods without service accounts (e.g., static pods).
# requiredServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are required to be present in the service account.
# The keys defined in this list will be extracted from the corresponding service account and passed
# to the plugin as part of the CredentialProviderRequest. If any of the keys defined in this list
# are not present in the service account, kubelet will not invoke the plugin and will return an error.
# This field is optional and may be empty. Plugins may use this field to extract additional information
# required to fetch credentials or allow workloads to opt in to using service account tokens for image pull.
# The keys defined in this list must be unique and not overlap with the keys defined in the
# optionalServiceAccountAnnotationKeys is the list of annotation keys that the plugin is interested in
# and that are optional to be present in the service account.
# The keys defined in this list will be extracted from the corresponding service account and passed
# to the plugin as part of the CredentialProviderRequest. The plugin is responsible for validating the
# existence of annotations and their values. This field is optional and may be empty.
# Plugins may use this field to extract additional information required to fetch credentials.
# The keys defined in this list must be unique and not overlap with the keys defined in the
# +optional
optionalServiceAccountAnnotationKeys:
- "example.com/optional-annotation-key-1"
- "example.com/optional-annotation-key-2"
`
````

The `providers` field is a list of enabled plugins used by the kubelet. Each entry has a few required fields:

* `name`: the name of the plugin which MUST match the name of the executable binary that exists
  in the directory passed into `--image-credential-provider-bin-dir`.
* `matchImages`: a list of strings used to match against images in order to determine
  if this provider should be invoked. More on this below.
* `defaultCacheDuration`: the default duration the kubelet will cache credentials in-memory
  if a cache duration was not specified by the plugin.
* `apiVersion`: the API version that the kubelet and the exec plugin will use when communicating.
  Each credential provider can also be given optional args and environment variables as well.
  Consult the plugin implementors to determine what set of arguments and environment variables are required for a given plugin.
  If you are using the KubeletServiceAccountTokenForCredentialProviders feature gate
  and configuring the plugin to use the service account token
  by setting the tokenAttributes field,
  the following fields are required:
* `serviceAccountTokenAudience`:
  the intended audience for the projected service account token.
  This cannot be the empty string.
* `cacheType`:
  the type of cache key used for caching the credentials returned by the plugin
  when the service account token is used.
  The most conservative option is to set this to `Token`,
  which means the kubelet will cache returned credentials
  on a per-token basis.
  This should be set if the returned credential’s lifetime
  is limited to the service account token’s lifetime.
  If the plugin’s credential retrieval logic depends only on the service account
  and not on pod-specific claims,
  then the plugin can set this to `ServiceAccount`.
  In this case, the kubelet will cache returned credentials
  on a per-service account basis.
  Use this when the returned credential is valid for all pods using the same service account.
* `requireServiceAccount`:
  whether the plugin requires the pod to have a service account.
* If set to `true`, kubelet will only invoke the plugin
  if the pod has a service account.
* If set to `false`, kubelet will invoke the plugin
  even if the pod does not have a service account
  and will not include a token in the `CredentialProviderRequest`.
  This is useful for plugins that are used
  to pull images for pods without service accounts
  (e.g., static pods).

### Configure image matching

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
* `foo.registry.io:8080/path`\## What’s next
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

* [Adding entries to Pod /etc/hosts with HostAliases](./ref-docs-tasks-network-customize-hosts-file-for-pods.md-docs-tasks-network-customize-hosts-file-for-pods.md)
* [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](./tutorial-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
* [Example: Deploying Cassandra with a StatefulSet](./tutorial-docs-tutorials-stateful-application-cassandra.md-docs-tutorials-stateful-application-cassandra.md)
* [Configure Quality of Service for Pods](./tutorial-docs-tasks-configure-pod-container-quality-service-pod.md-docs-tasks-configure-pod-container-quality-service-pod.md)
* [Configure Certificate Rotation for the Kubelet](./tutorial-docs-tasks-tls-certificate-rotation.md-docs-tasks-tls-certificate-rotation.md)
## See Also

- [Documentation Index](./COMPASS.md)
