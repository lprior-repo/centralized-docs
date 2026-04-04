---
doc_id: ops/docs-contribute-advanced.md/docs-contribute-advanced
chunk_id: ops/docs-contribute-advanced.md/docs-contribute-advanced#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Serve as a New Contributor Ambassador
token_count: 931
summary: # Advanced contributing This page assumes that you understand how to [contribute to new content](/docs/contribute/new-content/) and [review others' work](/docs/contribute/review/reviewing-prs/), and...
---

# Advanced contributing
This page assumes that you understand how to
[contribute to new content](/docs/contribute/new-content/) and
[review others' work](/docs/contribute/review/reviewing-prs/), and are ready
to learn about more ways to contribute. You need to use the Git command line
client and other tools for some of these tasks.
## Propose improvements
SIG Docs [members](/docs/contribute/participate/roles-and-responsibilities/#members)
can propose improvements.
After you've been contributing to the Kubernetes documentation for a while, you
may have ideas for improving the [Style Guide](/docs/contribute/style/style-guide/)
, the [Content Guide](/docs/contribute/style/content-guide/), the toolchain used to build
the documentation, the website style, the processes for reviewing and merging
pull requests, or other aspects of the documentation. For maximum transparency,
these types of proposals need to be discussed in a SIG Docs meeting or on the
[kubernetes-sig-docs mailing list](https://groups.google.com/forum/#!forum/kubernetes-sig-docs).
In addition, it can help to have some context about the way things
currently work and why past decisions have been made before proposing sweeping
changes. The quickest way to get answers to questions about how the documentation
currently works is to ask in the `#sig-docs` Slack channel on
[kubernetes.slack.com](https://kubernetes.slack.com)
After the discussion has taken place and the SIG is in agreement about the desired
outcome, you can work on the proposed changes in the way that is the most
appropriate. For instance, an update to the style guide or the website's
functionality might involve opening a pull request, while a change related to
documentation testing might involve working with sig-testing.
## Coordinate docs for a Kubernetes release
SIG Docs [approvers](/docs/contribute/participate/roles-and-responsibilities/#approvers)
can coordinate docs for a Kubernetes release.
Each Kubernetes release is coordinated by a team of people participating in the
sig-release Special Interest Group (SIG). Others on the release team for a given
release include an overall release lead, as well as representatives from
sig-testing and others. To find out more about Kubernetes release processes,
refer to
[https://github.com/kubernetes/sig-release](https://github.com/kubernetes/sig-release).
The SIG Docs representative for a given release coordinates the following tasks:
* Monitor the feature-tracking spreadsheet for new or changed features with an
impact on documentation. If the documentation for a given feature won't be ready
for the release, the feature may not be allowed to go into the release.
* Attend sig-release meetings regularly and give updates on the status of the
docs for the release.
* Review and copyedit feature documentation drafted by the SIG responsible for
implementing the feature.
* Merge release-related pull requests and maintain the Git feature branch for
the release.
* Mentor other SIG Docs contributors who want to learn how to do this role in
the future. This is known as "shadowing".
* Publish the documentation changes related to the release when the release
artifacts are published.
Coordinating a release is typically a 3-4 month commitment, and the duty is
rotated among SIG Docs approvers.
## Serve as a New Contributor Ambassador
SIG Docs [approvers](/docs/contribute/participate/roles-and-responsibilities/#approvers)
can serve as New Contributor Ambassadors.
New Contributor Ambassadors welcome new contributors to SIG-Docs,
suggest PRs to new contributors, and mentor new contributors through their first
few PR submissions.
Responsibilities for New Contributor Ambassadors include:
* Monitoring the [#sig-docs Slack channel](https://kubernetes.slack.com) for questions from new contributors.
* Working with PR wranglers to identify [good first issues](https://kubernetes.dev/docs/guide/help-wanted/#good-first-issue) for new contributors.
* Mentoring new contributors through their first few PRs to the docs repo.
* Helping new contributors create the more complex PRs they need to become Kubernetes members.
* [Sponsoring contributors](/docs/contribute/advanced/#sponsor-a-new-contributor) on their path to becoming Kubernetes members.
* Hosting a monthly meeting to help and mentor new contributors.
Current New Contributor Ambassadors are announced at each SIG-Docs meeting and in the [Kubernetes #sig-docs channel](https://kubernetes.slack.com).