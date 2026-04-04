---
doc_id: ref/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity
chunk_id: ref/docs-reference-kubernetes-api-common-definitions-quantity.md/docs-reference-kubernetes-api-common-definitions-quantity#1-standard
chunk_level: standard
chunk_type: table
heading: Table of Contents
token_count: 421
summary: # Quantity Quantity is a fixed-point representation of a number. `import \"k8s.io/apimachinery/pkg/api/resource\"` Quantity is a fixed-point representation of a number. It provides convenient...
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