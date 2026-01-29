---
url: https://docs.aws.amazon.com/lambda/latest/dg/durable-invocation.html
title: Invoking durable Lambda functions
word_count: 383
filtered: true
elements_removed: 0
density_score: 0.88
---

Invoking durable Lambda functions - AWS Lambda
Invoking durable Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#durable-invocation)
[Synchronous invocation limits](#synchronous-invocation-limits)[Asynchronous invocation for long-running workflows](#asynchronous-invocation)[Execution management APIs](#execution-management-apis)
# Invoking durable Lambda functions
Durable Lambda functions can be invoked using the same methods as default Lambda functions, but with important considerations for long-running executions.
This section covers invocation patterns, execution management, and best practices for durable functions.
## Synchronous invocation limits
Synchronous invocations of durable Lambda functions are limited to 15 minutes, the same as default Lambda functions. If your durable function needs to run longer than 15 minutes, it must be invoked asynchronously.
**When to use synchronous invocation:** Use for durable functions that complete within 15 minutes and when you need immediate results, such as quick approval workflows or short data processing tasks.
## Asynchronous invocation for long-running workflows
For durable functions that may run longer than 15 minutes, use asynchronous invocation. This allows the function to continue running while your client receives an immediate acknowledgment.
TypeScript
```
`
import { LambdaClient, InvokeCommand } from "@aws-sdk/client-lambda";
const client = new LambdaClient({});
// Asynchronous invocation
const command = new InvokeCommand({
FunctionName: "my-durable-function",
InvocationType: "Event", // Asynchronous
Payload: JSON.stringify({ orderId: "12345" })
});
await client.send(command);
`
```
Python
```
`
import boto3
import json
client = boto3.client('lambda')
# Asynchronous invocation
response = client.invoke(
FunctionName='my-durable-function',
InvocationType='Event', # Asynchronous
Payload=json.dumps({'order\_id': '12345'})
)
`
```
## Execution management APIs
Lambda provides APIs to manage and monitor durable function executions, including listing executions, getting execution status, and stopping running executions.
TypeScript
```
`
// Get execution status
const statusCommand = new InvokeCommand({
FunctionName: "my-durable-function",
InvocationType: "RequestResponse",
Payload: JSON.stringify({
action: "getStatus",
executionId: "exec-123"
})
});
const result = await client.send(statusCommand);
`
```
Python
```
`
# Get execution status
response = client.invoke(
FunctionName='my-durable-function',
InvocationType='RequestResponse',
Payload=json.dumps({
'action': 'get\_status',
'execution\_id': 'exec-123'
})
)
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Retaining records
Event source mappings
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.