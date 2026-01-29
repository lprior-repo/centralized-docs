---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-lambda.html
title: Invoke an AWS Lambda function with Step Functions
word_count: 678
filtered: true
elements_removed: 0
density_score: 0.82
---

Invoke an AWS Lambda function with Step Functions - AWS Step Functions
Invoke an AWS Lambda function with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-lambda)
[Supported APIs](#connect-lambda-api)[Examples](#connect-lambda-api-examples)[Directly specified function resource](#w2aac33c40c13)[IAM policies](#lambda-iam)
# Invoke an AWS Lambda function with Step Functions
Learn how to use Step Functions to invoke Lambda functions either synchronously or asynchronously as part of an event-driven serverless application.
To learn about integrating with AWS services in Step Functions, see [Integrating services](./integrate-services.html) and [Passing parameters to a service API in Step Functions](./connect-parameters.html).
###### Key features of Optimized Lambda integration
* The `Payload` field of the response is parsed from escaped Json to
Json.
* If an exception is raised within the Lambda function, the Task will fail. For a
practical example, see [Handling error conditions in a Step Functions
state machine](./tutorial-handling-error-conditions.html).
## Workflow Examples
The following includes a `Task` state that invokes a Lambda function.
```
`{
"StartAt":"CallLambda",
"States":{
"CallLambda":{
"Type":"Task",
"Resource":"arn:aws:states:::lambda:invoke",
"Arguments":{
"FunctionName":"arn:aws:lambda:`region`:`account-id`:function:`MyFunction`"
},
"End":true
}
}
}`
```
The following includes a `Task` state that implements the [callback](./connect-to-resource.html#connect-wait-token) service integration pattern.
```
`{
"StartAt":"GetManualReview",
"States":{
"GetManualReview":{
"Type":"Task",
"Resource":"arn:aws:states:::lambda:invoke**.waitForTaskToken**",
"Arguments":{
"FunctionName":"arn:aws:lambda:`region`:`account-id`:function:`get-model-review-decision`",
"Payload":{
"model":"{% $states.input.my-model %}",
"TaskToken": "{% $states.context.Task.Token %}"
},
"Qualifier":"prod-v1"
},
"End":true
}
}
}
`
```
When you invoke a Lambda function, the execution will wait for the function to complete. If you invoke the Lambda function with a callback task, the heartbeat timeout
does not start counting until after the Lambda function has completed executing and returned a result. As long as the Lambda function executes, the heartbeat
timeout is not enforced.
It is also possible to call Lambda asynchronously using the `InvocationType` parameter, as seen in the following example:
```
`{
"Comment": "A Hello World example of the Amazon States Language using Pass states",
"StartAt": "Hello",
"States": {
"Hello": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"Arguments": {
"FunctionName": "arn:aws:lambda:`region`:`account-id`:function:`echo`",
"InvocationType": "Event"
},
"End": true
}
}
}
`
```
###### Note
For asynchronous invocations of Lambda functions, the heartbeat timeout period starts immediately.
When the `Task` result is returned, the function output is nested inside a dictionary of metadata.
For example:
```
`{
"ExecutedVersion":"$LATEST",
"Payload":"`FUNCTION OUTPUT`",
"SdkHttpMetadata":{
"HttpHeaders":{
"Connection":"keep-alive",
"Content-Length":"4",
"Content-Type":"application/json",
"Date":"Fri, 26 Mar 2021 07:42:02 GMT",
"X-Amz-Executed-Version":"$LATEST",
"x-amzn-Remapped-Content-Length":"0",
"x-amzn-RequestId":"0101aa0101-1111-111a-aa55-1010aaa1010",
"X-Amzn-Trace-Id":"root=1-1a1a000a2a2-fe0101aa10ab;sampled=0"
},
"HttpStatusCode":200
},
"SdkResponseMetadata":{
"RequestId":"6b3bebdb-9251-453a-ae45-512d9e2bf4d3"
},
"StatusCode":200
}`
```
## Directly specified function resource
Alternatively, you can invoke a Lambda function by specifying a function ARN directly in the "Resource" field. When you invoke a Lambda function in this way, you can't specify `.waitForTaskToken`, and the task result contains only the function output.
```
`{
"StartAt":"CallFunction",
"States":{
"CallFunction": {
"Type":"Task",
"Resource":"arn:aws:lambda:`region`:`account-id`:function:`HelloFunction`",
"End": true
}
}
} `
```
With this form of integration, the function could succeed yet send a response that contains a `FunctionError` field. In that scenario, the workflow Task will fail.
You can invoke a specific Lambda function version or alias by specifying those options
in the ARN in the `Resource` field. See the following in the Lambda
documentation:
* [AWS Lambda
versioning](https://docs.aws.amazon.com/lambda/latest/dg/versioning-intro.html)
* [AWS Lambda aliases](https://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html)
## IAM policies for calling AWS Lambda
The following example templates show how AWS Step Functions generates IAM policies based on the resources in your state machine definition. For more information, see [How Step Functions generates IAM policies for integrated
services](./service-integration-iam-templates.html) and [Discover service integration patterns in Step Functions](./connect-to-resource.html).
In the following example, a state machine with two AWS Lambda task states which call `function1` and `function2`, the autogenerated policy includes `lambda:Invoke` permission for both functions.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"lambda:InvokeFunction"
],
"Resource": [
"arn:aws:lambda:`us-east-1`:`123456789012`:function:myFn1",
"arn:aws:lambda:`us-east-1`:`123456789012`:function:myFn2"
]
}
]
}
`
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS Glue DataBrew
AWS Elemental MediaConvert
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.