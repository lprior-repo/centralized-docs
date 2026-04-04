---
id: ref/docs-reference-access-authn-authz-kubelet-authn-authz.md/docs-reference-access-authn-authz-kubelet-authn-authz
title: Docs Reference Access Authn Authz Kubelet Authn Authz
category: ref
tags: ["authentication", "contents", "kubelet", "overview", "ref"]
---

# Docs Reference Access Authn Authz Kubelet Authn Authz



 > 
 > **Context**: A kubelet's HTTPS endpoint exposes APIs which give access to data of varying sensitivity, and allow you to perform operations with varying levels of p



## Table of Contents

* [Overview](#overview)
* [Kubelet authentication](#kubelet-authentication)
  * [Warning:](#warning)
* [Feedback](#feedback)

---

## Overview

A kubelet’s HTTPS endpoint exposes APIs which give access to data of varying sensitivity,
and allow you to perform operations with varying levels of power on the node and within containers.
This document describes how to authenticate and authorize access to the kubelet’s HTTPS endpoint.

## Kubelet authentication

By default, requests to the kubelet’s HTTPS endpoint that are not rejected by other configured
authentication methods are treated as anonymous requests, and given a username of `system:anonymous`
and a group of `system:unauthenticated`.
To disable anonymous access and send `401 Unauthorized` responses to unauthenticated requests:

* start the kubelet with the `--anonymous-auth=false` flag
  To enable X509 client certificate authentication to the kubelet’s HTTPS endpoint:
* start the kubelet with the `--client-ca-file` flag, providing a CA bundle to verify client certificates with
* start the apiserver with `--kubelet-client-certificate` and `--kubelet-client-key` flags
* see the [apiserver authentication documentation](/docs/reference/access-authn-authz/authentication/#x509-client-certificates) for more details
  To enable API bearer tokens (including service account tokens) to be used to authenticate to the kubelet’s HTTPS endpoint:
* ensure the `authentication.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authentication-token-webhook` and `--kubeconfig` flags
* the kubelet calls the `TokenReview` API on the configured API server to determine user information from bearer tokens## Kubelet authorization
  Any request that is successfully authenticated (including an anonymous request) is then authorized. The default authorization mode is `AlwaysAllow`, which allows all requests.
  There are many possible reasons to subdivide access to the kubelet API:
* anonymous auth is enabled, but anonymous users’ ability to call the kubelet API should be limited
* bearer token auth is enabled, but arbitrary API users’ (like service accounts) ability to call the kubelet API should be limited
* client certificate auth is enabled, but only some of the client certificates signed by the configured CA should be allowed to use the kubelet API
  To subdivide access to the kubelet API, delegate authorization to the API server:
* ensure the `authorization.k8s.io/v1` API group is enabled in the API server
* start the kubelet with the `--authorization-mode=Webhook` and the `--kubeconfig` flags
* the kubelet calls the `SubjectAccessReview` API on the configured API server to determine whether each request is authorized
  The kubelet authorizes API requests using the same [request attributes](/docs/reference/access-authn-authz/authorization/#review-your-request-attributes) approach as the apiserver.
  The verb is determined from the incoming request’s HTTP verb:
  \|HTTP verb|request verb|
  \|POST|create|
  \|GET, HEAD|get|
  \|PUT|update|
  \|PATCH|patch|
  \|DELETE|delete|
  The resource and subresource is determined from the incoming request’s path:
  \|Kubelet API|resource|subresource|
  \|/stats/\*|nodes|stats|
  \|/metrics/\*|nodes|metrics|
  \|/logs/\*|nodes|log|
  \|/spec/\*|nodes|spec|
  \|/checkpoint/\*|nodes|checkpoint|
  \|*all others*\|nodes|proxy|

### Warning:

`nodes/proxy` permission grants access to all other kubelet APIs.
This includes APIs that can be used to execute commands in any container running on the node.
Some of these endpoints support Websocket protocols via HTTP `GET` requests, which are authorized with the **get** verb.
This means that **get** permission on `nodes/proxy` is not a read-only permission,
and authorizes executing commands in any container running on the node.
The namespace and API group attributes are always an empty string, and
the resource name is always the name of the kubelet’s `Node` API object.
When running in this mode, ensure the user identified by the `--kubelet-client-certificate` and `--kubelet-client-key`
flags passed to the apiserver is authorized for the following attributes:

* verb=\*, resource=nodes, subresource=proxy
* verb=\*, resource=nodes, subresource=stats
* verb=\*, resource=nodes, subresource=log
* verb=\*, resource=nodes, subresource=spec
* verb=\*, resource=nodes, subresource=metrics### Fine-grained authorization
  FEATURE STATE:
  `Kubernetes v1.33 [beta]`(enabled by default)
  When the feature gate `KubeletFineGrainedAuthz` is enabled kubelet performs a
  fine-grained check before falling back to the `proxy` subresource for the `/pods`,
  `/runningPods`, `/configz` and `/healthz` endpoints. The resource and subresource
  are determined from the incoming request’s path:
  \|Kubelet API|resource|subresource|
  \|/stats/\*|nodes|stats|
  \|/metrics/\*|nodes|metrics|
  \|/logs/\*|nodes|log|
  \|/pods|nodes|pods, proxy|
  \|/runningPods/|nodes|pods, proxy|
  \|/healthz|nodes|healthz, proxy|
  \|/configz|nodes|configz, proxy|
  \|*all others*\|nodes|proxy|
  When the feature-gate `KubeletFineGrainedAuthz` is enabled, ensure the user
  identified by the `--kubelet-client-certificate` and `--kubelet-client-key`
  flags passed to the API server is authorized for the following attributes:
* verb=\*, resource=nodes, subresource=proxy
* verb=\*, resource=nodes, subresource=stats
* verb=\*, resource=nodes, subresource=log
* verb=\*, resource=nodes, subresource=metrics
* verb=\*, resource=nodes, subresource=configz
* verb=\*, resource=nodes, subresource=healthz
* verb=\*, resource=nodes, subresource=pods
  If [RBAC authorization](/docs/reference/access-authn-authz/rbac/) is used,
  enabling this gate also ensure that the builtin `system:kubelet-api-admin` ClusterRole
  is updated with permissions to access all the above mentioned subresources.

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
Last modified January 28, 2026 at 10:04 AM PST: [Strengthen and clarify nodes/proxy warnings (b78e1d54b8)](https://github.com/kubernetes/website/commit/b78e1d54b8caee2f7b3433386ad9d7a8ebefeb25)

## Related Pages

* [Certificates and Certificate Signing Requests](./ref-docs-reference-access-authn-authz-certificate-signing-requests.md-docs-reference-access-authn-authz-certificate-signing-requests.md)
* [Controlling Access to the Kubernetes API](./ref-docs-concepts-security-controlling-access.md-docs-concepts-security-controlling-access.md)
* [Hardening Guide - Authentication Mechanisms](./ref-docs-concepts-security-hardening-guide-authentication-mechanisms.md-docs-concepts-security-hardening-guide-authentication-mechanisms.md)
* [Securing a Cluster](./tutorial-docs-tasks-administer-cluster-securing-a-cluster.md-docs-tasks-administer-cluster-securing-a-cluster.md)
* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
## See Also

- [Documentation Index](./COMPASS.md)
