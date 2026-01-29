---
url: https://docs.aws.amazon.com/step-functions/latest/dg/connect-parameters.html
title: Passing parameters to a service API in Step Functions
word_count: 653
filtered: true
elements_removed: 0
density_score: 0.86
---

Passing parameters to a service API in Step Functions - AWS Step Functions
Passing parameters to a service API in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#connect-parameters)
[Pass static JSON as parameters](#connect-parameters-json)[Pass state input as parameters using Paths](#connect-parameters-path)[Pass Context object nodes as parameters](#connect-parameters-context)
# Passing parameters to a service API in Step Functions
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
Use the `Parameters` field in a `Task` state to control what parameters are passed to a service API.
Inside the `Parameters` field, you must use the plural form of the array parameters in an API action. For example, if you use the [Filter](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html#API_DescribeSnapshots_RequestParameters) field of the `DescribeSnapshots` API action for integrating with Amazon EC2, you must define the field as `Filters`. If you don't use the plural form, Step Functions returns the following error:
```
`The field Filter is not supported by Step Functions.`
```
## Pass static JSON as parameters
You can include a JSON object directly in your state machine definition to pass as a parameter to a resource.
For example, to set the `RetryStrategy` parameter for the `SubmitJob` API for AWS Batch, you could include the following in your parameters.
```
`"RetryStrategy": {
"attempts": 5
}`
```
You can also pass multiple parameters with static JSON. As a more complete example, the following are the `Resource` and `Parameters` fields of the specification of a task that
publishes
to an Amazon SNS topic named ``myTopic``.
```
`"Resource": "arn:aws:states:::sns:publish",
"Parameters": {
"TopicArn": "arn:aws:sns:us-east-2:`account-id`:`myTopic`",
"Message": "test message",
"MessageAttributes": {
"my attribute no 1": {
"DataType": "String",
"StringValue": "value of my attribute no 1"
},
"my attribute no 2": {
"DataType": "String",
"StringValue": "value of my attribute no 2"
}
}
},`
```
## Pass state input as parameters using Paths
You can pass portions of the state input as parameters by using [paths](./amazon-states-language-paths.html). A path is a string, beginning with `$`, that's used to identify components within JSON text. Step Functions paths use [JsonPath](https://datatracker.ietf.org/wg/jsonpath/about/) syntax.
To specify that a parameter use a path, end the parameter name with `.$`. For example, if your state input contains text within a node named `message`, you could pass that text as a parameter using a path.
Consider the following state input:
```
`{
"comment": "A message in the state input",
"input": {
"message": "foo",
"otherInfo": "bar"
},
"data": "example"
}`
```
To pass the value of the node named `message` as a parameter named `myMessage`, specify the following syntax:
```
`"Parameters": {"myMessage.$": "$.input.message"},`
```
Step Functions then passes the value `foo` as a parameter.
For more information about using parameters in Step Functions, see the following:
* [Processing input and output](./concepts-input-output-filtering.html)
* [Manipulate parameters in Step Functions workflows](./input-output-inputpath-params.html)
## Pass Context object nodes as parameters
In addition to static content, and nodes from the state input, you can pass nodes from the
Context object as parameters. The Context object is dynamic JSON data that exists during a
state machine execution. It includes information about your state machine and the current
execution. You can access the Context object using a path in the `Parameters`
field of a state definition.
For more information about the Context object and how to access that data from a
`"Parameters"` field, see the following:
* [Accessing execution data from the Context object
in Step Functions ](./input-output-contextobject.html)
* [Accessing the Context object](./input-output-contextobject.html#contextobject-access)
* [Get a Token from the Context object](./connect-to-resource.html#wait-token-contextobject)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Call HTTPS APIs
Integrating optimized services
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.