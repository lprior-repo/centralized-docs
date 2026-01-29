---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_Kinesis_Lambda_section.html
title: Invoke a Lambda function from a Kinesis trigger
word_count: 1374
filtered: true
elements_removed: 0
density_score: 0.83
---

Invoke a Lambda function from a Kinesis trigger - AWS Lambda
Invoke a Lambda function from a Kinesis trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_Kinesis_Lambda_section)
# Invoke a Lambda function from a Kinesis trigger
The following code examples show how to implement a Lambda function that receives an event triggered by receiving records from a Kinesis stream. The function retrieves the Kinesis payload, decodes from Base64, and logs the record contents.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming a Kinesis event with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using System.Text;
using Amazon.Lambda.Core;
using Amazon.Lambda.KinesisEvents;
using AWS.Lambda.Powertools.Logging;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace KinesisIntegrationSampleCode;
public class Function
{
// Powertools Logger requires an environment variables against your function
// POWERTOOLS\_SERVICE\_NAME
[Logging(LogEvent = true)]
public async Task FunctionHandler(KinesisEvent evnt, ILambdaContext context)
{
if (evnt.Records.Count == 0)
{
Logger.LogInformation("Empty Kinesis Event received");
return;
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
throw;
}
}
Logger.LogInformation($"Successfully processed {evnt.Records.Count} records.");
}
private async Task&lt;string&gt; GetRecordDataAsync(KinesisEvent.Record record, ILambdaContext context)
{
byte[] bytes = record.Data.ToArray();
string data = Encoding.UTF8.GetString(bytes);
await Task.CompletedTask; //Placeholder for actual async work
return data;
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming a Kinesis event with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package main
import (
"context"
"log"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
func handler(ctx context.Context, kinesisEvent events.KinesisEvent) error {
if len(kinesisEvent.Records) == 0 {
log.Printf("empty Kinesis event received")
return nil
}
for \_, record := range kinesisEvent.Records {
log.Printf("processed Kinesis event with EventId: %v", record.EventID)
recordDataBytes := record.Kinesis.Data
recordDataText := string(recordDataBytes)
log.Printf("record data: %v", recordDataText)
// TODO: Do interesting work based on the new data
}
log.Printf("successfully processed %v records", len(kinesisEvent.Records))
return nil
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming a Kinesis event with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
package example;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.LambdaLogger;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.KinesisEvent;
public class Handler implements RequestHandler&lt;KinesisEvent, Void&gt; {
@Override
public Void handleRequest(final KinesisEvent event, final Context context) {
LambdaLogger logger = context.getLogger();
if (event.getRecords().isEmpty()) {
logger.log("Empty Kinesis Event received");
return null;
}
for (KinesisEvent.KinesisEventRecord record : event.getRecords()) {
try {
logger.log("Processed Event with EventId: "+record.getEventID());
String data = new String(record.getKinesis().getData().array());
logger.log("Data:"+ data);
// TODO: Do interesting work based on the new data
}
catch (Exception ex) {
logger.log("An error occurred:"+ex.getMessage());
throw ex;
}
}
logger.log("Successfully processed:"+event.getRecords().size()+" records");
return null;
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/blob/main/integration-kinesis-to-lambda)
repository.
Consuming a Kinesis event with Lambda using JavaScript.
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
throw err;
}
}
console.log(`Successfully processed ${event.Records.length} records.`);
};
async function getRecordDataAsync(payload) {
var data = Buffer.from(payload.data, "base64").toString("utf-8");
await Promise.resolve(1); //Placeholder for actual async work
return data;
}
`
```
Consuming a Kinesis event with Lambda using TypeScript.
```
// SPDX-License-Identifier: Apache-2.0
import {
KinesisStreamEvent,
Context,
KinesisStreamHandler,
KinesisStreamRecordPayload,
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
): Promise&lt;void&gt; =&gt; {
for (const record of event.Records) {
try {
logger.info(`Processed Kinesis Event - EventID: ${record.eventID}`);
const recordData = await getRecordDataAsync(record.kinesis);
logger.info(`Record Data: ${recordData}`);
// TODO: Do interesting work based on the new data
} catch (err) {
logger.error(`An error occurred ${err}`);
throw err;
}
logger.info(`Successfully processed ${event.Records.length} records.`);
}
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming an Kinesis event with Lambda using PHP.
```
// SPDX-License-Identifier: Apache-2.0
&lt;&lt;?php
# using bref/bref and bref/logger for simplicity
use Bref\\Context\\Context;
use Bref\\Event\\Kinesis\\KinesisEvent;
use Bref\\Event\\Kinesis\\KinesisHandler;
use Bref\\Logger\\StderrLogger;
require \_\_DIR\_\_ . '/vendor/autoload.php';
class Handler extends KinesisHandler
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
public function handleKinesis(KinesisEvent $event, Context $context): void
{
$this-&gt;logger-&gt;info("Processing records");
$records = $event-&gt;getRecords();
foreach ($records as $record) {
$data = $record-&gt;&gt;getData();
$this-&gt;&gt;logger-&gt;&gt;info(json\_encode($data));
// TODO: Do interesting work based on the new data
// Any exception thrown will be logged and the invocation will be marked as failed
}
$totalRecords = count($records);
$this-&gt;&gt;logger-&gt;&gt;info("Successfully processed $totalRecords records");
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming a Kinesis event with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
import base64
def lambda\_handler(event, context):
for record in event['Records']:
try:
print(f"Processed Kinesis Event - EventID: {record['eventID']}")
record\_data = base64.b64decode(record['kinesis']['data']).decode('utf-8')
print(f"Record Data: {record\_data}")
# TODO: Do interesting work based on the new data
except Exception as e:
print(f"An error occurred {e}")
raise e
print(f"Successfully processed {len(event['Records'])} records.")
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming an Kinesis event with Lambda using Ruby.
```
# SPDX-License-Identifier: Apache-2.0
require 'aws-sdk'
def lambda\_handler(event:, context:)
event['Records'].each do |record|
begin
puts "Processed Kinesis Event - EventID: #{record['eventID']}"
record\_data = get\_record\_data\_async(record['kinesis'])
puts "Record Data: #{record\_data}"
# TODO: Do interesting work based on the new data
rescue =&gt;&gt; err
$stderr.puts "An error occurred #{err}"
raise err
end
end
puts "Successfully processed #{event['Records'].length} records."
end
def get\_record\_data\_async(payload)
data = Base64.decode64(payload['data']).force\_encoding('UTF-8')
# You can use Ruby's asynchronous programming tools like async/await or fibers here.
return data
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-kinesis-to-lambda)
repository.
Consuming an Kinesis event with Lambda using Rust.
```
// SPDX-License-Identifier: Apache-2.0
use aws\_lambda\_events::event::kinesis::KinesisEvent;
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
async fn function\_handler(event: LambdaEvent&lt;&lt;KinesisEvent&gt;&gt;) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
if event.payload.records.is\_empty() {
tracing::info!("No records found. Exiting.");
return Ok(());
}
event.payload.records.iter().for\_each(|record| {
tracing::info!("EventId: {}",record.event\_id.as\_deref().unwrap\_or\_default());
let record\_data = std::str::from\_utf8(&amp;&amp;record.kinesis.data);
match record\_data {
Ok(data) =&gt; {
// log the record data
tracing::info!("Data: {}", data);
}
Err(e) =&gt; {
tracing::error!("Error: {}", e);
}
}
});
tracing::info!(
"Successfully processed {} records",
event.payload.records.len()
);
Ok(())
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
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Connecting to an Amazon RDS database in a Lambda function
Invoke a Lambda function from a DynamoDB trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.