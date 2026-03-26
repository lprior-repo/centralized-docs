---
doc_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-scale-scale-intro.md/docs-tutorials-kubernetes-basics-scale-scale-intro#1-detailed
chunk_level: detailed
chunk_type: code
heading: Scaling overview
token_count: 721
summary: ## Scaling overview ![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg) ![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg) *Scaling is accomplished by...
---

## Scaling overview
![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg)
![](/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg)
*Scaling is accomplished by changing the number of replicas in a Deployment.*
Scaling out a Deployment will ensure new Pods are created and scheduled to Nodes
with available resources. Scaling will increase the number of Pods to the new desired
state. Kubernetes also supports [autoscaling](/docs/concepts/workloads/autoscaling/)
of Pods, but it is outside of the scope of this tutorial. Scaling to zero is also
possible, and it will terminate all Pods of the specified Deployment.
Running multiple instances of an application will require a way to distribute the
traffic to all of them. Services have an integrated load-balancer that will distribute
network traffic to all Pods of an exposed Deployment. Services will monitor continuously
the running Pods using endpoints, to ensure the traffic is sent only to available Pods.
Once you have multiple instances of an application running, you would be able to
do Rolling updates without downtime. We'll cover that in the next section of the
tutorial. Now, let's go to the terminal and scale our application.
### Scaling a Deployment
To list your Deployments, use the `get deployments` subcommand:
```
`kubectl get deployments
`
```
The output should be similar to:
```
`NAME READY UP-TO-DATE AVAILABLE AGE
kubernetes-bootcamp 1/1 1 1 11m
`
```
We should have 1 Pod. If not, run the command again. This shows:
* *NAME* lists the names of the Deployments in the cluster.
* *READY* shows the ratio of CURRENT/DESIRED replicas
* *UP-TO-DATE* displays the number of replicas that have been updated to achieve the desired state.
* *AVAILABLE* displays how many replicas of the application are available to your users.
* *AGE* displays the amount of time that the application has been running.
To see the ReplicaSet created by the Deployment, run:
```
`kubectl get rs
`
```
Notice that the name of the ReplicaSet is always formatted as
[DEPLOYMENT-NAME]-[RANDOM-STRING].
The random string is randomly generated and uses the pod-template-hash as a seed.
Two important columns of this output are:
* *DESIRED* displays the desired number of replicas of the application, which you
define when you create the Deployment. This is the desired state.
* *CURRENT* displays how many replicas are currently running.
Next, let’s scale the Deployment to 4 replicas. We’ll use the `kubectl scale` command,
followed by the Deployment type, name and desired number of instances:
```
`kubectl scale deployments/kubernetes-bootcamp --replicas=4
`
```
To list your Deployments once again, use `get deployments`:
```
`kubectl get deployments
`
```
The change was applied, and we have 4 instances of the application available. Next,
let’s check if the number of Pods changed:
```
`kubectl get pods -o wide
`
```
There are 4 Pods now, with different IP addresses. The change was registered in
the Deployment events log. To check that, use the `describe` subcommand:
```
`kubectl describe deployments/kubernetes-bootcamp
`
```
You can also view in the output of this command that there are 4 replicas now.