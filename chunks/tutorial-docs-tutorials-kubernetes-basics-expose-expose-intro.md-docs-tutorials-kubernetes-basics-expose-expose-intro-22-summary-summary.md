---
doc_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-expose-expose-intro.md/docs-tutorials-kubernetes-basics-expose-expose-intro#22-summary
chunk_level: summary
chunk_type: prose
heading: Services and Labels
token_count: 125
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