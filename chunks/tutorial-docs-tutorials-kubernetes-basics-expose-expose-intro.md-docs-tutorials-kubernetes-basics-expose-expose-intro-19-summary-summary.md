---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#19-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 108
summary: ``` `kubectl expose deployment/kubernetes-bootcamp --type=\"NodePort\" --port 8080 ` ``` We have now a running Service called kubernetes-bootcamp. Here we see that the Service received a unique...
---

```
`kubectl expose deployment/kubernetes-bootcamp --type="NodePort" --port 8080
`
```
We have now a running Service called kubernetes-bootcamp. Here we see that the Service
received a unique cluster-IP, an internal port and an external-IP (the IP of the Node).
To find out what port was opened externally (for the `type: NodePort` Service) we’ll
run the `describe service` subcommand:
```
`kubectl describe services/kubernetes-bootcamp
`
```