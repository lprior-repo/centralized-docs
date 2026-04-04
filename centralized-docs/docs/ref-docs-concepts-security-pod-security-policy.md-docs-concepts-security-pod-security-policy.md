---
id: ref/docs-concepts-security-pod-security-policy.md/docs-concepts-security-pod-security-policy
title: Docs Concepts Security Pod Security Policy
category: ref
tags: ["contents", "feature", "feedback", "ref", "removed"]
---

# Docs Concepts Security Pod Security Policy



 > 
 > **Context**: PodSecurityPolicy was  deprecated in Kubernetes v1.21, and removed from Kubernetes in v1.25. Instead of using PodSecurityPolicy, you can enforce simil



## Table of Contents

      - [Removed feature](#removed-feature)
    

* [Feedback](#feedback)

---

### Removed feature

PodSecurityPolicy was [deprecated](/blog/2021/04/08/kubernetes-1-21-release-announcement/#podsecuritypolicy-deprecation)
in Kubernetes v1.21, and removed from Kubernetes in v1.25.
Instead of using PodSecurityPolicy, you can enforce similar restrictions on Pods using
either or both:

* [Pod Security Admission](/docs/concepts/security/pod-security-admission/)
* a 3rd party admission plugin, that you deploy and configure yourself
  For a migration guide, see [Migrate from PodSecurityPolicy to the Built-In PodSecurity Admission Controller](/docs/tasks/configure-pod-container/migrate-from-psp/).
  For more information on the removal of this API,
  see [PodSecurityPolicy Deprecation: Past, Present, and Future](/blog/2021/04/06/podsecuritypolicy-deprecation-past-present-and-future/).
  If you are not running Kubernetes v1.35, check the documentation for
  your version of Kubernetes.

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
Last modified November 05, 2022 at 6:22 PM PST: [Tweak page about PSP removal (4e006c898d)](https://github.com/kubernetes/website/commit/4e006c898ddea5556da5872cefbac1ac8e6c5308)

## Related Pages

* [Securing a Cluster](./tutorial-docs-tasks-administer-cluster-securing-a-cluster.md-docs-tasks-administer-cluster-securing-a-cluster.md)
* [Binding](./ref-docs-reference-kubernetes-api-workload-resources-binding-v1.md-docs-reference-kubernetes-api-workload-resources-binding-v1.md)
* [conventions](./ref-docs-reference-kubectl-conventions.md-docs-reference-kubectl-conventions.md)
* [HorizontalPodAutoscaler](./ref-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md-docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
* [Концепции](./ref-ru-docs-concepts.md-ru-docs-concepts.md)
## See Also

- [Documentation Index](./COMPASS.md)
