---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-execution-idempotency.html
title: Idempotency
word_count: 886
filtered: true
elements_removed: 0
density_score: 0.87
---

Idempotency - AWS Lambda
Idempotency - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-execution-idempotency)
[Execution names](#durable-idempotency-execution-names)[Idempotency behavior](#durable-idempotency-behavior)[Step idempotency](#durable-idempotency-steps)
# Idempotency
Durable functions provide built-in idempotency for execution starts through execution names. When you provide an execution name, Lambda uses it to prevent duplicate executions and enable safe retries of invocation requests. Steps have at-least-once execution semantics by default—during replay, the SDK returns checkpointed results without re-executing completed steps, but your business logic must be idempotent to handle potential retries before completion.
###### Note
Lambda event source mappings (ESM) don't support idempotency at launch. Therefore, each invocation (including retries) starts a new durable execution. To ensure idempotent execution with event source mappings, either implement idempotency logic in your function code such as with [Powertools for AWS Lambda](https://docs.aws.amazon.com//powertools/) or use a regular Lambda function as proxy (dispatcher) to invoke a durable function with an idempotency key (execution name parameter).
## Execution names
You can provide an execution name when invoking a durable function. The execution name acts as an idempotency key, allowing you to safely retry invocation requests without creating duplicate executions. If you don't provide a name, Lambda generates a unique execution ID automatically.
Execution names must be unique within your account and region. When you invoke a function with an execution name that already exists, Lambda behavior depends on the existing execution's state and whether the payload matches.
## Idempotency behavior
The following table describes how Lambda handles invocation requests based on whether you provide an execution name, the existing execution state, and whether the payload matches:
|Scenario|Name provided?|Existing execution status|Payload identical?|Behavior|
|1|No|N/A|N/A|**New execution started:** Lambda generates a unique execution ID and starts a new execution|
|2|Yes|Never existed or retention expired|N/A|**New execution started:** Lambda starts a new execution with the provided name|
|3|Yes|Running|Yes|**Idempotent start:** Lambda returns the existing execution information without starting a duplicate. For synchronous invocations, this acts as a reattach to the running execution|
|4|Yes|Running|No|**Error:** Lambda returns `DurableExecutionAlreadyExists` error because an execution with this name is already running with different payload|
|5|Yes|Closed (succeeded, failed, stopped, or timed out)|Yes|**Idempotent start:** Lambda returns the existing execution information without starting a new execution. The closed execution result is returned|
|6|Yes|Closed (succeeded, failed, stopped, or timed out)|No|**Error:** Lambda returns `DurableExecutionAlreadyExists` error because an execution with this name already completed with different payload|
###### Note
Scenarios 3 and 5 demonstrate idempotent behavior where Lambda safely handles duplicate invocation requests by returning existing execution information instead of creating duplicates.
## Step idempotency
Steps have at-least-once execution semantics by default. When your function replays after a wait, callback, or failure, the SDK checks each step against the checkpoint log. For steps that already completed, the SDK returns the checkpointed result without re-executing the step logic. However, if a step fails or the function is interrupted before the step completes, the step may execute multiple times.
Your business logic wrapped in steps must be idempotent to handle potential retries. Use idempotency keys to ensure operations like payments or database writes execute only once, even if the step retries.
**Example: Using idempotency keys in steps**
TypeScript
```
`
import { withDurableExecution, DurableContext } from '@aws/durable-execution-sdk-js';
import { randomUUID } from 'crypto';
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
// Generate idempotency key once
const idempotencyKey = await context.step('generate-key', async () =&gt; {
return randomUUID();
});
// Use idempotency key in payment API to prevent duplicate charges
const payment = await context.step('process-payment', async () =&gt; {
return paymentAPI.charge({
amount: event.amount,
idempotencyKey: idempotencyKey
});
});
return { statusCode: 200, payment };
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import durable\_execution, DurableContext
import uuid
@durable\_execution
def handler(event, context: DurableContext):
# Generate idempotency key once
idempotency\_key = context.step(
lambda \_: str(uuid.uuid4()),
name='generate-key'
)
# Use idempotency key in payment API to prevent duplicate charges
payment = context.step(
lambda \_: payment\_api.charge(
amount=event['amount'],
idempotency\_key=idempotency\_key
),
name='process-payment'
)
return {'statusCode': 200, 'payment': payment}
`
```
You can configure steps to use at-most-once execution semantics by setting the execution mode to `AT\_MOST\_ONCE\_PER\_RETRY`. This ensures the step executes at most once per retry attempt, but may not execute at all if the function is interrupted before the step completes.
The SDK enforces deterministic replay by validating that step names and order match the checkpoint log during replay. If your code attempts to execute steps in a different order or with different names, the SDK throws a `NonDeterministicExecutionError`.
**How replay works with completed steps:**
1. First invocation: Function executes step A, creates checkpoint, then waits
2. Second invocation (after wait): Function replays from beginning, step A returns checkpointed result instantly without re-executing, then continues to step B
3. Third invocation (after another wait): Function replays from beginning, steps A and B return checkpointed results instantly, then continues to step C
This replay mechanism ensures that completed steps don't re-execute, but your business logic must still be idempotent to handle retries before completion.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Retries
Testing durable functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.