---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-basic-concepts.html
title: Basic concepts
word_count: 1404
filtered: true
elements_removed: 0
density_score: 0.88
---

Basic concepts - AWS Lambda
Basic concepts - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-basic-concepts)
[Durable execution](#durable-execution-concept)[DurableContext](#durable-context-concept)[Steps](#steps-concept)[Wait States](#wait-states-concept)[Invoking other functions](#invoke-concept)[Durable function configuration](#durable-configuration-basic)[See also](#durable-basic-concepts-see-also)
# Basic concepts
Lambda provides durable execution SDKs for JavaScript, TypeScript, and Python. These SDKs are the foundation for building durable functions, providing the primitives you need to checkpoint progress, handle retries, and manage execution flow. For complete SDK documentation and examples, see the [JavaScript/TypeScript SDK](https://github.com/aws/aws-durable-execution-sdk-js) and [Python SDK](https://github.com/aws/aws-durable-execution-sdk-python) on GitHub.
## Durable execution
A **durable execution** represents the complete lifecycle of a Lambda durable function, using a checkpoint and replay mechanism to track business logic progress, suspend execution, and recover from failures. When functions resume after suspension or interruptions, previously completed checkpoints are replayed and the function continues execution.
The lifecycle may include multiple invocations of a Lambda function to complete the execution, particularly after suspensions or failure recovery. This approach enables your function to run for extended periods (up to one year) while maintaining reliable progress despite interruptions.
###### How replay works
Lambda keeps a running log of all durable operations (steps, waits, and other operations) as your function executes. When your function needs to pause or encounters an interruption, Lambda saves this checkpoint log and stops the execution. When it's time to resume, Lambda invokes your function again from the beginning and replays the checkpoint log, substituting stored values for completed operations. This means your code runs again, but previously completed steps don't re-execute. Their stored results are used instead.
This replay mechanism is fundamental to understanding durable functions. Your code must be deterministic during replay, meaning it produces the same results given the same inputs. Avoid operations with side effects (like generating random numbers or getting the current time) outside of steps, as these can produce different values during replay and cause non-deterministic behavior.
## DurableContext
**DurableContext** is the context object your durable function receives. It provides methods for durable operations like steps and waits that create checkpoints and manage execution flow.
Your durable function receives a `DurableContext` instead of the default Lambda context:
TypeScript
```
`import {
DurableContext,
withDurableExecution,
} from "@aws/durable-execution-sdk-js";
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const result = await context.step(async () =&gt; {
return "step completed";
});
return result;
},
);
`
```
Python
```
`from aws\_durable\_execution\_sdk\_python import (
DurableContext,
durable\_execution,
durable\_step,
)
@durable\_step
def my\_step(step\_context, data):
# Your business logic
return result
@durable\_execution
def handler(event, context: DurableContext):
result = context.step(my\_step(event["data"]))
return result`
```
The Python SDK for durable functions uses synchronous methods and doesn't support `await`. The TypeScript SDK uses `async/await`.
## Steps
**Steps** runs business logic with built-in retries and automatic checkpointing. Each step saves its result, ensuring your function can resume from any completed step after interruptions.
TypeScript
```
`// Each step is automatically checkpointed
const order = await context.step(async () =&gt; processOrder(event));
const payment = await context.step(async () =&gt; processPayment(order));
const result = await context.step(async () =&gt; completeOrder(payment));`
```
Python
```
`# Each step is automatically checkpointed
order = context.step(lambda: process\_order(event))
payment = context.step(lambda: process\_payment(order))
result = context.step(lambda: complete\_order(payment))`
```
## Wait States
**Wait states** are planned pauses where your function stops running (and stops charging) until it's time to continue. Use them to wait for time periods, external callbacks, or specific conditions.
TypeScript
```
`// Wait for 1 hour without consuming resources
await context.wait({ seconds:3600 });
// Wait for external callback
const approval = await context.waitForCallback(
async (callbackId) =&gt; sendApprovalRequest(callbackId)
);`
```
Python
```
`# Wait for 1 hour without consuming resources
context.wait(3600)
# Wait for external callback
approval = context.wait\_for\_callback(
lambda callback\_id: send\_approval\_request(callback\_id)
)`
```
When your function encounters a wait or needs to pause, Lambda saves the checkpoint log and stops the execution. When it's time to resume, Lambda invokes your function again and replays the checkpoint log, substituting stored values for completed operations.
For more complex workflows, durable Lambda functions also come with advanced operations like `parallel()` for concurrent execution, `map()` for processing arrays, `runInChildContext()` for nested operations, and `waitForCondition()` for polling. See [Examples](./durable-examples.html) for detailed examples and guidance on when to use each operation.
## Invoking other functions
**Invoke** allows a durable function to call other Lambda functions and wait for their results. The calling function suspends while the invoked function executes, creating a checkpoint that preserves the result. This enables you to build modular workflows where specialized functions handle specific tasks.
Use `context.invoke()` to call other functions from within your durable function. The invocation is checkpointed, so if your function is interrupted after the invoked function completes, it resumes with the stored result without re-invoking the function.
TypeScript
```
`// Invoke another function and wait for result
const customerData = await context.invoke(
'validate-customer',
'arn:aws:lambda:us-east-1:123456789012:function:customer-service:1',
{ customerId: event.customerId }
);
// Use the result in subsequent steps
const order = await context.step(async () =&gt; {
return processOrder(customerData);
});`
```
Python
```
`# Invoke another function and wait for result
customer\_data = context.invoke(
'arn:aws:lambda:us-east-1:123456789012:function:customer-service:1',
{'customerId': event['customerId']},
name='validate-customer'
)
# Use the result in subsequent steps
order = context.step(
lambda: process\_order(customer\_data),
name='process-order'
)`
```
The invoked function can be either a durable or standard Lambda function. If you invoke a durable function, the calling function waits for the complete durable execution to finish. This pattern is common in microservices architectures where each function handles a specific domain, allowing you to compose complex workflows from specialized, reusable functions.
###### Note
Cross-account invocations are not supported. The invoked function must be in the same AWS account as the calling function.
## Durable function configuration
Durable functions have specific configuration settings that control execution behavior and data retention. These settings are separate from standard Lambda function configuration and apply to the entire durable execution lifecycle.
The **DurableConfig** object defines the configuration for durable functions:
```
`{
"ExecutionTimeout": Integer,
"RetentionPeriodInDays": Integer
}`
```
### Execution timeout
The **execution timeout** controls how long a durable execution can run from start to completion. This is different from the Lambda function timeout, which controls how long a single function invocation can run.
A durable execution can span multiple Lambda function invocations as it progresses through checkpoints, waits, and replays. The execution timeout applies to the total elapsed time of the durable execution, not to individual function invocations.
###### Understanding the difference
The Lambda function timeout (maximum 15 minutes) limits each individual invocation of your function. The durable execution timeout (maximum 1 year) limits the total time from when the execution starts until it completes, fails, or times out. During this period, your function may be invoked multiple times as it processes steps, waits, and recovers from failures.
For example, if you set a durable execution timeout of 24 hours and a Lambda function timeout of 5 minutes:
* Each function invocation must complete within 5 minutes
* The entire durable execution can run for up to 24 hours
* Your function can be invoked many times during those 24 hours
* Wait operations don't count against the Lambda function timeout but do count against the execution timeout
You can configure the execution timeout when creating a durable function using the Lambda console, AWS CLI, or AWS SAM. In the Lambda console, choose your function, then Configuration, Durable execution. Set the Execution timeout value in seconds (default: 86400 seconds / 24 hours, minimum: 60 seconds, maximum: 31536000 seconds / 1 year).
###### Note
The execution timeout and Lambda function timeout are different settings. The Lambda function timeout controls how long each individual invocation can run (maximum 15 minutes). The execution timeout controls the total elapsed time for the entire durable execution (maximum 1 year).
### Retention period
The **retention period** controls how long Lambda retains execution history and checkpoint data after a durable execution completes. This data includes step results, execution state, and the complete checkpoint log.
After the retention period expires, Lambda deletes the execution history and checkpoint data. You can no longer retrieve execution details or replay the execution. The retention period starts when the execution reaches a terminal state (SUCCEEDED, FAILED, STOPPED, or TIMED\_OUT).
You can configure the retention period when creating a durable function using the Lambda console, AWS CLI, or AWS SAM. In the Lambda console, choose your function, then Configuration, Durable execution. Set the Retention period value in days (default: 14 days, minimum: 1 day, maximum: 90 days).
Choose a retention period based on your compliance requirements, debugging needs, and cost considerations. Longer retention periods provide more time for debugging and auditing but increase storage costs.