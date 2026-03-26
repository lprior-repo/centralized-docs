---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#28-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 115
summary: ### Step 3: Deleting a service To delete Services you can use the `delete service` subcommand. Labels can be used also here: ``` `kubectl delete service -l app=kubernetes-bootcamp ` ``` Confirm that...
---

### Step 3: Deleting a service
To delete Services you can use the `delete service` subcommand. Labels can be used
also here:
```
`kubectl delete service -l app=kubernetes-bootcamp
`
```
Confirm that the Service is gone:
```
`kubectl get services
`
```
This confirms that our Service was removed. To confirm that route is not exposed
anymore you can `curl` the previously exposed IP and port:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```