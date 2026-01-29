---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-best-practices.html
title: Best practices for Lambda durable functions
word_count: 1631
filtered: true
elements_removed: 0
density_score: 0.84
---

Best practices for Lambda durable functions - AWS Lambda
Best practices for Lambda durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-best-practices)
[Write deterministic code](#durable-determinism)[Design for idempotency](#durable-idempotency)[Manage state efficiently](#durable-state-management)[Design effective steps](#durable-step-design)[Use wait operations efficiently](#durable-wait-operations)[Additional considerations](#durable-additional-considerations)[Additional resources](#durable-additional-resources)
# Best practices for Lambda durable functions
Durable functions use a replay-based execution model that requires different patterns than traditional Lambda functions. Follow these best practices to build reliable, cost-effective workflows.
## Write deterministic code
During replay, your function runs from the beginning and must follow the same execution path as the original run. Code outside durable operations must be deterministic, producing the same results given the same inputs.
**Wrap non-deterministic operations in steps:**
* Random number generation and UUIDs
* Current time or timestamps
* External API calls and database queries
* File system operations
TypeScript
```
`
import { withDurableExecution, DurableContext } from '@aws/durable-execution-sdk-js';
import { randomUUID } from 'crypto';
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
// Generate transaction ID inside a step
const transactionId = await context.step('generate-transaction-id', async () =&gt; {
return randomUUID();
});
// Use the same ID throughout execution, even during replay
const payment = await context.step('process-payment', async () =&gt; {
return processPayment(event.amount, transactionId);
});
return { statusCode: 200, transactionId, payment };
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
# Generate transaction ID inside a step
transaction\_id = context.step(
lambda \_: str(uuid.uuid4()),
name='generate-transaction-id'
)
# Use the same ID throughout execution, even during replay
payment = context.step(
lambda \_: process\_payment(event['amount'], transaction\_id),
name='process-payment'
)
return {'statusCode': 200, 'transactionId': transaction\_id, 'payment': payment}
`
```
###### Important
Don't use global variables or closures to share state between steps. Pass data through return values. Global state breaks during replay because steps return cached results but global variables reset.
**Avoid closure mutations:** Variables captured in closures can lose mutations during replay. Steps return cached results, but variable updates outside the step aren't replayed.
TypeScript
```
`
// ❌ WRONG: Mutations lost on replay
export const handler = withDurableExecution(async (event, context) =&gt; {
let total = 0;
for (const item of items) {
await context.step(async () =&gt; {
total += item.price; // ⚠️ Mutation lost on replay!
return saveItem(item);
});
}
return { total }; // Inconsistent value!
});
// ✅ CORRECT: Accumulate with return values
export const handler = withDurableExecution(async (event, context) =&gt; {
let total = 0;
for (const item of items) {
total = await context.step(async () =&gt; {
const newTotal = total + item.price;
await saveItem(item);
return newTotal; // Return updated value
});
}
return { total }; // Consistent!
});
// ✅ EVEN BETTER: Use map for parallel processing
export const handler = withDurableExecution(async (event, context) =&gt; {
const results = await context.map(
items,
async (ctx, item) =&gt; {
await ctx.step(async () =&gt; saveItem(item));
return item.price;
}
);
const total = results.getResults().reduce((sum, price) =&gt; sum + price, 0);
return { total };
});
`
```
Python
```
`
# ❌ WRONG: Mutations lost on replay
@durable\_execution
def handler(event, context: DurableContext):
total = 0
for item in items:
context.step(
lambda \_: save\_item\_and\_mutate(item, total), # ⚠️ Mutation lost on replay!
name=f'save-item-{item["id"]}'
)
return {'total': total} # Inconsistent value!
# ✅ CORRECT: Accumulate with return values
@durable\_execution
def handler(event, context: DurableContext):
total = 0
for item in items:
total = context.step(
lambda \_: save\_item\_and\_return\_total(item, total),
name=f'save-item-{item["id"]}'
)
return {'total': total} # Consistent!
# ✅ EVEN BETTER: Use map for parallel processing
@durable\_execution
def handler(event, context: DurableContext):
def process\_item(ctx, item):
ctx.step(lambda \_: save\_item(item))
return item['price']
results = context.map(items, process\_item)
total = sum(results.get\_results())
return {'total': total}
`
```
## Design for idempotency
Operations may execute multiple times due to retries or replay. Non-idempotent operations cause duplicate side effects like charging customers twice or sending multiple emails.
**Use idempotency tokens:** Generate tokens inside steps and include them with external API calls to prevent duplicate operations.
TypeScript
```
`
import { withDurableExecution, DurableContext } from '@aws/durable-execution-sdk-js';
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
// Generate idempotency token once
const idempotencyToken = await context.step('generate-idempotency-token', async () =&gt; {
return crypto.randomUUID();
});
// Use token to prevent duplicate charges
const charge = await context.step('charge-payment', async () =&gt; {
return paymentService.charge({
amount: event.amount,
cardToken: event.cardToken,
idempotencyKey: idempotencyToken
});
});
return { statusCode: 200, charge };
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
# Generate idempotency token once
idempotency\_token = context.step(
lambda \_: str(uuid.uuid4()),
name='generate-idempotency-token'
)
# Use token to prevent duplicate charges
def charge\_payment(\_):
return payment\_service.charge(
amount=event['amount'],
card\_token=event['cardToken'],
idempotency\_key=idempotency\_token
)
charge = context.step(charge\_payment, name='charge-payment')
return {'statusCode': 200, 'charge': charge}
`
```
**Use at-most-once semantics:** For critical operations that must never duplicate (financial transactions, inventory deductions), configure at-most-once execution mode.
TypeScript
```
`
// Critical operation that must not duplicate
await context.step('deduct-inventory', async () =&gt; {
return inventoryService.deduct(event.productId, event.quantity);
}, {
executionMode: 'AT\_MOST\_ONCE\_PER\_RETRY'
});
`
```
Python
```
`
# Critical operation that must not duplicate
context.step(
lambda \_: inventory\_service.deduct(event['productId'], event['quantity']),
name='deduct-inventory',
config=StepConfig(execution\_mode='AT\_MOST\_ONCE\_PER\_RETRY')
)
`
```
**Database idempotency:** Use check-before-write patterns, conditional updates, or upsert operations to prevent duplicate records.
## Manage state efficiently
Every checkpoint saves state to persistent storage. Large state objects increase costs, slow checkpointing, and impact performance. Store only essential workflow coordination data.
**Keep state minimal:**
* Store IDs and references, not full objects
* Fetch detailed data within steps as needed
* Use Amazon S3 or DynamoDB for large data, pass references in state
* Avoid passing large payloads between steps
TypeScript
```
`
import { withDurableExecution, DurableContext } from '@aws/durable-execution-sdk-js';
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
// Store only the order ID, not the full order object
const orderId = event.orderId;
// Fetch data within each step as needed
await context.step('validate-order', async () =&gt; {
const order = await orderService.getOrder(orderId);
return validateOrder(order);
});
await context.step('process-payment', async () =&gt; {
const order = await orderService.getOrder(orderId);
return processPayment(order);
});
return { statusCode: 200, orderId };
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import durable\_execution, DurableContext
@durable\_execution
def handler(event, context: DurableContext):
# Store only the order ID, not the full order object
order\_id = event['orderId']
# Fetch data within each step as needed
context.step(
lambda \_: validate\_order(order\_service.get\_order(order\_id)),
name='validate-order'
)
context.step(
lambda \_: process\_payment(order\_service.get\_order(order\_id)),
name='process-payment'
)
return {'statusCode': 200, 'orderId': order\_id}
`
```
## Design effective steps
Steps are the fundamental unit of work in durable functions. Well-designed steps make workflows easier to understand, debug, and maintain.
**Step design principles:**
* **Use descriptive names** - Names like `validate-order` instead of `step1` make logs and errors easier to understand
* **Keep names static** - Don't use dynamic names with timestamps or random values. Step names must be deterministic for replay
* **Balance granularity** - Break complex operations into focused steps, but avoid excessive tiny steps that increase checkpoint overhead
* **Group related operations** - Operations that should succeed or fail together belong in the same step
## Use wait operations efficiently
Wait operations suspend execution without consuming resources or incurring costs. Use them instead of keeping Lambda running.
**Time-based waits:** Use `context.wait()` for delays instead of `setTimeout` or `sleep`.
**External callbacks:** Use `context.waitForCallback()` when waiting for external systems. Always set timeouts to prevent indefinite waits.
**Polling:** Use `context.waitForCondition()` with exponential backoff to poll external services without overwhelming them.
TypeScript
```
`
// Wait 24 hours without cost
await context.wait({ seconds: 86400 });
// Wait for external callback with timeout
const result = await context.waitForCallback(
'external-job',
async (callbackId) =&gt; {
await externalService.submitJob({
data: event.data,
webhookUrl: `https://api.example.com/callbacks/${callbackId}`
});
},
{ timeout: { seconds: 3600 } }
);
`
```
Python
```
`
# Wait for external callback with timeout
result = context.wait\_for\_callback(
lambda callback\_id: external\_service.submit\_job(
data=event['data'],
webhook\_url=f'https://api.example.com/callbacks/{callback\_id}'
),
name='external-job',
config=WaitForCallbackConfig(timeout\_seconds=3600)
)
`
```
## Additional considerations
**Error handling:** Retry transient failures like network timeouts and rate limits. Don't retry permanent failures like invalid input or authentication errors. Configure retry strategies with appropriate max attempts and backoff rates. For detailed examples, see [Error handling and retries](./durable-execution-sdk-retries.html).
**Performance:** Minimize checkpoint size by storing references instead of full payloads. Use `context.parallel()` and `context.map()` to execute independent operations concurrently. Batch related operations to reduce checkpoint overhead.
**Versioning:** Invoke functions with version numbers or aliases to pin executions to specific code versions. Ensure new code versions can handle state from older versions. Don't rename steps or change their behavior in ways that break replay.
**Serialization:** Use JSON-compatible types for operation inputs and results. Convert dates to ISO strings and custom objects to plain objects before passing them to durable operations.
**Monitoring:** Enable structured logging with execution IDs and step names. Set up CloudWatch alarms for error rates and execution duration. Use tracing to identify bottlenecks. For detailed guidance, see [Monitoring and debugging](./durable-monitoring.html).
**Testing:** Test happy path, error handling, and replay behavior. Test timeout scenarios for callbacks and waits. Use local testing to reduce iteration time. For detailed guidance, see [Testing durable functions](./durable-testing.html).
**Common mistakes to avoid:** Don't nest `context.step()` calls, use child contexts instead. Wrap non-deterministic operations in steps. Always set timeouts for callbacks. Balance step granularity with checkpoint overhead. Store references instead of large objects in state.
## Additional resources
* [Python SDK documentation](https://github.com/aws/aws-durable-execution-sdk-python/tree/main/docs) - Complete API reference, testing patterns, and advanced examples
* [TypeScript SDK documentation](https://github.com/aws/aws-durable-execution-sdk-js/tree/main/docs) - Complete API reference, testing patterns, and advanced examples
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Monitoring durable functions
Lambda Managed Instances
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.