---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-execution-environment.html
title: Understanding the Lambda Managed Instances execution environment
word_count: 594
filtered: true
elements_removed: 0
density_score: 0.89
---

Understanding the Lambda Managed Instances execution environment - AWS Lambda
Understanding the Lambda Managed Instances execution environment - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-execution-environment)
[Execution environment lifecycle](#lambda-managed-instances-execution-lifecycle)[Error handling and recovery](#lambda-managed-instances-error-handling)
# Understanding the Lambda Managed Instances execution environment
Lambda Managed Instances provide an alternative deployment model that runs your function code on customer-owned Amazon EC2 instances while Lambda manages the operational aspects. The execution environment for Managed Instances has several important differences from Lambda (default) functions, particularly in how it handles concurrent invocations and manages container lifecycles.
**Note:** For information about the Lambda (default) execution environment, see Understanding the Lambda execution environment lifecycle.
## Execution environment lifecycle
The lifecycle of a Lambda Managed Instances function execution environment differs from Lambda (default) in several key ways:
### Init phase
During the Init phase, Lambda performs the following steps:
* Initialize and register all extensions
* Bootstrap the runtime entrypoint. Runtime spawns the configured number of runtime workers (implementation depends on runtime)
* Run function initialization code (code outside the handler)
* Wait for at least one runtime worker to signal readiness by calling `/runtime/invocation/next`
The Init phase is considered complete when extensions have initialized and at least one runtime worker has called `/runtime/invocation/next`. The function is then ready to process invocations.
###### Note
For Lambda Managed Instances functions, initialization can take up to 15 minutes. The timeout is the maximum of 130 seconds or the configured function timeout (up to 900 seconds).
### Invoke phase
The Invoke phase for Lambda Managed Instances functions has several unique characteristics:
**Continuous operation.** Unlike Lambda (default), the execution environment remains continuously active, processing invocations as they arrive without freezing between invocations.
**Parallel processing.** Multiple invocations can execute simultaneously within the same execution environment, each handled by a different runtime worker.
**Independent timeouts.** The function's configured timeout applies to each individual invocation. When an invocation times out, Lambda marks that specific invocation as failed but does not interrupt other running invocations or terminate the execution environment.
**Backpressure handling.** If all runtime workers are busy processing invocations, new invocation requests are rejected until a worker becomes available.
## Error handling and recovery
Error handling in Lambda Managed Instances function execution environments differs from Lambda (default):
**Invoke timeouts.** When an individual invocation times out, Lambda returns a timeout error for that invocation. However, Lambda Managed Instances does not enforce the timeout—your code will keep running. As a function developer, you are responsible for detecting and handling the timeout. The context object exposes the remaining time for the invocation, with a zero or negative value indicating a timeout. Other concurrent invocations in the execution environment continue processing normally.
**Runtime worker failures.** If a runtime worker process crashes, the execution environment continues operating with the remaining healthy workers.
**Extension crashes.** If an extension process crashes during initialization or operation, the entire execution environment is marked as unhealthy and is terminated. Lambda creates a new execution environment to replace it.
**No reset/repair.** Unlike Lambda (default), Managed Instances do not attempt to reset and reinitialize the execution environment after errors. Instead, unhealthy containers are terminated and replaced with new ones.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Operator role
Version publishing
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.