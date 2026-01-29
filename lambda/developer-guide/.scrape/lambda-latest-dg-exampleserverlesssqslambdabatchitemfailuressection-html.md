---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_SQS_Lambda_batch_item_failures_section.html
title: Reporting batch item failures for Lambda functions with an Amazon SQS trigger
word_count: 1214
filtered: true
elements_removed: 0
density_score: 0.84
---

Reporting batch item failures for Lambda functions with an Amazon SQS trigger - AWS Lambda
Reporting batch item failures for Lambda functions with an Amazon SQS trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_SQS_Lambda_batch_item_failures_section)
# Reporting batch item failures for Lambda functions with an Amazon SQS trigger
The following code examples show how to implement partial batch response for Lambda functions that receive events from an SQS queue. The function reports the batch item failures in the response, signaling to Lambda to retry those messages later.
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
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Reporting batch item failures for Lambda functions with a DynamoDB trigger
AWS community contributions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.