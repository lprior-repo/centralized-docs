---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#5-standard
chunk_level: standard
chunk_type: code
heading: Services and Labels
token_count: 495
summary: #### Note: If you're running minikube with Docker Desktop as the container driver, a minikube tunnel is needed. This is because containers inside Docker Desktop are isolated from your host computer....
---

#### Note:
If you're running minikube with Docker Desktop as the container driver, a minikube
tunnel is needed. This is because containers inside Docker Desktop are isolated
from your host computer.
In a separate terminal window, execute:
```
`minikube service kubernetes-bootcamp --url
`
```
The output looks like this:
```
`http://127.0.0.1:51082
! Because you are using a Docker driver on darwin, the terminal needs to be open to run it.
`
```
Then use the given URL to access the app:
```
`curl 127.0.0.1:51082
`
```
And we get a response from the server. The Service is exposed.
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
```
`kubectl get services -l app=kubernetes-bootcamp
`
```
Get the name of the Pod and store it in the POD\_NAME environment variable:
```
`export POD\_NAME="$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\\n"}}{{end}}')"
echo "Name of the Pod: $POD\_NAME"
`
```
To apply a new label we use the label subcommand followed by the object type,
object name and the new label:
```
`kubectl label pods "$POD\_NAME" version=v1
`
```
This will apply a new label to our Pod (we pinned the application version to the Pod),
and we can check it with the `describe pod` command:
```
`kubectl describe pods "$POD\_NAME"
`
```
We see here that the label is attached now to our Pod. And we can query now the
list of pods using the new label:
```
`kubectl get pods -l version=v1
`
```
And we see the Pod.