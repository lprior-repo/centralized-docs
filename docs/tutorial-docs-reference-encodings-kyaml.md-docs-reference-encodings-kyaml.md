---
id: tutorial/docs-reference-encodings-kyaml.md/docs-reference-encodings-kyaml
title: KYAML Reference
category: tutorial
tags: ["contents", "getting", "kyaml", "reference", "started"]
---

## Table of Contents

* [KYAML Reference](#kyaml-reference)
  * [Getting started with KYAML](#getting-started-with-kyaml)
    * [Basic Structure](#basic-structure)
  * [Feedback](#feedback)

---

# KYAML Reference



 > 
 > **Context**: KYAML  is a safer and less ambiguous subset of YAML, initially introduced in Kubernetes v1.34 (alpha) and enabled by default in v1.35 (beta). Designed



**KYAML** is a safer and less ambiguous subset of YAML, initially introduced in Kubernetes v1.34 (alpha) and enabled by default in v1.35 (beta). Designed specifically for Kubernetes, KYAML addresses common YAML pitfalls such as whitespace sensitivity and implicit type coercion while maintaining full compatibility with existing YAML parsers and tooling.
This reference describes KYAML syntax.

## Getting started with KYAML

YAML’s reliance on indentation and implicit type coercion often leads to configuration errors, especially in CI/CD pipelines and templating systems like Helm. KYAML eliminates these issues by enforcing explicit syntax and structure, making configurations more reliable and easier to debug.

### Basic Structure

KYAML uses *flow style* syntax with `{}` for objects and `[]` for arrays. All string values must be **double-quoted**.

````
`---
{
apiVersion: "v1",
kind: "Pod",
metadata: {
name: "my-pod",
labels: {
app: "demo"
},
},
spec: {
containers: [{
name: "nginx",
image: "nginx:1.20"
}]
}
}
`
````

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
Last modified November 20, 2025 at 7:34 PM PST: [Add stub reference for KYAML (#52583) (048ea42ed6)](https://github.com/kubernetes/website/commit/048ea42ed65d5c583fd300bd3c0e06d7944c4697)

## Related Pages

* [Adding entries to Pod /etc/hosts with HostAliases](./ref-docs-tasks-network-customize-hosts-file-for-pods.md-docs-tasks-network-customize-hosts-file-for-pods.md)
* [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](./tutorial-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md-docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
* [Example: Deploying Cassandra with a StatefulSet](./tutorial-docs-tutorials-stateful-application-cassandra.md-docs-tutorials-stateful-application-cassandra.md)
* [Configure Quality of Service for Pods](./tutorial-docs-tasks-configure-pod-container-quality-service-pod.md-docs-tasks-configure-pod-container-quality-service-pod.md)
* [Configure Certificate Rotation for the Kubelet](./tutorial-docs-tasks-tls-certificate-rotation.md-docs-tasks-tls-certificate-rotation.md)
## See Also

- [Documentation Index](./COMPASS.md)
