---
doc_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro
chunk_id: tutorial/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro.md/docs-tutorials-kubernetes-basics-deploy-app-deploy-intro#16-summary
chunk_level: summary
chunk_type: prose
heading: Deploying your first app on Kubernetes
token_count: 121
summary: The common format of a kubectl command is: `kubectl action resource`. This performs the specified *action* (like `create`, `describe` or `delete`) on the specified *resource* (like `node` or...
---

The common format of a kubectl command is: `kubectl action resource`.
This performs the specified *action* (like `create`, `describe` or `delete`) on the
specified *resource* (like `node` or `deployment`. You can use `--help` after the
subcommand to get additional info about possible parameters (for example: `kubectl get nodes --help`).
Check that kubectl is configured to talk to your cluster, by running the `kubectl version` command.
Check that kubectl is installed and that you can see both the client and the server versions.