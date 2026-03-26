---
doc_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint
chunk_id: ref/docs-reference-kubectl-generated-kubectltaint.md/docs-reference-kubectl-generated-kubectltaint#1-standard
chunk_level: standard
chunk_type: prose
heading: Examples
token_count: 419
summary: # kubectl taint Update the taints on one or more nodes ## Synopsis Update the taints on one or more nodes. * A taint consists of a key, value, and effect. As an argument here, it is expressed as...
---

# kubectl taint
Update the taints on one or more nodes
## Synopsis
Update the taints on one or more nodes.
* A taint consists of a key, value, and effect. As an argument here, it is expressed as key=value:effect.
* The key must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 253 characters.
* Optionally, the key can begin with a DNS subdomain prefix and a single '/', like example.com/my-app.
* The value is optional. If given, it must begin with a letter or number, and may contain letters, numbers, hyphens, dots, and underscores, up to 63 characters.
* The effect must be NoSchedule, PreferNoSchedule or NoExecute.
* Currently taint can only apply to node.
```
`kubectl taint NODE NAME KEY\_1=VAL\_1:TAINT\_EFFECT\_1 ... KEY\_N=VAL\_N:TAINT\_EFFECT\_N
`
```
## Examples
```
` # Update node 'foo' with a taint with key 'dedicated' and value 'special-user' and effect 'NoSchedule'
# If a taint with that key and effect already exists, its value is replaced as specified
kubectl taint nodes foo dedicated=special-user:NoSchedule
# Remove from node 'foo' the taint with key 'dedicated' and effect 'NoSchedule' if one exists
kubectl taint nodes foo dedicated:NoSchedule-
# Remove from node 'foo' all the taints with key 'dedicated'
kubectl taint nodes foo dedicated-
# Add a taint with key 'dedicated' on nodes having label myLabel=X
kubectl taint node -l myLabel=X dedicated=foo:PreferNoSchedule
# Add to node 'foo' a taint with key 'bar' and no value
kubectl taint nodes foo bar:NoSchedule
`
```