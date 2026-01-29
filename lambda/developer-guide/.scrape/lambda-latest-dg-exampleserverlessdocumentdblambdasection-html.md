---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_serverless_DocumentDB_Lambda_section.html
title: Invoke a Lambda function from a Amazon DocumentDB trigger
word_count: 1471
filtered: true
elements_removed: 0
density_score: 0.84
---

Invoke a Lambda function from a Amazon DocumentDB trigger - AWS Lambda
Invoke a Lambda function from a Amazon DocumentDB trigger - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_serverless_DocumentDB_Lambda_section)
# Invoke a Lambda function from a Amazon DocumentDB trigger
The following code examples show how to implement a Lambda function that receives an event triggered by receiving records from a DocumentDB change stream. The function retrieves the DocumentDB payload and logs the record contents.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using .NET.
```
`using Amazon.Lambda.Core;
using System.Text.Json;
using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;
//Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace LambdaDocDb;
public class Function
{
/// &lt;summary&gt;
/// Lambda function entry point to process Amazon DocumentDB events.
/// &lt;/summary&gt;
/// &lt;param name="event"&gt;The Amazon DocumentDB event.&lt;/param&gt;
/// &lt;param name="context"&gt;The Lambda context object.&lt;/param&gt;
/// &lt;returns&gt;A string to indicate successful processing.&lt;/returns&gt;
public string FunctionHandler(Event evnt, ILambdaContext context)
{
foreach (var record in evnt.Events)
{
ProcessDocumentDBEvent(record, context);
}
return "OK";
}
private void ProcessDocumentDBEvent(DocumentDBEventRecord record, ILambdaContext context)
{
var eventData = record.Event;
var operationType = eventData.OperationType;
var databaseName = eventData.Ns.Db;
var collectionName = eventData.Ns.Coll;
var fullDocument = JsonSerializer.Serialize(eventData.FullDocument, new JsonSerializerOptions { WriteIndented = true });
context.Logger.LogLine($"Operation type: {operationType}");
context.Logger.LogLine($"Database: {databaseName}");
context.Logger.LogLine($"Collection: {collectionName}");
context.Logger.LogLine($"Full document:\\n{fullDocument}");
}
public class Event
{
[JsonPropertyName("eventSourceArn")]
public string EventSourceArn { get; set; }
[JsonPropertyName("events")]
public List&lt;DocumentDBEventRecord&gt; Events { get; set; }
[JsonPropertyName("eventSource")]
public string EventSource { get; set; }
}
public class DocumentDBEventRecord
{
[JsonPropertyName("event")]
public EventData Event { get; set; }
}
public class EventData
{
[JsonPropertyName("\_id")]
public IdData Id { get; set; }
[JsonPropertyName("clusterTime")]
public ClusterTime ClusterTime { get; set; }
[JsonPropertyName("documentKey")]
public DocumentKey DocumentKey { get; set; }
[JsonPropertyName("fullDocument")]
public Dictionary&lt;string, object&gt; FullDocument { get; set; }
[JsonPropertyName("ns")]
public Namespace Ns { get; set; }
[JsonPropertyName("operationType")]
public string OperationType { get; set; }
}
public class IdData
{
[JsonPropertyName("\_data")]
public string Data { get; set; }
}
public class ClusterTime
{
[JsonPropertyName("$timestamp")]
public Timestamp Timestamp { get; set; }
}
public class Timestamp
{
[JsonPropertyName("t")]
public long T { get; set; }
[JsonPropertyName("i")]
public int I { get; set; }
}
public class DocumentKey
{
[JsonPropertyName("\_id")]
public Id Id { get; set; }
}
public class Id
{
[JsonPropertyName("$oid")]
public string Oid { get; set; }
}
public class Namespace
{
[JsonPropertyName("db")]
public string Db { get; set; }
[JsonPropertyName("coll")]
public string Coll { get; set; }
}
}
`
```
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using Go.
```
`
package main
import (
"context"
"encoding/json"
"fmt"
"github.com/aws/aws-lambda-go/lambda"
)
type Event struct {
Events []Record `json:"events"`
}
type Record struct {
Event struct {
OperationType string `json:"operationType"`
NS struct {
DB string `json:"db"`
Coll string `json:"coll"`
} `json:"ns"`
FullDocument interface{} `json:"fullDocument"`
} `json:"event"`
}
func main() {
lambda.Start(handler)
}
func handler(ctx context.Context, event Event) (string, error) {
fmt.Println("Loading function")
for \_, record := range event.Events {
logDocumentDBEvent(record)
}
return "OK", nil
}
func logDocumentDBEvent(record Record) {
fmt.Printf("Operation type: %s\\n", record.Event.OperationType)
fmt.Printf("db: %s\\n", record.Event.NS.DB)
fmt.Printf("collection: %s\\n", record.Event.NS.Coll)
docBytes, \_ := json.MarshalIndent(record.Event.FullDocument, "", " ")
fmt.Printf("Full document: %s\\n", string(docBytes))
}
`
```
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using Java.
```
`import java.util.List;
import java.util.Map;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
public class Example implements RequestHandler&lt;Map&lt;String, Object&gt;, String&gt; {
@SuppressWarnings("unchecked")
@Override
public String handleRequest(Map&lt;String, Object&gt; event, Context context) {
List&lt;Map&lt;String, Object&gt;&gt; events = (List&lt;Map&lt;String, Object&gt;&gt;) event.get("events");
for (Map&lt;String, Object&gt; record : events) {
Map&lt;String, Object&gt; eventData = (Map&lt;String, Object&gt;) record.get("event");
processEventData(eventData);
}
return "OK";
}
@SuppressWarnings("unchecked")
private void processEventData(Map&lt;String, Object&gt; eventData) {
String operationType = (String) eventData.get("operationType");
System.out.println("operationType: %s".formatted(operationType));
Map&lt;String, Object&gt; ns = (Map&lt;String, Object&gt;) eventData.get("ns");
String db = (String) ns.get("db");
System.out.println("db: %s".formatted(db));
String coll = (String) ns.get("coll");
System.out.println("coll: %s".formatted(coll));
Map&lt;String, Object&gt; fullDocument = (Map&lt;String, Object&gt;) eventData.get("fullDocument");
System.out.println("fullDocument: %s".formatted(fullDocument));
}
}
`
```
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using JavaScript.
```
`console.log('Loading function');
exports.handler = async (event, context) =&gt; {
event.events.forEach(record =&gt; {
logDocumentDBEvent(record);
});
return 'OK';
};
const logDocumentDBEvent = (record) =&gt; {
console.log('Operation type: ' + record.event.operationType);
console.log('db: ' + record.event.ns.db);
console.log('collection: ' + record.event.ns.coll);
console.log('Full document:', JSON.stringify(record.event.fullDocument, null, 2));
};
`
```
Consuming a Amazon DocumentDB event with Lambda using TypeScript
```
`import { DocumentDBEventRecord, DocumentDBEventSubscriptionContext } from 'aws-lambda';
console.log('Loading function');
export const handler = async (
event: DocumentDBEventSubscriptionContext,
context: any
): Promise&lt;string&gt; =&gt; {
event.events.forEach((record: DocumentDBEventRecord) =&gt; {
logDocumentDBEvent(record);
});
return 'OK';
};
const logDocumentDBEvent = (record: DocumentDBEventRecord): void =&gt; {
console.log('Operation type: ' + record.event.operationType);
console.log('db: ' + record.event.ns.db);
console.log('collection: ' + record.event.ns.coll);
console.log('Full document:', JSON.stringify(record.event.fullDocument, null, 2));
};
`
```
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using PHP.
```
`&lt;&lt;?php
require \_\_DIR\_\_.'/vendor/autoload.php';
use Bref\\Context\\Context;
use Bref\\Event\\Handler;
class DocumentDBEventHandler implements Handler
{
public function handle($event, Context $context): string
{
$events = $event['events'] ?? [];
foreach ($events as $record) {
$this-&gt;logDocumentDBEvent($record['event']);
}
return 'OK';
}
private function logDocumentDBEvent($event): void
{
// Extract information from the event record
$operationType = $event['operationType'] ?? 'Unknown';
$db = $event['ns']['db'] ?? 'Unknown';
$collection = $event['ns']['coll'] ?? 'Unknown';
$fullDocument = $event['fullDocument'] ?? [];
// Log the event details
echo "Operation type: $operationType\\n";
echo "Database: $db\\n";
echo "Collection: $collection\\n";
echo "Full document: " . json\_encode($fullDocument, JSON\_PRETTY\_PRINT) . "\\n";
}
}
return new DocumentDBEventHandler();
`
```
Python
**SDK for Python (Boto3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using Python.
```
`import json
def lambda\_handler(event, context):
for record in event.get('events', []):
log\_document\_db\_event(record)
return 'OK'
def log\_document\_db\_event(record):
event\_data = record.get('event', {})
operation\_type = event\_data.get('operationType', 'Unknown')
db = event\_data.get('ns', {}).get('db', 'Unknown')
collection = event\_data.get('ns', {}).get('coll', 'Unknown')
full\_document = event\_data.get('fullDocument', {})
print(f"Operation type: {operation\_type}")
print(f"db: {db}")
print(f"collection: {collection}")
print("Full document:", json.dumps(full\_document, indent=2))
`
```
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using Ruby.
```
`require 'json'
def lambda\_handler(event:, context:)
event['events'].each do |record|
log\_document\_db\_event(record)
end
'OK'
end
def log\_document\_db\_event(record)
event\_data = record['event'] || {}
operation\_type = event\_data['operationType'] || 'Unknown'
db = event\_data.dig('ns', 'db') || 'Unknown'
collection = event\_data.dig('ns', 'coll') || 'Unknown'
full\_document = event\_data['fullDocument'] || {}
puts "Operation type: #{operation\_type}"
puts "db: #{db}"
puts "collection: #{collection}"
puts "Full document: #{JSON.pretty\_generate(full\_document)}"
end
`
```
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[Serverless examples](https://github.com/aws-samples/serverless-snippets/tree/main/integration-docdb-to-lambda)
repository.
Consuming a Amazon DocumentDB event with Lambda using Rust.
```
`
use lambda\_runtime::{service\_fn, tracing, Error, LambdaEvent};
use aws\_lambda\_events::{
event::documentdb::{DocumentDbEvent, DocumentDbInnerEvent},
};
//lambda\_runtime = "0.11.1"
//serde\_json = "1.0"
//tokio = { version = "1", features = ["macros"] }
//tracing = { version = "0.1", features = ["log"] }
//tracing-subscriber = { version = "0.3", default-features = false, features = ["fmt"] }
//aws\_lambda\_events = "0.15.0"
async fn function\_handler(event: LambdaEvent&lt;&lt;DocumentDbEvent&gt;&gt;) -&gt;&gt;Result&lt;&lt;(), Error&gt;&gt; {
tracing::info!("Event Source ARN: {:?}", event.payload.event\_source\_arn);
tracing::info!("Event Source: {:?}", event.payload.event\_source);
let records = &amp;&amp;event.payload.events;
if records.is\_empty() {
tracing::info!("No records found. Exiting.");
return Ok(());
}
for record in records{
log\_document\_db\_event(record);
}
tracing::info!("Document db records processed");
// Prepare the response
Ok(())
}
fn log\_document\_db\_event(record: &amp;&amp;DocumentDbInnerEvent)-&gt;&gt; Result&lt;&lt;(), Error&gt;&gt;{
tracing::info!("Change Event: {:?}", record.event);
Ok(())
}
#[tokio::main]
async fn main() -&gt; Result&lt;(), Error&gt; {
tracing\_subscriber::fmt()
.with\_max\_level(tracing::Level::INFO)
.with\_target(false)
.without\_time()
.init();
let func = service\_fn(function\_handler);
lambda\_runtime::run(func).await?;
Ok(())
}
`
```
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke a Lambda function from a DynamoDB trigger
Invoke a Lambda function from an Amazon MSK trigger
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.