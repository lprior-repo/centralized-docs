---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-execution-sdk.html
title: Durable execution SDK
word_count: 2381
filtered: true
elements_removed: 0
density_score: 0.88
---

Durable execution SDK - AWS Lambda
Durable execution SDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-execution-sdk)
[DurableContext](#durable-sdk-context)[What the SDK does](#durable-sdk-what-it-does)[How checkpointing works](#durable-sdk-how-checkpointing-works)[Replay behavior](#durable-sdk-replay-behavior)[Available durable operations](#durable-sdk-operations)[How durable operations are metered](#durable-operations-checkpoint-consumption)
# Durable execution SDK
The durable execution SDK is the foundation for building durable functions. It provides the primitives you need to checkpoint progress, handle retries, and manage execution flow. The SDK abstracts the complexity of checkpoint management and replay, letting you write sequential code that automatically becomes fault-tolerant.
The SDK is available for JavaScript, TypeScript, and Python. For complete API documentation and examples, see the [JavaScript/TypeScript SDK](https://github.com/aws/aws-durable-execution-sdk-js) and [Python SDK](https://github.com/aws/aws-durable-execution-sdk-python) on GitHub.
## DurableContext
The SDK provides your function with a `DurableContext` object that exposes all durable operations. This context replaces the standard Lambda context and provides methods for creating checkpoints, managing execution flow, and coordinating with external systems.
To use the SDK, wrap your Lambda handler with the durable execution wrapper:
TypeScript
```
`
import { withDurableExecution, DurableContext } from '@aws/durable-execution-sdk-js';
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
// Your function receives DurableContext instead of Lambda context
// Use context.step(), context.wait(), etc.
return result;
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import durable\_execution, DurableContext
@durable\_execution
def handler(event: dict, context: DurableContext):
# Use context.step(), context.wait(), etc.
return result
`
```
The wrapper intercepts your function invocation, loads any existing checkpoint log, and provides the `DurableContext` that manages replay and checkpointing.
## What the SDK does
The SDK handles three critical responsibilities that enable durable execution:
**Checkpoint management:** The SDK automatically creates checkpoints as your function executes durable operations. Each checkpoint records the operation type, inputs, and results. When your function completes a step, the SDK persists the checkpoint before continuing. This ensures your function can resume from any completed operation if interrupted.
**Replay coordination:** When your function resumes after a pause or interruption, the SDK performs replay. It runs your code from the beginning but skips completed operations, using stored checkpoint results instead of re-executing them. The SDK ensures replay is deterministic—given the same inputs and checkpoint log, your function produces the same results.
**State isolation:** The SDK maintains execution state separately from your business logic. Each durable execution has its own checkpoint log that other executions cannot access. The SDK encrypts checkpoint data at rest and ensures state remains consistent across replays.
## How checkpointing works
When you call a durable operation, the SDK follows this sequence:
1. **Check for existing checkpoint:** The SDK checks if this operation already completed in a previous invocation. If a checkpoint exists, the SDK returns the stored result without re-executing the operation.
2. **Execute the operation:** If no checkpoint exists, the SDK executes your operation code. For steps, this means calling your function. For waits, this means scheduling resumption.
3. **Create checkpoint:** After the operation completes, the SDK serializes the result and creates a checkpoint. The checkpoint includes the operation type, name, inputs, result, and timestamp.
4. **Persist checkpoint:** The SDK calls the Lambda checkpoint API to persist the checkpoint. This ensures the checkpoint is durable before continuing execution.
5. **Return result:** The SDK returns the operation result to your code, which continues to the next operation.
This sequence ensures that once an operation completes, its result is safely stored. If your function is interrupted at any point, the SDK can replay up to the last completed checkpoint.
## Replay behavior
When your function resumes after a pause or interruption, the SDK performs replay:
1. **Load checkpoint log:** The SDK retrieves the checkpoint log for this execution from Lambda.
2. **Run from beginning:** The SDK invokes your handler function from the start, not from where it paused.
3. **Skip completed durable operations:** As your code calls durable operations, the SDK checks each against the checkpoint log. For completed durable operations, the SDK returns the stored result without executing the operation code.
###### Note
If a child context's result was larger than the maximum checkpoint size (256 KB), the context's code is executed again during replay. This allows you to construct large results from the durable operations that ran inside the context, which will be looked up from the checkpoint log. Therefore it is imperative to only run deterministic code in the context itself. When using child contexts with large results, it is a best practice to perform long-running or non-deterministic work inside of steps and only perform short-running tasks which combine the results in the context itself.
4. **Resume at interruption point:** When the SDK reaches an operation without a checkpoint, it executes normally and creates new checkpoints as durable operations complete.
This replay mechanism requires your code to be deterministic. Given the same inputs and checkpoint log, your function must make the same sequence of durable operation calls. The SDK enforces this by validating that operation names and types match the checkpoint log during replay.
## Available durable operations
The `DurableContext` provides operations for different coordination patterns. Each durable operation creates checkpoints automatically, ensuring your function can resume from any point.
### Steps
Executes business logic with automatic checkpointing and retry. Use steps for operations that call external services, perform calculations, or execute any logic that should be checkpointed. The SDK creates a checkpoint before and after the step, storing the result for replay.
TypeScript
```
`
const result = await context.step('process-payment', async () =&gt; {
return await paymentService.charge(amount);
});
`
```
Python
```
`
result = context.step(
lambda \_: payment\_service.charge(amount),
name='process-payment'
)
`
```
Steps support configurable retry strategies, execution semantics (at-most-once or at-least-once), and custom serialization.
### Waits
Pauses execution for a specified duration without consuming compute resources. The SDK creates a checkpoint, terminates the function invocation, and schedules resumption. When the wait completes, Lambda invokes your function again and the SDK replays to the wait point before continuing.
TypeScript
```
`
// Wait 1 hour without charges
await context.wait({ seconds: 3600 });
`
```
Python
```
`
### Callbacks
Callbacks enable your function to pause and wait for external systems to provide input. When you create a callback, the SDK generates a unique callback ID and creates a checkpoint. Your function then suspends (terminates the invocation) without incurring compute charges. External systems submit callback results using the `SendDurableExecutionCallbackSuccess` or `SendDurableExecutionCallbackFailure` Lambda APIs. When a callback is submitted, Lambda invokes your function again, the SDK replays to the callback point, and your function continues with the callback result.
The SDK provides two methods for working with callbacks:
**createCallback:** Creates a callback and returns both a promise and a callback ID. You send the callback ID to an external system, which submits the result using the Lambda API.
TypeScript
```
`
const [promise, callbackId] = await context.createCallback('approval', {
timeout: { hours: 24 }
});
await sendApprovalRequest(callbackId, requestData);
const approval = await promise;
`
```
Python
```
`
callback = context.create\_callback(
name='approval',
config=CallbackConfig(timeout\_seconds=86400)
)
context.step(
lambda \_: send\_approval\_request(callback.callback\_id),
name='send\_request'
)
approval = callback.result()
`
```
**waitForCallback:** Simplifies callback handling by combining callback creation and submission in one operation. The SDK creates the callback, executes your submitter function with the callback ID, and waits for the result.
TypeScript
```
`
const result = await context.waitForCallback(
'external-api',
async (callbackId, ctx) =&gt; {
await submitToExternalAPI(callbackId, requestData);
},
{ timeout: { minutes: 30 } }
);
`
```
Python
```
`
result = context.wait\_for\_callback(
lambda callback\_id: submit\_to\_external\_api(callback\_id, request\_data),
name='external-api',
config=WaitForCallbackConfig(timeout\_seconds=1800)
)
`
```
Configure timeouts to prevent functions from waiting indefinitely. If a callback times out, the SDK throws a `CallbackError` and your function can handle the timeout case. Use heartbeat timeouts for long-running callbacks to detect when external systems stop responding.
Use callbacks for human-in-the-loop workflows, external system integration, webhook responses, or any scenario where execution must pause for external input.
### Parallel execution
Executes multiple operations concurrently with optional concurrency control. The SDK manages parallel execution, creates checkpoints for each operation, and handles failures according to your completion policy.
TypeScript
```
`
const results = await context.parallel([
async (ctx) =&gt; ctx.step('task1', async () =&gt; processTask1()),
async (ctx) =&gt; ctx.step('task2', async () =&gt; processTask2()),
async (ctx) =&gt; ctx.step('task3', async () =&gt; processTask3())
]);
`
```
Python
```
`
results = context.parallel(
lambda ctx: ctx.step(lambda \_: process\_task1(), name='task1'),
lambda ctx: ctx.step(lambda \_: process\_task2(), name='task2'),
lambda ctx: ctx.step(lambda \_: process\_task3(), name='task3')
)
`
```
Use `parallel` to execute independent operations concurrently.
### Map
Concurrently execute an operation on each item in an array with optional concurrency control. The SDK manages concurrent execution, creates checkpoints for each operation, and handles failures according to your completion policy.
TypeScript
```
`
const results = await context.map(itemArray, async (ctx, item, index) =&gt;
ctx.step('task', async () =&gt; processItem(item, index))
);
`
```
Python
```
`
results = context.map(
item\_array,
lambda ctx, item, index: ctx.step(
lambda \_: process\_item(item, index),
name='task'
)
)
`
```
Use `map` to process arrays with concurrency control.
### Child contexts
Creates an isolated execution context for grouping operations. Child contexts have their own checkpoint log and can contain multiple steps, waits, and other operations. The SDK treats the entire child context as a single unit for retry and recovery.
Use child contexts to organize complex workflows, implement sub-workflows, or isolate operations that should retry together.
TypeScript
```
`
const result = await context.runInChildContext(
'batch-processing',
async (childCtx) =&gt; {
return await processBatch(childCtx, items);
}
);
`
```
Python
```
`
result = context.run\_in\_child\_context(
lambda child\_ctx: process\_batch(child\_ctx, items),
name='batch-processing'
)
`
```
The replay mechanism demands that durable operations happen in a deterministic order. Using multiple child contexts you can have multiple streams of work execute concurrently, and the determinism applies separately within each context. This allows you to build high-performance functions which efficiently utilize multiple CPU cores.
For example, imagine we start two child contexts, A and B. On the initial invocation, the steps within the contexts were run in this order, with the 'A' steps running concurrently with the 'B' steps: A1, B1, B2, A2, A3. Upon replay, the timing is much faster as results are retrieved from checkpoint log, and the steps happen to be encountered in a different order: B1, A1, A2, B2, A3. Because the 'A' steps were encountered in the correct order (A1, A2, A3) and the 'B' steps were encountered in the correct order (B1, B2), the need for determinism was satisfied correctly.
### Conditional waits
Polls for a condition with automatic checkpointing between attempts. The SDK executes your check function, creates a checkpoint with the result, waits according to your strategy, and repeats until the condition is met.
TypeScript
```
`
const result = await context.waitForCondition(
async (state, ctx) =&gt; {
const status = await checkJobStatus(state.jobId);
return { ...state, status };
},
{
initialState: { jobId: 'job-123', status: 'pending' },
waitStrategy: (state) =&gt;
state.status === 'completed'
? { shouldContinue: false }
: { shouldContinue: true, delay: { seconds: 30 } }
}
);
`
```
Python
```
`
result = context.wait\_for\_condition(
lambda state, ctx: check\_job\_status(state['jobId']),
config=WaitForConditionConfig(
initial\_state={'jobId': 'job-123', 'status': 'pending'},
wait\_strategy=lambda state, attempt:
{'should\_continue': False} if state['status'] == 'completed'
else {'should\_continue': True, 'delay': 30}
)
)
`
```
Use `waitForCondition` for polling external systems, waiting for resources to be ready, or implementing retry with backoff.
### Function invocation
Invokes another Lambda function and waits for its result. The SDK creates a checkpoint, invokes the target function, and resumes your function when the invocation completes. This enables function composition and workflow decomposition.
TypeScript
```
`
const result = await context.invoke(
'invoke-processor',
'arn:aws:lambda:us-east-1:123456789012:function:processor:1',
{ data: inputData }
);
`
```
Python
```
`
result = context.invoke(
'arn:aws:lambda:us-east-1:123456789012:function:processor:1',
{'data': input\_data},
name='invoke-processor'
)
`
```
## How durable operations are metered
Each durable operation you call through `DurableContext` creates checkpoints to track execution progress and store state data. These operations incur charges based on their usage, and the checkpoints may contain data that contributes to your data write and retention costs. Stored data includes invocation event data, payloads returned from steps, and data passed when completing callbacks. Understanding how durable operations are metered helps you estimate execution costs and optimize your workflows. For details on pricing, see the [Lambda pricing page](https://aws.amazon.com/lambda/pricing/).
Payload size refers to the size of the serialized data that a durable operation persists. The data is measured in bytes and the size can vary depending on the serializer used by the operation. The payload of an operation could be the result itself for successful completions, or the serialized error object if the operation failed.
### Basic operations
Basic operations are the fundamental building blocks for durable functions:
|Operation|Checkpoint timing|Number of operations|Data persisted|
|**Execution**|Started|1|Input payload size|
|**Execution**|Completed (Succeeded/Failed/Stopped)|0|Output payload size|
|**Step**|Retry/Succeeded/Failed|1 + N retries|Returned payload size from each attempt|
|**Wait**|Started|1|N/A|
|**WaitForCondition**|Each poll attempt|1 + N polls|Returned payload size from each poll attempt|
|**Invocation-level Retry**|Started|1|Payload for error object|
### Callback operations
Callback operations enable your function to pause and wait for external systems to provide input. These operations create checkpoints when the callback is created and when it's completed:
|Operation|Checkpoint timing|Number of operations|Data persisted|
|**CreateCallback**|Started|1|N/A|
|**Callback completion via API call**|Completed|0|Callback payload|
|**WaitForCallback**|Started|3 + N retries (context + callback + step)|Payloads returned by submitter step attempts, plus two copies of the callback payload|
### Compound operations
Compound operations combine multiple durable operations to handle complex coordination patterns like parallel execution, array processing, and nested contexts:
|Operation|Checkpoint timing|Number of operations|Data persisted|
|**Parallel**|Started|1 + N branches (1 parent context + N child contexts)|Up to two copies of the returned payload size from each branch, plus the statuses of each branch|
|**Map**|Started|1 + N branches (1 parent context + N child contexts)|Up to two copies of the returned payload size from each iteration, plus the statuses of each iteration|
|**Promise helpers**|Completed|1|Returned payload size from the promise|
|**RunInChildContext**|Succeeded/Failed|1|Returned payload size from the child context|
For contexts, such as from `runInChildContext` or used internally by compound operations, results smaller than 256 KB are checkpointed directly. Larger results aren't stored—instead, they're reconstructed during replay by re-processing the context's operations.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Security and permissions
Supported runtimes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.