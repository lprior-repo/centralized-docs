---
url: https://docs.aws.amazon.com/lambda/latest/dg/with-sqs-filtering.html
title: Using event filtering with an Amazon SQS event source
word_count: 1055
filtered: true
elements_removed: 0
density_score: 0.85
---

Using event filtering with an Amazon SQS event source - AWS Lambda
Using event filtering with an Amazon SQS event source - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#with-sqs-filtering)
[Amazon SQS event filtering basics](#filtering-SQS)
# Using event filtering with an Amazon SQS event source
You can use event filtering to control which records from a stream or queue Lambda sends to your function.
For general information about how event filtering works, see [Control which events Lambda sends to your function](./invocation-eventfiltering.html).
This section focuses on event filtering for Amazon SQS event sources.
###### Note
Amazon SQS event source mappings only support filtering on the `body` key.
## Amazon SQS event filtering basics
Suppose your Amazon SQS queue contains messages in the following JSON format.
```
`{
"RecordNumber": 1234,
"TimeStamp": "yyyy-mm-ddThh:mm:ss",
"RequestCode": "AAAA"
}`
```
An example record for this queue would look as follows.
```
`{
"messageId": "059f36b4-87a3-44ab-83d2-661975830a7d",
"receiptHandle": "AQEBwJnKyrHigUMZj6rYigCgxlaS3SLy0a...",
"body": "{\\n "RecordNumber": 1234,\\n "TimeStamp": "yyyy-mm-ddThh:mm:ss",\\n "RequestCode": "AAAA"\\n}",
"attributes": {
"ApproximateReceiveCount": "1",
"SentTimestamp": "1545082649183",
"SenderId": "AIDAIENQZJOLO23YVJ4VO",
"ApproximateFirstReceiveTimestamp": "1545082649185"
},
"messageAttributes": {},
"md5OfBody": "e4e68fb7bd0e697a0ae8f1bb342846b3",
"eventSource": "aws:sqs",
"eventSourceARN": "arn:aws:sqs:us-west-2:123456789012:my-queue",
"awsRegion": "us-west-2"
}`
```
To filter based on the contents of your Amazon SQS messages, use the `body` key in the Amazon SQS message record. Suppose you want to process
only those records where the `RequestCode` in your Amazon SQS message is “BBBB.” The `FilterCriteria` object would be
as follows.
```
`{
"Filters": [
{
"Pattern": "{ \\"body\\" : { \\"RequestCode\\" : [ \\"BBBB\\" ] } }"
}
]
}`
```
For added clarity, here is the value of the filter's `Pattern` expanded in plain JSON.
```
`{
"body": {
"RequestCode": [ "BBBB" ]
}
}`
```
You can add your filter using the console, AWS CLI or an AWS SAM template.
Console
To add this filter using the console, follow the instructions in [Attaching filter criteria to an event source mapping (console)](./invocation-eventfiltering.html#filtering-console) and enter the following
string for the **Filter criteria**.
```
`{ "body" : { "RequestCode" : [ "BBBB" ] } }`
```
AWS CLI
To create a new event source mapping with these filter criteria using the AWS Command Line Interface (AWS CLI), run the following
command.
```
`aws lambda create-event-source-mapping \\
--function-name `my-function` \\
--event-source-arn `arn:aws:sqs:us-east-2:123456789012:my-queue` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"body\\" : { \\"RequestCode\\" : [ \\"BBBB\\" ] } }"}]}'`
```
To add these filter criteria to an existing event source mapping, run the following command.
```
`aws lambda update-event-source-mapping \\
--uuid `"a1b2c3d4-5678-90ab-cdef-11111EXAMPLE"` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"body\\" : { \\"RequestCode\\" : [ \\"BBBB\\" ] } }"}]}'`
```
AWS SAM
To add this filter using AWS SAM, add the following snippet to the YAML template for your event source.
```
`FilterCriteria:
Filters:
- Pattern: '{ "body" : { "RequestCode" : [ "BBBB" ] } }'`
```
Suppose you want your function to process only those records where `RecordNumber` is greater than 9999. The `FilterCriteria`
object would be as follows.
```
`{
"Filters": [
{
"Pattern": "{ \\"body\\" : { \\"RecordNumber\\" : [ { \\"numeric\\": [ \\"&gt;&gt;\\", 9999 ] } ] } }"
}
]
}`
```
For added clarity, here is the value of the filter's `Pattern` expanded in plain JSON.
```
`{
"body": {
"RecordNumber": [
{
"numeric": [ "&gt;", 9999 ]
}
]
}
}`
```
You can add your filter using the console, AWS CLI or an AWS SAM template.
Console
To add this filter using the console, follow the instructions in [Attaching filter criteria to an event source mapping (console)](./invocation-eventfiltering.html#filtering-console) and enter the following
string for the **Filter criteria**.
```
`{ "body" : { "RecordNumber" : [ { "numeric": [ "&gt;", 9999 ] } ] } }`
```
AWS CLI
To create a new event source mapping with these filter criteria using the AWS Command Line Interface (AWS CLI), run the following
command.
```
`aws lambda create-event-source-mapping \\
--function-name `my-function` \\
--event-source-arn `arn:aws:sqs:us-east-2:123456789012:my-queue` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"body\\" : { \\"RecordNumber\\" : [ { \\"numeric\\": [ \\"&gt;&gt;\\", 9999 ] } ] } }"}]}'`
```
To add these filter criteria to an existing event source mapping, run the following command.
```
`aws lambda update-event-source-mapping \\
--uuid `"a1b2c3d4-5678-90ab-cdef-11111EXAMPLE"` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"body\\" : { \\"RecordNumber\\" : [ { \\"numeric\\": [ \\"&gt;&gt;\\", 9999 ] } ] } }"}]}'`
```
AWS SAM
To add this filter using AWS SAM, add the following snippet to the YAML template for your event source.
```
`FilterCriteria:
Filters:
- Pattern: '{ "body" : { "RecordNumber" : [ { "numeric": [ "&gt;", 9999 ] } ] } }'`
```
For Amazon SQS, the message body can be any string. However, this can be problematic if your `FilterCriteria` expect `body`
to be in a valid JSON format. The reverse scenario is also true—if the incoming message body is in JSON format but your filter criteria
expects `body` to be a plain string, this can lead to unintended behavior.
To avoid this issue, ensure that the format of body in your `FilterCriteria` matches the expected format of `body` in messages
that you receive from your queue. Before filtering your messages, Lambda automatically evaluates the format of the incoming message body and
of your filter pattern for `body`. If there is a mismatch, Lambda drops the message. The following table summarizes this evaluation:
|Incoming message `body` format|Filter pattern `body` format|Resulting action|
|
Plain string
|
Plain string
|
Lambda filters based on your filter criteria.
|
|
Plain string
|
No filter pattern for data properties
|
Lambda filters (on the other metadata properties only) based on your filter criteria.
|
|
Plain string
|
Valid JSON
|
Lambda drops the message.
|
|
Valid JSON
|
Plain string
|
Lambda drops the message.
|
|
Valid JSON
|
No filter pattern for data properties
|
Lambda filters (on the other metadata properties only) based on your filter criteria.
|
|
Valid JSON
|
Valid JSON
|
Lambda filters based on your filter criteria.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Parameters
Tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.