---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-functions.html
title: Lambda durable functions
word_count: 935
filtered: true
elements_removed: 0
density_score: 0.88
---

Lambda durable functions - AWS Lambda
Lambda durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-functions)
[Key benefits](#durable-functions-benefits)[When to use durable functions](#durable-functions-use-cases)[How it works](#durable-functions-how-it-works)[Next steps](#durable-functions-next-steps)
# Lambda durable functions
Lambda durable functions enable you to build resilient multi-step applications and AI workflows that can execute for up to one year while maintaining reliable progress despite interruptions. When a durable function runs, this complete lifecycle is called a durable execution, which uses checkpoints to track progress and automatically recover from failures through replay, re-executing from the beginning while skipping completed work.
Within each function, you use durable operations as fundamental building blocks. Steps execute business logic with built-in retries and progress tracking, while waits suspend execution without incurring compute charges, making them ideal for long-running processes like human-in-the-loop workflows or polling external dependencies. Whether you're processing orders, coordinating microservices, or orchestrating agentic AI applications, durable functions maintain state automatically and recover from failures while you write code in familiar programming languages.
## Key benefits
**Write resilient code naturally:** With familiar programming constructs, you write code that handles failures automatically. Built-in checkpointing, transparent retries, and automatic recovery mean your business logic stays clean and focused.
**Pay only for what you use:** During wait operations, your function suspends without incurring compute charges. For long-running workflows that wait hours or days, you pay only for actual processing time, not idle waiting.
**Operational simplicity:** With Lambda's serverless model, you get automatic scaling, including scale-to-zero, without managing infrastructure. Durable functions handle state management, retry logic, and failure recovery automatically, reducing operational overhead.
## When to use durable functions
**Short-lived coordination:** Coordinate payments, inventory, and shipping across multiple services with automatic rollback on failures. Process orders through validation, payment authorization, inventory allocation, and fulfillment with guaranteed completion.
**Process payments with confidence:** Build resilient payment flows that maintain transaction state through failures and handle retries automatically. Coordinate multi-step authorization, fraud checks, and settlement across payment providers with full auditability across steps.
**Build reliable AI workflows:** Create multi-step AI workflows that chain model calls, incorporate human feedback, and handle long-running tasks deterministically during failures. Automatically resume after suspension, and only pay for active execution time.
**Orchestrate complex order fulfillment:** Coordinate order processing across inventory, payment, shipping, and notification systems with built-in resilience. Automatically handle partial failures, preserve order state despite interruptions, and efficiently wait for external events without consuming compute resources.
**Automate multi-step business workflows:** Build reliable workflows for employee onboarding, loan approvals, and compliance processes that span days or weeks. Maintain workflow state across human approvals, system integrations, and scheduled tasks while providing full visibility into process status and history.
### How durable functions compare to Step Functions
Both, durable functions and Step Functions, provide workflow orchestration with automatic state management. The key differences are where they run and how you define workflows:
* **Durable functions:** Run within Lambda, use standard programming languages, managed within Lambda environment
* **Step Functions:** Standalone service, graph-based DSL or visual designer, fully managed with zero maintenance
Durable functions are ideal for application development in Lambda where workflows are tightly coupled with business logic. Step Functions excels at workflow orchestration across AWS services where you need visual design, native integrations to 220+ services, and zero-maintenance infrastructure.
For a detailed comparison, see [Durable functions or Step Functions](./durable-step-functions.html).
## How it works
Under the hood, durable functions are regular Lambda functions using a checkpoint/replay mechanism to track progress and support long-running operations through user-defined suspension points,
commonly referred to as durable execution. When a durable function resumes from a wait point or interruption like retries, the system performs replay. During replay,
your code runs from the beginning but skips over completed checkpoints, using stored results instead of re-executing completed operations.
This replay mechanism ensures consistency while enabling long-running executions.
After your function resumes from a pause or interruption, the system performs replay. During replay, your code runs from the beginning but skips over completed checkpoints, using stored results instead of re-executing completed operations. This replay mechanism ensures consistency while enabling long-running executions.
To harness this checkpoint-and-replay mechanism in your applications, Lambda provides a durable execution SDK. The SDK abstracts away the complexity of managing checkpoints and replay, exposing simple primitives called durable operations that you use in your code. The SDK is available for JavaScript, TypeScript, and Python, integrating seamlessly with your existing Lambda development workflow.
With the SDK, you wrap your Lambda event handler, which then provides a DurableContext alongside your event. This context gives you access to durable operations like steps and waits. You write your function logic as normal sequential code, but instead of calling services directly, you wrap those calls in steps for automatic checkpointing and retries. When you need to pause execution, you add waits that suspend your function without incurring charges. The SDK handles all the complexity of state management and replay behind the scenes, so your code remains clean and readable.
![Filter for Amazon Inspector results related to Lambda functions](https://docs.aws.amazon.com/images/lambda/latest/dg/images/how_durable_works.png)
## Next steps
* [Get started with durable functions](./durable-getting-started.html)
* [Explore the durable execution SDK](./durable-execution-sdk.html)
* [Durable functions or Step Functions](./durable-step-functions.html)
* [Monitor and debug durable functions](./durable-monitoring.html)
* [Review security and permissions](./durable-security.html)
* [Follow best practices](./durable-best-practices.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Workflows and events
Basic concepts
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.