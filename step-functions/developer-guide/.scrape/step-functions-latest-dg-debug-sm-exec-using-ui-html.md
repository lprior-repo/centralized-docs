---
url: https://docs.aws.amazon.com/step-functions/latest/dg/debug-sm-exec-using-ui.html
title: Examining state machine executions in Step Functions
word_count: 1852
filtered: true
elements_removed: 0
density_score: 0.68
---

Examining state machine executions in Step Functions - AWS Step Functions
Examining state machine executions in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#debug-sm-exec-using-ui)
[Step 1: Create and test the required Lambda functions](#step-create-all-lambda-functions)[Step 2: Create and execute the state machine](#step-create-exec-sm)[Step 3: View the state machine execution details](#view-sm-exec-details)[Step 4: Explore the different View modes](#sm-exec-details-exp-view-modes)
# Examining state machine executions in Step Functions
In this tutorial, you will learn how to inspect the execution information displayed on the
*Execution Details* page and view the reason for a failed execution.
Then, you'll learn how to access different iterations of a `Map` state
execution. Finally, you'll learn how to configure the columns on the **Table
view** and apply suitable filters to view only the information of interest to
you.
In this tutorial, you create a Standard type state machine, which obtains the price of a
set of fruits. To do this, the state machine uses three AWS Lambda functions which return a
random list of four fruits, the price of each fruit, and the average cost of the fruits. The
Lambda functions are designed to throw an error if the price of the fruits is less than or
equal to a threshold value.
###### Note
While the following procedure contains instructions for how to examine the details of
a Standard workflow execution, you can also examine the details for Express workflow
executions. For information about the differences between the execution details for
Standard and Express workflow types, see [Standard and Express console experience differences](./concepts-view-execution-details.html#console-exp-differences).
## Step 1: Create and test the required Lambda functions
1. Open the [Lambda console](https://console.aws.amazon.com/lambda/home)
and then perform steps 1 through 4 in the [Step 1: Create a Lambda
function](./tutorial-creating-lambda-state-machine.html#create-lambda-function) section. Make sure to name the Lambda
function `GetListOfFruits`.
2. After you create your Lambda function, copy the function's Amazon Resource Name (ARN)
displayed in the upper-right corner of the page. To copy the ARN, click the copy icon to copy the Lambda function's Amazon Resource Name. The following is an example ARN, where
``function-name`` is the name of the Lambda
function (in this case, `GetListOfFruits`):
```
`arn:aws:lambda:`region`:123456789012:function:`function-name``
```
3. Copy the following code for the Lambda function into the **Code source** area of the
**GetListOfFruits** page.
```
`function getRandomSubarray(arr, size) {
var shuffled = arr.slice(0), i = arr.length, temp, index;
while (i--) {
index = Math.floor((i + 1) \* Math.random());
temp = shuffled[index];
shuffled[index] = shuffled[i];
shuffled[i] = temp;
}
return shuffled.slice(0, size);
}
exports.handler = async function(event, context) {
const fruits = ['Abiu','Açaí','Acerola','Ackee','African cucumber','Apple','Apricot','Avocado','Banana','Bilberry','Blackberry','Blackcurrant','Jostaberry'];
const errorChance = 45;
const waitTime = Math.floor( 100 \* Math.random() );
await new Promise( r =&gt;&gt; setTimeout(() =&gt;&gt; r(), waitTime));
const num = Math.floor( 100 \* Math.random() );
// const num = 51;
if (num &lt;&lt;= errorChance) {
throw(new Error('Error'));
}
return getRandomSubarray(fruits, 4);
};`
```
4. Choose **Deploy**, and then choose **Test**,
to deploy the changes and see the output of your Lambda function.
5. Create two additional Lambda functions, named
`GetFruitPrice` and `CalculateAverage`
respectively, with the following steps:
1. Copy the following code into the **Code source** area
of the **GetFruitPrice** Lambda function:
```
`exports.handler = async function(event, context) {
const errorChance = 0;
const waitTime = Math.floor( 100 \* Math.random() );
await new Promise( r =&gt;&gt; setTimeout(() =&gt;&gt; r(), waitTime));
const num = Math.floor( 100 \* Math.random() );
if (num &lt;&lt;= errorChance) {
throw(new Error('Error'));
}
return Math.floor(Math.random()\*100)/10;
};`
```
2. Copy the following code into the **Code source** area
of the **CalculateAverage** Lambda function:
```
`function getRandomSubarray(arr, size) {
var shuffled = arr.slice(0), i = arr.length, temp, index;
while (i--) {
index = Math.floor((i + 1) \* Math.random());
temp = shuffled[index];
shuffled[index] = shuffled[i];
shuffled[i] = temp;
}
return shuffled.slice(0, size);
}
const average = arr =&gt;&gt; arr.reduce( ( p, c ) =&gt;&gt; p + c, 0 ) / arr.length;
exports.handler = async function(event, context) {
const errors = [
"Error getting data from DynamoDB",
"Error connecting to DynamoDB",
"Network error",
"MemoryError - Low memory"
]
const errorChance = 0;
const waitTime = Math.floor( 100 \* Math.random() );
await new Promise( r =&gt;&gt; setTimeout(() =&gt;&gt; r(), waitTime));
const num = Math.floor( 100 \* Math.random() );
if (num &lt;&lt;= errorChance) {
throw(new Error(getRandomSubarray(errors, 1)[0]));
}
return average(event);
};`
```
3. Make sure to copy the ARNs of these two Lambda functions, and then
**Deploy** and **Test**
them.
## Step 2: Create and execute the state machine
Use the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/) to create a state machine that invokes the [Lambda functions you created in Step 1](#step-create-all-lambda-functions).
In this state machine, three `Map` states are defined. Each of these
`Map` states contains a `Task` state that invokes one of your
Lambda functions. Additionally, a `Retry` field is defined in each
`Task` state with a number of retry attempts defined for each state. If a
`Task` state encounters a runtime error, it's executed again up to the
number of retry attempts defined for that `Task`.
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home) and choose **Write your workflow in
code**.
###### Important
Ensure that your state machine is under the same AWS account and Region as the Lambda function you created
earlier.
2. For **Type**, keep the default selection of
**Standard**.
3. Copy the following Amazon States Language definition and paste it under
**Definition**. Make sure to replace the ARNs shown with those
of the Lambda functions that you previously created.
```
`{
"StartAt": "LoopOverStores",
"States": {
"LoopOverStores": {
"Type": "Map",
"Iterator": {
"StartAt": "GetListOfFruits",
"States": {
"GetListOfFruits": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"FunctionName": "arn:aws:lambda:``region``:`123456789012`:function:GetListofFruits:$LATEST",
"Payload": {
"storeName.$": "$"
}
},
"Retry": [
{
"ErrorEquals": [
"States.ALL"
],
"IntervalSeconds": 2,
"MaxAttempts": 1,
"BackoffRate": 1.3
}
],
"Next": "LoopOverFruits"
},
"LoopOverFruits": {
"Type": "Map",
"Iterator": {
"StartAt": "GetFruitPrice",
"States": {
"GetFruitPrice": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"FunctionName": "arn:aws:lambda:``region``:`123456789012`:function:GetFruitPrice:$LATEST",
"Payload": {
"fruitName.$": "$"
}
},
"Retry": [
{
"ErrorEquals": [
"States.ALL"
],
"IntervalSeconds": 2,
"MaxAttempts": 3,
"BackoffRate": 1.3
}
],
"End": true
}
}
},
"ItemsPath": "$",
"End": true
}
}
},
"ItemsPath": "$.stores",
"Next": "LoopOverStoreFruitsPrice",
"ResultPath": "$.storesFruitsPrice"
},
"LoopOverStoreFruitsPrice": {
"Type": "Map",
"End": true,
"Iterator": {
"StartAt": "CalculateAverage",
"States": {
"CalculateAverage": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"FunctionName": "arn:aws:lambda:``region``:`123456789012`:function:Calculate-average:$LATEST",
"Payload.$": "$"
},
"Retry": [
{
"ErrorEquals": [
"States.ALL"
],
"IntervalSeconds": 2,
"MaxAttempts": 2,
"BackoffRate": 1.3
}
],
"End": true
}
}
},
"ItemsPath": "$.storesFruitsPrice",
"ResultPath": "$.storesPriceAverage",
"MaxConcurrency": 1
}
}
}`
```
4. Enter a name for your state machine. Keep the default selections for the other
options on this page and choose **Create state
machine**.
5. Open the page titled with your state machine name. Perform steps 1 through 4
in the [Step 4: Run the state machine](./tutorial-creating-lambda-state-machine.html#start-lambda-function) section, but use the
following data as the execution input:
```
`{
"stores": [
"Store A",
"Store B",
"Store C",
"Store D"
]
}`
```
## Step 3: View the state machine execution details
On the page titled with your execution ID, you can review the results of your
execution and debug any errors.
1. (Optional) Choose from the tabs displayed on the *Execution
Details* page to see the information present in each of them. For
example, to view the state machine input and its execution output, choose
**Execution input &amp; output** on the *[Execution summary](./concepts-view-execution-details.html#exec-details-intf-exec-summ)*
section.
2. If your state machine execution failed, choose **Cause** or
**Show step detail** on the error message. Details about the
error are displayed in the *[Step details](./concepts-view-execution-details.html#exec-details-intf-step-details)* section.
Notice that the step that caused the error, which is a `Task` state
named **GetListofFruits**, is highlighted in the
**Graph view** and **Table view**.
###### Note
Because the **GetListofFruits** step is defined inside a
`Map` state, and the step failed to execute successfully, the
**Status** of `Map` state step is displayed
as **Failed**.
## Step 4: Explore the different *View modes*
You can choose a preferred mode to view either the state machine workflow or the
execution event history. Some of the tasks that you can perform in these *View
modes* are as follows:
If your **Map** state has five iterations and you want to
view the execution details for the third and fourth iterations, do the
following:
1. Choose the `Map` state that you want to view the
iteration data for.
2. From **Map iteration viewer**, choose the
iteration that you want to view. Iterations are counted from zero. To
choose the third iteration out of five, choose **#2**
from the dropdown list next to the **Map** state's
name.
###### Note
If your state machine contains nested `Map` states,
Step Functions displays the parent and child `Map` state
iterations as two separate dropdown lists representing the iteration
data for the nested states.
3. (Optional) If one or more of your `Map` state
iterations failed to execute or was stopped in an aborted state, you can
view details about the failed iteration. To see these details, choose
the affected iteration numbers under **Failed** or
**Aborted** in the dropdown list.
If your **Map** state has five iterations and you want to
view the execution details for the iteration number three and four, do the
following:
1. Choose the `Map` state for which you want to
view the different iteration data.
2. In the tree view display of the `Map` state iterations,
choose the row for iteration named **#2** for iteration
number three. Similarly, choose the row named **#3**
for iteration number four.
Choose the settings icon. Then, in the **Preferences**
dialog box, choose the columns you want to display under
**Select visible columns**.
By default, this mode displays the **Name**, **Type**,
**Status**, **Resource**, and **Started After**
columns.
Limit the amount of information displayed by applying one or more filters based on a property, such as
**Status**, or a date and time range. For example, to view the steps that failed execution,
apply the following filter:
1. Choose **Filter by properties or search by keyword**, and then choose
**Status** under **Properties**.
2. Under **Operators**, choose **Status =**.
3. Choose **Status = Failed**.
4. (Optional) Choose **Clear filters** to remove the applied filters.
Limit the amount of information displayed by applying one or more filters based on a property, such as
**Type**, or a date and time range. For example, to view the `Task` state steps that
failed execution, apply the following filter:
1. Choose **Filter by properties or search by keyword**, and then choose
**Type** under **Properties**.
2. Under **Operators**, choose **Type =**.
3. Choose **Type = TaskFailed**.
4. (Optional) Choose **Clear filters** to remove the applied filters.
Choose the arrow icon next to the ID of a **TaskFailed** event
to inspect its details, including input, output, and resource invocation that
appear in a dropdown box.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a state machine using AWS SAM
Create a state machine that uses Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.