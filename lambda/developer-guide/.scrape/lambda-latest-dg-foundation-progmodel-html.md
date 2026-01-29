---
url: https://docs.aws.amazon.com/lambda/latest/dg/foundation-progmodel.html
title: Understanding the Lambda programming model
word_count: 645
filtered: true
elements_removed: 0
density_score: 0.87
---

Understanding the Lambda programming model - AWS Lambda
Understanding the Lambda programming model - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#foundation-progmodel)
# Understanding the Lambda programming model
Lambda offers two programming models: standard functions that run up to 15 minutes, and Durable Functions that can run up to one year. While both share core concepts, Durable Functions add capabilities for long-running, stateful workflows.
Lambda provides a programming model that is common to all of the runtimes. The programming model defines the
interface between your code and the Lambda system. You tell Lambda the entry point to your function by defining a
*handler* in the function configuration. The runtime passes in objects to the handler that
contain the invocation *event* and the *context*, such as the function name
and request ID.
**For Durable Functions, the handler also receives a DurableContext object that provides:**
* Checkpointing capabilities through step()
* Wait state management through wait() and waitForCallback()
* Automatic state persistence between invocations
When the handler finishes processing the first event, the runtime sends it another. For Durable Functions, the handler can pause execution between steps, and Lambda will automatically save and restore state when the function resumes. The function's class stays
in memory, so clients and variables that are declared outside of the handler method in *initialization
code* can be reused. To save processing time on subsequent events, create reusable resources like
AWS SDK clients during initialization. Once initialized, each instance of your function can process thousands of
requests.
Your function also has access to local storage in the `/tmp` directory, a transient cache that can be used for multiple invocations. For more information,
see [Execution environment](./lambda-runtime-environment.html).
When [AWS X-Ray tracing](./services-xray.html) is enabled, the runtime records separate
subsegments for initialization and execution.
The runtime captures logging output from your function and sends it to Amazon CloudWatch Logs. In addition to logging your
function's output, the runtime also logs entries when function invocation starts and ends. This includes a report
log with the request ID, billed duration, initialization duration, and other details. If your function throws an
error, the runtime returns that error to the invoker.
###### Note
Logging is subject to [CloudWatch Logs quotas](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/cloudwatch_limits_cwl.html). Log data can be lost due
to throttling or, in some cases, when an instance of your function is stopped.
**Key differences for Durable Functions:**
* State is automatically persisted between steps
* Functions can pause execution without consuming resources
* Steps are automatically retried on failure
* Progress is tracked through checkpoints
Lambda scales your function by running additional instances of it as demand increases, and by stopping
instances as demand decreases. This model leads to variations in application architecture, such as:
* Unless noted otherwise, incoming requests might be processed out of order or concurrently.
* Do not rely on instances of your function being long lived, instead store your application's state elsewhere.
* Use local storage and class-level objects to increase performance, but keep to a minimum the size of your deployment package and the amount of data that you transfer onto the execution environment.
For a hands-on introduction to the programming model in your preferred programming language, see the following
chapters.
* [Building Lambda functions with Node.js](./lambda-nodejs.html)
* [Building Lambda functions with Python](./lambda-python.html)
* [Building Lambda functions with Ruby](./lambda-ruby.html)
* [Building Lambda functions with Java](./lambda-java.html)
* [Building Lambda functions with Go](./lambda-golang.html)
* [Building Lambda functions with C#](./lambda-csharp.html)
* [Building Lambda functions with PowerShell](./lambda-powershell.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Running code
Execution environment
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.