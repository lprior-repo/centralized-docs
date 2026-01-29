---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-scaling.html
title: Scaling Lambda Managed Instances
word_count: 872
filtered: true
elements_removed: 0
density_score: 0.89
---

Scaling Lambda Managed Instances - AWS Lambda
Scaling Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-scaling)
[The scaling lifecycle](#lambda-managed-instances-scaling-lifecycle)[Adjusting scaling behavior](#lambda-managed-instances-adjusting-scaling)[Next steps](#lambda-managed-instances-scaling-next-steps)
# Scaling Lambda Managed Instances
Lambda Managed Instances does not scale when invocations arrive and does not support cold starts. Instead, it scales asynchronously using resource consumption signals. Managed Instances currently scales based on CPU resource utilization and multi-concurrency saturation.
**Key differences:**
* **Lambda (default):** Scales when there is no free execution environment to handle an incoming invocation (cold start)
* **Lambda Managed Instances:** Scales asynchronously based on CPU resource utilization and multi-concurrency saturation of execution environments
If your traffic more than doubles within 5 minutes, you may see throttles as Lambda scales up instances and execution environments to meet demand.
## The scaling lifecycle
Lambda Managed Instances uses a distributed architecture to manage scaling:
**Components:**
* **Managed Instances** - Run in your account in the subnets you provide
* **Router and Scaler** - Shared Lambda components that route invocations and manage scaling
* **Lambda Agent** - Runs on each Managed Instance to manage execution environment lifecycle and monitor resource consumption
**How it works:**
1. When you publish a function version with a capacity provider, Lambda launches Managed Instances in your account. It launches three by default for AZ resiliency and starts three execution environments before marking your function version ACTIVE.
2. Each Managed Instance can run execution environments for multiple functions mapped to the same capacity provider.
3. As traffic flows into your application, execution environments consume resources. The Lambda Agent notifies the Scaler, which decides whether to scale new execution environments or Managed Instances.
4. If Router attempts to send an invocation to an execution environment with high resource consumption, the Lambda Agent on that instance notifies it to retry on another.
5. As traffic decreases, the Lambda Agent notifies Scaler, which makes a decision to scale down execution environments and scale in Managed Instances.
## Adjusting scaling behavior
You can customize the scaling behavior of Managed Instances through four controls:
#### 1. Function memory and vCPUs
Choose the memory size and vCPU allocation for your function. The smallest supported function size is 2GB and 1vCPU.
**Considerations:**
* Pick a memory and vCPU setting that will support multi-concurrent executions of your function
* You cannot configure a function with less than 1 vCPU because functions running on Managed Instances should support multi-concurrent workloads
* You cannot choose less than 2GB because this matches the 2 to 1 memory to vCPU ratio of c instances, which have the lowest ratio
* For Python applications, you may need to choose a higher ratio of memory to vCPUs, such as 4 to 1 or 8 to 1, because of the way Python handles multi-concurrency
* If you are running CPU-intensive operations or perform little IO, you should choose more than one vCPU
#### 2. Maximum concurrency
Set the maximum concurrency per execution environment.
**Default behavior:** Lambda chooses sensible defaults that balance resource consumption and throughput that work for a wide variety of applications.
**Adjustment guidelines:**
* **Increase concurrency:** If your function invocations use very little CPU, you can increase maximum concurrency up to a maximum of 64 per vCPU
* **Decrease concurrency:** If your application consumes a large amount of memory and very little CPU, you can reduce your maximum concurrency
**Important:** Since Lambda Managed Instances are meant for multi-concurrent applications, execution environments with very low concurrency may experience throttles when scaling.
#### 3. Target resource utilization
Choose your own target for CPU utilization consumption.
**Default behavior:** Lambda maintains enough headroom for your traffic to double within 5 minutes without throttles.
**Optimization options:**
* If your workload is very steady or if your application is not sensitive to throttles, you may set the target to a high level to achieve higher utilization and lower costs
* If you want to maintain headroom for bursts of traffic, you can set resource targets to a low level, which will require more capacity
#### 4. Instance type selection
Set allowed or excluded instance types.
**Default behavior:** Lambda chooses the best instance types for your workload. We recommend letting Lambda Managed Instances choose instance types for you, as restricting the number of possible instance types may result in lower availability.
**Custom configuration:**
* **Specific hardware requirements:** Set allowed instance types to a list of compatible instances. For example, if you have an application that requires high network bandwidth, you can select several n instance types
* **Cost optimization:** For testing or development environments, you might choose smaller instance types, like m7a.large instance types
## Next steps
* Learn about [capacity providers for Lambda Managed Instances](./lambda-managed-instances-capacity-providers.html)
* Review runtime-specific guides for handling multi-concurrency
* Configure [VPC connectivity for your capacity providers](./lambda-managed-instances-networking.html)
* Monitor scaling metrics to optimize scaling behavior
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Capacity providers
Security
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.