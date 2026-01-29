---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-invoking-esm.html
title: Event source mappings with durable functions
word_count: 1528
filtered: true
elements_removed: 0
density_score: 0.85
---

Event source mappings with durable functions - AWS Lambda
Event source mappings with durable functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-invoking-esm)
[How event source mappings invoke durable functions](#durable-esm-invocation-behavior)[15-minute execution limit](#durable-esm-duration-limit)[Configuring event source mappings](#durable-esm-configuration)[Error handling with event source mappings](#durable-esm-error-handling)[Using an intermediary function for long-running workflows](#durable-esm-intermediary-function)[Supported event sources](#durable-esm-supported-sources)
# Event source mappings with durable functions
Durable functions work with all Lambda event source mappings. Configure event source mappings for durable functions the same way you configure them for standard functions. Event source mappings automatically poll event sources like Amazon SQS, Kinesis, and DynamoDB Streams, and invoke your function with batches of records.
Event source mappings are useful for durable functions that process streams or queues with complex, multi-step workflows. For example, you can create a durable function that processes Amazon SQS messages with retries, external API calls, and human approvals.
## How event source mappings invoke durable functions
Event source mappings invoke durable functions synchronously, waiting for the complete durable execution to finish before processing the next batch or marking records as processed. If the total durable execution time exceeds 15 minutes, the execution times out and fails. The event source mapping receives a timeout exception and handles it according to its retry configuration.
## 15-minute execution limit
When durable functions are invoked by event source mappings, the total durable execution duration cannot exceed 15 minutes. This limit applies to the entire durable execution from start to completion, not just individual function invocations.
This 15-minute limit is separate from the Lambda function timeout (also 15 minutes maximum). The function timeout controls how long each individual invocation can run, while the durable execution timeout controls the total elapsed time from execution start to completion.
**Example scenarios:**
* **Valid:** A durable function processes an Amazon SQS message with three steps, each taking 2 minutes, then waits 5 minutes before completing a final step. Total execution time: 11 minutes. This works because the total is under 15 minutes.
* **Invalid:** A durable function processes an Amazon SQS message, completes initial processing in 2 minutes, then waits 20 minutes for an external callback before completing. Total execution time: 22 minutes. This exceeds the 15-minute limit and will fail.
* **Invalid:** A durable function processes a Kinesis record with multiple wait operations totaling 30 minutes between steps. Even though each individual invocation completes quickly, the total execution time exceeds 15 minutes.
###### Important
Configure your durable execution timeout to 15 minutes or less when using event source mappings, otherwise creation of the event source mapping will fail. If your workflow requires longer execution times, use the intermediary function pattern described below.
## Configuring event source mappings
Configure event source mappings for durable functions using the Lambda console, AWS CLI, or AWS SDKs. All standard event source mapping properties apply to durable functions:
```
`
aws lambda create-event-source-mapping \\
--function-name arn:aws:lambda:us-east-1:123456789012:function:my-durable-function:1 \\
--event-source-arn arn:aws:sqs:us-east-1:123456789012:my-queue \\
--batch-size 10 \\
--maximum-batching-window-in-seconds 5
`
```
Remember to use a qualified ARN (with version number or alias) when configuring event source mappings for durable functions.
## Error handling with event source mappings
Event source mappings provide built-in error handling that works with durable functions:
* **Retry behavior:** If the initial invocation fails, the event source mapping retries according to its retry configuration. Configure maximum retry attempts and retry intervals based on your requirements.
* **Dead-letter queues:** Configure a dead-letter queue to capture records that fail after all retries. This prevents message loss and enables manual inspection of failed records.
* **Partial batch failures:** For Amazon SQS and Kinesis, use partial batch failure reporting to process records individually and only retry failed records.
* **Bisect on error:** For Kinesis and DynamoDB Streams, enable bisect on error to split failed batches and isolate problematic records.
###### Note
Durable functions support dead-letter queues (DLQs) for error handling, but don't support Lambda destinations. Configure a DLQ to capture records from failed invocations.
For complete information about event source mapping error handling, see [event source mappings](./invocation-eventsourcemapping.html).
## Using an intermediary function for long-running workflows
If your workflow requires more than 15 minutes to complete, use an intermediary standard Lambda function between the event source mapping and your durable function. The intermediary function receives events from the event source mapping and invokes the durable function asynchronously, removing the 15-minute execution limit.
This pattern decouples the event source mapping's synchronous invocation model from the durable function's long-running execution model. The event source mapping invokes the intermediary function, which quickly returns after starting the durable execution. The durable function then runs independently for as long as needed (up to 1 year).
### Architecture
The intermediary function pattern uses three components:
1. **Event source mapping:** Polls the event source (Amazon SQS, Kinesis, DynamoDB Streams) and invokes the intermediary function synchronously with batches of records.
2. **Intermediary function:** A standard Lambda function that receives events from the event source mapping, validates and transforms the data if needed, and invokes the durable function asynchronously. This function completes quickly (typically under 1 second) and returns control to the event source mapping.
3. **Durable function:** Processes the event with complex, multi-step logic that can run for extended periods. Invoked asynchronously, so it's not constrained by the 15-minute limit.
### Implementation
The intermediary function receives the entire event from the event source mapping and invokes the durable function asynchronously. Use the execution name parameter to ensure idempotent execution starts, preventing duplicate processing if the event source mapping retries:
TypeScript
```
`
import { LambdaClient, InvokeCommand } from '@aws-sdk/client-lambda';
import { SQSEvent } from 'aws-lambda';
import { createHash } from 'crypto';
const lambda = new LambdaClient({});
export const handler = async (event: SQSEvent) =&gt; {
// Invoke durable function asynchronously with execution name
await lambda.send(new InvokeCommand({
FunctionName: 'arn:aws:lambda:us-east-1:123456789012:function:my-durable-function:1',
InvocationType: 'Event',
Payload: JSON.stringify({
executionName: event.Name,
event: event
})
}));
return { statusCode: 200 };
};
`
```
Python
```
`
import boto3
import json
import hashlib
lambda\_client = boto3.client('lambda')
def handler(event, context):
# Invoke durable function asynchronously with execution name
lambda\_client.invoke(
FunctionName='arn:aws:lambda:us-east-1:123456789012:function:my-durable-function:1',
InvocationType='Event',
Payload=json.dumps({
'executionName': execution\_name,
'event': event["name"]
})
)
return {'statusCode': 200}
`
```
For idempotency in the intermediary function itself, use [Powertools for AWS Lambda](https://docs.aws.amazon.com//powertools/) to prevent duplicate invocations of the durable function if the event source mapping retries the intermediary function.
The durable function receives the payload with the execution name and processes all records with long-running logic:
TypeScript
```
`
import { withDurableExecution, DurableContext } from '@aws/durable-execution-sdk-js';
export const handler = withDurableExecution(
async (payload: any, context: DurableContext) =&gt; {
const sqsEvent = payload.event;
// Process each record with complex, multi-step logic
const results = await context.map(
sqsEvent.Records,
async (ctx, record) =&gt; {
const validated = await ctx.step('validate', async () =&gt; {
return validateOrder(JSON.parse(record.body));
});
// Wait for external approval (could take hours or days)
const approval = await ctx.waitForCallback(
'approval',
async (callbackId) =&gt; {
await requestApproval(callbackId, validated);
},
{ timeout: { hours: 48 } }
);
// Complete processing
return await ctx.step('complete', async () =&gt; {
return completeOrder(validated, approval);
});
}
);
return { statusCode: 200, processed: results.getResults().length };
}
);
`
```
Python
```
`
from aws\_durable\_execution\_sdk\_python import durable\_execution, DurableContext
import json
@durable\_execution
def handler(payload, context: DurableContext):
sqs\_event = payload['event']
# Process each record with complex, multi-step logic
def process\_record(ctx, record):
validated = ctx.step(
lambda \_: validate\_order(json.loads(record['body'])),
name='validate'
)
# Wait for external approval (could take hours or days)
approval = ctx.wait\_for\_callback(
lambda callback\_id: request\_approval(callback\_id, validated),
name='approval',
config=WaitForCallbackConfig(timeout\_seconds=172800) # 48 hours
)
# Complete processing
return ctx.step(
lambda \_: complete\_order(validated, approval),
name='complete'
)
results = context.map(sqs\_event['Records'], process\_record)
return {'statusCode': 200, 'processed': len(results.get\_results())}
`
```
### Key considerations
This pattern removes the 15-minute execution limit by decoupling the event source mapping from the durable execution. The intermediary function returns immediately after starting the durable execution, allowing the event source mapping to continue processing. The durable function then runs independently for as long as needed.
The intermediary function succeeds when it invokes the durable function, not when the durable execution completes. If the durable execution fails later, the event source mapping won't retry because it already processed the batch successfully. Implement error handling in the durable function and configure dead-letter queues for failed executions.
Use the execution name parameter to ensure idempotent execution starts. If the event source mapping retries the intermediary function, the durable function won't start a duplicate execution because the execution name already exists.
## Supported event sources
Durable functions support all Lambda event sources that use event source mappings:
* Amazon SQS queues (standard and FIFO)
* Kinesis streams
* DynamoDB Streams
* Amazon Managed Streaming for Apache Kafka (Amazon MSK)
* Self-managed Apache Kafka
* Amazon MQ (ActiveMQ and RabbitMQ)
* Amazon DocumentDB change streams
All event source types are subject to the 15-minute durable execution limit when invoking durable functions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoking durable functions
Retries
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.