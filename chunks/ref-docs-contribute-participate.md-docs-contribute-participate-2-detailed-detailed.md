---
doc_id: ref/docs-contribute-participate.md/docs-contribute-participate
chunk_id: ref/docs-contribute-participate.md/docs-contribute-participate#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Related Pages
token_count: 621
summary: ## How merging works When a pull request is merged to the branch used to publish content, that content is published to [https://kubernetes.io](https://kubernetes.io). To ensure that the quality of...
---

## How merging works
When a pull request is merged to the branch used to publish content, that content is published to [https://kubernetes.io](https://kubernetes.io). To ensure that
the quality of our published content is high, we limit merging pull requests to
SIG Docs approvers. Here's how it works.
* When a pull request has both the `lgtm` and `approve` labels, has no `hold`
labels, and all tests are passing, the pull request merges automatically.
* Kubernetes organization members and SIG Docs approvers can add comments to
prevent automatic merging of a given pull request (by adding a `/hold` comment
or withholding a `/lgtm` comment).
* Any Kubernetes member can add the `lgtm` label by adding a `/lgtm` comment.
* Only SIG Docs approvers can merge a pull request
by adding an `/approve` comment. Some approvers also perform additional
specific roles, such as [PR Wrangler](/docs/contribute/participate/pr-wranglers/) or
[SIG Docs chairperson](#sig-docs-chairperson).## What's next
For more information about contributing to the Kubernetes documentation, see:
* [Contributing new content](/docs/contribute/new-content/)
* [Reviewing content](/docs/contribute/review/reviewing-prs/)
* [Documentation style guide](/docs/contribute/style/)
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
Last modified November 04, 2024 at 1:03 AM PST: [[en] update kubernetes link (bb0f2565ba)](https://github.com/kubernetes/website/commit/bb0f2565badc65d8e13eca23225134bda53f0057)
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)