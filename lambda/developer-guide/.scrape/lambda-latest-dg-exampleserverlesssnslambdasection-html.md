---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_SNS_Lambda_section.html
title: Invoke a Lambda function from an Amazon SNS trigger
word_count: 1208
filtered: true
elements_removed: 0
density_score: 0.85
---

Invoke a Lambda function from an Amazon SNS trigger - AWS Lambda
Invoke a Lambda function from an Amazon SNS trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_SNS_Lambda_section)
# Invoke a Lambda function from an Amazon SNS trigger
The following code examples show how to implement a Lambda function that receives an event triggered by receiving messages from an SNS topic. The function retrieves the messages from the event parameter and logs the content of each message.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using Amazon.Lambda.Core;
using Amazon.Lambda.SNSEvents;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace SnsIntegration;
public class Function
{
public async Task FunctionHandler(SNSEvent evnt, ILambdaContext context)
{
foreach (var record in evnt.Records)
{
await ProcessRecordAsync(record, context);
}
context.Logger.LogInformation("done");
}
private async Task ProcessRecordAsync(SNSEvent.SNSRecord record, ILambdaContext context)
{
try
{
context.Logger.LogInformation($"Processed record {record.Sns.Message}");
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package main
import (
"context"
"fmt"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
)
func handler(ctx context.Context, snsEvent events.SNSEvent) {
for \_, record := range snsEvent.Records {
processMessage(record)
}
fmt.Println("done")
}
func processMessage(record events.SNSEventRecord) {
message := record.SNS.Message
fmt.Printf("Processed message: %s\\n", message)
// TODO: Process your record here
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
package example;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.LambdaLogger;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.SNSEvent;
import com.amazonaws.services.lambda.runtime.events.SNSEvent.SNSRecord;
import java.util.Iterator;
import java.util.List;
public class SNSEventHandler implements RequestHandler&lt;SNSEvent, Boolean&gt; {
LambdaLogger logger;
@Override
public Boolean handleRequest(SNSEvent event, Context context) {
logger = context.getLogger();
List&lt;SNSRecord&gt; records = event.getRecords();
if (!records.isEmpty()) {
Iterator&lt;SNSRecord&gt; recordsIter = records.iterator();
while (recordsIter.hasNext()) {
processRecord(recordsIter.next());
}
}
return Boolean.TRUE;
}
public void processRecord(SNSRecord record) {
try {
String message = record.getSNS().getMessage();
logger.log("message: " + message);
} catch (Exception e) {
throw new RuntimeException(e);
}
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/blob/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using JavaScript.
```
// SPDX-License-Identifier: Apache-2.0
exports.handler = async (event, context) =&gt; {
for (const record of event.Records) {
await processMessageAsync(record);
}
console.info("done");
};
async function processMessageAsync(record) {
try {
const message = JSON.stringify(record.Sns.Message);
console.log(`Processed message ${message}`);
await Promise.resolve(1); //Placeholder for actual async work
} catch (err) {
console.error("An error occurred");
throw err;
}
}
`
```
Consuming an SNS event with Lambda using TypeScript.
```
// SPDX-License-Identifier: Apache-2.0
import { SNSEvent, Context, SNSHandler, SNSEventRecord } from "aws-lambda";
export const functionHandler: SNSHandler = async (
event: SNSEvent,
context: Context
): Promise&lt;void&gt; =&gt; {
for (const record of event.Records) {
await processMessageAsync(record);
}
console.info("done");
};
async function processMessageAsync(record: SNSEventRecord): Promise&lt;any&gt; {
try {
const message: string = JSON.stringify(record.Sns.Message);
console.log(`Processed message ${message}`);
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using PHP.
```
// SPDX-License-Identifier: Apache-2.0
&lt;&lt;?php
/\*
Since native PHP support for AWS Lambda is not available, we are utilizing Bref's PHP functions runtime for AWS Lambda.
For more information on Bref's PHP runtime for Lambda, refer to: https://bref.sh/docs/runtimes/function
Another approach would be to create a custom runtime.
A practical example can be found here: https://aws.amazon.com/blogs/apn/aws-lambda-custom-runtime-for-php-a-practical-example/
\*/
// Additional composer packages may be required when using Bref or any other PHP functions runtime.
// require \_\_DIR\_\_ . '/vendor/autoload.php';
use Bref\\Context\\Context;
use Bref\\Event\\Sns\\SnsEvent;
use Bref\\Event\\Sns\\SnsHandler;
class Handler extends SnsHandler
{
public function handleSns(SnsEvent $event, Context $context): void
{
foreach ($event-&gt;getRecords() as $record) {
$message = $record-&gt;&gt;getMessage();
// TODO: Implement your custom processing logic here
// Any exception thrown will be logged and the invocation will be marked as failed
echo "Processed Message: $message" . PHP\_EOL;
}
}
}
return new Handler();
`
```
Python
**SDK for Python (Boto3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
def lambda\_handler(event, context):
for record in event['Records']:
process\_message(record)
print("done")
def process\_message(record):
try:
message = record['Sns']['Message']
print(f"Processed message {message}")
# TODO; Process your record here
except Exception as e:
print("An error occurred")
raise e
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using Ruby.
```
# SPDX-License-Identifier: Apache-2.0
def lambda\_handler(event:, context:)
event['Records'].map { |record| process\_message(record) }
end
def process\_message(record)
message = record['Sns']['Message']
puts("Processing message: #{message}")
rescue StandardError =&gt; e
puts("Error processing message: #{e}")
raise
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-sns-to-lambda)
repository.
Consuming an SNS event with Lambda using Rust.
```
// SPDX-License-Identifier: Apache-2.0
use aws\_lambda\_events::event::sns::SnsEvent;
use aws\_lambda\_events::sns::SnsRecord;
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
use tracing::info;
// aws\_lambda\_events = { version = "0.10.0", default-features = false, features = ["sns"] }
// lambda\_runtime = "0.8.1"
// tokio = { version = "1", features = ["macros"] }
// tracing = { version = "0.1", features = ["log"] }
// tracing-subscriber = { version = "0.3", default-features = false, features = ["fmt"] }
async fn function\_handler(event: LambdaEvent&lt;&lt;SnsEvent&gt;&gt;) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
for event in event.payload.records {
process\_record(&amp;&amp;event)?;
}
Ok(())
}
fn process\_record(record: &amp;&amp;SnsRecord) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
info!("Processing SNS Message: {}", record.sns.message);
// Implement your record handling code here.
Ok(())
}
#[tokio::main]
async fn main() -&gt; Result&lt;(), Error&gt; {
tracing\_subscriber::fmt()
.with\_max\_level(tracing::Level::INFO)
.with\_target(false)
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
Invoke a Lambda function from an Amazon S3 trigger
Invoke a Lambda function from an Amazon SQS trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.