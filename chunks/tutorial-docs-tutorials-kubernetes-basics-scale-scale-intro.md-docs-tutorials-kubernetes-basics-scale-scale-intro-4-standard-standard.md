---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#4-standard
chunk_level: standard
chunk_type: code
heading: Scaling overview
token_count: 485
summary: ### Load Balancing Let's check that the Service is load-balancing the traffic. To find out the exposed IP and Port we can use `describe service` as we learned in the previous part of the tutorial:...
---

### Load Balancing
Let's check that the Service is load-balancing the traffic. To find out the exposed
IP and Port we can use `describe service` as we learned in the previous part of the tutorial:
```
`kubectl describe services/kubernetes-bootcamp
`
```
Create an environment variable called NODE\_PORT that has a value as the Node port:
```
`export NODE\_PORT="$(kubectl get services/kubernetes-bootcamp -o go-template='{{(index .spec.ports 0).nodePort}}')"
echo NODE\_PORT=$NODE\_PORT
`
```
Next, we’ll do a `curl` to the exposed IP address and port. Execute the command multiple times:
```
`curl http://"$(minikube ip):$NODE\_PORT"
`
```
We hit a different Pod with every request. This demonstrates that the load-balancing is working.
The output should be similar to:
```
`Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-wp67j | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-hs9dj | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-4hjvf | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-wp67j | v=1
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-644c5687f4-4hjvf | v=1
`
```
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