---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis-example.html
title: Tutorial: Using Lambda with Kinesis Data Streams
word_count: 2328
filtered: true
elements_removed: 0
density_score: 0.84
---

Tutorial: Using Lambda with Kinesis Data Streams - AWS Lambda
Tutorial: Using Lambda with Kinesis Data Streams - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-kinesis-example)
[Prerequisites](#with-kinesis-prepare)[Create the execution role](#with-kinesis-example-create-iam-role)[Create the function](#with-kinesis-example-create-function)[Test the
Lambda function](#walkthrough-kinesis-events-adminuser-create-test-function-upload-zip-test-manual-invoke)[Create a Kinesis stream](#with-kinesis-example-configure-event-source-create)[Add an event source in
AWS Lambda](#with-kinesis-example-configure-event-source-add-event-source)[Test the setup](#with-kinesis-example-configure-event-source-test-end-to-end)[Clean up your resources](#cleanup)
# Tutorial: Using Lambda with Kinesis Data Streams
In this tutorial, you create a Lambda function to consume events from a Amazon Kinesis data stream.
1. Custom app writes records to the stream.
2. AWS Lambda polls the stream and, when it detects new records in the stream, invokes your Lambda
function.
3. AWS Lambda runs the Lambda function by assuming the execution role you specified at the time you created
the Lambda function.
## Prerequisites
If you have not yet installed the AWS Command Line Interface, follow the steps at [Installing or updating the latest version of the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
to install it.
The tutorial requires a command line terminal or shell to run commands. In Linux and macOS, use your preferred shell and package manager.
###### Note
In Windows, some Bash CLI commands that you commonly use with Lambda (such as `zip`) are not supported by the operating system's built-in terminals.
To get a Windows-integrated version of Ubuntu and Bash, [install the Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install-win10).
## Create the execution role
Create the [execution role](./lambda-intro-execution-role.html) that gives your function
permission to access AWS resources.
###### To create an execution role
1. Open the [roles page](https://console.aws.amazon.com/iam/home#/roles) in the IAM console.
2. Choose **Create role**.
3. Create a role with the following properties.
* **Trusted entity** – **AWS Lambda**.
* **Permissions** – **AWSLambdaKinesisExecutionRole**.
* **Role name** – `lambda-kinesis-role`.
The **AWSLambdaKinesisExecutionRole** policy has the permissions that the function needs to
read items from Kinesis and write logs to CloudWatch Logs.
## Create the function
Create a Lambda function that processes your Kinesis messages. The function code logs the event ID
and event data of the Kinesis record to CloudWatch Logs.
This tutorial uses the Node.js 24 runtime, but we've also provided example code in other runtime
languages. You can select the tab in the following box to see code for the runtime you're interested in.
The JavaScript code you'll use in this step is in the first example shown in the
**JavaScript** tab.
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
###### To create the function
1. Create a directory for the project, and then switch to that directory.
```
`mkdir kinesis-tutorial
cd kinesis-tutorial`
```
2. Copy the sample JavaScript code into a new file named `index.js`.
3. Create a deployment package.
```
``zip function.zip index.js``
```
4. Create a Lambda function with the `create-function` command.
```
``aws lambda create-function --function-name ProcessKinesisRecords \\
--zip-file fileb://function.zip --handler index.handler --runtime nodejs24.x \\
--role arn:aws:iam::`111122223333`:role/lambda-kinesis-role``
```
## Test the
Lambda function
Invoke your Lambda function manually using the `invoke` AWS Lambda CLI command and a sample Kinesis
event.
###### To test the Lambda function
1. Copy the following JSON into a file and save it as `input.txt`.
```
`{
"Records": [
{
"kinesis": {
"kinesisSchemaVersion": "1.0",
"partitionKey": "1",
"sequenceNumber": "49590338271490256608559692538361571095921575989136588898",
"data": "SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==",
"approximateArrivalTimestamp": 1545084650.987
},
"eventSource": "aws:kinesis",
"eventVersion": "1.0",
"eventID": "shardId-000000000006:49590338271490256608559692538361571095921575989136588898",
"eventName": "aws:kinesis:record",
"invokeIdentityArn": "arn:aws:iam::111122223333:role/lambda-kinesis-role",
"awsRegion": "us-east-2",
"eventSourceARN": "arn:aws:kinesis:us-east-2:111122223333:stream/lambda-stream"
}
]
}`
```
2. Use the `invoke` command to send the event to the function.
```
``aws lambda invoke --function-name ProcessKinesisRecords \\
--cli-binary-format raw-in-base64-out \\
--payload file://input.txt outputfile.txt``
```
The **cli-binary-format** option is required if you're using AWS CLI version 2. To make this the default setting, run `aws configure set cli-binary-format raw-in-base64-out`. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide for Version 2*.
The response is saved to `out.txt`.
## Create a Kinesis stream
Use the `create-stream ` command to create a stream.
```
``aws kinesis create-stream --stream-name lambda-stream --shard-count 1``
```
Run the following `describe-stream` command to get the stream ARN.
```
``aws kinesis describe-stream --stream-name lambda-stream``
```
You should see the following output:
```
{
"StreamDescription": {
"Shards": [
{
"ShardId": "shardId-000000000000",
"HashKeyRange": {
"StartingHashKey": "0",
"EndingHashKey": "340282366920746074317682119384634633455"
},
"SequenceNumberRange": {
"StartingSequenceNumber": "49591073947768692513481539594623130411957558361251844610"
}
}
],
"StreamARN": "arn:aws:kinesis:us-east-1:111122223333:stream/lambda-stream",
"StreamName": "lambda-stream",
"StreamStatus": "ACTIVE",
"RetentionPeriodHours": 24,
"EnhancedMonitoring": [
{
"ShardLevelMetrics": []
}
],
"EncryptionType": "NONE",
"KeyId": null,
"StreamCreationTimestamp": 1544828156.0
}
}
```
You use the stream ARN in the next step to associate the stream with your Lambda function.
## Add an event source in
AWS Lambda
Run the following AWS CLI `add-event-source` command.
```
``aws lambda create-event-source-mapping --function-name ProcessKinesisRecords \\
--event-source arn:aws:kinesis:us-east-1:111122223333:stream/lambda-stream \\
--batch-size 100 --starting-position LATEST``
```
Note the mapping ID for later use. You can get a list of event source mappings by running the
`list-event-source-mappings` command.
```
``aws lambda list-event-source-mappings --function-name ProcessKinesisRecords \\
--event-source arn:aws:kinesis:us-east-1:111122223333:stream/lambda-stream``
```
In the response, you can verify the status value is `enabled`. Event source mappings can be
disabled to pause polling temporarily without losing any records.
## Test the setup
To test the event source mapping, add event records to your Kinesis stream. The `--data` value is a
string that the CLI encodes to base64 prior to sending it to Kinesis. You can run the same command more than once to
add multiple records to the stream.
```
``aws kinesis put-record --stream-name lambda-stream --partition-key 1 \\
--data "Hello, this is a test."``
```
Lambda uses the execution role to read records from the stream. Then it invokes your Lambda function, passing in
batches of records. The function decodes data from each record and logs it, sending the output to CloudWatch Logs. View the
logs in the [CloudWatch console](https://console.aws.amazon.com/cloudwatch).
## Clean up your resources
You can now delete the resources that you created for this tutorial, unless you want to retain them. By deleting AWS resources that you're no longer using, you prevent unnecessary charges to your AWS account.
###### To delete the execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) of the IAM console.
2. Select the execution role that you created.
3. Choose **Delete**.
4. Enter the name of the role in the text input field and choose **Delete**.
###### To delete the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function that you created.
3. Choose **Actions**, **Delete**.
4. Type `confirm` in the text input field and choose **Delete**.
###### To delete the Kinesis stream
1. Sign in to the AWS Management Console and open the Kinesis console at
[https://console.aws.amazon.com/kinesis](https://console.aws.amazon.com/kinesis).
2. Select the stream you created.
3. Choose **Actions**, **Delete**.
4. Enter `delete` in the text input field.
5. Choose **Delete**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Event filtering
Kubernetes
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.