---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_Kinesis_Lambda_batch_item_failures_section.html
title: Reporting batch item failures for Lambda functions with a Kinesis trigger
word_count: 1701
filtered: true
elements_removed: 0
density_score: 0.83
---

Reporting batch item failures for Lambda functions with a Kinesis trigger - AWS Lambda
Reporting batch item failures for Lambda functions with a Kinesis trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_Kinesis_Lambda_batch_item_failures_section)
# Reporting batch item failures for Lambda functions with a Kinesis trigger
The following code examples show how to implement partial batch response for Lambda functions that receive events from a Kinesis stream. The function reports the batch item failures in the response, signaling to Lambda to retry those messages later.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using System.Text;
using System.Text.Json.Serialization;
using Amazon.Lambda.Core;
using Amazon.Lambda.KinesisEvents;
using AWS.Lambda.Powertools.Logging;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace KinesisIntegration;
public class Function
{
// Powertools Logger requires an environment variables against your function
// POWERTOOLS\_SERVICE\_NAME
[Logging(LogEvent = true)]
public async Task&lt;&lt;StreamsEventResponse&gt;&gt; FunctionHandler(KinesisEvent evnt, ILambdaContext context)
{
if (evnt.Records.Count == 0)
{
Logger.LogInformation("Empty Kinesis Event received");
return new StreamsEventResponse();
}
foreach (var record in evnt.Records)
{
try
{
Logger.LogInformation($"Processed Event with EventId: {record.EventId}");
string data = await GetRecordDataAsync(record.Kinesis, context);
Logger.LogInformation($"Data: {data}");
// TODO: Do interesting work based on the new data
}
catch (Exception ex)
{
Logger.LogError($"An error occurred {ex.Message}");
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
return new StreamsEventResponse
{
BatchItemFailures = new List&lt;StreamsEventResponse.BatchItemFailure&gt;
{
new StreamsEventResponse.BatchItemFailure { ItemIdentifier = record.Kinesis.SequenceNumber }
}
};
}
}
Logger.LogInformation($"Successfully processed {evnt.Records.Count} records.");
return new StreamsEventResponse();
}
private async Task&lt;string&gt; GetRecordDataAsync(KinesisEvent.Record record, ILambdaContext context)
{
byte[] bytes = record.Data.ToArray();
string data = Encoding.UTF8.GetString(bytes);
await Task.CompletedTask; //Placeholder for actual async work
return data;
}
}
public class StreamsEventResponse
{
[JsonPropertyName("batchItemFailures")]
public IList&lt;BatchItemFailure&gt; BatchItemFailures { get; set; }
public class BatchItemFailure
{
[JsonPropertyName("itemIdentifier")]
public string ItemIdentifier { get; set; }
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package main
import (
"context"
"fmt"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
func handler(ctx context.Context, kinesisEvent events.KinesisEvent) (map[string]interface{}, error) {
batchItemFailures := []map[string]interface{}{}
for \_, record := range kinesisEvent.Records {
curRecordSequenceNumber := ""
// Process your record
if /\* Your record processing condition here \*/ {
curRecordSequenceNumber = record.Kinesis.SequenceNumber
}
// Add a condition to check if the record processing failed
if curRecordSequenceNumber != "" {
batchItemFailures = append(batchItemFailures, map[string]interface{}{"itemIdentifier": curRecordSequenceNumber})
}
}
kinesisBatchResponse := map[string]interface{}{
"batchItemFailures": batchItemFailures,
}
return kinesisBatchResponse, nil
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.KinesisEvent;
import com.amazonaws.services.lambda.runtime.events.StreamsEventResponse;
import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;
public class ProcessKinesisRecords implements RequestHandler&lt;KinesisEvent, StreamsEventResponse&gt; {
@Override
public StreamsEventResponse handleRequest(KinesisEvent input, Context context) {
List&lt;StreamsEventResponse.BatchItemFailure&gt; batchItemFailures = new ArrayList&lt;&gt;();
String curRecordSequenceNumber = "";
for (KinesisEvent.KinesisEventRecord kinesisEventRecord : input.getRecords()) {
try {
//Process your record
KinesisEvent.Record kinesisRecord = kinesisEventRecord.getKinesis();
curRecordSequenceNumber = kinesisRecord.getSequenceNumber();
} catch (Exception e) {
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
batchItemFailures.add(new StreamsEventResponse.BatchItemFailure(curRecordSequenceNumber));
return new StreamsEventResponse(batchItemFailures);
}
}
return new StreamsEventResponse(batchItemFailures);
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/blob/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using Javascript.
```
// SPDX-License-Identifier: Apache-2.0
exports.handler = async (event, context) =&gt; {
for (const record of event.Records) {
try {
console.log(`Processed Kinesis Event - EventID: ${record.eventID}`);
const recordData = await getRecordDataAsync(record.kinesis);
console.log(`Record Data: ${recordData}`);
// TODO: Do interesting work based on the new data
} catch (err) {
console.error(`An error occurred ${err}`);
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
return {
batchItemFailures: [{ itemIdentifier: record.kinesis.sequenceNumber }],
};
}
}
console.log(`Successfully processed ${event.Records.length} records.`);
return { batchItemFailures: [] };
};
async function getRecordDataAsync(payload) {
var data = Buffer.from(payload.data, "base64").toString("utf-8");
await Promise.resolve(1); //Placeholder for actual async work
return data;
}
`
```
Reporting Kinesis batch item failures with Lambda using TypeScript.
```
// SPDX-License-Identifier: Apache-2.0
import {
KinesisStreamEvent,
Context,
KinesisStreamHandler,
KinesisStreamRecordPayload,
KinesisStreamBatchResponse,
} from "aws-lambda";
import { Buffer } from "buffer";
import { Logger } from "@aws-lambda-powertools/logger";
const logger = new Logger({
logLevel: "INFO",
serviceName: "kinesis-stream-handler-sample",
});
export const functionHandler: KinesisStreamHandler = async (
event: KinesisStreamEvent,
context: Context
): Promise&lt;KinesisStreamBatchResponse&gt; =&gt; {
for (const record of event.Records) {
try {
logger.info(`Processed Kinesis Event - EventID: ${record.eventID}`);
const recordData = await getRecordDataAsync(record.kinesis);
logger.info(`Record Data: ${recordData}`);
// TODO: Do interesting work based on the new data
} catch (err) {
logger.error(`An error occurred ${err}`);
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
return {
batchItemFailures: [{ itemIdentifier: record.kinesis.sequenceNumber }],
};
}
}
logger.info(`Successfully processed ${event.Records.length} records.`);
return { batchItemFailures: [] };
};
async function getRecordDataAsync(
payload: KinesisStreamRecordPayload
): Promise&lt;string&gt; {
var data = Buffer.from(payload.data, "base64").toString("utf-8");
await Promise.resolve(1); //Placeholder for actual async work
return data;
}
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using PHP.
```
// SPDX-License-Identifier: Apache-2.0
&lt;&lt;?php
# using bref/bref and bref/logger for simplicity
use Bref\\Context\\Context;
use Bref\\Event\\Kinesis\\KinesisEvent;
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
$kinesisEvent = new KinesisEvent($event);
$this-&gt;logger-&gt;info("Processing records");
$records = $kinesisEvent-&gt;getRecords();
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
def handler(event, context):
records = event.get("Records")
curRecordSequenceNumber = ""
for record in records:
try:
# Process your record
curRecordSequenceNumber = record["kinesis"]["sequenceNumber"]
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using Ruby.
```
# SPDX-License-Identifier: Apache-2.0
require 'aws-sdk'
def lambda\_handler(event:, context:)
batch\_item\_failures = []
event['Records'].each do |record|
begin
puts "Processed Kinesis Event - EventID: #{record['eventID']}"
record\_data = get\_record\_data\_async(record['kinesis'])
puts "Record Data: #{record\_data}"
# TODO: Do interesting work based on the new data
rescue StandardError =&gt;&gt; err
puts "An error occurred #{err}"
# Since we are working with streams, we can return the failed item immediately.
# Lambda will immediately begin to retry processing from this failed item onwards.
return { batchItemFailures: [{ itemIdentifier: record['kinesis']['sequenceNumber'] }] }
end
end
puts "Successfully processed #{event['Records'].length} records."
{ batchItemFailures: batch\_item\_failures }
end
def get\_record\_data\_async(payload)
data = Base64.decode64(payload['data']).force\_encoding('utf-8')
# Placeholder for actual async work
sleep(1)
data
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda-with-batch-item-handling)
repository.
Reporting Kinesis batch item failures with Lambda using Rust.
```
// SPDX-License-Identifier: Apache-2.0
use aws\_lambda\_events::{
event::kinesis::KinesisEvent,
kinesis::KinesisEventRecord,
streams::{KinesisBatchItemFailure, KinesisEventResponse},
};
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
async fn function\_handler(event: LambdaEvent&lt;&lt;KinesisEvent&gt;&gt;) -&gt;&gt; Result&lt;&lt;KinesisEventResponse, Error&gt;&gt; {
let mut response = KinesisEventResponse {
batch\_item\_failures: vec![],
};
if event.payload.records.is\_empty() {
tracing::info!("No records found. Exiting.");
return Ok(response);
}
for record in &amp;event.payload.records {
tracing::info!(
"EventId: {}",
record.event\_id.as\_deref().unwrap\_or\_default()
);
let record\_processing\_result = process\_record(record);
if record\_processing\_result.is\_err() {
response.batch\_item\_failures.push(KinesisBatchItemFailure {
item\_identifier: record.kinesis.sequence\_number.clone(),
});
/\* Since we are working with streams, we can return the failed item immediately.
Lambda will immediately begin to retry processing from this failed item onwards. \*/
return Ok(response);
}
}
tracing::info!(
"Successfully processed {} records",
event.payload.records.len()
);
Ok(response)
}
fn process\_record(record: &amp;&amp;KinesisEventRecord) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
let record\_data = std::str::from\_utf8(record.kinesis.data.as\_slice());
if let Some(err) = record\_data.err() {
tracing::error!("Error: {}", err);
return Err(Error::from(err));
}
let record\_data = record\_data.unwrap\_or\_default();
// do something interesting with the data
tracing::info!("Data: {}", record\_data);
Ok(())
}
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
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
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke a Lambda function from an Amazon SQS trigger
Reporting batch item failures for Lambda functions with a DynamoDB trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.