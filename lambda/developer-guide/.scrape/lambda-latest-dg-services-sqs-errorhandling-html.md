---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-sqs-errorhandling.html
title: Handling errors for an SQS event source in Lambda
word_count: 2157
filtered: true
elements_removed: 0
density_score: 0.84
---

Handling errors for an SQS event source in Lambda - AWS Lambda
Handling errors for an SQS event source in Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-sqs-errorhandling)
[Backoff strategy for failed invocations](#services-sqs-backoff-strategy)[Implementing partial batch responses](#services-sqs-batchfailurereporting)
# Handling errors for an SQS event source in Lambda
To handle errors related to an SQS event source, Lambda automatically uses a retry strategy with a
backoff strategy. You can also customize error handling behavior by configuring your SQS event
source mapping to return [partial batch responses](#services-sqs-batchfailurereporting).
## Backoff strategy for failed invocations
When an invocation fails, Lambda attempts to retry the invocation while implementing a backoff strategy.
The backoff strategy differs slightly depending on whether Lambda encountered the failure due to an error in
your function code, or due to throttling.
*
If your **function code** caused the error, Lambda will stop processing and retrying the invocation.
In the meantime, Lambda gradually backs off, reducing the amount of concurrency allocated to your Amazon SQS event source mapping.
After your queue's visibility timeout runs out, the message will again reappear in the queue.
* If the invocation fails due to **throttling**, Lambda gradually backs off
retries by reducing the amount of concurrency allocated to your Amazon SQS event source mapping. Lambda continues
to retry the message until the message's timestamp exceeds your queue's visibility timeout, at which point
Lambda drops the message.
## Implementing partial batch responses
When your Lambda function encounters an error while processing a batch, all messages in that batch become
visible in the queue again by default, including messages that Lambda processed successfully. As a result, your
function can end up processing the same message several times.
To avoid reprocessing successfully processed messages in a failed batch, you can configure your event
source mapping to make only the failed messages visible again. This is called a partial batch response.
To turn on partial batch responses, specify `ReportBatchItemFailures` for the
[FunctionResponseTypes](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html#lambda-UpdateEventSourceMapping-request-FunctionResponseTypes)
action when configuring your event source mapping. This lets your function
return a partial success, which can help reduce the number of unnecessary retries on records.
###### Note
The [Batch Processor utility](https://docs.powertools.aws.dev/lambda/python/latest/utilities/batch/) from Powertools for AWS Lambda handles all
of the partial batch response logic automatically. This utility simplifies implementing batch processing patterns and reduces the custom code needed to handle batch item failures correctly.
It is available for Python, Java, Typescript, and .NET.
When `ReportBatchItemFailures` is activated, Lambda doesn't [scale down message polling](#services-sqs-backoff-strategy) when function invocations fail. If you expect some messages to fail—and you don't want those failures to impact the message processing rate—use `ReportBatchItemFailures`.
###### Note
Keep the following in mind when using partial batch responses:
* If your function throws an exception, the entire batch is considered a complete failure.
* If you're using this feature with a FIFO queue, your function should stop processing messages after the
first failure and return all failed and unprocessed messages in `batchItemFailures`. This helps
preserve the ordering of messages in your queue.
###### To activate partial batch reporting
1. Review the [Best practices for implementing partial batch responses](https://docs.aws.amazon.com/prescriptive-guidance/latest/lambda-event-filtering-partial-batch-responses-for-sqs/best-practices-partial-batch-responses.html).
2. Run the following command to activate `ReportBatchItemFailures` for your function. To retrieve your event source mapping's UUID, run the [list-event-source-mappings](https://docs.aws.amazon.com/cli/latest/reference/lambda/list-event-source-mappings.html) AWS CLI command.
```
`aws lambda update-event-source-mapping \\
--uuid `"a1b2c3d4-5678-90ab-cdef-11111EXAMPLE"` \\
--function-response-types `"ReportBatchItemFailures"``
```
3. Update your function code to catch all exceptions and return failed messages in a `batchItemFailures` JSON response. The `batchItemFailures` response must include a list of message IDs, as `itemIdentifier` JSON values.
For example, suppose you have a batch of five messages, with message IDs `id1`, `id2`, `id3`, `id4`, and `id5`. Your function successfully processes `id1`, `id3`, and `id5`. To make messages `id2` and `id4` visible again in your queue, your function should return the following response:
```
`{
"batchItemFailures": [
{
"itemIdentifier": "id2"
},
{
"itemIdentifier": "id4"
}
]
}`
```
Here are some examples of function code that return the list of failed message IDs in the batch:
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using Amazon.Lambda.Core;
using Amazon.Lambda.SQSEvents;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace sqsSample;
public class Function
{
public async Task&lt;SQSBatchResponse&gt; FunctionHandler(SQSEvent evnt, ILambdaContext context)
{
List&lt;SQSBatchResponse.BatchItemFailure&gt; batchItemFailures = new List&lt;SQSBatchResponse.BatchItemFailure&gt;();
foreach(var message in evnt.Records)
{
try
{
//process your message
await ProcessMessageAsync(message, context);
}
catch (System.Exception)
{
//Add failed message identifier to the batchItemFailures list
batchItemFailures.Add(new SQSBatchResponse.BatchItemFailure{ItemIdentifier=message.MessageId});
}
}
return new SQSBatchResponse(batchItemFailures);
}
private async Task ProcessMessageAsync(SQSEvent.SQSMessage message, ILambdaContext context)
{
if (String.IsNullOrEmpty(message.Body))
{
throw new Exception("No Body in SQS Message.");
}
context.Logger.LogInformation($"Processed message {message.Body}");
// TODO: Do interesting work based on the new message
await Task.CompletedTask;
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package main
import (
"context"
"fmt"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
func handler(ctx context.Context, sqsEvent events.SQSEvent) (map[string]interface{}, error) {
batchItemFailures := []map[string]interface{}{}
for \_, message := range sqsEvent.Records {
if len(message.Body) &gt; 0 {
// Your message processing condition here
fmt.Printf("Successfully processed message: %s\\n", message.Body)
} else {
// Message processing failed
fmt.Printf("Failed to process message %s\\n", message.MessageId)
batchItemFailures = append(batchItemFailures, map[string]interface{}{"itemIdentifier": message.MessageId})
}
}
sqsBatchResponse := map[string]interface{}{
"batchItemFailures": batchItemFailures,
}
return sqsBatchResponse, nil
}
func main() {
lambda.Start(handler)
}
`
```
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.SQSEvent;
import com.amazonaws.services.lambda.runtime.events.SQSBatchResponse;
import java.util.ArrayList;
import java.util.List;
public class ProcessSQSMessageBatch implements RequestHandler&lt;SQSEvent, SQSBatchResponse&gt; {
@Override
public SQSBatchResponse handleRequest(SQSEvent sqsEvent, Context context) {
List&lt;SQSBatchResponse.BatchItemFailure&gt; batchItemFailures = new ArrayList&lt;SQSBatchResponse.BatchItemFailure&gt;();
for (SQSEvent.SQSMessage message : sqsEvent.getRecords()) {
try {
//process your message
} catch (Exception e) {
//Add failed message identifier to the batchItemFailures list
batchItemFailures.add(new SQSBatchResponse.BatchItemFailure(message.getMessageId()));
}
}
return new SQSBatchResponse(batchItemFailures);
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using JavaScript.
```
`// Node.js 20.x Lambda runtime, AWS SDK for Javascript V3
export const handler = async (event, context) =&gt; {
const batchItemFailures = [];
for (const record of event.Records) {
try {
await processMessageAsync(record, context);
} catch (error) {
batchItemFailures.push({ itemIdentifier: record.messageId });
}
}
return { batchItemFailures };
};
async function processMessageAsync(record, context) {
if (record.body &amp;&amp; record.body.includes("error")) {
throw new Error("There is an error in the SQS Message.");
}
console.log(`Processed message: ${record.body}`);
}
`
```
Reporting SQS batch item failures with Lambda using TypeScript.
```
// SPDX-License-Identifier: Apache-2.0
import { SQSEvent, SQSBatchResponse, Context, SQSBatchItemFailure, SQSRecord } from 'aws-lambda';
export const handler = async (event: SQSEvent, context: Context): Promise&lt;SQSBatchResponse&gt; =&gt; {
const batchItemFailures: SQSBatchItemFailure[] = [];
for (const record of event.Records) {
try {
await processMessageAsync(record);
} catch (error) {
batchItemFailures.push({ itemIdentifier: record.messageId });
}
}
return {batchItemFailures: batchItemFailures};
};
async function processMessageAsync(record: SQSRecord): Promise&lt;void&gt; {
if (record.body &amp;&amp; record.body.includes("error")) {
throw new Error('There is an error in the SQS Message.');
}
console.log(`Processed message ${record.body}`);
}
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using PHP.
```
// SPDX-License-Identifier: Apache-2.0
&lt;&lt;?php
use Bref\\Context\\Context;
use Bref\\Event\\Sqs\\SqsEvent;
use Bref\\Event\\Sqs\\SqsHandler;
use Bref\\Logger\\StderrLogger;
require \_\_DIR\_\_ . '/vendor/autoload.php';
class Handler extends SqsHandler
{
private StderrLogger $logger;
public function \_\_construct(StderrLogger $logger)
{
$this-&gt;&gt;logger = $logger;
}
/\*\*
\* @throws JsonException
\* @throws \\Bref\\Event\\InvalidLambdaEvent
\*/
public function handleSqs(SqsEvent $event, Context $context): void
{
$this-&gt;logger-&gt;info("Processing SQS records");
$records = $event-&gt;getRecords();
foreach ($records as $record) {
try {
// Assuming the SQS message is in JSON format
$message = json\_decode($record-&gt;&gt;getBody(), true);
$this-&gt;&gt;logger-&gt;&gt;info(json\_encode($message));
// TODO: Implement your custom processing logic here
} catch (Exception $e) {
$this-&gt;logger-&gt;error($e-&gt;getMessage());
// failed processing the record
$this-&gt;markAsFailed($record);
}
}
$totalRecords = count($records);
$this-&gt;logger-&gt;info("Successfully processed $totalRecords SQS records");
}
}
$logger = new StderrLogger();
return new Handler($logger);
`
```
Python
**SDK for Python (Boto3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
def lambda\_handler(event, context):
if event:
batch\_item\_failures = []
sqs\_batch\_response = {}
for record in event["Records"]:
try:
print(f"Processed message: {record['body']}")
except Exception as e:
batch\_item\_failures.append({"itemIdentifier": record['messageId']})
sqs\_batch\_response["batchItemFailures"] = batch\_item\_failures
return sqs\_batch\_response
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda-with-batch-item-handling)
repository.
Reporting SQS batch item failures with Lambda using Ruby.
```
# SPDX-License-Identifier: Apache-2.0
require 'json'
def lambda\_handler(event:, context:)
if event
batch\_item\_failures = []
sqs\_batch\_response = {}
event["Records"].each do |record|
begin
# process message
rescue StandardError =&gt;&gt; e
batch\_item\_failures &lt;&lt;&lt;&lt; {"itemIdentifier" =&gt;&gt; record['messageId']}
end
end
sqs\_batch\_response["batchItemFailures"] = batch\_item\_failures
return sqs\_batch\_response
end
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/lambda-function-sqs-report-batch-item-failures)
repository.
Reporting SQS batch item failures with Lambda using Rust.
```
// SPDX-License-Identifier: Apache-2.0
use aws\_lambda\_events::{
event::sqs::{SqsBatchResponse, SqsEvent},
sqs::{BatchItemFailure, SqsMessage},
};
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
async fn process\_record(\_: &amp;&amp;SqsMessage) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
Err(Error::from("Error processing message"))
}
async fn function\_handler(event: LambdaEvent&lt;&lt;SqsEvent&gt;&gt;) -&gt;&gt; Result&lt;&lt;SqsBatchResponse, Error&gt;&gt; {
let mut batch\_item\_failures = Vec::new();
for record in event.payload.records {
match process\_record(&amp;&amp;record).await {
Ok(\_) =&gt;&gt; (),
Err(\_) =&gt;&gt; batch\_item\_failures.push(BatchItemFailure {
item\_identifier: record.message\_id.unwrap(),
}),
}
}
Ok(SqsBatchResponse {
batch\_item\_failures,
})
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
run(service\_fn(function\_handler)).await
}
`
```
If the failed events do not return to the queue, see [How do I troubleshoot Lambda function SQS ReportBatchItemFailures?](https://aws.amazon.com/premiumsupport/knowledge-center/lambda-sqs-report-batch-item-failures/) in the AWS Knowledge Center.
### Success and failure conditions
Lambda treats a batch as a complete success if your function returns any of the following:
* An empty `batchItemFailures` list
* A null `batchItemFailures` list
* An empty `EventResponse`
* A null `EventResponse`
Lambda treats a batch as a complete failure if your function returns any of the following:
* An invalid JSON response
* An empty string `itemIdentifier`
* A null `itemIdentifier`
* An `itemIdentifier` with a bad key name
* An `itemIdentifier` value with a message ID that doesn't exist
### CloudWatch metrics
To determine whether your function is correctly reporting batch item failures, you can monitor the
`NumberOfMessagesDeleted` and `ApproximateAgeOfOldestMessage` Amazon SQS metrics in
Amazon CloudWatch.
* `NumberOfMessagesDeleted` tracks the number of messages removed from your queue. If this
drops to 0, this is a sign that your function response is not correctly returning failed messages.
* `ApproximateAgeOfOldestMessage` tracks how long the oldest message has stayed in your queue.
A sharp increase in this metric can indicate that your function is not correctly returning failed
messages.
### Using Powertools for AWS Lambda batch processor
The batch processor utility from Powertools for AWS Lambda automatically handles partial batch response logic, reducing the complexity of
implementing batch failure reporting. Here are examples using the batch processor:
**Python**
###### Note
For complete examples and setup instructions, see the [batch processor documentation](https://docs.powertools.aws.dev/lambda/python/latest/utilities/batch/).
Processing Amazon SQS messages with AWS Lambda batch processor.
```
`import json
from aws\_lambda\_powertools import Logger
from aws\_lambda\_powertools.utilities.batch import BatchProcessor, EventType, process\_partial\_response
from aws\_lambda\_powertools.utilities.data\_classes import SQSEvent
from aws\_lambda\_powertools.utilities.typing import LambdaContext
processor = BatchProcessor(event\_type=EventType.SQS)
logger = Logger()
def record\_handler(record):
logger.info(record)
# Raise an exception to mark this record as failed
def lambda\_handler(event, context: LambdaContext):
return process\_partial\_response(
event=event,
record\_handler=record\_handler,
processor=processor,
context=context
)`
```
**TypeScript**
###### Note
For complete examples and setup instructions,
see the [batch processor documentation](https://docs.aws.amazon.com/powertools/typescript/latest/features/batch/).
Processing Amazon SQS messages with AWS Lambda batch processor.
```
`import { BatchProcessor, EventType, processPartialResponse } from '@aws-lambda-powertools/batch';
import { Logger } from '@aws-lambda-powertools/logger';
import type { SQSEvent, Context } from 'aws-lambda';
const processor = new BatchProcessor(EventType.SQS);
const logger = new Logger();
const recordHandler = async (record: any): Promise&lt;void&gt; =&gt; {
logger.info('Processing record', { record });
// Your business logic here
// Throw an error to mark this record as failed
};
export const handler = async (event: SQSEvent, context: Context) =&gt; {
return processPartialResponse(event, recordHandler, processor, {
context,
});
};`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Scaling behavior
Parameters
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.