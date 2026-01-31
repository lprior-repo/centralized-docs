---
doc_id: ops/general/docs-integration
chunk_id: ops/general/docs-integration#1-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 537
summary: CUE’s Go API and the cue command read Protobuf definitions, enabling them. to be used to constrain and validate data directly and to be expressed in other
---


CUE’s Go API and the cue command read Protobuf definitions, enabling them
to be used to constrain and validate data directly and to be expressed in other
formats. CUE constraints can be extracted from Protobuf options, allowing
richer data validation than Protobuf’s type-based defaults.
Learn more

Learn more [/docs/concept/how-cue-works-with-protocol-buffers/]


JSON SCHEMA

CUE understands JSON Schema constraints through its Go API and the cue
command, enabling schemas to be used to constrain and validate data directly,
and to be expressed in other formats - including CUE.
Read more

Read more [/docs/concept/how-cue-works-with-json-schema/]


JAVA

Technology preview

Support for using CUE in Java is available through an experimental Java library.

Follow this introductory tutorial to get started with CUE in Java.

Learn more

Learn more [/docs/tutorial/get-started-cue-java/]

TECHNOLOGIES

CUE is independent of the technologies it can be used alongside.
Some examples of its use with specific tools, systems, and providers are collected in
CUE By Example [https://github.com/cue-labs/cue-by-example/]:


CONTROLLING KUBERNETES

A worked example of converting a set of Kubernetes configuration files
for a collection of microservices into smaller, validated CUE configurations by
automatically removing boilerplate;
automating commands that don’t know CUE yet (such as kubectl);
and extracting schema definitions from Kubernetes source code.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/tree/main/003_kubernetes_tutorial/README.md]


DRIVING GITHUB ACTIONS WORKFLOWS

A guide explaining how to convert GitHub Actions workflow files from YAML to
CUE, check those workflows are valid, and then use CUE’s tooling layer to
regenerate YAML - allowing safer and more predictable changes.
Read on CUE By Example

Read on CUE By Example [https://github.com/cue-labs/cue-by-example/blob/main/001_github_actions_importing_workflows/README.md]


WRITING TERRAFORM PLAN POLICIES

A pair of guides showing how to validate the JSON output from terraform plan using CUE as a policy language.
