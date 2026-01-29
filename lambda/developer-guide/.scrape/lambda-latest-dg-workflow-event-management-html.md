---
url: https://docs.aws.amazon.com/lambda/latest/dg/workflow-event-management.html
title: Managing Lambda workflows and events
word_count: 925
filtered: true
elements_removed: 0
density_score: 0.88
---

Managing Lambda workflows and events - AWS Lambda
Managing Lambda workflows and events - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#workflow-event-management)
[Code-first orchestration with durable functions](#durable-functions-orchestration)[Orchestrating workflows with Step Functions](#orchestrating-workflows)[Managing events with EventBridge and EventBridge Scheduler](#managing-events)
# Managing Lambda workflows and events
When building serverless applications with Lambda, you often need ways to orchestrate function execution and handle events. AWS provides several approaches for coordinating Lambda functions:
* [Lambda durable functions](./durable-functions.html) for code-first workflow orchestration within Lambda
* AWS Step Functions for visual workflow orchestration across multiple services
* Amazon EventBridge Scheduler and Amazon EventBridge for event-driven architectures and scheduling
You can also integrate these approaches together. For example, you might use EventBridge Scheduler to trigger durable functions or Step Functions workflows when specific events occur,
or configure workflows to publish events to EventBridge Scheduler at defined execution points. The following topics in this section provide more information on how you can use these orchestration options.
## Code-first orchestration with durable functions
Lambda durable functions provide a code-first approach to workflow orchestration, allowing you to build stateful, long-running workflows directly within your Lambda functions.
Unlike external orchestration services, durable functions keep your workflow logic in code, making it easier to version, test, and maintain alongside your business logic.
Durable functions are ideal when you need:
* **Use standard programming languages:** Define workflows using familiar programming languages like JavaScript and Python
* **Long-running processes:** Execute workflows that can run for up to one year, far beyond the 15-minute limit of standard Lambda functions
* **Simplified development:** Keep workflow logic and business logic in the same codebase for easier maintenance and testing
* **Cost-effective waiting:** Pause execution during wait states without consuming compute resources
* **Built-in state management:** Automatic checkpointing and state persistence without external storage configuration
For help choosing between durable functions and Step Functions, see [Durable functions or Step Functions](./durable-step-functions.html).
For more information on durable functions, see [Lambda durable functions](./durable-functions.html).
## Orchestrating workflows with Step Functions
AWS Step Functions is a workflow orchestration service that helps you coordinate multiple Lambda functions and other AWS services into structured workflows.
These workflows can maintain state, handle errors with sophisticated retry mechanisms, and process data at scale.
Step Functions offers two types of workflows to meet different orchestration needs:
**Standard workflows**
Ideal for long-running, auditable workflows that require exactly-once execution semantics. Standard workflows can run for up to one year, provide detailed execution history, and support visual debugging.
They are suitable for processes like order fulfillment, data processing pipelines, or multi-step analytics jobs.
**Express workflows**
Designed for high-event-rate, short-duration workloads with at-least-once execution semantics. Express workflows can run for up to five minutes and are ideal for high-volume event processing,
streaming data transformations, or IoT data ingestion scenarios. They offer higher throughput and potentially lower cost compared to Standard workflows.
###### Note
For more information on Step Functions workflow types, see [Choosing workflow type in Step Functions](https://docs.aws.amazon.com/step-functions/latest/dg/choosing-workflow-type.html).
Within these workflows, Step Functions provides two types of Map states for parallel processing:
**Inline Map**
Processes items from a JSON array within the execution history of the parent workflow. Inline Map supports up to 40 concurrent iterations and is suitable for smaller datasets or when you need to
keep all processing within a single execution. For more information, see [Using Map state in Inline mode](https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-map-state.html).
**Distributed Map**
Enables processing of large-scale parallel workloads by iterating over datasets that exceed 256 KiB or require more than 40 concurrent iterations. With support for up to 10,000 parallel child workflow executions,
Distributed Map excels at processing semi-structured data stored in Amazon S3, such as JSON or CSV files, making it ideal for batch processing and ETL operations. For more information,
see [Using Map state in Distributed mode](https://docs.aws.amazon.com/step-functions/latest/dg/state-map-distributed.html).
By combining these workflow types and Map states, Step Functions provides a flexible and powerful toolset for orchestrating complex serverless applications, from small-scale operations to large-scale data processing pipelines.
To get started with using Lambda with Step Functions, see [Orchestrating Lambda functions with Step Functions](./with-step-functions.html).
## Managing events with EventBridge and EventBridge Scheduler
Amazon EventBridge is an event bus service that helps you build event-driven architectures. It routes events between AWS services, integrated applications, and software as a service (SaaS) applications.
EventBridge Scheduler is a serverless scheduler that enables you to create, run, and manage tasks from one central service, allowing you to invoke Lambda functions on a schedule using cron and rate expressions, or configure one-time invocations.
Amazon EventBridge and EventBridge Scheduler help you build event-driven architectures with Lambda.
EventBridge routes events between AWS services, integrated applications, and SaaS applications, while EventBridge Scheduler provides specific scheduling capabilities for invoking Lambda functions on a recurring or one-time basis.
These services provide several key capabilities for working with Lambda functions:
* Create rules that match and route events to Lambda functions using EventBridge
* Set up recurring function invocations using cron and rate expressions with EventBridge Scheduler
* Configure one-time function invocations at specific dates and times
* Define flexible time windows and retry policies for scheduled invocations
For more information, see [Invoke a Lambda function on a schedule](./with-eventbridge-scheduler.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Powertools for AWS Lambda
Lambda durable functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.