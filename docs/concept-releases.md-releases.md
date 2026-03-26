---
id: concept/releases.md/releases
title: Releases
category: concept
tags: ["concept", "contents", "releases", "table"]
---

## Table of Contents

* [Releases](#releases)
  * [1.35](#135)
  * [1.34](#134)
  * [1.33](#133)
  * [End-of-Life Releases](#end-of-life-releases)
    * [Note:](#note)
  * [Upcoming Release](#upcoming-release)
    * [Note:](#note)
  * [Helpful Resources](#helpful-resources)

---

# Releases



 > 
 > **Context**: The Kubernetes project maintains release branches for the most recent three minor releases (1.35, 1.34, 1.33). Kubernetes 1.19 and newer receive appro



The Kubernetes project maintains release branches for the most recent three minor releases
(1.35, 1.34, 1.33).
Kubernetes 1.19 and newer receive
[approximately 1 year of patch support](/releases/patch-releases/#support-period).
Kubernetes 1.18 and older received approximately 9 months of patch support.
Kubernetes versions are expressed as **x.y.z**,
where **x** is the major version, **y** is the minor version, and **z** is the patch version,
following [Semantic Versioning](https://semver.org/) terminology.
More information in the [version skew policy](/releases/version-skew-policy/) document.

## 1.35

\*\*Latest Release:\*\*1.35.3 (released: 2026-03-19)
\*\*End of Life:\*\*2027-02-28
**Patch Releases:**
[1.35.1](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.35.md#v1351),
[1.35.2](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.35.md#v1352),
[1.35.3](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.35.md#v1353)
Complete 1.35
[Schedule](/releases/patch-releases/#1-35) and
[Changelog](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.35.md)
[View release details](/releases/1.35//)

### 1.34

\*\*Latest Release:\*\*1.34.6 (released: 2026-03-19)
\*\*End of Life:\*\*2026-10-27
**Patch Releases:**
[1.34.0](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1340),
[1.34.1](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1341),
[1.34.2](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1342),
[1.34.3](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1343),
[1.34.4](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1344),
[1.34.5](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1345),
[1.34.6](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md#v1346)
Complete 1.34
[Schedule](/releases/patch-releases/#1-34) and
[Changelog](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.34.md)
[View release details](/releases/1.34//)

### 1.33

\*\*Latest Release:\*\*1.33.10 (released: 2026-03-19)
\*\*End of Life:\*\*2026-06-28
**Patch Releases:**
[1.33.1](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1331),
[1.33.2](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1332),
[1.33.3](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1333),
[1.33.4](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1334),
[1.33.5](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1335),
[1.33.6](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1336),
[1.33.7](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1337),
[1.33.8](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1338),
[1.33.9](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v1339),
[1.33.10](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md#v13310)
Complete 1.33
[Schedule](/releases/patch-releases/#1-33) and
[Changelog](https://git.k8s.io/kubernetes/CHANGELOG/CHANGELOG-1.33.md)
[View release details](/releases/1.33//)

## End-of-Life Releases

Older Kubernetes releases that are no longer maintained are listed below.
End-of-life releases

### Note:

These releases are no longer supported and do not receive security updates or bug fixes.
If you are running one of these releases, the Kubernetes project strongly recommends upgrading to a [supported version](#release-history).
\|Minor Version|Final Patch Release|End Of Life Date|Note|
\|[1.32](/releases/1.32/)\|1.32.13|2026-02-28||
\|[1.31](/releases/1.31/)\|1.31.14|2025-11-11||
\|[1.30](/releases/1.30/)\|1.30.14|2025-07-15||
\|[1.29](/releases/1.29/)\|1.29.14|2025-02-28||
\|[1.28](/releases/1.28/)\|1.28.15|2024-10-22||
\|[1.27](/releases/1.27/)\|1.27.16|2024-07-16||
\|[1.26](/releases/1.26/)\|1.26.15|2024-02-28|1.26.15 was released in March 2024 (after the EOL date) to pick up a new version of Go to [address several Go CVEs](https://groups.google.com/g/golang-dev/c/o1I1Vv8Rfgs/m/Wr8tD1RlAgAJ)\|
\|[1.25](/releases/1.25/)\|1.25.16|2023-10-28|1.25.16 was released in November 2023 (after the EOL date) to fix [CVE-2023-5528](https://groups.google.com/g/kubernetes-announce/c/c3py6Fw0DTI/m/cScFSdk1BwAJ)\|
\|[1.24](/releases/1.24/)\|1.24.17|2023-07-28|1.24.17 was released in August 2023 (after the EOL date) to fix CVE-2023-3676 and CVE-2023-3955|
\|[1.23](/releases/1.23/)\|1.23.17|2023-02-28||
\|[1.22](/releases/1.22/)\|1.22.17|2022-12-08|1.22.17 was released in December 2022 (after the EOL date) to backport registry changes and fix two critical issues.|
\|[1.21](/releases/1.21/)\|1.21.14|2022-06-28||
\|[1.20](/releases/1.20/)\|1.20.15|2022-02-28||
\|[1.19](/releases/1.19/)\|1.19.16|2021-10-28||
\|[1.18](/releases/1.18/)\|1.18.20|2021-06-18|Created to solve regression introduced in 1.18.19|
\|[1.17](/releases/1.17/)\|1.17.17|2021-01-13||
\|[1.16](/releases/1.16/)\|1.16.15|2020-09-02||
\|[1.15](/releases/1.15/)\|1.15.12|2020-05-06||
\|[1.14](/releases/1.14/)\|1.14.10|2019-12-11||
\|[1.13](/releases/1.13/)\|1.13.12|2019-10-15||
\|[1.12](/releases/1.12/)\|1.12.10|2019-07-08||
\|[1.11](/releases/1.11/)\|1.11.10|2019-05-01||
\|[1.10](/releases/1.10/)\|1.10.13|2019-02-13||
\|[1.9](/releases/1.9/)\|1.9.11|2018-09-29||
\|[1.8](/releases/1.8/)\|1.8.15|2018-07-12||
\|[1.7](/releases/1.7/)\|1.7.16|2018-04-04||
\|[1.6](/releases/1.6/)\|1.6.13|2017-11-23||
\|[1.5](/releases/1.5/)\|1.5.8|2017-10-01||
\|[1.4](/releases/1.4/)\|1.4.12|2017-04-21||
\|[1.3](/releases/1.3/)\|1.3.10|2016-11-01||
\|[1.2](/releases/1.2/)\|1.2.7|2016-10-23||

## Upcoming Release

Check out the [schedule](https://github.com/kubernetes/sig-release/tree/master/releases/release-1.36)
for the upcoming **1.36** Kubernetes release!

### Note:

This schedule link may be temporarily unavailable during early release planning phases.
Check the [SIG Release repository](https://github.com/kubernetes/sig-release/tree/master/releases) for the latest updates.

## Helpful Resources

Refer to the [Kubernetes Release Team](https://github.com/kubernetes/sig-release/tree/master/release-team) resources
for key information on roles and the release process.

## Related Pages

* [Creating a cluster with kubeadm](./ref-docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md-docs-setup-production-environment-tools-kubeadm-create-cluster-kubeadm.md)
## See Also

- [Documentation Index](./COMPASS.md)
