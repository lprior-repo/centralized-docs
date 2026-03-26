---
doc_id: ref/docs-contribute-participate.md/docs-contribute-participate
chunk_id: ref/docs-contribute-participate.md/docs-contribute-participate#1-detailed
chunk_level: detailed
chunk_type: prose
heading: SIG Docs teams and automation
token_count: 829
summary: # Participating in SIG Docs SIG Docs is one of the [special interest groups](https://github.com/kubernetes/community/blob/master/sig-list.md) within the Kubernetes project, focused on writing,...
---

# Participating in SIG Docs
SIG Docs is one of the
[special interest groups](https://github.com/kubernetes/community/blob/master/sig-list.md)
within the Kubernetes project, focused on writing, updating, and maintaining
the documentation for Kubernetes as a whole. See
[SIG Docs from the community github repo](https://github.com/kubernetes/community/tree/master/sig-docs)
for more information about the SIG.
SIG Docs welcomes content and reviews from all contributors. Anyone can open a
pull request (PR), and anyone is welcome to file issues about content or comment
on pull requests in progress.
You can also become a [member](/docs/contribute/participate/roles-and-responsibilities/#members),
[reviewer](/docs/contribute/participate/roles-and-responsibilities/#reviewers), or
[approver](/docs/contribute/participate/roles-and-responsibilities/#approvers).
These roles require greater access and entail certain responsibilities for
approving and committing changes. See
[community-membership](https://github.com/kubernetes/community/blob/master/community-membership.md)
for more information on how membership works within the Kubernetes community.
The rest of this document outlines some unique ways these roles function within
SIG Docs, which is responsible for maintaining one of the most public-facing
aspects of Kubernetes -- the Kubernetes website and documentation.
## SIG Docs chairperson
Each SIG, including SIG Docs, selects one or more SIG members to act as
chairpersons. These are points of contact between SIG Docs and other parts of
the Kubernetes organization. They require extensive knowledge of the structure
of the Kubernetes project as a whole and how SIG Docs works within it. See
[Leadership](https://github.com/kubernetes/community/tree/master/sig-docs#leadership)
for the current list of chairpersons.
## SIG Docs teams and automation
Automation in SIG Docs relies on two different mechanisms:
GitHub teams and OWNERS files.
### GitHub teams
There are two categories of SIG Docs [teams](https://github.com/orgs/kubernetes/teams?query=sig-docs) on GitHub:
* `@sig-docs-{language}-owners` are approvers and leads
* `@sig-docs-{language}-reviews` are reviewers
Each can be referenced with their `@name` in GitHub comments to communicate with
everyone in that group.
Sometimes Prow and GitHub teams overlap without matching exactly. For
assignment of issues, pull requests, and to support PR approvals, the
automation uses information from `OWNERS` files.
### OWNERS files and front-matter
The Kubernetes project uses an automation tool called prow for automation
related to GitHub issues and pull requests. The
[Kubernetes website repository](https://github.com/kubernetes/website) uses
two [prow plugins](https://github.com/kubernetes-sigs/prow/tree/main/pkg/plugins):
* blunderbuss
* approve
These two plugins use the
[OWNERS](https://github.com/kubernetes/website/blob/main/OWNERS) and
[OWNERS\_ALIASES](https://github.com/kubernetes/website/blob/main/OWNERS_ALIASES)
files in the top level of the `kubernetes/website` GitHub repository to control
how prow works within the repository.
An OWNERS file contains a list of people who are SIG Docs reviewers and
approvers. OWNERS files can also exist in subdirectories, and can override who
can act as a reviewer or approver of files in that subdirectory and its
descendants. For more information about OWNERS files in general, see
[OWNERS](https://github.com/kubernetes/community/blob/master/contributors/guide/owners.md).
In addition, an individual Markdown file can list reviewers and approvers in its
front-matter, either by listing individual GitHub usernames or GitHub groups.
The combination of OWNERS files and front-matter in Markdown files determines
the advice PR owners get from automated systems about who to ask for technical
and editorial review of their PR.