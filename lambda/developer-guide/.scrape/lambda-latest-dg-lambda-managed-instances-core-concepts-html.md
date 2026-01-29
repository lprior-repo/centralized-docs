---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-managed-instances-core-concepts.html
title: Core concepts
word_count: 341
filtered: true
elements_removed: 0
density_score: 0.89
---

Core concepts - AWS Lambda
Core concepts - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-managed-instances-core-concepts)
# Core concepts
Lambda Managed Instances introduces several core concepts that differ from traditional Lambda functions. Understanding these concepts is essential for effectively deploying and managing your functions on EC2 infrastructure.
**Capacity providers** form the foundation of Lambda Managed Instances. A capacity provider defines the compute infrastructure where your functions execute, including VPC configuration, instance requirements, and scaling policies. Capacity providers also serve as the security boundary for your functions, meaning all functions assigned to the same capacity provider must be mutually trusted.
**Scaling behavior** differs significantly from traditional Lambda functions. Instead of scaling on-demand when invocations arrive, Managed Instances scale asynchronously based on CPU resource utilization. This approach eliminates cold starts but requires planning for traffic growth. If your traffic more than doubles within 5 minutes, you may experience throttles as Lambda scales up capacity to meet demand.
**Security and permissions** require careful consideration. You need operator role permissions to allow Lambda to manage EC2 resources in your capacity providers. Additionally, users need the `lambda:PassCapacityProvider` permission to assign functions to capacity providers, acting as a security gate to control which functions can run on specific infrastructure.
**Multi-concurrent execution** is a key characteristic of Managed Instances. Each execution environment can handle multiple invocations simultaneously, maximizing resource utilization for IO-heavy applications. This differs from traditional Lambda where each environment processes one request at a time. This execution model requires attention to thread safety, state management, and context isolation depending on your runtime.
The following sections provide detailed information about each core concept.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Getting started
Capacity providers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.