---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_S3_Lambda_section.html
title: Invoke a Lambda function from an Amazon S3 trigger
word_count: 1545
filtered: true
elements_removed: 0
density_score: 0.83
---

Invoke a Lambda function from an Amazon S3 trigger - AWS Lambda
Invoke a Lambda function from an Amazon S3 trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_S3_Lambda_section)
# Invoke a Lambda function from an Amazon S3 trigger
The following code examples show how to implement a Lambda function that receives an event triggered by uploading an object to an S3 bucket. The function retrieves the S3 bucket name and object key from the event parameter and calls the Amazon S3 API to retrieve and log the content type of the object.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using .NET.
```
// SPDX-License-Identifier: Apache-2.0
using System.Threading.Tasks;
using Amazon.Lambda.Core;
using Amazon.S3;
using System;
using Amazon.Lambda.S3Events;
using System.Web;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace S3Integration
{
public class Function
{
private static AmazonS3Client \_s3Client;
public Function() : this(null)
{
}
internal Function(AmazonS3Client s3Client)
{
\_s3Client = s3Client ?? new AmazonS3Client();
}
public async Task&lt;&lt;string&gt;&gt; Handler(S3Event evt, ILambdaContext context)
{
try
{
if (evt.Records.Count &lt;= 0)
{
context.Logger.LogLine("Empty S3 Event received");
return string.Empty;
}
var bucket = evt.Records[0].S3.Bucket.Name;
var key = HttpUtility.UrlDecode(evt.Records[0].S3.Object.Key);
context.Logger.LogLine($"Request is for {bucket} and {key}");
var objectResult = await \_s3Client.GetObjectAsync(bucket, key);
context.Logger.LogLine($"Returning {objectResult.Key}");
return objectResult.Key;
}
catch (Exception e)
{
context.Logger.LogLine($"Error processing request - {e.Message}");
return string.Empty;
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using Go.
```
// SPDX-License-Identifier: Apache-2.0
package main
import (
"context"
"log"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
"github.com/aws/aws-sdk-go-v2/config"
"github.com/aws/aws-sdk-go-v2/service/s3"
)
func handler(ctx context.Context, s3Event events.S3Event) error {
sdkConfig, err := config.LoadDefaultConfig(ctx)
if err != nil {
log.Printf("failed to load default config: %s", err)
return err
}
s3Client := s3.NewFromConfig(sdkConfig)
for \_, record := range s3Event.Records {
bucket := record.S3.Bucket.Name
key := record.S3.Object.URLDecodedKey
headOutput, err := s3Client.HeadObject(ctx, &amp;s3.HeadObjectInput{
Bucket: &amp;bucket,
Key: &amp;key,
})
if err != nil {
log.Printf("error getting head of object %s/%s: %s", bucket, key, err)
return err
}
log.Printf("successfully retrieved %s/%s of type %s", bucket, key, \*headOutput.ContentType)
}
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using Java.
```
// SPDX-License-Identifier: Apache-2.0
package example;
import software.amazon.awssdk.services.s3.model.HeadObjectRequest;
import software.amazon.awssdk.services.s3.model.HeadObjectResponse;
import software.amazon.awssdk.services.s3.S3Client;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.amazonaws.services.lambda.runtime.events.S3Event;
import com.amazonaws.services.lambda.runtime.events.models.s3.S3EventNotification.S3EventNotificationRecord;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
public class Handler implements RequestHandler&lt;S3Event, String&gt; {
private static final Logger logger = LoggerFactory.getLogger(Handler.class);
@Override
public String handleRequest(S3Event s3event, Context context) {
try {
S3EventNotificationRecord record = s3event.getRecords().get(0);
String srcBucket = record.getS3().getBucket().getName();
String srcKey = record.getS3().getObject().getUrlDecodedKey();
S3Client s3Client = S3Client.builder().build();
HeadObjectResponse headObject = getHeadObject(s3Client, srcBucket, srcKey);
logger.info("Successfully retrieved " + srcBucket + "/" + srcKey + " of type " + headObject.contentType());
return "Ok";
} catch (Exception e) {
throw new RuntimeException(e);
}
}
private HeadObjectResponse getHeadObject(S3Client s3Client, String bucket, String key) {
HeadObjectRequest headObjectRequest = HeadObjectRequest.builder()
.bucket(bucket)
.key(key)
.build();
return s3Client.headObject(headObjectRequest);
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using JavaScript.
```
`import { S3Client, HeadObjectCommand } from "@aws-sdk/client-s3";
const client = new S3Client();
export const handler = async (event, context) =&gt; {
// Get the object from the event and show its content type
const bucket = event.Records[0].s3.bucket.name;
const key = decodeURIComponent(event.Records[0].s3.object.key.replace(/\\+/g, ' '));
try {
const { ContentType } = await client.send(new HeadObjectCommand({
Bucket: bucket,
Key: key,
}));
console.log('CONTENT TYPE:', ContentType);
return ContentType;
} catch (err) {
console.log(err);
const message = `Error getting object ${key} from bucket ${bucket}. Make sure they exist and your bucket is in the same region as this function.`;
console.log(message);
throw new Error(message);
}
};
`
```
Consuming an S3 event with Lambda using TypeScript.
```
// SPDX-License-Identifier: Apache-2.0
import { S3Event } from 'aws-lambda';
import { S3Client, HeadObjectCommand } from '@aws-sdk/client-s3';
const s3 = new S3Client({ region: process.env.AWS\_REGION });
export const handler = async (event: S3Event): Promise&lt;&lt;string | undefined&gt;&gt; =&gt;&gt; {
// Get the object from the event and show its content type
const bucket = event.Records[0].s3.bucket.name;
const key = decodeURIComponent(event.Records[0].s3.object.key.replace(/\\+/g, ' '));
const params = {
Bucket: bucket,
Key: key,
};
try {
const { ContentType } = await s3.send(new HeadObjectCommand(params));
console.log('CONTENT TYPE:', ContentType);
return ContentType;
} catch (err) {
console.log(err);
const message = `Error getting object ${key} from bucket ${bucket}. Make sure they exist and your bucket is in the same region as this function.`;
console.log(message);
throw new Error(message);
}
};
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using PHP.
```
`&lt;&lt;?php
use Bref\\Context\\Context;
use Bref\\Event\\S3\\S3Event;
use Bref\\Event\\S3\\S3Handler;
use Bref\\Logger\\StderrLogger;
require \_\_DIR\_\_ . '/vendor/autoload.php';
class Handler extends S3Handler
{
private StderrLogger $logger;
public function \_\_construct(StderrLogger $logger)
{
$this-&gt;logger = $logger;
}
public function handleS3(S3Event $event, Context $context) : void
{
$this-&gt;logger-&gt;info("Processing S3 records");
// Get the object from the event and show its content type
$records = $event-&gt;getRecords();
foreach ($records as $record)
{
$bucket = $record-&gt;getBucket()-&gt;getName();
$key = urldecode($record-&gt;getObject()-&gt;getKey());
try {
$fileSize = urldecode($record-&gt;&gt;getObject()-&gt;&gt;getSize());
echo "File Size: " . $fileSize . "\\n";
// TODO: Implement your custom processing logic here
} catch (Exception $e) {
echo $e-&gt;&gt;getMessage() . "\\n";
echo 'Error getting object ' . $key . ' from bucket ' . $bucket . '. Make sure they exist and your bucket is in the same region as this function.' . "\\n";
throw $e;
}
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
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using Python.
```
# SPDX-License-Identifier: Apache-2.0
import json
import urllib.parse
import boto3
print('Loading function')
s3 = boto3.client('s3')
def lambda\_handler(event, context):
# Get the object from the event and show its content type
bucket = event['Records'][0]['s3']['bucket']['name']
key = urllib.parse.unquote\_plus(event['Records'][0]['s3']['object']['key'], encoding='utf-8')
try:
response = s3.get\_object(Bucket=bucket, Key=key)
print("CONTENT TYPE: " + response['ContentType'])
return response['ContentType']
except Exception as e:
print(e)
print('Error getting object {} from bucket {}. Make sure they exist and your bucket is in the same region as this function.'.format(key, bucket))
raise e
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using Ruby.
```
`require 'json'
require 'uri'
require 'aws-sdk'
puts 'Loading function'
def lambda\_handler(event:, context:)
s3 = Aws::S3::Client.new(region: 'region') # Your AWS region
# Get the object from the event and show its content type
bucket = event['Records'][0]['s3']['bucket']['name']
key = URI.decode\_www\_form\_component(event['Records'][0]['s3']['object']['key'], Encoding::UTF\_8)
begin
response = s3.get\_object(bucket: bucket, key: key)
puts "CONTENT TYPE: #{response.content\_type}"
return response.content\_type
rescue StandardError =&gt;&gt; e
puts e.message
puts "Error getting object #{key} from bucket #{bucket}. Make sure they exist and your bucket is in the same region as this function."
raise e
end
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-s3-to-lambda)
repository.
Consuming an S3 event with Lambda using Rust.
```
// SPDX-License-Identifier: Apache-2.0
use aws\_lambda\_events::event::s3::S3Event;
use aws\_sdk\_s3::{Client};
use lambda\_runtime::{run, service\_fn, Error, LambdaEvent};
/// Main function
#[tokio::main]
async fn main() -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
tracing\_subscriber::fmt()
.with\_max\_level(tracing::Level::INFO)
.with\_target(false)
.without\_time()
.init();
// Initialize the AWS SDK for Rust
let config = aws\_config::load\_from\_env().await;
let s3\_client = Client::new(&amp;&amp;config);
let res = run(service\_fn(|request: LambdaEvent&lt;&lt;S3Event&gt;&gt;| {
function\_handler(&amp;&amp;s3\_client, request)
})).await;
res
}
async fn function\_handler(
s3\_client: &amp;&amp;Client,
evt: LambdaEvent&lt;&lt;S3Event&gt;&gt;
) -&gt;&gt; Result&lt;&lt;(), Error&gt;&gt; {
tracing::info!(records = ?evt.payload.records.len(), "Received request from SQS");
if evt.payload.records.len() == 0 {
tracing::info!("Empty S3 event received");
}
let bucket = evt.payload.records[0].s3.bucket.name.as\_ref().expect("Bucket name to exist");
let key = evt.payload.records[0].s3.object.key.as\_ref().expect("Object key to exist");
tracing::info!("Request is for {} and object {}", bucket, key);
let s3\_get\_object\_result = s3\_client
.get\_object()
.bucket(bucket)
.key(key)
.send()
.await;
match s3\_get\_object\_result {
Ok(\_) =&gt;&gt; tracing::info!("S3 Get Object success, the s3GetObjectResult contains a 'body' property of type ByteStream"),
Err(\_) =&gt;&gt; tracing::info!("Failure with S3 Get Object request")
}
Ok(())
}
`
```
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke a Lambda function from an Amazon MSK trigger
Invoke a Lambda function from an Amazon SNS trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.