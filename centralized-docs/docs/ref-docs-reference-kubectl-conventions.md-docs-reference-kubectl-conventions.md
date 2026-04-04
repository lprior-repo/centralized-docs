---
id: ref/docs-reference-kubectl-conventions.md/docs-reference-kubectl-conventions
title: Docs Reference Kubectl Conventions
category: ref
tags: ["contents", "kubectl", "ref", "reusable", "scripts"]
---

# Docs Reference Kubectl Conventions



 > 
 > **Context**: For a stable output in a script:



## Table of Contents

* [Using `kubectl` in Reusable Scripts](#using-kubectl-in-reusable-scripts)
  * [`kubectl run`](#kubectl-run)
  * [`kubectl apply`](#kubectl-apply)
* [Feedback](#feedback)

---

## Using `kubectl` in Reusable Scripts

For a stable output in a script:

* Request one of the machine-oriented output forms, such as `-o name`, `-o json`, `-o yaml`, `-o go-template`, or `-o jsonpath`.
* Fully-qualify the version. For example, `jobs.v1.batch/myjob`. This will ensure that kubectl does not use its default version that can change over time.
* Don’t rely on context, preferences, or other implicit states.## Subresources
* You can use the `--subresource` argument for kubectl subcommands such as `get`, `patch`,
  `edit`, `apply` and `replace` to fetch and update subresources for all resources that
  support them. In Kubernetes version 1.35, only the `status`, `scale`
  and `resize` subresources are supported.
* For `kubectl edit`, the `scale` subresource is not supported. If you use `--subresource` with
  `kubectl edit` and specify `scale` as the subresource, the command will error out.
* The API contract against a subresource is identical to a full resource. While updating the
  `status` subresource to a new value, keep in mind that the subresource could be potentially
  reconciled by a controller to a different value.## Best Practices

### `kubectl run`

For `kubectl run` to satisfy infrastructure as code:

* Tag the image with a version-specific tag and don’t move that tag to a new version. For example, use `:v1234`, `v1.2.3`, `r03062016-1-4`, rather than `:latest` (For more information, see [Kubernetes Configuration Good Practices](/blog/2025/11/25/configuration-good-practices/)).
* Check in the script for an image that is heavily parameterized.
* Switch to configuration files checked into source control for features that are needed, but not expressible via `kubectl run` flags.
  You can use the `--dry-run=client` flag to preview the object that would be sent to your cluster, without really submitting it.

### `kubectl apply`

* You can use `kubectl apply` to create or update resources. For more information about using kubectl apply to update resources, see [Kubectl Book](https://kubectl.docs.kubernetes.io).

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
Last modified November 24, 2025 at 9:22 PM PST: [Migrate K8s Configuration Good Practices to blog (b90c59015c)](https://github.com/kubernetes/website/commit/b90c59015c5d3affd8e83c298dd6dc30e331d92e)

## Related Pages

* [Objects In Kubernetes](./ref-docs-concepts-overview-working-with-objects.md-docs-concepts-overview-working-with-objects.md)
* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
* [Using RBAC Authorization](./ref-docs-reference-access-authn-authz-rbac.md-docs-reference-access-authn-authz-rbac.md)
## See Also

- [Documentation Index](./COMPASS.md)
