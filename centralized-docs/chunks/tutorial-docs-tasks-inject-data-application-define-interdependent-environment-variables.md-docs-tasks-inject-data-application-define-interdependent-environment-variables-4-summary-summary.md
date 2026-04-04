---
doc_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables
chunk_id: tutorial/docs-tasks-inject-data-application-define-interdependent-environment-variables.md/docs-tasks-inject-data-application-define-interdependent-environment-variables#4-summary
chunk_level: summary
chunk_type: prose
heading: Before you begin
token_count: 116
summary: * [KodeKloud](https://kodekloud.com/public-playgrounds)## Define an environment dependent variable for a container When you create a Pod, you can set dependent environment variables for the...
---

* [KodeKloud](https://kodekloud.com/public-playgrounds)## Define an environment dependent variable for a container
When you create a Pod, you can set dependent environment variables for the containers that run in the Pod. To set dependent environment variables, you can use $(VAR\_NAME) in the `value` of `env` in the configuration file.
In this exercise, you create a Pod that runs one container. The configuration
file for the Pod defines a dependent environment variable with common usage defined. Here is the configuration manifest for the
Pod: