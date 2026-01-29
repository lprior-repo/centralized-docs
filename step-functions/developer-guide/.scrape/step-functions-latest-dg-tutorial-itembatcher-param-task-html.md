---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-itembatcher-param-task.html
title: Processing batch data with a Lambda function in Step Functions
word_count: 1033
filtered: true
elements_removed: 0
density_score: 0.82
---

Processing batch data with a Lambda function in Step Functions - AWS Step Functions
Processing batch data with a Lambda function in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-itembatcher-param-task)
[Step 1: Create the state machine](#itembatcher-param-task-create-state-machine)[Step 2: Create the Lambda function](#itembatcher-param-task-config-resource)[Step 3: Run the state machine](#itembatcher-param-task-run-state-machine)
# Processing batch data with a Lambda function in Step Functions
In this tutorial, you use the *Distributed Map state*'s [ItemBatcher (Map)](./input-output-itembatcher.html) field to process an entire batch of items inside a Lambda function. Each batch contains a maximum of three items. The *Distributed Map state* starts four child workflow executions, where each execution processes three items, while one execution processes a single item. Each child workflow execution invokes a Lambda function that iterates over the individual items present in the batch.
You'll create a state machine that performs multiplication on an array of integers. Say that the integer array you provide as input is `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]` and the multiplication factor is `7`. Then, the resulting array formed after multiplying these integers with a factor of 7, will be `[7, 14, 21, 28, 35, 42, 49, 56, 63, 70]`.
## Step 1: Create the state machine
In this step, you create the workflow prototype of the state machine that passes an entire batch of data to the Lambda function you'll create in [Step 2](#itembatcher-param-task-config-resource).
* Use the following definition to create a state machine using the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/). For information about creating a state machine, see [Step 1: Create the workflow prototype](./tutorial-map-distributed.html#use-dist-map-create-workflow) in the [Getting started with using Distributed Map state](./tutorial-map-distributed.html) tutorial.
In this state machine, you define a *Distributed Map state* that accepts an array of 10 integers as input and passes this array to a Lambda function in batches of `3`. The Lambda function iterates over the individual items present in the batch and returns an output array named `multiplied`. The output array contains the result of the multiplication performed on the items passed in the input array.
###### Important
Make sure to replace the Amazon Resource Name (ARN) of the Lambda function in the following code with the ARN of the function you'll create in [Step 2](#itembatcher-param-task-config-resource).
```
`{
"StartAt": "Pass",
"States": {
"Pass": {
"Type": "Pass",
"Next": "Map",
"Result": {
"MyMultiplicationFactor": 7,
"MyItems": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}
},
"Map": {
"Type": "Map",
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "DISTRIBUTED",
"ExecutionType": "STANDARD"
},
"StartAt": "Lambda Invoke",
"States": {
"Lambda Invoke": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"Payload.$": "$",
"FunctionName": "arn:aws:lambda:`region`:`account-id`:function:`functionName`"
},
"Retry": [
{
"ErrorEquals": [
"Lambda.ServiceException",
"Lambda.AWSLambdaException",
"Lambda.SdkClientException",
"Lambda.TooManyRequestsException"
],
"IntervalSeconds": 2,
"MaxAttempts": 6,
"BackoffRate": 2
}
],
"End": true
}
}
},
"End": true,
"Label": "Map",
"MaxConcurrency": 1000,
"ItemBatcher": {
"MaxItemsPerBatch": 3,
"BatchInput": {
"MyMultiplicationFactor.$": "$.MyMultiplicationFactor"
}
},
"ItemsPath": "$.MyItems"
}
}
}`
```
## Step 2: Create the Lambda function
In this step, you create the Lambda function that processes all the items passed in the batch.
###### Important
Ensure that your Lambda function is under the same AWS Region as your state machine.
###### To create the Lambda function
1. Use the [Lambda console](https://console.aws.amazon.com/lambda/home) to create a **Python** Lambda function named `ProcessEntireBatch`. For information about creating a Lambda function, see [Step 4: Configure the Lambda function](./tutorial-map-distributed.html#use-dist-map-config-resource) in the [Getting started with using Distributed Map state](./tutorial-map-distributed.html) tutorial.
2. Copy the following code for the Lambda function and paste it into the **Code source** section of your Lambda function.
```
`import json
def lambda\_handler(event, context):
multiplication\_factor = event['BatchInput']['MyMultiplicationFactor']
items = event['Items']
results = [multiplication\_factor \* item for item in items]
return {
'statusCode': 200,
'multiplied': results
}`
```
3. After you create your Lambda function, copy the function's ARN displayed in the upper-right corner of the page. The following is an example ARN, where ``function-name`` is the name of the
Lambda function (in this case, `ProcessEntireBatch`):
```
`arn:aws:lambda:`region`:123456789012:function:`function-name``
```
You'll need to provide the function ARN in the state machine you created in [Step 1](#itembatcher-param-task-create-state-machine).
4. Choose **Deploy** to deploy the changes.
## Step 3: Run the state machine
When you run the [state machine](#itembatcher-param-task-create-state-machine), the *Distributed Map state* starts four child workflow executions, where each execution processes three items, while one execution processes a single item.
The following example shows the data passed to the [ProcessEntireBatch](#itembatcher-param-task-config-resource) function by one of the child workflow executions.
```
`{
"BatchInput": {
"MyMultiplicationFactor": 7
},
"Items": [1, 2, 3]
}`
```
Given this input, the following example shows the output array named `multiplied` that is returned by the Lambda function.
```
`{
"statusCode": 200,
"multiplied": [7, 14, 21]
}`
```
The state machine returns the following output that contains four arrays named `multiplied` for the four child workflow executions. These arrays contain the multiplication results of the individual input items.
```
`[
{
"statusCode": 200,
"multiplied": [7, 14, 21]
},
{
"statusCode": 200,
"multiplied": [28, 35, 42]
},
{
"statusCode": 200,
"multiplied": [49, 56, 63]
},
{
"statusCode": 200,
"multiplied": [70]
}
]`
```
To combine all the array items returned into a single output array, you can use the [ResultSelector](./input-output-inputpath-params.html#input-output-resultselector) field. Define this field inside the *Distributed Map state* to find all the `multiplied` arrays, extract all the items inside these arrays, and then combine them into a single output array.
To use the `ResultSelector` field, update your state machine definition as shown in the following example.
```
`{
"StartAt": "Pass",
"States": {
...
...
"Map": {
"Type": "Map",
...
...
"ItemsPath": "$.MyItems",
**"ResultSelector": {
"multiplied.$": "$..multiplied[\*]"
}**
}
}
}`
```
The updated state machine returns a consolidated output array as shown in the following example.
```
`{
"multiplied": [7, 14, 21, 28, 35, 42, 49, 56, 63, 70]
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Iterate a loop with Lambda
Process individual items with Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.