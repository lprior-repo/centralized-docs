---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-best-practices.html
title: lambda managed instances best practices.html
word_count: 956
filtered: true
elements_removed: 0
density_score: 0.89
---

Best practices for Lambda Managed Instances - AWS Lambda
Best practices for Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-best-practices)
[Capacity provider configuration](#lambda-managed-instances-bp-capacity-provider)[Instance type selection](#lambda-managed-instances-bp-instance-types)[Function configuration](#lambda-managed-instances-bp-function-config)[Scaling configuration](#lambda-managed-instances-bp-scaling)[Security](#lambda-managed-instances-bp-security)[Cost optimization](#lambda-managed-instances-bp-cost)[Monitoring and observability](#lambda-managed-instances-bp-monitoring)[Runtime-specific considerations](#lambda-managed-instances-bp-runtime)[Next steps](#lambda-managed-instances-bp-next-steps)
## Capacity provider configuration
**Separate capacity providers by trust level.** Create different capacity providers for workloads with different security requirements. All functions assigned to the same capacity provider must be mutually trusted, as capacity providers serve as the security boundary.
**Use descriptive names.** Name capacity providers to clearly indicate their intended use and trust level (for example, `production-trusted`, `dev-sandbox`). This helps teams understand the purpose and security posture of each capacity provider.
**Use multiple Availability Zones.** Specify subnets across multiple Availability Zones when creating capacity providers. Lambda launches three instances by default for AZ resiliency, ensuring high availability for your functions.
## Instance type selection
**Let Lambda choose instance types.** By default, Lambda chooses the best instance types for your workload. We recommend letting Lambda Managed Instances choose instance types for you, as restricting the number of possible instance types may result in lower availability.
**Specify instance types for specific requirements.** If you have specific hardware requirements, set allowed instance types to a list of compatible instances. For example:
* For applications requiring high network bandwidth, select several n instance types
* For testing or development environments with cost constraints, choose smaller instance types like m7a.large
## Function configuration
**Choose appropriate memory and vCPU settings.** Select memory and vCPU configurations that support multi-concurrent executions of your function. The minimum supported function size is 2GB and 1 vCPU.
* For Python applications, choose a higher ratio of memory to vCPUs (such as 4 to 1 or 8 to 1) because of the way Python handles multi-concurrency
* For CPU-intensive operations or functions that perform little IO, choose more than one vCPU
* For IO-heavy applications like web services or batch jobs, multi-concurrency provides the most benefit
**Configure maximum concurrency appropriately.** Lambda chooses sensible defaults for maximum concurrency that balance resource consumption and throughput. Adjust this setting based on your function's resource usage:
* Increase maximum concurrency (up to 64 per vCPU) if your function invocations use very little CPU
* Decrease maximum concurrency if your application consumes a large amount of memory and very little CPU
Note that execution environments with very low concurrency may experience throttles and difficulty scaling.
## Scaling configuration
**Set appropriate target resource utilization.** By default, Lambda maintains enough headroom for your traffic to double within 5 minutes without throttles. Adjust this based on your workload characteristics:
* For very steady workloads or applications not sensitive to throttles, set the target to a high level to achieve higher utilization and lower costs
* For workloads with potential traffic bursts, set resource targets to a low level to maintain additional headroom
**Plan for traffic growth.** If your traffic more than doubles within 5 minutes, you may see throttles as Lambda scales up instances and execution environments. Design your application to handle potential throttling during rapid scale-up periods.
## Security
**Apply least privilege for PassCapacityProvider permissions.** Grant `lambda:PassCapacityProvider` permissions only for necessary capacity providers. Use resource-level permissions to restrict which capacity providers users can assign to functions.
**Monitor capacity provider usage.** Use AWS CloudTrail to monitor capacity provider assignments and access patterns. This helps identify unauthorized access attempts and ensures compliance with security policies.
**Separate untrusted workloads.** Do not rely on containers for security isolation between untrusted workloads. Use different capacity providers to separate workloads that are not mutually trusted.
## Cost optimization
**Leverage EC2 pricing options.** Take advantage of EC2 Savings Plans and Reserved Instances to reduce costs. These pricing options apply to the underlying EC2 compute (the 15% management fee is not discounted).
**Optimize for steady-state workloads.** Lambda Managed Instances are best suited for steady-state functions with predictable high-volume traffic. For bursty traffic patterns, Lambda (default) may be more cost-effective.
**Monitor resource utilization.** Track CloudWatch metrics to understand CPU and memory utilization. Adjust function memory allocation and instance type selection based on actual usage patterns to optimize costs.
## Monitoring and observability
**Monitor capacity provider metrics.** Track capacity provider level metrics including CPUUtilization, MemoryUtilization, vCPUAvailable, and MemoryAvailable to ensure sufficient resources are available for your workloads.
**Monitor execution environment metrics.** Track execution environment level metrics including ExecutionEnvironmentConcurrency and ExecutionEnvironmentConcurrencyLimit to understand scaling behavior and identify potential throttling.
**Set up CloudWatch alarms.** Create CloudWatch alarms for key metrics to proactively identify issues:
* High CPU or memory utilization
* Low available capacity
* Approaching concurrency limits
## Runtime-specific considerations
**Follow runtime-specific best practices.** Each runtime handles multi-concurrency differently. Review the runtime-specific guides for detailed recommendations:
* Java: Use thread-safe collections, `AtomicInteger`, and `ThreadLocal` for request-specific state
* Node.js: Use InvokeStore for all request-specific state and avoid global variables
* Python: Use unique file names in `/tmp` with request IDs and consider process-based memory isolation
**Test for thread safety and concurrency issues.** Before deploying to production, thoroughly test your functions for thread safety issues, race conditions, and proper state isolation under concurrent load.
## Next steps
* Learn about [capacity providers for Lambda Managed Instances](./lambda-managed-instances-capacity-providers.html)
* Understand [scaling for Lambda Managed Instances](./lambda-managed-instances-scaling.html)
* Review runtime-specific guides for [Java](./lambda-managed-instances-java-runtime.html), [Node.js](./lambda-managed-instances-nodejs-runtime.html), and [Python](./lambda-managed-instances-python-runtime.html)
* Configure [VPC connectivity for your capacity providers](./lambda-managed-instances-networking.html)
* Monitor Lambda Managed Instances with [CloudWatch metrics](./lambda-managed-instances-monitoring.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Quotas
Troubleshooting
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.