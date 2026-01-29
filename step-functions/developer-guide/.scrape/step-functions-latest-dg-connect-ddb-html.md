---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-ddb.html
title: Perform DynamoDB CRUD operations with Step Functions
word_count: 550
filtered: true
elements_removed: 0
density_score: 0.83
---

Perform DynamoDB CRUD operations with Step Functions - AWS Step Functions
Perform DynamoDB CRUD operations with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-ddb)
[Supported APIs](#connect-dynamodb-api)[IAM policies](#dynamo-iam)
# Perform DynamoDB CRUD operations with Step Functions
You can integrate Step Functions with DynamoDB to perform CRUD operations on a DynamoDB table. This page lists the supported
DynamoDB APIs and provides an example `Task` state to retrieve an item from DynamoDB.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of optimized DynamoDB integration
* There is no specific optimization for the [Request Response](./connect-to-resource.html#connect-default) integration pattern.
* [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token)
integration pattern is not supported.
* Only [`GetItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_GetItem.html), [`PutItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html), [`UpdateItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html), and [`DeleteItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteItem.html) API actions are available
through optimized integration. Other API actions, such as [`CreateTable`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_CreateTable.html) are available using the
DynamoDB AWS SDK integration.
The following is an example `Task` state that retrieves a message from DynamoDB.
```
`"Read next Message from DynamoDB": {
"Type": "Task",
"Resource": "arn:aws:states:::dynamodb:getItem",
"Arguments": {
"TableName": "`DYNAMO\_DB\_TABLE\_NAME`",
"Key": {
"MessageId": {"S": "{% $List[0] %}"}
}
}`
```
To see this state in a working example, see the [Transfer data records with Lambda, DynamoDB,
and Amazon SQS](./sample-project-transfer-data-sqs.html) starter template.
###### Exception prefix differences
When standard DynamoDB connections experience an error, the exception prefix will be `DynamoDb` (mixed case).
For optimized integrations, the exception prefix will be `DynamoDB` (uppercase `DB`).
###### Quota for input or result data
When sending or receiving data between services, the maximum input or result for a task is 256 KiB of data as a UTF-8 encoded string. See [Quotas related to state
machine executions](./service-quotas.html#service-limits-state-machine-executions).
## Optimized DynamoDB APIs
* [`GetItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_GetItem.html)
* [`PutItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html)
* [`DeleteItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteItem.html)
* [`UpdateItem`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html)
###### Parameters in Step Functions are expressed in PascalCase
Even if the native service API is in camelCase, for example the API action `startSyncExecution`, you specify parameters in PascalCase, such as: `StateMachineArn`.
## IAM policies for calling DynamoDB
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
*Static resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"dynamodb:GetItem",
"dynamodb:PutItem",
"dynamodb:UpdateItem",
"dynamodb:DeleteItem"
],
"Resource": [
"arn:aws:dynamodb:`us-east-1`:`123456789012`:table/myTableName"
]
}
]
}
`
`
```
*Dynamic resources*
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"dynamodb:GetItem",
"dynamodb:PutItem",
"dynamodb:UpdateItem",
"dynamodb:DeleteItem"
],
"Resource": "\*"
}
]
}`
`
```
For more information about the IAM policies for all DynamoDB API actions, see
[IAM policies with DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/using-identity-based-policies.html) in the
*Amazon DynamoDB Developer Guide*. Additionally, for information about the IAM policies for PartiQL for DynamoDB, see
[IAM policies with PartiQL for DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ql-iam.html) in the
*Amazon DynamoDB Developer Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS CodeBuild
Amazon ECS/Fargate
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.