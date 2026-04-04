---
doc_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor
chunk_id: tutorial/docs-tutorials-security-apparmor.md/docs-tutorials-security-apparmor#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 799
summary: ## Authoring Profiles Getting AppArmor profiles specified correctly can be a tricky business. Fortunately there are some tools to help with that: * `aa-genprof` and `aa-logprof` generate profile...
---

## Authoring Profiles
Getting AppArmor profiles specified correctly can be a tricky business. Fortunately there are some
tools to help with that:
* `aa-genprof` and `aa-logprof` generate profile rules by monitoring an application's activity and
logs, and admitting the actions it takes. Further instructions are provided by the
[AppArmor documentation](https://gitlab.com/apparmor/apparmor/wikis/Profiling_with_tools).
* [bane](https://github.com/jfrazelle/bane) is an AppArmor profile generator for Docker that uses a
simplified profile language.
To debug problems with AppArmor, you can check the system logs to see what, specifically, was
denied. AppArmor logs verbose messages to `dmesg`, and errors can usually be found in the system
logs or through `journalctl`. More information is provided in
[AppArmor failures](https://gitlab.com/apparmor/apparmor/wikis/AppArmor_Failures).
#### Caution:
Prior to Kubernetes v1.30, AppArmor was specified through annotations. Use the documentation version
selector to view the documentation with this deprecated API.
### AppArmor profile within security context
You can specify the `appArmorProfile` on either a container's `securityContext` or on a Pod's
`securityContext`. If the profile is set at the pod level, it will be used as the default profile
for all containers in the pod (including init, sidecar, and ephemeral containers). If both a pod &amp; container
AppArmor profile are set, the container's profile will be used.
An AppArmor profile has 2 fields:
`type` *(required)* - indicates which kind of AppArmor profile will be applied. Valid options are:
`Localhost`a profile pre-loaded on the node (specified by `localhostProfile`).`RuntimeDefault`the container runtime's default profile.`Unconfined`no AppArmor enforcement.
`localhostProfile` - The name of a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
This option must be provided if and only if the `type` is `Localhost`.
## What's next
Additional resources:
* [Quick guide to the AppArmor profile language](https://gitlab.com/apparmor/apparmor/wikis/QuickProfileLanguage)
* [AppArmor core policy reference](https://gitlab.com/apparmor/apparmor/wikis/Policy_Layout)
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
Last modified January 08, 2025 at 10:50 AM PST: [Fix feature gate name conflicts (2/2) (3782732ce4)](https://github.com/kubernetes/website/commit/3782732ce415e1008ba589954371332b67993a66)
## Related Pages

- [Securing a Cluster](docs-tasks-administer-cluster-securing-a-cluster.md)
- [Binding](docs-reference-kubernetes-api-workload-resources-binding-v1.md)
- [conventions](docs-reference-kubectl-conventions.md)
- [HorizontalPodAutoscaler](docs-reference-kubernetes-api-workload-resources-horizontal-pod-autoscaler-v2.md)
- [Концепции](ru-docs-concepts.md)