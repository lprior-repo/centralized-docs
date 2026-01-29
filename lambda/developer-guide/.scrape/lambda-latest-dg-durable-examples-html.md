---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-examples.html
title: Examples and use cases
word_count: 3703
filtered: true
elements_removed: 0
density_score: 0.79
---

Examples and use cases - AWS Lambda
Examples and use cases - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-examples)
[Short-lived fault-tolerant processes](#durable-examples-short-lived)[Long-running processes](#durable-examples-long-running)[Advanced patterns](#durable-examples-advanced)[Next steps](#durable-examples-next-steps)
# Examples and use cases
Lambda durable functions enable you to build fault-tolerant, multi-step applications using durable operations like steps and waits. With automatic checkpointing and a checkpoint-replay model. where execution restarts from the beginning after failure but skips completed checkpoints, your functions can recover from failures and resume execution without losing progress.
## Short-lived fault-tolerant processes
Use durable functions to build reliable operations that typically complete within minutes. While these processes are shorter than long-running workflows, they still benefit from automatic checkpointing and fault tolerance across distributed systems. Durable functions ensure your multi-step processes complete successfully even when individual service calls fail, without requiring complex error handling or state management code.
Common scenarios include hotel booking systems, restaurant reservation platforms, ride-sharing trip requests, event ticket purchases, and SaaS subscription upgrades. These scenarios share common characteristics: multiple service calls that must complete together, the need for automatic retry on transient failures, and the requirement to maintain consistent state across distributed systems.
### Distributed transactions across microservices
Coordinate payments, inventory, and shipping across multiple services with automatic rollback on failures. Each service operation is wrapped in a step, ensuring the transaction can recover from any point if a service fails.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { orderId, amount, items } = event;
// Reserve inventory across multiple warehouses
const inventory = await context.step("reserve-inventory", async () =&gt; {
return await inventoryService.reserve(items);
});
// Process payment
const payment = await context.step("process-payment", async () =&gt; {
return await paymentService.charge(amount);
});
// Create shipment
const shipment = await context.step("create-shipment", async () =&gt; {
return await shippingService.createShipment(orderId, inventory);
});
return { orderId, status: 'completed', shipment };
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution
@durable\_execution
def lambda\_handler(event, context: DurableContext):
order\_id = event['orderId']
amount = event['amount']
items = event['items']
# Reserve inventory across multiple warehouses
inventory = context.step(
lambda \_: inventory\_service.reserve(items),
name='reserve-inventory'
)
# Process payment
payment = context.step(
lambda \_: payment\_service.charge(amount),
name='process-payment'
)
# Create shipment
shipment = context.step(
lambda \_: shipping\_service.create\_shipment(order\_id, inventory),
name='create-shipment'
)
return {'orderId': order\_id, 'status': 'completed', 'shipment': shipment}
`
```
If any step fails, the function automatically retries from the last successful checkpoint. The inventory reservation persists even if payment processing fails temporarily. When the function retries, it skips the completed inventory step and proceeds directly to payment processing. This eliminates duplicate reservations and ensures consistent state across your distributed system.
### Order processing with multiple steps
Process orders through validation, payment authorization, inventory allocation, and fulfillment with automatic retry and recovery. Each step is checkpointed, ensuring the order progresses even if individual steps fail and retry.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { orderId, customerId, items } = event;
// Validate order details
const validation = await context.step("validate-order", async () =&gt; {
const customer = await customerService.validate(customerId);
const itemsValid = await inventoryService.validateItems(items);
return { customer, itemsValid };
});
if (!validation.itemsValid) {
return { orderId, status: 'rejected', reason: 'invalid\_items' };
}
// Authorize payment
const authorization = await context.step("authorize-payment", async () =&gt;&gt; {
return await paymentService.authorize(
validation.customer.paymentMethod,
calculateTotal(items)
);
});
// Allocate inventory
const allocation = await context.step("allocate-inventory", async () =&gt; {
return await inventoryService.allocate(items);
});
// Fulfill order
const fulfillment = await context.step("fulfill-order", async () =&gt; {
return await fulfillmentService.createShipment({
orderId,
items: allocation.allocatedItems,
address: validation.customer.shippingAddress
});
});
return {
orderId,
status: 'completed',
trackingNumber: fulfillment.trackingNumber
};
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution
@durable\_execution
def lambda\_handler(event, context: DurableContext):
order\_id = event['orderId']
customer\_id = event['customerId']
items = event['items']
# Validate order details
def validate\_order(\_):
customer = customer\_service.validate(customer\_id)
items\_valid = inventory\_service.validate\_items(items)
return {'customer': customer, 'itemsValid': items\_valid}
validation = context.step(validate\_order, name='validate-order')
if not validation['itemsValid']:
return {'orderId': order\_id, 'status': 'rejected', 'reason': 'invalid\_items'}
# Authorize payment
authorization = context.step(
lambda \_: payment\_service.authorize(
validation['customer']['paymentMethod'],
calculate\_total(items)
),
name='authorize-payment'
)
# Allocate inventory
allocation = context.step(
lambda \_: inventory\_service.allocate(items),
name='allocate-inventory'
)
# Fulfill order
fulfillment = context.step(
lambda \_: fulfillment\_service.create\_shipment({
'orderId': order\_id,
'items': allocation['allocatedItems'],
'address': validation['customer']['shippingAddress']
}),
name='fulfill-order'
)
return {
'orderId': order\_id,
'status': 'completed',
'trackingNumber': fulfillment['trackingNumber']
}
`
```
This pattern ensures orders never get stuck in intermediate states. If validation fails, the order is rejected before payment authorization. If payment authorization fails, inventory isn't allocated. Each step builds on the previous one with automatic retry and recovery.
###### Note
The conditional check `if (!validation.itemsValid)` is outside a step and will re-execute during replay. This is safe because it's deterministic—it always produces the same result given the same validation object.
## Long-running processes
Use durable functions for processes that span hours, days, or weeks. Wait operations suspend execution without incurring compute charges, making long-running processes cost-effective. During wait periods, your function stops running and Lambda recycles the execution environment. When it's time to resume, Lambda invokes your function again and replays from the last checkpoint.
This execution model makes durable functions ideal for processes that need to pause for extended periods, whether waiting for human decisions, external system responses, scheduled processing windows, or time-based delays. You pay only for active compute time, not for waiting.
Common scenarios include document approval processes, scheduled batch processing, multi-day onboarding processes, subscription trial processes, and delayed notification systems. These scenarios share common characteristics: extended wait periods measured in hours or days, the need to maintain execution state across those waits, and cost-sensitive requirements where paying for idle compute time is prohibitive.
### Human-in-the-loop approvals
Pause execution for document reviews, approvals, or decisions while maintaining execution state. The function waits for external callbacks without consuming resources, resuming automatically when approval is received.
This pattern is essential for processes that require human judgment or external validation. The function suspends at the callback point, incurring no compute charges while waiting. When someone submits their decision via API, Lambda invokes your function again and replays from the checkpoint, continuing with the approval result.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { documentId, reviewers } = event;
// Step 1: Prepare document for review
const prepared = await context.step("prepare-document", async () =&gt; {
return await documentService.prepare(documentId);
});
// Step 2: Request approval with callback
const approval = await context.waitForCallback(
"approval-callback",
async (callbackId) =&gt; {
await notificationService.sendApprovalRequest({
documentId,
reviewers,
callbackId,
expiresIn: 86400
});
},
{
timeout: { seconds: 86400 }
}
);
// Function resumes here when approval is received
if (approval?.approved) {
const finalized = await context.step("finalize-document", async () =&gt; {
return await documentService.finalize(documentId, approval.comments);
});
return {
status: 'approved',
documentId,
finalizedAt: finalized.timestamp
};
}
// Handle rejection
await context.step("archive-rejected", async () =&gt; {
await documentService.archive(documentId, approval?.reason);
});
return {
status: 'rejected',
documentId,
reason: approval?.reason
};
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution, WaitConfig
@durable\_execution
def lambda\_handler(event, context: DurableContext):
document\_id = event['documentId']
reviewers = event['reviewers']
# Step 1: Prepare document for review
prepared = context.step(
lambda \_: document\_service.prepare(document\_id),
name='prepare-document'
)
# Step 2: Request approval with callback
def send\_approval\_request(callback\_id):
notification\_service.send\_approval\_request({
'documentId': document\_id,
'reviewers': reviewers,
'callbackId': callback\_id,
'expiresIn': 86400
})
approval = context.wait\_for\_callback(
send\_approval\_request,
name='approval-callback',
config=WaitConfig(timeout=86400)
)
# Function resumes here when approval is received
if approval and approval.get('approved'):
finalized = context.step(
lambda \_: document\_service.finalize(document\_id, approval.get('comments')),
name='finalize-document'
)
return {
'status': 'approved',
'documentId': document\_id,
'finalizedAt': finalized['timestamp']
}
# Handle rejection
context.step(
lambda \_: document\_service.archive(document\_id, approval.get('reason') if approval else None),
name='archive-rejected'
)
return {
'status': 'rejected',
'documentId': document\_id,
'reason': approval.get('reason') if approval else None
}
`
```
When the callback is received and your function resumes, it replays from the beginning. The prepare-document step returns its checkpointed result instantly. The waitForCallback operation also returns instantly with the stored approval result instead of waiting again. Execution then continues to the finalization or archival steps.
### Multi-stage data pipelines
Process large datasets through extraction, transformation, and loading phases with checkpoints between stages. Each stage can take hours to complete, and checkpoints ensure the pipeline can resume from any stage if interrupted.
This pattern is ideal for ETL workflows, data migrations, or batch processing jobs where you need to process data in stages with recovery points between them. If a stage fails, the pipeline resumes from the last completed stage rather than restarting from the beginning. You can also use wait operations to pause between stages; respecting rate limits, waiting for downstream systems to be ready, or scheduling processing during off-peak hours.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { datasetId, batchSize } = event;
// Stage 1: Extract data from source
const extracted = await context.step("extract-data", async () =&gt; {
const records = await sourceDatabase.extractRecords(datasetId);
return { recordCount: records.length, records };
});
// Wait 5 minutes to respect source system rate limits
await context.wait({ seconds: 300 });
// Stage 2: Transform data in batches
const transformed = await context.step("transform-data", async () =&gt; {
const batches = chunkArray(extracted.records, batchSize);
const results = [];
for (const batch of batches) {
const transformed = await transformService.processBatch(batch);
results.push(transformed);
}
return { batchCount: batches.length, results };
});
// Wait until off-peak hours (e.g., 2 AM)
const now = new Date();
const targetHour = 2;
const msUntilTarget = calculateMsUntilHour(now, targetHour);
await context.wait({ seconds: Math.floor(msUntilTarget / 1000) });
// Stage 3: Load data to destination
const loaded = await context.step("load-data", async () =&gt; {
let loadedCount = 0;
for (const result of transformed.results) {
await destinationDatabase.loadBatch(result);
loadedCount += result.length;
}
return { loadedCount };
});
// Stage 4: Verify and finalize
const verified = await context.step("verify-pipeline", async () =&gt; {
const verification = await destinationDatabase.verifyRecords(datasetId);
await pipelineService.markComplete(datasetId, verification);
return verification;
});
return {
datasetId,
recordsProcessed: extracted.recordCount,
batchesProcessed: transformed.batchCount,
recordsLoaded: loaded.loadedCount,
verified: verified.success
};
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution
from datetime import datetime
@durable\_execution
def lambda\_handler(event, context: DurableContext):
dataset\_id = event['datasetId']
batch\_size = event['batchSize']
# Stage 1: Extract data from source
def extract\_data(\_):
records = source\_database.extract\_records(dataset\_id)
return {'recordCount': len(records), 'records': records}
extracted = context.step(extract\_data, name='extract-data')
# Wait 5 minutes to respect source system rate limits
context.wait(300)
# Stage 2: Transform data in batches
def transform\_data(\_):
batches = chunk\_array(extracted['records'], batch\_size)
results = []
for batch in batches:
transformed = transform\_service.process\_batch(batch)
results.append(transformed)
return {'batchCount': len(batches), 'results': results}
transformed = context.step(transform\_data, name='transform-data')
# Wait until off-peak hours (e.g., 2 AM)
now = datetime.now()
target\_hour = 2
ms\_until\_target = calculate\_ms\_until\_hour(now, target\_hour)
context.wait(ms\_until\_target // 1000)
# Stage 3: Load data to destination
def load\_data(\_):
loaded\_count = 0
for result in transformed['results']:
destination\_database.load\_batch(result)
loaded\_count += len(result)
return {'loadedCount': loaded\_count}
loaded = context.step(load\_data, name='load-data')
# Stage 4: Verify and finalize
def verify\_pipeline(\_):
verification = destination\_database.verify\_records(dataset\_id)
pipeline\_service.mark\_complete(dataset\_id, verification)
return verification
verified = context.step(verify\_pipeline, name='verify-pipeline')
return {
'datasetId': dataset\_id,
'recordsProcessed': extracted['recordCount'],
'batchesProcessed': transformed['batchCount'],
'recordsLoaded': loaded['loadedCount'],
'verified': verified['success']
}
`
```
Each stage is wrapped in a step, creating a checkpoint that allows the pipeline to resume from any stage if interrupted. The 5-minute wait between extract and transform respects source system rate limits without consuming compute resources, while the wait until 2 AM schedules the expensive load operation during off-peak hours.
###### Note
The `new Date()` call and `calculateMsUntilHour()` function are outside steps and will re-execute during replay. For time-based operations that must be consistent across replays, calculate the timestamp inside a step or use it only for wait durations (which are checkpointed).
## Advanced patterns
Use durable functions to build complex multi-step applications that combine multiple durable operations, parallel execution, array processing, conditional logic, and polling. These patterns let you build sophisticated applications that coordinate many tasks while maintaining fault tolerance and automatic recovery.
Advanced patterns go beyond simple sequential steps. You can run operations concurrently with `parallel()`, process arrays with `map()`, wait for external conditions with `waitForCondition()`, and combine these primitives to build reliable applications. Each durable operation creates its own checkpoints, so your application can recover from any point if interrupted.
### User onboarding processes
Guide users through registration, email verification, profile setup, and initial configuration with retry handling. This example combines sequential steps, callbacks, and conditional logic to create a complete onboarding process.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { userId, email } = event;
// Step 1: Create user account
const user = await context.step("create-account", async () =&gt; {
return await userService.createAccount(userId, email);
});
// Step 2: Send verification email
await context.step("send-verification", async () =&gt; {
return await emailService.sendVerification(email);
});
// Step 3: Wait for email verification (up to 48 hours)
const verified = await context.waitForCallback(
"email-verification",
async (callbackId) =&gt; {
await notificationService.sendVerificationLink({
email,
callbackId,
expiresIn: 172800
});
},
{
timeout: { seconds: 172800 }
}
);
if (!verified) {
await context.step("send-reminder", async () =&gt; {
await emailService.sendReminder(email);
});
return {
status: "verification\_timeout",
userId,
message: "Email verification not completed within 48 hours"
};
}
// Step 4: Initialize user profile in parallel
const setupResults = await context.parallel("profile-setup", [
async (ctx: DurableContext) =&gt;&gt; {
return await ctx.step("create-preferences", async () =&gt; {
return await preferencesService.createDefaults(userId);
});
},
async (ctx: DurableContext) =&gt; {
return await ctx.step("setup-notifications", async () =&gt; {
return await notificationService.setupDefaults(userId);
});
},
async (ctx: DurableContext) =&gt; {
return await ctx.step("create-welcome-content", async () =&gt; {
return await contentService.createWelcome(userId);
});
}
]);
// Step 5: Send welcome email
await context.step("send-welcome", async () =&gt; {
const [preferences, notifications, content] = setupResults.getResults();
return await emailService.sendWelcome({
email,
preferences,
notifications,
content
});
});
return {
status: "onboarding\_complete",
userId,
completedAt: new Date().toISOString()
};
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution, WaitConfig
from datetime import datetime
@durable\_execution
def lambda\_handler(event, context: DurableContext):
user\_id = event['userId']
email = event['email']
# Step 1: Create user account
user = context.step(
lambda \_: user\_service.create\_account(user\_id, email),
name='create-account'
)
# Step 2: Send verification email
context.step(
lambda \_: email\_service.send\_verification(email),
name='send-verification'
)
# Step 3: Wait for email verification (up to 48 hours)
def send\_verification\_link(callback\_id):
notification\_service.send\_verification\_link({
'email': email,
'callbackId': callback\_id,
'expiresIn': 172800
})
verified = context.wait\_for\_callback(
send\_verification\_link,
name='email-verification',
config=WaitConfig(timeout=172800)
)
if not verified:
context.step(
lambda \_: email\_service.send\_reminder(email),
name='send-reminder'
)
return {
'status': 'verification\_timeout',
'userId': user\_id,
'message': 'Email verification not completed within 48 hours'
}
# Step 4: Initialize user profile in parallel
def create\_preferences(ctx: DurableContext):
return ctx.step(
lambda \_: preferences\_service.create\_defaults(user\_id),
name='create-preferences'
)
def setup\_notifications(ctx: DurableContext):
return ctx.step(
lambda \_: notification\_service.setup\_defaults(user\_id),
name='setup-notifications'
)
def create\_welcome\_content(ctx: DurableContext):
return ctx.step(
lambda \_: content\_service.create\_welcome(user\_id),
name='create-welcome-content'
)
setup\_results = context.parallel(
[create\_preferences, setup\_notifications, create\_welcome\_content],
name='profile-setup'
)
# Step 5: Send welcome email
def send\_welcome(\_):
results = setup\_results.get\_results()
preferences, notifications, content = results[0], results[1], results[2]
return email\_service.send\_welcome({
'email': email,
'preferences': preferences,
'notifications': notifications,
'content': content
})
context.step(send\_welcome, name='send-welcome')
return {
'status': 'onboarding\_complete',
'userId': user\_id,
'completedAt': datetime.now().isoformat()
}
`
```
The process combines sequential steps with checkpoints for account creation and email sending, then pauses for up to 48 hours waiting for email verification without consuming resources. Conditional logic handles different paths based on whether verification completes or times out. Profile setup tasks run concurrently using parallel operations to reduce total execution time, and each step retries automatically on transient failures to ensure the onboarding completes reliably.
### Chained invocations across functions
Invoke other Lambda functions from within a durable function using `context.invoke()`. The calling function suspends while waiting for the invoked function to complete, creating a checkpoint that preserves the result. If the calling function is interrupted after the invoked function completes, it resumes with the stored result without re-invoking the function.
Use this pattern when you have specialized functions that handle specific domains (customer validation, payment processing, inventory management) and need to coordinate them in a workflow. Each function maintains its own logic and can be invoked by multiple orchestrator functions, avoiding code duplication.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
// Main orchestrator function
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { orderId, customerId } = event;
// Step 1: Validate customer by invoking customer service function
const customer = await context.invoke(
"validate-customer",
"arn:aws:lambda:us-east-1:123456789012:function:customer-service:1",
{ customerId }
);
if (!customer.isValid) {
return { orderId, status: "rejected", reason: "invalid\_customer" };
}
// Step 2: Check inventory by invoking inventory service function
const inventory = await context.invoke(
"check-inventory",
"arn:aws:lambda:us-east-1:123456789012:function:inventory-service:1",
{ orderId, items: event.items }
);
if (!inventory.available) {
return { orderId, status: "rejected", reason: "insufficient\_inventory" };
}
// Step 3: Process payment by invoking payment service function
const payment = await context.invoke(
"process-payment",
"arn:aws:lambda:us-east-1:123456789012:function:payment-service:1",
{
customerId,
amount: inventory.totalAmount,
paymentMethod: customer.paymentMethod
}
);
// Step 4: Create shipment by invoking fulfillment service function
const shipment = await context.invoke(
"create-shipment",
"arn:aws:lambda:us-east-1:123456789012:function:fulfillment-service:1",
{
orderId,
items: inventory.allocatedItems,
address: customer.shippingAddress
}
);
return {
orderId,
status: "completed",
trackingNumber: shipment.trackingNumber,
estimatedDelivery: shipment.estimatedDelivery
};
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution
# Main orchestrator function
@durable\_execution
def lambda\_handler(event, context: DurableContext):
order\_id = event['orderId']
customer\_id = event['customerId']
# Step 1: Validate customer by invoking customer service function
customer = context.invoke(
'arn:aws:lambda:us-east-1:123456789012:function:customer-service:1',
{'customerId': customer\_id},
name='validate-customer'
)
if not customer['isValid']:
return {'orderId': order\_id, 'status': 'rejected', 'reason': 'invalid\_customer'}
# Step 2: Check inventory by invoking inventory service function
inventory = context.invoke(
'arn:aws:lambda:us-east-1:123456789012:function:inventory-service:1',
{'orderId': order\_id, 'items': event['items']},
name='check-inventory'
)
if not inventory['available']:
return {'orderId': order\_id, 'status': 'rejected', 'reason': 'insufficient\_inventory'}
# Step 3: Process payment by invoking payment service function
payment = context.invoke(
'arn:aws:lambda:us-east-1:123456789012:function:payment-service:1',
{
'customerId': customer\_id,
'amount': inventory['totalAmount'],
'paymentMethod': customer['paymentMethod']
},
name='process-payment'
)
# Step 4: Create shipment by invoking fulfillment service function
shipment = context.invoke(
'arn:aws:lambda:us-east-1:123456789012:function:fulfillment-service:1',
{
'orderId': order\_id,
'items': inventory['allocatedItems'],
'address': customer['shippingAddress']
},
name='create-shipment'
)
return {
'orderId': order\_id,
'status': 'completed',
'trackingNumber': shipment['trackingNumber'],
'estimatedDelivery': shipment['estimatedDelivery']
}
`
```
Each invocation creates a checkpoint in the orchestrator function. If the orchestrator is interrupted after the customer validation completes, it resumes from that checkpoint with the stored customer data, skipping the validation invocation. This prevents duplicate calls to downstream services and ensures consistent execution across interruptions.
The invoked functions can be either durable or standard Lambda functions. If you invoke a durable function, it can have its own multi-step workflow with waits and checkpoints. The orchestrator simply waits for the complete durable execution to finish, receiving the final result.
###### Note
Cross-account invocations are not supported. All invoked functions must be in the same AWS account as the calling function.
### Batch processing with checkpoints
Process millions of records with automatic recovery from the last successful checkpoint after failures. This example demonstrates how durable functions combine `map()` operations with chunking and rate limiting to handle large-scale data processing.
TypeScript
```
`
import { DurableContext, withDurableExecution } from "@aws/durable-execution-sdk-js";
interface Batch {
batchIndex: number;
recordIds: string[];
}
export const handler = withDurableExecution(
async (event: any, context: DurableContext) =&gt; {
const { datasetId, batchSize = 1000 } = event;
// Step 1: Get all record IDs to process
const recordIds = await context.step("fetch-record-ids", async () =&gt; {
return await dataService.getRecordIds(datasetId);
});
// Step 2: Split into batches
const batches: Batch[] = [];
for (let i = 0; i &lt; recordIds.length; i += batchSize) {
batches.push({
batchIndex: Math.floor(i / batchSize),
recordIds: recordIds.slice(i, i + batchSize)
});
}
// Step 3: Process batches with controlled concurrency
const batchResults = await context.map(
"process-batches",
batches,
async (ctx: DurableContext, batch: Batch, index: number) =&gt; {
const processed = await ctx.step(`batch-${batch.batchIndex}`, async () =&gt; {
const results = [];
for (const recordId of batch.recordIds) {
const result = await recordService.process(recordId);
results.push(result);
}
return results;
});
const validated = await ctx.step(`validate-${batch.batchIndex}`, async () =&gt; {
return await validationService.validateBatch(processed);
});
return {
batchIndex: batch.batchIndex,
recordCount: batch.recordIds.length,
successCount: validated.successCount,
failureCount: validated.failureCount
};
},
{
maxConcurrency: 5
}
);
// Step 4: Aggregate results
const summary = await context.step("aggregate-results", async () =&gt; {
const results = batchResults.getResults();
const totalSuccess = results.reduce((sum, r) =&gt; sum + r.successCount, 0);
const totalFailure = results.reduce((sum, r) =&gt; sum + r.failureCount, 0);
return {
datasetId,
totalRecords: recordIds.length,
batchesProcessed: batches.length,
successCount: totalSuccess,
failureCount: totalFailure,
completedAt: new Date().toISOString()
};
});
return summary;
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import DurableContext, durable\_execution, MapConfig
from datetime import datetime
from typing import List, Dict
@durable\_execution
def lambda\_handler(event, context: DurableContext):
dataset\_id = event['datasetId']
batch\_size = event.get('batchSize', 1000)
# Step 1: Get all record IDs to process
record\_ids = context.step(
lambda \_: data\_service.get\_record\_ids(dataset\_id),
name='fetch-record-ids'
)
# Step 2: Split into batches
batches = []
for i in range(0, len(record\_ids), batch\_size):
batches.append({
'batchIndex': i // batch\_size,
'recordIds': record\_ids[i:i + batch\_size]
})
# Step 3: Process batches with controlled concurrency
def process\_batch(ctx: DurableContext, batch: Dict, index: int):
batch\_index = batch['batchIndex']
def process\_records(\_):
results = []
for record\_id in batch['recordIds']:
result = record\_service.process(record\_id)
results.append(result)
return results
processed = ctx.step(process\_records, name=f'batch-{batch\_index}')
validated = ctx.step(
lambda \_: validation\_service.validate\_batch(processed),
name=f'validate-{batch\_index}'
)
return {
'batchIndex': batch\_index,
'recordCount': len(batch['recordIds']),
'successCount': validated['successCount'],
'failureCount': validated['failureCount']
}
batch\_results = context.map(
process\_batch,
batches,
name='process-batches',
config=MapConfig(max\_concurrency=5)
)
# Step 4: Aggregate results
def aggregate\_results(\_):
results = batch\_results.get\_results()
total\_success = sum(r['successCount'] for r in results)
total\_failure = sum(r['failureCount'] for r in results)
return {
'datasetId': dataset\_id,
'totalRecords': len(record\_ids),
'batchesProcessed': len(batches),
'successCount': total\_success,
'failureCount': total\_failure,
'completedAt': datetime.now().isoformat()
}
summary = context.step(aggregate\_results, name='aggregate-results')
return summary
`
```
Records are split into manageable batches to avoid overwhelming memory or downstream services, then multiple batches process concurrently with `maxConcurrency` controlling the parallelism. Each batch has its own checkpoint, so failures only retry the failed batch rather than reprocessing all records. This pattern is ideal for ETL jobs, data migrations, or bulk operations where processing can take hours.
## Next steps
* Explore [basic concepts](./durable-basic-concepts.html) to understand DurableContext, steps, and waits
* Review [best practices](./durable-best-practices.html) for writing deterministic code and optimizing performance
* Learn about [testing durable functions](./durable-testing.html) locally and in the cloud
* Compare durable functions with Step Functions to understand when each approach is most effective. See [Durable functions or Step Functions](./durable-step-functions.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Durable functions or Step Functions
Security and permissions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.