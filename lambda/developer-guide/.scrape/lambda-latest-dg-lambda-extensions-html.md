---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-extensions.html
title: Augment Lambda functions using Lambda extensions
word_count: 1069
filtered: true
elements_removed: 0
density_score: 0.86
---

Augment Lambda functions using Lambda extensions - AWS Lambda
Augment Lambda functions using Lambda extensions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-extensions)
[Execution environment](#using-extensions-env)[Impact on performance and resources ](#using-extensions-reg)[Permissions](#using-extensions-permissions)
# Augment Lambda functions using Lambda extensions
You can use Lambda extensions to augment your Lambda functions. For example, use Lambda extensions to integrate
functions with your preferred monitoring, observability, security, and governance tools. You can choose from a broad
set of tools that [AWS Lambda Partners](https://aws.amazon.com/lambda/partners/) provides, or you can
[create your own Lambda extensions](./runtimes-extensions-api.html).
Lambda supports external and internal extensions. An external extension runs as an independent process in the
execution environment and continues to run after the function invocation is fully processed. Because extensions run
as separate processes, you can write them in a different language than the function. All [Lambda runtimes](./lambda-runtimes.html) support extensions.
An internal extension runs as part of the runtime process. Your function accesses internal extensions by using
wrapper scripts or in-process mechanisms such as `JAVA\_TOOL\_OPTIONS`. For more information, see [Modifying the runtime environment](./runtimes-modify.html).
You can add extensions to a function using the Lambda console, the AWS Command Line Interface (AWS CLI), or infrastructure as code
(IaC) services and tools such as CloudFormation, AWS Serverless Application Model (AWS SAM), and Terraform.
You are charged for the execution time that the extension consumes (in 1 ms increments). There is no cost to
install your own extensions. For more pricing information for extensions, see
[AWS Lambda Pricing](https://aws.amazon.com/lambda/pricing/). For pricing
information for partner extensions, see those partners' websites. See [AWS Lambda extensions partners](./extensions-api-partners.html)
for a list of official partner extensions.
For a tutorial on extensions and how to use them with your Lambda functions, see the
[AWS Lambda Extensions Workshop](https://catalog.workshops.aws/lambdaextensions/en-US).
###### Topics
* [Execution environment](#using-extensions-env)
* [Impact on performance and resources ](#using-extensions-reg)
* [Permissions](#using-extensions-permissions)
* [Configuring Lambda extensions](./extensions-configuration.html)
* [AWS Lambda extensions partners](./extensions-api-partners.html)
* [Using the Lambda Extensions API to create extensions](./runtimes-extensions-api.html)
* [Accessing real-time telemetry data for extensions using the Telemetry API](./telemetry-api.html)
## Execution environment
Lambda invokes your function in an [execution environment](./lambda-runtime-environment.html), which
provides a secure and isolated runtime environment. The execution environment manages the resources required to
run your function and provides lifecycle support for the function's runtime and extensions.
The lifecycle of the execution environment includes the following phases:
* `**Init**`: In this phase, Lambda creates or
unfreezes an execution environment with the configured resources, downloads the code for the function and
all layers, initializes any extensions, initializes the runtime, and then runs the function’s initialization
code (the code outside the main handler). The `Init` phase happens either during the first
invocation, or in advance of function invocations if you have enabled [provisioned concurrency](./provisioned-concurrency.html).
The `Init` phase is split into three sub-phases: `Extension init`,
`Runtime init`, and `Function init`. These sub-phases ensure that all extensions and the runtime complete their setup tasks before the function code runs.
When [Lambda SnapStart](./snapstart.html) is activated, the `Init` phase happens when you publish a function version. Lambda saves a snapshot of the memory and disk state of the initialized execution environment, persists the encrypted snapshot, and caches it for low-latency access. If you have a before-checkpoint [runtime hook](./snapstart-runtime-hooks.html), then the code runs at the end of `Init` phase.
* `**Restore**` (SnapStart only): When you first invoke a [SnapStart](./snapstart.html) function and as the function scales up, Lambda resumes new execution environments from the persisted snapshot instead of initializing the function from scratch. If you have an after-restore [runtime hook](./snapstart-runtime-hooks.html), the code runs at the end of the `Restore` phase. You are charged for the duration of after-restore runtime hooks. The runtime must load and after-restore runtime hooks must complete within the timeout limit (10 seconds). Otherwise, you'll get a SnapStartTimeoutException. When the `Restore` phase completes, Lambda invokes the function handler (the [Invoke phase](./lambda-runtime-environment.html#runtimes-lifecycle-invoke)).
* `**Invoke**`: In this phase, Lambda invokes the function handler.
After the function runs to completion, Lambda prepares to handle another function
invocation.
* `**Shutdown**`: This phase is triggered if the Lambda function does not
receive any invocations for a period of time. In the `Shutdown` phase, Lambda shuts down
the runtime, alerts the extensions to let them stop cleanly, and then removes the environment. Lambda
sends a `Shutdown` event to each extension, which tells the extension that the environment is about
to be shut down.
During the `Init` phase, Lambda extracts layers containing extensions into the `/opt`
directory in the execution environment. Lambda looks for extensions in the `/opt/extensions/` directory,
interprets each file as an executable bootstrap for launching the extension, and starts all extensions in
parallel.
## Impact on performance and resources
The size of your function's extensions counts towards the deployment package size limit. For a .zip file
archive, the total unzipped size of the function and all extensions cannot exceed the unzipped deployment package
size limit of 250 MB.
Extensions can impact the performance of your function because they share function resources such as CPU,
memory, and storage. For example, if an extension performs compute-intensive operations, you may see your
function's execution duration increase.
Each extension must complete its initialization before Lambda invokes the function. Therefore, an extension
that consumes significant initialization time can increase the latency of the function invocation.
To measure the extra time that the extension takes after the function execution, you can use the
`PostRuntimeExtensionsDuration`
[function metric](./monitoring-metrics.html). To measure the increase in memory used, you can use the
`MaxMemoryUsed` metric. To understand the impact of a specific extension, you can run different
versions of your functions side by side.
###### Note
`MaxMemoryUsed` metric is one of the [Metrics collected by Lambda Insights](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Lambda-Insights-metrics.html) and not a Lambda native metric.
## Permissions
Extensions have access to the same resources as functions. Because extensions are executed within the same
environment as the function, permissions are shared between the function and the extension.
For a .zip file archive, you can create an CloudFormation template to simplify the task of attaching the same extension
configuration—including AWS Identity and Access Management (IAM) permissions—to multiple functions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Layers with AWS SAM
Configuring extensions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.