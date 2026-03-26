---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#24-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 114
summary: ### Step 2: Using labels The Deployment created automatically a label for our Pod. With the `describe deployment` subcommand you can see the name (the *key*) of that label: ``` `kubectl describe...
---

### Step 2: Using labels
The Deployment created automatically a label for our Pod. With the `describe deployment`
subcommand you can see the name (the *key*) of that label:
```
`kubectl describe deployment
`
```
Let’s use this label to query our list of Pods. We’ll use the `kubectl get pods`
command with `-l` as a parameter, followed by the label values:
```
`kubectl get pods -l app=kubernetes-bootcamp
`
```
You can do the same to list the existing Services: