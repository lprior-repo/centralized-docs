---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#0-detailed
chunk_level: detailed
chunk_type: prose
heading: Kubernetes Deployments
token_count: 563
summary: ## Table of Contents    - [Objectives](#objectives)   - [Kubernetes Deployments](#kubernetes-deployments)       - [Note:](#note)   - [Deploying your first app on...
---

## Table of Contents

  - [Objectives](#objectives)
  - [Kubernetes Deployments](#kubernetes-deployments)
      - [Note:](#note)
  - [Deploying your first app on Kubernetes](#deploying-your-first-app-on-kubernetes)
    - [kubectl basics](#kubectl-basics)
    - [Deploy an app](#deploy-an-app)
    - [View the app](#view-the-app)
      - [Note:](#note)
  - [What's next](#whats-next)
  - [Feedback](#feedback)

---

## Objectives
* Learn about application Deployments.
* Deploy your first app on Kubernetes with kubectl.## Before you begin
The shell commands in this tutorial use POSIX shell syntax, which is supported by
the default shells on most Linux and macOS systems (for example, bash, zsh, or sh).
Windows users must use a POSIX-compatible shell such as
[Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)
or [Git Bash](https://gitforwindows.org/) to run the commands as written.
Commands that use `export`, `$()`, and similar constructs are **not** compatible
with PowerShell or the Windows Command Prompt.
## Kubernetes Deployments
*A Deployment is responsible for creating and updating instances of your application.*
#### Note:
This tutorial uses a container that requires the AMD64 architecture. If you are using
minikube on a computer with a different CPU architecture, you could try using minikube with
a driver that can emulate AMD64. For example, the Docker Desktop driver can do this.
Once you have a [running Kubernetes cluster](/docs/tutorials/kubernetes-basics/create-cluster/cluster-intro/),
you can deploy your containerized applications on top of it. To do so, you create a
Kubernetes **Deployment**. The Deployment instructs Kubernetes how to create and
update instances of your application. Once you've created a Deployment, the Kubernetes
control plane schedules the application instances included in that Deployment to run
on individual Nodes in the cluster.
Once the application instances are created, a Kubernetes Deployment controller continuously
monitors those instances. If the Node hosting an instance goes down or is deleted,
the Deployment controller replaces the instance with an instance on another Node
in the cluster. **This provides a self-healing mechanism to address machine failure
or maintenance.**
In a pre-orchestration world, installation scripts would often be used to start
applications, but they did not allow recovery from machine failure. By both creating
your application instances and keeping them running across Nodes, Kubernetes Deployments
provide a fundamentally different approach to application management.