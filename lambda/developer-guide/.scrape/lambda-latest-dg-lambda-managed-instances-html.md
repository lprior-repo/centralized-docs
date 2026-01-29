---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances.html
title: Lambda Managed Instances
word_count: 1109
filtered: true
elements_removed: 0
density_score: 0.89
---

Lambda Managed Instances - AWS Lambda
Lambda Managed Instances - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances)
[Key capabilities](#lambda-managed-instances-key-capabilities)[When to use Lambda Managed Instances](#lambda-managed-instances-when-to-use)[How it works](#lambda-managed-instances-how-it-works)[Concurrency model](#lambda-managed-instances-concurrency-model)[Tenancy and isolation](#lambda-managed-instances-tenancy-isolation)[Pricing](#lambda-managed-instances-pricing)[How Lambda Managed Instances differs from the Lambda (default) compute type](#lambda-managed-instances-comparison)[Next steps](#lambda-managed-instances-next-steps)
# Lambda Managed Instances
Lambda Managed Instances enables you to run Lambda functions on your current-generation Amazon EC2 instances, including Graviton4, network-optimized instances, and other specialized compute options, without managing instance lifecycles, operating system and language runtime patching, routing, load balancing, or scaling policies. With Lambda Managed Instances, you benefit from EC2 pricing advantages, including EC2 Savings Plans and Reserved Instances.
For a list of supported instance types, go to the [AWS Lambda Pricing](<https://aws.amazon.com/lambda/pricing/#:~:text=EPU pricing applies.-,Management Fees,-Pricing Example: High>) page and select your AWS Region.
## Key capabilities
Lambda Managed Instances provides the following capabilities:
* **Choose suitable instances** - Select [appropriate instances](<https://aws.amazon.com/lambda/pricing/#:~:text=EPU pricing applies.-,Management Fees,-Pricing Example: High>) based on performance and cost requirements, including access to the latest CPUs like Graviton4, configurable memory-CPU ratios, and high-bandwidth networking.
* **Automatic provisioning** - AWS automatically provisions suitable instances and spins up function execution environments.
* **Dynamic scaling** - Instances scale dynamically based on your function traffic patterns.
* **Fully managed experience** - AWS handles infrastructure management, scaling, patching, and routing, with the same extensive event-source integrations you're familiar with.
## When to use Lambda Managed Instances
Consider Lambda Managed Instances for the following use cases:
* **High volume-predictable workloads** - Best suited for steady-state workloads without unexpected traffic spikes. Lambda Managed Instances scale to handle traffic doubling within five minutes by default.
* **Performance-critical applications** - Access to latest CPUs, varying memory-CPU ratios, and high network throughput
* **Regulatory requirements** - Granular governance needs with control over VPC and instance placement
* **Variety of applications** - Event-driven applications, media/data processing, web applications, and legacy workloads migrating to serverless
## How it works
Lambda Managed Instances uses capacity providers as the foundation for running your functions:
1. **Create a capacity provider** - Define where your functions run by specifying VPC configuration and optionally, instance requirements, and scaling configuration
2. **Create your function** - Create Lambda functions as usual and attach them to a capacity provider
3. **Publish a function version** - Function versions become active on capacity provider instances once published
When you publish a function version with a capacity provider, Lambda launches Managed Instances in your account. It launches three instances by default for AZ resiliency and starts three execution environments before marking your function version ACTIVE. If you attach a function to an existing capacity provider that is already running other functions, Lambda may not spin up new instances if the available instances already have capacity to accommodate the new function's execution environments.
## Concurrency model
Lambda Managed Instances support multi-concurrent invocations, where one execution environment can handle multiple invocations at the same time. This differs from the Lambda (default) compute type, which provides a single concurrency model where one execution environment can run a maximum of one invoke at a time. Multi-concurrency yields better utilization of your underlying EC2 instances and is especially beneficial for IO-heavy applications like web services or batch jobs. This change in execution model means that thread safety, state management, and context isolation must be handled differently depending on the runtime.
## Tenancy and isolation
Lambda (default) compute type is multi-tenant, making use of Firecracker microVM technology to provide isolation between execution environments running on shared Lambda fleets. Lambda Managed Instances run in your account, providing the latest EC2 hardware and pricing options. Managed Instances use containers running on EC2 Nitro instances to provide isolation rather than Firecracker. Capacity providers serve as the security boundary for Lambda functions. Functions execute in containers within instances.
### Understanding managed instances
Lambda Managed Instances functions run on EC2 managed instances in your account. These instances are fully managed by Lambda, which means you have restricted permissions on them compared to standard EC2 instances. You can identify Lambda Managed Instances in your account by:
* The presence of the `Operator` field in EC2 `DescribeInstances` output
* The `aws:lambda:capacity-provider` tag on the instance
You cannot perform standard EC2 operations directly on these instances, such as terminating them manually. To destroy managed instances, delete the associated capacity provider. Lambda will then terminate the instances as part of the capacity provider deletion process.
## Pricing
Lambda Managed Instances uses EC2-based pricing with a 15% management fee on top of the EC2 instance cost. This pricing model supports EC2 Savings Plans, Reserved Instances and any other pricing discounts applied to your EC2 usage. Refer to pricing page for additional details: [https://aws.amazon.com/lambda/pricing/](https://aws.amazon.com/lambda/pricing/)
**Important:** EC2 pricing discounts only apply to the underlying EC2 compute, not to the management fee.
## How Lambda Managed Instances differs from the Lambda (default) compute type
Lambda Managed Instances changes how Lambda processes requests compared to Lambda (default).
**Key differences:**
||Lambda (default)|Lambda Managed Instances|
|Concurrency model|Single concurrency model where one execution environment can support a maximum of one invocation at a time|Multi-concurrent invocations where one execution environment can handle multiple invocations simultaneously, increasing throughput especially for IO-heavy applications|
|Tenancy and isolation|Multi-tenant, using Firecracker microVM technology to provide isolation between execution environments running on shared Lambda fleets|Run in your account, using EC2 Nitro to provide isolation. Capacity providers serve as the security boundary, with functions executing in containers within instances|
|Pricing model|Per-request duration pricing|Instance-based pricing with EC2 pricing models, including On-Demand and Reserved Instances, and savings options such as Compute Savings Plans|
|Scaling behavior|Scales when there is no free execution environment to handle an incoming invocation (cold start). Scales to zero without traffic|Scales asynchronously based on CPU resource utilization only, without cold starts. Scales to minimum execution environments configured without traffic|
|Best suited for|Functions with bursty traffic that can handle some cold-start time, or applications without sustained load that benefit from scale to zero|High volume predictable traffic functions when you want the flexibility, pricing plans, and hardware options of EC2|
## Next steps
* Learn about [capacity providers for Lambda Managed Instances](./lambda-managed-instances-capacity-providers.html)
* Understand [scaling for Lambda Managed Instances](./lambda-managed-instances-scaling.html)
* Review runtime-specific guides for [Java](./lambda-managed-instances-java-runtime.html), [Node.js](./lambda-managed-instances-nodejs-runtime.html), and [Python](./lambda-managed-instances-python-runtime.html)
* Configure [VPC connectivity for your capacity providers](./lambda-managed-instances-networking.html)
* Understand [security and permissions for Lambda Managed Instances](./lambda-managed-instances-security.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Best practices
Getting started
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.