---
url: https://docs.aws.amazon.com/lambda/latest/dg/kafka-filtering.html
title: Filtering events from Amazon MSK and self-managed Apache Kafka event sources
word_count: 1043
filtered: true
elements_removed: 0
density_score: 0.85
---

Filtering events from Amazon MSK and self-managed Apache Kafka event sources - AWS Lambda
Filtering events from Amazon MSK and self-managed Apache Kafka event sources - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#kafka-filtering)
[Kafka event filtering basics](#filtering-kafka)
# Filtering events from Amazon MSK and self-managed Apache Kafka event sources
You can use event filtering to control which records from a stream or queue Lambda sends to your function.
For general information about how event filtering works, see [Control which events Lambda sends to your function](./invocation-eventfiltering.html).
###### Note
Amazon MSK and self-managed Apache Kafka event source mappings only support filtering on the `value` key.
## Kafka event filtering basics
Suppose a producer is writing messages to a topic in your Kafka cluster, either in valid JSON format or as plain strings. An example record
would look like the following, with the message converted to a Base64 encoded string in the `value` field.
```
`{
"mytopic-0":[
{
"topic":"mytopic",
"partition":0,
"offset":15,
"timestamp":1545084650987,
"timestampType":"CREATE\_TIME",
"value":"SGVsbG8sIHRoaXMgaXMgYSB0ZXN0Lg==",
"headers":[]
}
]
}`
```
Suppose your Apache Kafka producer is writing messages to your topic in the following JSON format.
```
`{
"device\_ID": "AB1234",
"session":{
"start\_time": "yyyy-mm-ddThh:mm:ss",
"duration": 162
}
}`
```
You can use the `value` key to filter records. Suppose you wanted to filter only those records where `device\_ID`
begins with the letters AB. The `FilterCriteria` object would be as follows.
```
`{
"Filters": [
{
"Pattern": "{ \\"value\\" : { \\"device\_ID\\" : [ { \\"prefix\\": \\"AB\\" } ] } }"
}
]
}`
```
For added clarity, here is the value of the filter's `Pattern` expanded in plain JSON.
```
`{
"value": {
"device\_ID": [ { "prefix": "AB" } ]
}
}`
```
You can add your filter using the console, AWS CLI or an AWS SAM template.
Console
To add this filter using the console, follow the instructions in [Attaching filter criteria to an event source mapping (console)](./invocation-eventfiltering.html#filtering-console) and enter the following
string for the **Filter criteria**.
```
`{ "value" : { "device\_ID" : [ { "prefix": "AB" } ] } }`
```
AWS CLI
To create a new event source mapping with these filter criteria using the AWS Command Line Interface (AWS CLI), run the following
command.
```
`aws lambda create-event-source-mapping \\
--function-name `my-function` \\
--event-source-arn `arn:aws:kafka:us-east-2:123456789012:cluster/my-cluster/b-8ac7cc01-5898-482d-be2f-a6b596050ea8` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"value\\" : { \\"device\_ID\\" : [ { \\"prefix\\": \\"AB\\" } ] } }"}]}'`
```
To add these filter criteria to an existing event source mapping, run the following command.
```
`aws lambda update-event-source-mapping \\
--uuid `"a1b2c3d4-5678-90ab-cdef-11111EXAMPLE"` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"value\\" : { \\"device\_ID\\" : [ { \\"prefix\\": \\"AB\\" } ] } }"}]}'`
```
AWS SAM
To add this filter using AWS SAM, add the following snippet to the YAML template for your event source.
```
`FilterCriteria:
Filters:
- Pattern: '{ "value" : { "device\_ID" : [ { "prefix": "AB" } ] } }'`
```
With Kafka, you can also filter records where the message is a plain string. Suppose you want to ignore those messages where the string is
"error". The `FilterCriteria` object would look as follows.
```
`{
"Filters": [
{
"Pattern": "{ \\"value\\" : [ { \\"anything-but\\": [ \\"error\\" ] } ] }"
}
]
}`
```
For added clarity, here is the value of the filter's `Pattern` expanded in plain JSON.
```
`{
"value": [
{
"anything-but": [ "error" ]
}
]
}`
```
You can add your filter using the console, AWS CLI or an AWS SAM template.
Console
To add this filter using the console, follow the instructions in [Attaching filter criteria to an event source mapping (console)](./invocation-eventfiltering.html#filtering-console) and enter the following
string for the **Filter criteria**.
```
`{ "value" : [ { "anything-but": [ "error" ] } ] }`
```
AWS CLI
To create a new event source mapping with these filter criteria using the AWS Command Line Interface (AWS CLI), run the following
command.
```
`aws lambda create-event-source-mapping \\
--function-name `my-function` \\
--event-source-arn `arn:aws:kafka:us-east-2:123456789012:cluster/my-cluster/b-8ac7cc01-5898-482d-be2f-a6b596050ea8` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"value\\" : [ { \\"anything-but\\": [ \\"error\\" ] } ] }"}]}'`
```
To add these filter criteria to an existing event source mapping, run the following command.
```
`aws lambda update-event-source-mapping \\
--uuid `"a1b2c3d4-5678-90ab-cdef-11111EXAMPLE"` \\
--filter-criteria '{"Filters": [{"Pattern": "{ \\"value\\" : [ { \\"anything-but\\": [ \\"error\\" ] } ] }"}]}'`
```
AWS SAM
To add this filter using AWS SAM, add the following snippet to the YAML template for your event source.
```
`FilterCriteria:
Filters:
- Pattern: '{ "value" : [ { "anything-but": [ "error" ] } ] }'`
```
Kafka messages must be UTF-8 encoded strings, either plain strings or in JSON format. That's because Lambda decodes Kafka byte arrays into UTF-8 before
applying filter criteria. If your messages use another encoding, such as UTF-16 or ASCII, or if the message format doesn't match the
`FilterCriteria` format, Lambda processes metadata filters only. The following table summarizes the specific behavior:
|Incoming message format|Filter pattern format for message properties|Resulting action|
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
Lambda filters (on the other metadata properties only) based on your filter criteria.
|
|
Valid JSON
|
Plain string
|
Lambda filters (on the other metadata properties only) based on your filter criteria.
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
|
Non-UTF-8 encoded string
|
JSON, plain string, or no pattern
|
Lambda filters (on the other metadata properties only) based on your filter criteria.
|
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Consumer group ID
Schema registries with event sources
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.