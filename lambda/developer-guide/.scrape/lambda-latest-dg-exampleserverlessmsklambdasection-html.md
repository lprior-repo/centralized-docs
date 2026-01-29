---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_MSK_Lambda_section.html
title: Invoke a Lambda function from an Amazon MSK trigger
word_count: 1210
filtered: true
elements_removed: 0
density_score: 0.83
---

Invoke a Lambda function from an Amazon MSK trigger - AWS Lambda
Invoke a Lambda function from an Amazon MSK trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_MSK_Lambda_section)
# Invoke a Lambda function from an Amazon MSK trigger
The following code examples show how to implement a Lambda function that receives an event triggered by receiving records from an Amazon MSK cluster. The function retrieves the MSK payload and logs the record contents.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using .NET.
```
`using System.Text;
using Amazon.Lambda.Core;
using Amazon.Lambda.KafkaEvents;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace MSKLambda;
public class Function
{
/// &lt;param name="input"&gt;The event for the Lambda function handler to process.&lt;/param&gt;
/// &lt;param name="context"&gt;The ILambdaContext that provides methods for logging and describing the Lambda environment.&lt;/param&gt;
/// &lt;returns&gt;&lt;/returns&gt;
public void FunctionHandler(KafkaEvent evnt, ILambdaContext context)
{
foreach (var record in evnt.Records)
{
Console.WriteLine("Key:" + record.Key);
foreach (var eventRecord in record.Value)
{
var valueBytes = eventRecord.Value.ToArray();
var valueText = Encoding.UTF8.GetString(valueBytes);
Console.WriteLine("Message:" + valueText);
}
}
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using Go.
```
`
package main
import (
"encoding/base64"
"fmt"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
func handler(event events.KafkaEvent) {
for key, records := range event.Records {
fmt.Println("Key:", key)
for \_, record := range records {
fmt.Println("Record:", record)
decodedValue, \_ := base64.StdEncoding.DecodeString(record.Value)
message := string(decodedValue)
fmt.Println("Message:", message)
}
}
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using Java.
```
`
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.KafkaEvent;
import com.amazonaws.services.lambda.runtime.events.KafkaEvent.KafkaEventRecord;
import java.util.Base64;
import java.util.Map;
public class Example implements RequestHandler&lt;KafkaEvent, Void&gt; {
@Override
public Void handleRequest(KafkaEvent event, Context context) {
for (Map.Entry&lt;String, java.util.List&lt;KafkaEventRecord&gt;&gt; entry : event.getRecords().entrySet()) {
String key = entry.getKey();
System.out.println("Key: " + key);
for (KafkaEventRecord record : entry.getValue()) {
System.out.println("Record: " + record);
byte[] value = Base64.getDecoder().decode(record.getValue());
String message = new String(value);
System.out.println("Message: " + message);
}
}
return null;
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using JavaScript.
```
`
exports.handler = async (event) =&gt; {
// Iterate through keys
for (let key in event.records) {
console.log('Key: ', key)
// Iterate through records
event.records[key].map((record) =&gt; {
console.log('Record: ', record)
// Decode base64
const msg = Buffer.from(record.value, 'base64').toString()
console.log('Message:', msg)
})
}
}
`
```
Consuming an Amazon MSK event with Lambda using TypeScript.
```
`import { MSKEvent, Context } from "aws-lambda";
import { Buffer } from "buffer";
import { Logger } from "@aws-lambda-powertools/logger";
const logger = new Logger({
logLevel: "INFO",
serviceName: "msk-handler-sample",
});
export const handler = async (
event: MSKEvent,
context: Context
): Promise&lt;void&gt; =&gt; {
for (const [topic, topicRecords] of Object.entries(event.records)) {
logger.info(`Processing key: ${topic}`);
// Process each record in the partition
for (const record of topicRecords) {
try {
// Decode the message value from base64
const decodedMessage = Buffer.from(record.value, 'base64').toString();
logger.info({
message: decodedMessage
});
}
catch (error) {
logger.error('Error processing event', { error });
throw error;
}
};
}
}
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using PHP.
```
`&lt;&lt;?php
// SPDX-License-Identifier: Apache-2.0
// using bref/bref and bref/logger for simplicity
use Bref\\Context\\Context;
use Bref\\Event\\Kafka\\KafkaEvent;
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
public function handle(mixed $event, Context $context): void
{
$kafkaEvent = new KafkaEvent($event);
$this-&gt;logger-&gt;info("Processing records");
$records = $kafkaEvent-&gt;getRecords();
foreach ($records as $record) {
try {
$key = $record-&gt;&gt;getKey();
$this-&gt;&gt;logger-&gt;&gt;info("Key: $key");
$values = $record-&gt;&gt;getValue();
$this-&gt;&gt;logger-&gt;&gt;info(json\_encode($values));
foreach ($values as $value) {
$this-&gt;logger-&gt;info("Value: $value");
}
} catch (Exception $e) {
$this-&gt;logger-&gt;error($e-&gt;getMessage());
}
}
$totalRecords = count($records);
$this-&gt;logger-&gt;info("Successfully processed $totalRecords records");
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using Python.
```
`
import base64
def lambda\_handler(event, context):
# Iterate through keys
for key in event['records']:
print('Key:', key)
# Iterate through records
for record in event['records'][key]:
print('Record:', record)
# Decode base64
msg = base64.b64decode(record['value']).decode('utf-8')
print('Message:', msg)
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using Ruby.
```
`
require 'base64'
def lambda\_handler(event:, context:)
# Iterate through keys
event['records'].each do |key, records|
puts "Key: #{key}"
# Iterate through records
records.each do |record|
puts "Record: #{record}"
# Decode base64
msg = Base64.decode64(record['value'])
puts "Message: #{msg}"
end
end
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-msk-to-lambda)
repository.
Consuming an Amazon MSK event with Lambda using Rust.
```
`use aws\_lambda\_events::event::kafka::KafkaEvent;
use lambda\_runtime::{run, service\_fn, tracing, Error, LambdaEvent};
use base64::prelude::\*;
use serde\_json::{Value};
use tracing::{info};
/// Pre-Requisites:
/// 1. Install Cargo Lambda - see https://www.cargo-lambda.info/guide/getting-started.html
/// 2. Add packages tracing, tracing-subscriber, serde\_json, base64
///
/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function\_handler(event: LambdaEvent&lt;&lt;KafkaEvent&gt;&gt;) -&gt;&gt; Result&lt;&lt;Value, Error&gt;&gt; {
let payload = event.payload.records;
for (\_name, records) in payload.iter() {
for record in records {
let record\_text = record.value.as\_ref().ok\_or("Value is None")?;
info!("Record: {}", &amp;&amp;record\_text);
// perform Base64 decoding
let record\_bytes = BASE64\_STANDARD.decode(record\_text)?;
let message = std::str::from\_utf8(&amp;&amp;record\_bytes)?;
info!("Message: {}", message);
}
}
Ok(().into())
}
#[tokio::main]
async fn main() -&gt; Result&lt;(), Error&gt; {
// required to enable CloudWatch error logging by the runtime
tracing::init\_default\_subscriber();
info!("Setup CW subscriber!");
run(service\_fn(function\_handler)).await
}
`
```
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke a Lambda function from a Amazon DocumentDB trigger
Invoke a Lambda function from an Amazon S3 trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.