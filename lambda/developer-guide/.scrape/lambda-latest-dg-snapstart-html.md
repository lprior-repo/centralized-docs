---
url: https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html
title: Improving startup performance with Lambda SnapStart
word_count: 1172
filtered: true
elements_removed: 0
density_score: 0.89
---

Improving startup performance with Lambda SnapStart - AWS Lambda
Improving startup performance with Lambda SnapStart - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#snapstart)
[Use cases](#snapstart-use-cases)[Supported features and limitations](#snapstart-runtimes)[Supported Regions](#snapstart-supported-regions)[Compatibility considerations](#snapstart-compatibility)[Pricing](#snapstart-pricing)
# Improving startup performance with Lambda SnapStart
Lambda SnapStart can provide as low as sub-second startup performance, typically with no changes to your function code. SnapStart makes it easier to build highly responsive and scalable applications without provisioning resources or implementing complex performance optimizations.
The largest contributor to startup latency (often referred to as cold start time) is the time that Lambda spends initializing the function, which includes loading the function's code, starting the runtime, and initializing the function code. With SnapStart, Lambda initializes your function when you publish a function version. Lambda takes a [Firecracker
microVM](https://aws.amazon.com/blogs/opensource/firecracker-open-source-secure-fast-microvm-serverless/) snapshot of the memory and disk state of the initialized [execution environment](./lambda-runtime-environment.html), encrypts the snapshot, and intelligently caches it to optimize retrieval latency.
To ensure resiliency, Lambda maintains several copies of each snapshot. Lambda automatically patches snapshots and their copies with the latest runtime and security updates. When you invoke the function version for the first time, and as the invocations scale up,
Lambda resumes new execution environments from the cached snapshot instead of initializing them from scratch, improving
startup latency.
###### Important
If your applications depend on uniqueness of state, you must evaluate your function code and verify that it is
resilient to snapshot operations. For more information, see [Handling uniqueness with Lambda SnapStart](./snapstart-uniqueness.html).
###### Topics
* [When to use SnapStart](#snapstart-use-cases)
* [Supported features and limitations](#snapstart-runtimes)
* [Supported Regions](#snapstart-supported-regions)
* [Compatibility considerations](#snapstart-compatibility)
* [SnapStart pricing](#snapstart-pricing)
* [Activating and managing Lambda SnapStart](./snapstart-activate.html)
* [Handling uniqueness with Lambda SnapStart](./snapstart-uniqueness.html)
* [Implement code before or after Lambda function snapshots](./snapstart-runtime-hooks.html)
* [Monitoring for Lambda SnapStart](./snapstart-monitoring.html)
* [Security model for Lambda SnapStart](./snapstart-security.html)
* [Maximize Lambda SnapStart performance](./snapstart-best-practices.html)
* [Troubleshooting SnapStart errors for Lambda functions](./snapstart-troubleshooting.html)
## When to use SnapStart
Lambda SnapStart is designed to address the latency variability introduced by one-time initialization code, such as loading module dependencies or frameworks. These operations can sometimes take several seconds to complete during the initial invocation. Use SnapStart to reduce this latency from several seconds to as low as sub-second, in optimal scenarios. SnapStart works best when used with function invocations at scale. Functions that are invoked infrequently might not experience the same performance improvements.
SnapStart is particularly beneficial for two main types of applications:
* **Latency-sensitive APIs and user flows:** Functions that are part of critical API endpoints or user-facing flows can benefit from SnapStart's reduced latency and improved response times.
* **Latency-sensitive data processing workflows:** Time-bound data processing workflows that use Lambda functions can achieve better throughput by reducing outlier function initialization latency.
[Provisioned concurrency](./provisioned-concurrency.html) keeps functions initialized and ready to respond in double-digit milliseconds. Use provisioned concurrency if your application has strict cold start latency requirements that can't be adequately addressed by SnapStart.
## Supported features and limitations
SnapStart is available for the following [Lambda managed runtimes](./lambda-runtimes.html):
* Java 11 and later
* Python 3.12 and later
* .NET 8 and later. If you're using the [Lambda Annotations framework for .NET](./csharp-handler.html#csharp-handler-annotations), upgrade to [Amazon.Lambda.Annotations](https://www.nuget.org/packages/Amazon.Lambda.Annotations) version 1.6.0 or later to ensure compatibility with SnapStart.
Other managed runtimes (such as `nodejs24.x` and `ruby3.4`), [OS-only runtimes](./runtimes-provided.html), and [container images](./images-create.html) are not supported.
SnapStart does not support [provisioned concurrency](./provisioned-concurrency.html), [Amazon Elastic File System (Amazon EFS)](https://docs.aws.amazon.com/efs/latest/ug/accessing-fs.html), or ephemeral
storage greater than 512 MB.
###### Note
You can use SnapStart only on [published function
versions](./configuration-versions.html#configuration-versions-config) and [aliases](./configuration-aliases.html) that point to versions. You can't use
SnapStart on a function's unpublished version ($LATEST).
## Supported Regions
Lambda SnapStart is available in all [commercial Regions](https://docs.aws.amazon.com/general/latest/gr/glos-chap.html#region) except Asia Pacific (New Zealand) and Asia Pacific (Taipei).
## Compatibility considerations
With SnapStart, Lambda uses a single snapshot as the initial state for multiple execution environments. If
your function uses any of the following during the [initialization
phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib), then you might need to make some changes before using SnapStart:
**Uniqueness**
If your initialization code generates unique content that is included in the snapshot, then the content might not be unique when it is reused across execution environments. To maintain uniqueness when using SnapStart, you must generate unique content after initialization. This includes unique IDs, unique secrets, and entropy that's used to generate pseudorandomness. To learn how to restore uniqueness, see [Handling uniqueness with Lambda SnapStart](./snapstart-uniqueness.html).
**Network connections**
The state of connections that your function establishes during the initialization phase isn't guaranteed when Lambda resumes your function from a snapshot. Validate the state of your network connections and re-establish them as necessary.
In most cases, network connections that an AWS SDK establishes automatically resume. For other connections, review the [best
practices](./snapstart-best-practices.html).
**Temporary data**
Some functions download or initialize ephemeral data, such as temporary credentials or cached
timestamps, during the initialization phase. Refresh ephemeral data in the function handler before using it, even when not using SnapStart.
###### Note
For Java managed runtimes, there's no additional cost for SnapStart. You're charged based on the number of requests for your functions,
the time that it takes your code to run, and the memory configured for your function.
The cost of using SnapStart includes the following:
* **Caching:** For every function version that you publish with SnapStart enabled, you pay for the cost of caching and maintaining the snapshot. The price depends on the amount of [memory](./configuration-memory.html) that you allocate to your function. You're charged for a minimum of 3 hours. You will continue to be charged as long as your function remains [active](./snapstart-activate.html#snapstart-active). Use the [ListVersionsByFunction](https://docs.aws.amazon.com/lambda/latest/api/API_ListVersionsByFunction.html) API action to identify function versions, and then use [DeleteFunction](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteFunction.html) to delete unused versions. To automatically delete unused function versions, see the [Lambda Version Cleanup](https://serverlessland.com/workflows/step-functions-lambda-version-cleanup) pattern on Serverless Land.
* **Restoration:** Each time a function instance is restored from a snapshot, you pay a restoration charge. The price depends on the amount of memory you allocate to your function.
As with all Lambda functions, duration charges apply to code that runs in the function handler. For SnapStart functions, duration charges also apply to initialization code that's declared outside of the handler, the time it takes for the runtime to load, and any code that runs in a [runtime hook](./snapstart-runtime-hooks.html). Duration is calculated from the time that your code begins running until it returns or otherwise ends, rounded up to the nearest 1 ms. Lambda maintains cached copies of your snapshot for resiliency and automatically applies software updates, such as runtime upgrades and security patches to them. Charges apply each time that Lambda re-runs your initialization code to apply software updates.
For more information about the cost of using SnapStart, see [AWS Lambda Pricing](https://aws.amazon.com/lambda/pricing/#SnapStart_Pricing).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Testing serverless functions
Activating SnapStart
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.