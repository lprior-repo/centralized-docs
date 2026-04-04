---
doc_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you
chunk_id: ref/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you.md/docs-tasks-administer-cluster-migrating-from-dockershim-check-if-dockershim-removal-affects-you#2-standard
chunk_level: standard
chunk_type: prose
heading: Finding if your app has a dependencies on Docker
token_count: 199
summary: ## Finding if your app has a dependencies on Docker If you are using Docker for building your application containers, you can still run these containers on any container runtime. This use of Docker...
---

## Finding if your app has a dependencies on Docker
If you are using Docker for building your application containers, you can still
run these containers on any container runtime. This use of Docker does not count
as a dependency on Docker as a container runtime.
When alternative container runtime is used, executing Docker commands may either
not work or yield unexpected output. This is how you can find whether you have a
dependency on Docker:
1. Make sure no privileged Pods execute Docker commands (like `docker ps`),
restart the Docker service (commands such as `systemctl restart docker.service`),
or modify Docker-specific files such as `/etc/docker/daemon.json`.
2. Check for any private registries or image mirror settings in the Docker
configuration file (like `/etc/docker/daemon.json`). Those typically need to
be reconfigured for another container runtime.
3. Check that scripts and apps running on nodes outside of your Kubernetes
infrastructure do not execute Docker commands. It might be: