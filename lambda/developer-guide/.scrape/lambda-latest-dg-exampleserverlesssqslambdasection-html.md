---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_SQS_Lambda_section.html
title: Invoke a Lambda function from an Amazon SQS trigger
word_count: 1131
filtered: true
elements_removed: 0
density_score: 0.85
---

Invoke a Lambda function from an Amazon SQS trigger - AWS Lambda
Invoke a Lambda function from an Amazon SQS trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_SQS_Lambda_section)
# Invoke a Lambda function from an Amazon SQS trigger
The following code examples show how to implement a Lambda function that receives an event triggered by receiving messages from an SQS queue. The function retrieves the messages from the event parameter and logs the content of each message.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using Amazon.Lambda.Core;
using Amazon.Lambda.SQSEvents;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace SqsIntegrationSampleCode
{
public async Task FunctionHandler(SQSEvent evnt, ILambdaContext context)
{
foreach (var message in evnt.Records)
{
await ProcessMessageAsync(message, context);
}
context.Logger.LogInformation("done");
}
private async Task ProcessMessageAsync(SQSEvent.SQSMessage message, ILambdaContext context)
{
try
{
context.Logger.LogInformation($"Processed message {message.Body}");
// TODO: Do interesting work based on the new message
await Task.CompletedTask;
}
catch (Exception e)
{
//You can use Dead Letter Queue to handle failures. By configuring a Lambda DLQ.
context.Logger.LogError($"An error occurred");
throw;
}
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package integration\_sqs\_to\_lambda
import (
"fmt"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
func handler(event events.SQSEvent) error {
for \_, record := range event.Records {
err := processMessage(record)
if err != nil {
return err
}
}
fmt.Println("done")
return nil
}
func processMessage(record events.SQSMessage) error {
fmt.Printf("Processed message %s\\n", record.Body)
// TODO: Do interesting work based on the new message
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.SQSEvent;
import com.amazonaws.services.lambda.runtime.events.SQSEvent.SQSMessage;
public class Function implements RequestHandler&lt;SQSEvent, Void&gt; {
@Override
public Void handleRequest(SQSEvent sqsEvent, Context context) {
for (SQSMessage msg : sqsEvent.getRecords()) {
processMessage(msg, context);
}
context.getLogger().log("done");
return null;
}
private void processMessage(SQSMessage msg, Context context) {
try {
context.getLogger().log("Processed message " + msg.getBody());
// TODO: Do interesting work based on the new message
} catch (Exception e) {
context.getLogger().log("An error occurred");
throw e;
}
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/blob/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using JavaScript.
```
// SPDX-License-Identifier: Apache-2.0
exports.handler = async (event, context) =&gt; {
for (const message of event.Records) {
await processMessageAsync(message);
}
console.info("done");
};
async function processMessageAsync(message) {
try {
console.log(`Processed message ${message.body}`);
// TODO: Do interesting work based on the new message
await Promise.resolve(1); //Placeholder for actual async work
} catch (err) {
console.error("An error occurred");
throw err;
}
}
`
```
Consuming an SQS event with Lambda using TypeScript.
```
// SPDX-License-Identifier: Apache-2.0
import { SQSEvent, Context, SQSHandler, SQSRecord } from "aws-lambda";
export const functionHandler: SQSHandler = async (
event: SQSEvent,
context: Context
): Promise&lt;void&gt; =&gt; {
for (const message of event.Records) {
await processMessageAsync(message);
}
console.info("done");
};
async function processMessageAsync(message: SQSRecord): Promise&lt;any&gt; {
try {
console.log(`Processed message ${message.body}`);
// TODO: Do interesting work based on the new message
await Promise.resolve(1); //Placeholder for actual async work
} catch (err) {
console.error("An error occurred");
throw err;
}
}
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using PHP.
```
// SPDX-License-Identifier: Apache-2.0
&lt;&lt;?php
# using bref/bref and bref/logger for simplicity
use Bref\\Context\\Context;
use Bref\\Event\\InvalidLambdaEvent;
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
\* @throws InvalidLambdaEvent
\*/
public function handleSqs(SqsEvent $event, Context $context): void
{
foreach ($event-&gt;getRecords() as $record) {
$body = $record-&gt;getBody();
// TODO: Do interesting work based on the new message
}
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
def lambda\_handler(event, context):
for message in event['Records']:
process\_message(message)
print("done")
def process\_message(message):
try:
print(f"Processed message {message['body']}")
# TODO: Do interesting work based on the new message
except Exception as err:
print("An error occurred")
raise err
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using Ruby.
```
# SPDX-License-Identifier: Apache-2.0
def lambda\_handler(event:, context:)
event['Records'].each do |message|
process\_message(message)
end
puts "done"
end
def process\_message(message)
begin
puts "Processed message #{message['body']}"
# TODO: Do interesting work based on the new message
rescue StandardError =&gt; err
puts "An error occurred"
raise err
end
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sqs-to-lambda)
repository.
Consuming an SQS event with Lambda using Rust.
```
// SPDX-License-Identifier: Apache-2.0
use aws\_lambda\_events::event::sqs::SqsEvent;
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
async fn function\_handler(event: LambdaEvent&lt;&lt;SqsEvent&gt;&gt;) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
event.payload.records.iter().for\_each(|record| {
// process the record
tracing::info!("Message body: {}", record.body.as\_deref().unwrap\_or\_default())
});
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
Invoke a Lambda function from an Amazon SNS trigger
Reporting batch item failures for Lambda functions with a Kinesis trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.