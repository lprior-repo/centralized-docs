---
url: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/
title: Quantity
word_count: 509
filtered: true
elements_removed: 0
density_score: 0.84
---

## Table of Contents

- [Quantity](#quantity)
  - [Feedback](#feedback)

---

# Quantity
Quantity is a fixed-point representation of a number.
`import "k8s.io/apimachinery/pkg/api/resource"`
Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
The serialization format is:
```
` \\&lt;quantity&gt; ::= \\&lt;signedNumber&gt;\\&lt;suffix&gt;
(Note that \\&lt;suffix&gt; may be empty, from the "" case in \\&lt;decimalSI&gt;.)
\\&lt;digit&gt; ::= 0 | 1 | ... | 9 \\&lt;digits&gt; ::= \\&lt;digit&gt; | \\&lt;digit&gt;\\&lt;digits&gt; \\&lt;number&gt; ::= \\&lt;digits&gt; | \\&lt;digits&gt;.\\&lt;digits&gt; | \\&lt;digits&gt;. | .\\&lt;digits&gt; \\&lt;sign&gt; ::= "+" | "-" \\&lt;signedNumber&gt; ::= \\&lt;number&gt; | \\&lt;sign&gt;\\&lt;number&gt; \\&lt;suffix&gt; ::= \\&lt;binarySI&gt; | \\&lt;decimalExponent&gt; | \\&lt;decimalSI&gt; \\&lt;binarySI&gt; ::= Ki | Mi | Gi | Ti | Pi | Ei
(International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
\\&lt;decimalSI&gt; ::= m | "" | k | M | G | T | P | E
(Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
\\&lt;decimalExponent&gt; ::= "e" \\&lt;signedNumber&gt; | "E" \\&lt;signedNumber&gt;
`
```
No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
* No precision is lost - No fractional digits will be emitted - The exponent (or suffix) is as large as possible.
The sign will be omitted unless the number is negative.
Examples:
* 1.5 will be serialized as "1500m" - 1.5Gi will be serialized as "1536Mi"
Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
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
Last modified August 28, 2024 at 6:01 PM PST: [Update generated API reference for v1.31 (8ba98c79c1)](https://github.com/kubernetes/website/commit/8ba98c79c169bb070416a685db63074847399df5)
This page is automatically generated.
If you plan to report an issue with this page, mention that the page is auto-generated in your issue description. The fix may need to happen elsewhere in the Kubernetes project.
## Related Pages

- [Adding entries to Pod /etc/hosts with HostAliases](docs-tasks-network-customize-hosts-file-for-pods.md)
- [Change the Access Mode of a PersistentVolume to ReadWriteOncePod](docs-tasks-administer-cluster-change-pv-access-mode-readwriteoncepod.md)
- [Example: Deploying Cassandra with a StatefulSet](docs-tutorials-stateful-application-cassandra.md)
- [Configure Quality of Service for Pods](docs-tasks-configure-pod-container-quality-service-pod.md)
- [Configure Certificate Rotation for the Kubelet](docs-tasks-tls-certificate-rotation.md)
