---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-example.html
title: Example: Manipulating state data with paths in Step Functions workflows
word_count: 1099
filtered: true
elements_removed: 0
density_score: 0.81
---

Example: Manipulating state data with paths in Step Functions workflows - AWS Step Functions
Example: Manipulating state data with paths in Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-example)
[Filtering state output](#input-output-outputpath)
# Example: Manipulating state data with paths in Step Functions workflows
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
This topic contains examples of how to manipulate state input and output JSON using the InputPath, ResultPath, and OutputPath fields.
Any state other than a [Fail workflow state](./state-fail.html) state or a [Succeed workflow state](./state-succeed.html) state can include the input and output processing fields, such as `InputPath`, `ResultPath`, or `OutputPath`. Additionally, the [Wait workflow state](./state-wait.html) and [Choice workflow state](./state-choice.html) states don't support the `ResultPath` field.
With these fields, you can use a [JsonPath](https://datatracker.ietf.org/wg/jsonpath/about/) to filter the JSON data as it moves through your workflow.
You can also use the `Parameters` field to manipulate the JSON data as it moves through your workflow. For information about using `Parameters`, see
[Manipulate parameters in Step Functions workflows](./input-output-inputpath-params.html).
For example, start with the AWS Lambda function and state machine described in the [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html) tutorial. Modify the state machine so
that it includes the following `InputPath`, `ResultPath`, and
`OutputPath`.
```
`{
"Comment": "A Hello World example of the Amazon States Language using an AWS Lambda function",
"StartAt": "HelloWorld",
"States": {
"HelloWorld": {
"Type": "Task",
"Resource": "`arn:aws:lambda:`region`:123456789012:function:HelloFunction`",
"InputPath": `"$.lambda",`
"ResultPath": `"$.data.lambdaresult",`
"OutputPath": `"$.data",`
"End": true
}
}
}`
```
Start an execution using the following input.
```
`{
"comment": "An input comment.",
"data": {
"val1": 23,
"val2": 17
},
"extra": "foo",
"lambda": {
"who": "AWS Step Functions"
}
}
`
```
Assume that the `comment` and `extra` nodes can be discarded, but that
you want to include the output of the Lambda function, and preserve the information in the
`data` node.
In the updated state machine, the `Task` state is altered to process the input to
the task.
```
`"InputPath": "$.lambda",`
```
This line in the state machine definition limits the task input to only the
`lambda` node from the state input. The Lambda function receives only the JSON
object `{"who": "AWS Step Functions"}` as input.
```
`"ResultPath": "$.data.lambdaresult",`
```
This `ResultPath` tells the state machine to insert the result of the Lambda
function into a node named `lambdaresult`, as a child of the `data` node
in the original state machine input. Because you are not performing any other manipulation on the original input and the result using `OutputPath`, the output
of the state now includes the result of the Lambda function with the original input.
```
`{
"comment": "An input comment.",
"data": {
"val1": 23,
"val2": 17,
"lambdaresult": "Hello, AWS Step Functions!"
},
"extra": "foo",
"lambda": {
"who": "AWS Step Functions"
}
}`
```
But, our goal was to preserve only the `data` node, and include the result of the
Lambda function. `OutputPath` filters this combined JSON before passing it to the
state output.
```
`"OutputPath": "$.data",`
```
This selects only the `data` node from the original input (including the
`lambdaresult` child inserted by `ResultPath`) to be passed to the
output. The state output is filtered to the following.
```
`{
"val1": 23,
"val2": 17,
"lambdaresult": "Hello, AWS Step Functions!"
}`
```
In this `Task` state:
1. `InputPath` sends only the `lambda` node from the input to the
Lambda function.
2. `ResultPath` inserts the result as a child of the `data` node in
the original input.
3. `OutputPath` filters the state input (which now includes the result of the
Lambda function) so that it passes only the `data` node to the state
output.
###### Example to manipulate original state machine input, result, and final output using JsonPath
Consider the following state machine that verifies an insurance applicant's identity and address.
###### Note
To view the complete example, see [How to use JSON Path in Step Functions](https://github.com/aws-samples/serverless-account-signup-service).
```
`{
"Comment": "Sample state machine to verify an applicant's ID and address",
"StartAt": "Verify info",
"States": {
"Verify info": {
"Type": "Parallel",
"End": true,
"Branches": [
{
"StartAt": "Verify identity",
"States": {
"Verify identity": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"Parameters": {
"Payload.$": "$",
"FunctionName": "arn:aws:lambda:us-east-2:111122223333:function:`check-identity`:$LATEST"
},
"End": true
}
}
},
{
"StartAt": "Verify address",
"States": {
"Verify address": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"Parameters": {
"Payload.$": "$",
"FunctionName": "arn:aws:lambda:us-east-2:111122223333:function:`check-address`:$LATEST"
},
"End": true
}
}
}
]
}
}
}`
```
If you run this state machine using the following input, the execution fails because the Lambda functions that perform verification only expect the data that needs to
be verified as input. Therefore, you must specify the nodes that contain the information to be verified using an appropriate JsonPath.
```
`{
"data": {
"firstname": "Jane",
"lastname": "Doe",
"identity": {
"email": "jdoe@example.com",
"ssn": "123-45-6789"
},
"address": {
"street": "123 Main St",
"city": "Columbus",
"state": "OH",
"zip": "43219"
},
"interests": [
{
"category": "home",
"type": "own",
"yearBuilt": 2004
},
{
"category": "boat",
"type": "snowmobile",
"yearBuilt": 2020
},
{
"category": "auto",
"type": "RV",
"yearBuilt": 2015
},
]
}
}`
```
To specify the node that the ``check-identity`` Lambda function must use, use the `InputPath` field
as follows:
```
`"InputPath": "$.data.identity"`
```
And to specify the node that the ``check-address`` Lambda function must use, use the `InputPath` field as
follows:
```
`"InputPath": "$.data.address"`
```
Now if you want to store the verification result within the original state machine input, use the `ResultPath` field as follows:
```
`"ResultPath": "$.results"`
```
However, if you only need the identity and verification results and discard the original input, use the `OutputPath` field as follows:
```
`"OutputPath": "$.results"`
```
For more information, see [Processing input and output in Step Functions](./concepts-input-output-filtering.html).
## Filtering state output using OutputPath
With `OutputPath` you can select a portion of the state output to pass to the
next state. With this approach, you can filter out unwanted information, and pass only the portion of
JSON that you need.
If you don't specify an `OutputPath` the default value is `$`. This
passes the entire JSON node (determined by the state input, the task result, and
`ResultPath`) to the next state.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Manipulate parameters with paths
Specify state output with paths
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.