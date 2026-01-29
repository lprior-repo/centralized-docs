---
url: https://docs.aws.amazon.com/lambda/latest/dg/tenant-isolation.html
title: Tenant isolation
word_count: 780
filtered: true
elements_removed: 0
density_score: 0.89
---

Tenant isolation - AWS Lambda
Tenant isolation - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#tenant-isolation)
[When to use tenant isolation mode](#tenant-isolation-use)[Supported features and limitations](#tenant-isolation-features)[Supported AWS Regions](#tenant-isolation-regions)[Considerations](#tenant-isolation-considerations)[Pricing](#tenant-isolation-pricing)[Isolation mode](#tenant-isolation-modes)
# Tenant isolation
Use tenant isolation mode when you need isolated request processing for individual end-users or tenants invoking a Lambda function. This capability simplifies building multi-tenant applications that process tenant-specific code or data, such as SaaS platforms for workflow automation or code execution, by removing the need to manage tenant-specific function resources and request routing logic.
Multi-tenant applications have strict isolation requirements when running code or processing data for individual tenants or end-users. With tenant isolation mode, Lambda uses a customer-specified tenant identifier to route requests to underlying function execution environments, ensuring that a function’s execution environments are only used to serve invocations from the specified end-user or tenant. Lambda’s function execution environments leverage [Firecracker virtualization](https://firecracker-microvm.github.io/) to provide workload isolation.
When a function using tenant isolation mode receives an invoke with a tenant identifier, Lambda first attempts to locate an available execution environment associated with that tenant identifier. If no execution environments exist, Lambda creates and assigns a new execution environment to that tenant. As function invocations with the specified tenant identifier scale up, Lambda locates or creates new execution environments as necessary.
###### Topics
* [When to use tenant isolation mode](#tenant-isolation-use)
* [Supported features and limitations](#tenant-isolation-features)
* [Supported AWS Regions](#tenant-isolation-regions)
* [Considerations](#tenant-isolation-considerations)
* [Pricing](#tenant-isolation-pricing)
* [Isolation mode](#tenant-isolation-modes)
* [Enabling tenant isolation for Lambda functions](./tenant-isolation-configure.html)
* [Invoking Lambda functions with tenant isolation](./tenant-isolation-invoke.html)
* [Accessing tenant identifier in Lambda function code](./tenant-isolation-context.html)
* [Monitoring Lambda functions with tenant isolation](./tenant-isolation-monitor.html)
* [Troubleshooting tenant isolation for Lambda functions](./tenant-isolation-troubleshooting.html)
## When to use tenant isolation mode
Use tenant isolation mode when you need to serve multiple end-users or tenants using a single Lambda function, while ensuring that the execution environments used to process invocations for individual tenants remain isolated from one another. This strict isolation of execution environments is required for multi-tenant applications that:
* **Execute end-user supplied code**: Maintaining separate execution environments for individual tenants can limit the impact of executing user-supplied code that may be incorrect or malicious.
* **Process tenant-specific data**: Maintaining separate execution environments for individual tenants can prevent exposure of sensitive tenant-specific data to other tenants.
Multiple invocation requests from the same tenant can re-use the same function execution environment, reducing cold-starts and improving response times for latency sensitive applications.
## Supported features and limitations
Tenant isolation mode is not supported with functions that use [function URLs](./urls-configuration.html), [provisioned concurrency](./provisioned-concurrency.html), or [SnapStart](./snapstart.html). You can send requests to a tenant-isolated function using [synchronous invocations](./invocation-sync.html), [asynchronous invocations](./invocation-async.html), or by using [Amazon API Gateway as an event-trigger](./services-apigateway.html).
## Supported AWS Regions
Tenant isolation mode is supported in all [commercial Regions](https://docs.aws.amazon.com/general/latest/gr/glos-chap.html#region) except Asia Pacific (New Zealand).
## Considerations
When using tenant isolation with your Lambda functions, keep the following in mind:
* **Immutable configuration**: Tenant isolation is an immutable function property. It can only be enabled when creating a function.
* **Required tenant-id parameter**: Functions using tenant isolation mode must be invoked with a `tenant-id` parameter. Omitting this parameter will cause function invocations to fail.
* **Execution role applies to all tenants**: Invocations from all tenants use the permissions defined in your Lambda function’s [execution role](./lambda-intro-execution-role.html).
* **Concurrency**: There are no changes to your function’s [concurrency](./lambda-concurrency.html) or [scaling behavior](./scaling-behavior.html) when using tenant isolation. Lambda imposes a limit of 2,500 tenant-isolated execution environments (active or idle) for every 1,000 [concurrent executions](./gettingstarted-limits.html#compute-and-storage) configured for your Lambda function.
## Pricing
You are charged when Lambda creates a new tenant-isolated execution environment. The price depends on the amount of [memory](./configuration-memory.html) that you allocate to your function and the [CPU architecture](./foundation-arch.html) that you use. For more information, see [AWS Lambda pricing](https://aws.amazon.com/lambda/pricing).
## Isolation mode
The following table outlines differences between Lambda functions with and without
tenant isolation.
|Feature|With tenant isolation|Without tenant isolation|
|Isolation type|Tenant-level isolation|Function-level isolation|
|Environment reuse|Execution environments are never reused across different
tenants|Execution environments might be reused across invocations of the
same function|
|Data isolation|Data from other tenants is not accessible|Data from previous invocations of the same function version may
be accessible|
|Cold starts|More cold starts due to tenant-specific environments|Fewer cold starts due to environment reuse|
|Pricing|Additional charge besides the standard Lambda pricing|Standard Lambda pricing|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Troubleshooting
Enabling tenant isolation
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.