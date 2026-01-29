---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-ddb-batchfailurereporting.html
title: Configuring partial batch response with DynamoDB and Lambda
word_count: 2130
filtered: true
elements_removed: 0
density_score: 0.84
---

Configuring partial batch response with DynamoDB and Lambda - AWS Lambda
Configuring partial batch response with DynamoDB and Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-ddb-batchfailurereporting)
[Report syntax](#streams-batchfailurereporting-syntax)[Success and failure conditions](#streams-batchfailurereporting-conditions)[Bisecting a batch](#streams-batchfailurereporting-bisect)[Using Powertools for AWS Lambda batch processor](#services-ddb-batchfailurereporting-powertools)
# Configuring partial batch response with DynamoDB and Lambda
When consuming and processing streaming data from an event source, by default Lambda checkpoints to the highest
sequence number of a batch only when the batch is a complete success. Lambda treats all other results as a complete
failure and retries processing the batch up to the retry limit. To allow for partial successes while processing
batches from a stream, turn on `ReportBatchItemFailures`. Allowing partial successes can help to reduce
the number of retries on a record, though it doesn’t entirely prevent the possibility of retries in a successful record.
To turn on `ReportBatchItemFailures`, include the enum value
`ReportBatchItemFailures` in the [FunctionResponseTypes](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html#lambda-CreateEventSourceMapping-request-FunctionResponseTypes) list. This list indicates
which response types are enabled for your function. You can configure this list when you [create](https://docs.aws.amazon.com/lambda/latest/api/API_CreateEventSourceMapping.html) or [update](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateEventSourceMapping.html) an event source mapping.
###### Note
Even when your function code returns partial batch failure responses, these responses will not be processed by Lambda unless the
`ReportBatchItemFailures` feature is explicitly turned on for your event source mapping.
## Report syntax
When configuring reporting on batch item failures, the `StreamsEventResponse` class is returned with a
list of batch item failures. You can use a `StreamsEventResponse` object to return the sequence number
of the first failed record in the batch. You can also create your own custom class using the correct response
syntax. The following JSON structure shows the required response syntax:
```
`{
"batchItemFailures": [
{
"itemIdentifier": "&lt;SequenceNumber&gt;"
}
]
}`
```
###### Note
If the `batchItemFailures` array contains multiple items, Lambda uses the record with the lowest
sequence number as the checkpoint. Lambda then retries all records starting from that checkpoint.
## Success and failure conditions
Lambda treats a batch as a complete success if you return any of the following:
* An empty `batchItemFailure` list
* A null `batchItemFailure` list
* An empty `EventResponse`
* A null `EventResponse`
Lambda treats a batch as a complete failure if you return any of the following:
* An empty string `itemIdentifier`
* A null `itemIdentifier`
* An `itemIdentifier` with a bad key name
Lambda retries failures based on your retry strategy.
## Bisecting a batch
If your invocation fails and `BisectBatchOnFunctionError` is turned on, the batch is bisected
regardless of your `ReportBatchItemFailures` setting.
When a partial batch success response is received and both `BisectBatchOnFunctionError` and
`ReportBatchItemFailures` are turned on, the batch is bisected at the returned sequence number and
Lambda retries only the remaining records.
To simplify the implementation of partial batch response logic, consider using the [Batch Processor utility](https://docs.powertools.aws.dev/lambda/python/latest/utilities/batch/)
from Powertools for AWS Lambda, which automatically handles these complexities for you.
Here are some examples of function code that return the list of failed message IDs in the batch:
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using System.Text.Json;
using System.Text;
using Amazon.Lambda.Core;
using Amazon.Lambda.DynamoDBEvents;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace AWSLambda\_DDB;
public class Function
{
public StreamsEventResponse FunctionHandler(DynamoDBEvent dynamoEvent, ILambdaContext context)
{
context.Logger.LogInformation($"Beginning to process {dynamoEvent.Records.Count} records...");
List&lt;StreamsEventResponse.BatchItemFailure&gt; batchItemFailures = new List&lt;StreamsEventResponse.BatchItemFailure&gt;();
StreamsEventResponse streamsEventResponse = new StreamsEventResponse();
foreach (var record in dynamoEvent.Records)
{
try
{
var sequenceNumber = record.Dynamodb.SequenceNumber;
context.Logger.LogInformation(sequenceNumber);
}
catch (Exception ex)
{
context.Logger.LogError(ex.Message);
batchItemFailures.Add(new StreamsEventResponse.BatchItemFailure() { ItemIdentifier = record.Dynamodb.SequenceNumber });
}
}
if (batchItemFailures.Count &gt; 0)
{
streamsEventResponse.BatchItemFailures = batchItemFailures;
}
context.Logger.LogInformation("Stream processing complete.");
return streamsEventResponse;
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package main
import (
"context"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
type BatchItemFailure struct {
ItemIdentifier string `json:"ItemIdentifier"`
}
type BatchResult struct {
BatchItemFailures []BatchItemFailure `json:"BatchItemFailures"`
}
func HandleRequest(ctx context.Context, event events.DynamoDBEvent) (\*BatchResult, error) {
var batchItemFailures []BatchItemFailure
curRecordSequenceNumber := ""
for \_, record := range event.Records {
// Process your record
curRecordSequenceNumber = record.Change.SequenceNumber
}
if curRecordSequenceNumber != "" {
batchItemFailures = append(batchItemFailures, BatchItemFailure{ItemIdentifier: curRecordSequenceNumber})
}
batchResult := BatchResult{
BatchItemFailures: batchItemFailures,
}
return &amp;batchResult, nil
}
func main() {
lambda.Start(HandleRequest)
}
`
```
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.DynamodbEvent;
import com.amazonaws.services.lambda.runtime.events.StreamsEventResponse;
import com.amazonaws.services.lambda.runtime.events.models.dynamodb.StreamRecord;
import java.util.ArrayList;
import java.util.List;
public class ProcessDynamodbRecords implements RequestHandler&lt;DynamodbEvent, StreamsEventResponse&gt; {
@Override
public StreamsEventResponse handleRequest(DynamodbEvent input, Context context) {
List&lt;StreamsEventResponse.BatchItemFailure&gt; batchItemFailures = new ArrayList&lt;&gt;();
String curRecordSequenceNumber = "";
for (DynamodbEvent.DynamodbStreamRecord dynamodbStreamRecord : input.getRecords()) {
try {
//Process your record
StreamRecord dynamodbRecord = dynamodbStreamRecord.getDynamodb();
curRecordSequenceNumber = dynamodbRecord.getSequenceNumber();
} catch (Exception e) {
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
batchItemFailures.add(new StreamsEventResponse.BatchItemFailure(curRecordSequenceNumber));
return new StreamsEventResponse(batchItemFailures);
}
}
return new StreamsEventResponse();
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using JavaScript.
```
`export const handler = async (event) =&gt; {
const records = event.Records;
let curRecordSequenceNumber = "";
for (const record of records) {
try {
// Process your record
curRecordSequenceNumber = record.dynamodb.SequenceNumber;
} catch (e) {
// Return failed record's sequence number
return { batchItemFailures: [{ itemIdentifier: curRecordSequenceNumber }] };
}
}
return { batchItemFailures: [] };
};
`
```
Reporting DynamoDB batch item failures with Lambda using TypeScript.
```
`import {
DynamoDBBatchResponse,
DynamoDBBatchItemFailure,
DynamoDBStreamEvent,
} from "aws-lambda";
export const handler = async (
event: DynamoDBStreamEvent
): Promise&lt;DynamoDBBatchResponse&gt; =&gt; {
const batchItemFailures: DynamoDBBatchItemFailure[] = [];
let curRecordSequenceNumber;
for (const record of event.Records) {
curRecordSequenceNumber = record.dynamodb?.SequenceNumber;
if (curRecordSequenceNumber) {
batchItemFailures.push({
itemIdentifier: curRecordSequenceNumber,
});
}
}
return { batchItemFailures: batchItemFailures };
};
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using PHP.
```
`&lt;&lt;?php
# using bref/bref and bref/logger for simplicity
use Bref\\Context\\Context;
use Bref\\Event\\DynamoDb\\DynamoDbEvent;
use Bref\\Event\\Handler as StdHandler;
use Bref\\Logger\\StderrLogger;
require \_\_DIR\_\_ . '/vendor/autoload.php';
class Handler implements StdHandler
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
public function handle(mixed $event, Context $context): array
{
$dynamoDbEvent = new DynamoDbEvent($event);
$this-&gt;logger-&gt;info("Processing records");
$records = $dynamoDbEvent-&gt;getRecords();
$failedRecords = [];
foreach ($records as $record) {
try {
$data = $record-&gt;&gt;getData();
$this-&gt;&gt;logger-&gt;&gt;info(json\_encode($data));
// TODO: Do interesting work based on the new data
} catch (Exception $e) {
$this-&gt;&gt;logger-&gt;&gt;error($e-&gt;&gt;getMessage());
// failed processing the record
$failedRecords[] = $record-&gt;&gt;getSequenceNumber();
}
}
$totalRecords = count($records);
$this-&gt;&gt;logger-&gt;&gt;info("Successfully processed $totalRecords records");
// change format for the response
$failures = array\_map(
fn(string $sequenceNumber) =&gt;&gt; ['itemIdentifier' =&gt;&gt; $sequenceNumber],
$failedRecords
);
return [
'batchItemFailures' =&gt;&gt; $failures
];
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
def handler(event, context):
records = event.get("Records")
curRecordSequenceNumber = ""
for record in records:
try:
# Process your record
curRecordSequenceNumber = record["dynamodb"]["SequenceNumber"]
except Exception as e:
# Return failed record's sequence number
return {"batchItemFailures":[{"itemIdentifier": curRecordSequenceNumber}]}
return {"batchItemFailures":[]}
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using Ruby.
```
`def lambda\_handler(event:, context:)
records = event["Records"]
cur\_record\_sequence\_number = ""
records.each do |record|
begin
# Process your record
cur\_record\_sequence\_number = record["dynamodb"]["SequenceNumber"]
rescue StandardError =&gt;&gt; e
# Return failed record's sequence number
return {"batchItemFailures" =&gt; [{"itemIdentifier" =&gt;&gt; cur\_record\_sequence\_number}]}
end
end
{"batchItemFailures" =&gt; []}
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-ddb-to-lambda-with-batch-item-handling)
repository.
Reporting DynamoDB batch item failures with Lambda using Rust.
```
`use aws\_lambda\_events::{
event::dynamodb::{Event, EventRecord, StreamRecord},
streams::{DynamoDbBatchItemFailure, DynamoDbEventResponse},
};
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
/// Process the stream record
fn process\_record(record: &amp;&amp;EventRecord) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
let stream\_record: &amp;&amp;StreamRecord = &amp;&amp;record.change;
// process your stream record here...
tracing::info!("Data: {:?}", stream\_record);
Ok(())
}
/// Main Lambda handler here...
async fn function\_handler(event: LambdaEvent&lt;&lt;Event&gt;&gt;) -&gt;&gt; Result&lt;&lt;DynamoDbEventResponse, Error&gt;&gt; {
let mut response = DynamoDbEventResponse {
batch\_item\_failures: vec![],
};
let records = &amp;&amp;event.payload.records;
if records.is\_empty() {
tracing::info!("No records found. Exiting.");
return Ok(response);
}
for record in records {
tracing::info!("EventId: {}", record.event\_id);
// Couldn't find a sequence number
if record.change.sequence\_number.is\_none() {
response.batch\_item\_failures.push(DynamoDbBatchItemFailure {
item\_identifier: Some("".to\_string()),
});
return Ok(response);
}
// Process your record here...
if process\_record(record).is\_err() {
response.batch\_item\_failures.push(DynamoDbBatchItemFailure {
item\_identifier: record.change.sequence\_number.clone(),
});
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
return Ok(response);
}
}
tracing::info!("Successfully processed {} record(s)", records.len());
Ok(response)
}
#[tokio::main]
async fn main() -&gt; Result&lt;(), Error&gt; {
tracing\_subscriber::fmt()
.with\_max\_level(tracing::Level::INFO)
// disable printing the name of the module in every log line.
.with\_target(false)
// disabling time is handy because CloudWatch will add the ingestion time.
.without\_time()
.init();
run(service\_fn(function\_handler)).await
}
`
```
## Using Powertools for AWS Lambda batch processor
The batch processor utility from Powertools for AWS Lambda automatically handles partial batch response logic, reducing the complexity of
implementing batch failure reporting. Here are examples using the batch processor:
**Python**
###### Note
For complete examples and setup instructions, see the [batch processor documentation](https://docs.powertools.aws.dev/lambda/python/latest/utilities/batch/).
Processing DynamoDB stream records with AWS Lambda batch processor.
```
`import json
from aws\_lambda\_powertools import Logger
from aws\_lambda\_powertools.utilities.batch import BatchProcessor, EventType, process\_partial\_response
from aws\_lambda\_powertools.utilities.data\_classes import DynamoDBStreamEvent
from aws\_lambda\_powertools.utilities.typing import LambdaContext
processor = BatchProcessor(event\_type=EventType.DynamoDBStreams)
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
Processing DynamoDB stream records with AWS Lambda batch processor.
```
`import { BatchProcessor, EventType, processPartialResponse } from '@aws-lambda-powertools/batch';
import { Logger } from '@aws-lambda-powertools/logger';
import type { DynamoDBStreamEvent, Context } from 'aws-lambda';
const processor = new BatchProcessor(EventType.DynamoDBStreams);
const logger = new Logger();
const recordHandler = async (record: any): Promise&lt;void&gt; =&gt; {
logger.info('Processing record', { record });
// Your business logic here
// Throw an error to mark this record as failed
};
export const handler = async (event: DynamoDBStreamEvent, context: Context) =&gt; {
return processPartialResponse(event, recordHandler, processor, {
context,
});
};`
```
**Java**
###### Note
For complete examples and setup instructions, see the [batch processor documentation](https://docs.powertools.aws.dev/lambda/java/latest/utilities/batch/).
Processing DynamoDB stream records with AWS Lambda batch processor.
```
`import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.DynamodbEvent;
import com.amazonaws.services.lambda.runtime.events.StreamsEventResponse;
import software.amazon.lambda.powertools.batch.BatchMessageHandlerBuilder;
import software.amazon.lambda.powertools.batch.handler.BatchMessageHandler;
public class DynamoDBStreamBatchHandler implements RequestHandler&lt;DynamodbEvent, StreamsEventResponse&gt; {
private final BatchMessageHandler&lt;DynamodbEvent, StreamsEventResponse&gt; handler;
public DynamoDBStreamBatchHandler() {
handler = new BatchMessageHandlerBuilder()
.withDynamoDbBatchHandler()
.buildWithRawMessageHandler(this::processMessage);
}
@Override
public StreamsEventResponse handleRequest(DynamodbEvent ddbEvent, Context context) {
return handler.processBatch(ddbEvent, context);
}
private void processMessage(DynamodbEvent.DynamodbStreamRecord dynamodbStreamRecord, Context context) {
// Process the change record
}
}`
```
**.NET**
###### Note
For complete examples and setup instructions, see the [batch processor documentation](https://docs.aws.amazon.com/powertools/dotnet/utilities/batch-processing/).
Processing DynamoDB stream records with AWS Lambda batch processor.
```
`using System;
using System.Threading;
using System.Threading.Tasks;
using Amazon.Lambda.Core;
using Amazon.Lambda.DynamoDBEvents;
using Amazon.Lambda.Serialization.SystemTextJson;
using AWS.Lambda.Powertools.BatchProcessing;
[assembly: LambdaSerializer(typeof(DefaultLambdaJsonSerializer))]
namespace HelloWorld;
public class Customer
{
public string? CustomerId { get; set; }
public string? Name { get; set; }
public string? Email { get; set; }
public DateTime CreatedAt { get; set; }
}
internal class TypedDynamoDbRecordHandler : ITypedRecordHandler&lt;Customer&gt;
{
public async Task&lt;RecordHandlerResult&gt; HandleAsync(Customer customer, CancellationToken cancellationToken)
{
if (string.IsNullOrEmpty(customer.Email))
{
throw new ArgumentException("Customer email is required");
}
return await Task.FromResult(RecordHandlerResult.None);
}
}
public class Function
{
[BatchProcessor(TypedRecordHandler = typeof(TypedDynamoDbRecordHandler))]
public BatchItemFailuresResponse HandlerUsingTypedAttribute(DynamoDBEvent \_)
{
return TypedDynamoDbStreamBatchProcessor.Result.BatchItemFailuresResponse;
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create mapping
Error handling
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.