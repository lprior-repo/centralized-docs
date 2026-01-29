---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-use-lambda-cont-exec.html
title: Using a Lambda function to continue a new execution in Step Functions
word_count: 2444
filtered: true
elements_removed: 0
density_score: 0.83
---

Using a Lambda function to continue a new execution in Step Functions - AWS Step Functions
Using a Lambda function to continue a new execution in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-use-lambda-cont-exec)
[Prerequisites](#tutorial-continue-new-prereq)[Step 1: Create a Lambda function to iterate a count](#tutorial-continue-new-step-1)[Step 2: Create a Restart Lambda function to start a new Step Functions execution](#tutorial-continue-new-step-3)[Step 3: Create a state machine](#tutorial-continue-new-step-4)[Step 4: Update the IAM Policy](#tutorial-continue-new-step-2)[Step 5: Run the state machine](#tutorial-continue-new-step-5)
# Using a Lambda function to continue a new execution in Step Functions
###### Tip
The following approach uses a Lambda function to start a new workflow execution. We **recommend** using a Step Functions Task state to start new workflow executions. See how in the following tutorial: **
[Continue long-running workflows using Step Functions API (recommended)](./tutorial-continue-new.html)
**.
You can create a state machine that uses a Lambda function to start a new execution before the current execution terminates. With this approach to continue ongoing work in a new
execution, you can break large jobs into smaller workflows, or run a workflow indefinitely.
This tutorial builds on the concept of using an external Lambda function to modify your workflow, which was demonstrated in the [Iterate a loop with a Lambda function in Step Functions](./tutorial-create-iterate-pattern-section.html) tutorial. You use the same Lambda function (`Iterator`) to iterate a loop for a specific number of times. In
addition, you create another Lambda function to start a new execution of your workflow, and to decrement a count each time it starts a new execution. By setting the number of executions
in the input, this state machine ends and restarts an execution a specified number of times.
The state machine you'll create implements the following states.
|State|Purpose|
|
`ConfigureCount`
|
A `[Pass](./state-pass.html)` state that configures the `count`, `index`, and `step`
values that the `Iterator` Lambda function uses to step through iterations of work.
|
|
`Iterator`
|
A `[Task](./state-task.html)` state that references the `Iterator` Lambda function.
|
|
`IsCountReached`
|A `[Choice](./state-choice.html)` state that uses a Boolean value from the `Iterator` function to decide
whether the state machine should continue the example work, or move to the `ShouldRestart` state.|
|
`ExampleWork`
|A `Pass` state that represents the `Task` state that would perform work in an actual implementation.|
|
`ShouldRestart`
|A `[Choice](./state-choice.html)` state that uses the `executionCount` value to decide whether it should end
one execution and start another, or simply end. |
|
`Restart`
|A `[Task](./state-task.html)` state that uses a Lambda function to start a new execution of your state machine. Like the
`Iterator` function, this function also decrements a count. The `Restart` state passes the decremented value of the count to the input of the new
execution. |
## Prerequisites
Before you begin, go through the [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html) tutorial to ensure that you're
familiar with using Lambda and Step Functions together.
## Step 1: Create a Lambda function to iterate a count
###### Note
If you have completed the [Iterate a loop with a Lambda function in Step Functions](./tutorial-create-iterate-pattern-section.html) tutorial, you can skip this step
and use that Lambda function.
This section and the [Iterate a loop with a Lambda function in Step Functions](./tutorial-create-iterate-pattern-section.html) tutorial show how you can use a Lambda
function to track a count, for example, the number of iterations of a loop in your state machine.
The following Lambda function receives input values for `count`, `index`, and `step`. It returns these values with an updated `index`
and a Boolean named `continue`. The Lambda function sets `continue` to `true` if the `index` is less than `count`.
Your state machine then implements a `Choice` state that executes some application logic if `continue` is `true`, or moves on to
`ShouldRestart` if `continue` is `false`.
### Create the Iterate Lambda function
1. Open the [Lambda console](https://console.aws.amazon.com/lambda/home), and then choose **Create function**.
2. On the **Create function** page, choose **Author from scratch**.
3. In the **Basic information** section, configure your Lambda function, as follows:
1. For **Function name**, enter `Iterator`.
2. For **Runtime**, choose **Node.js 16.x**.
3. Keep all the default selections on the page, and then choose **Create function**.
When your Lambda function is created, make a note of its Amazon Resource Name (ARN) in the upper-right corner of the page, for example:
```
`arn:aws:lambda:`region`:123456789012:function:Iterator`
```
4. Copy the following code for the Lambda function into the **Code source** section of the **`Iterator`** page in
the Lambda console.
```
`exports.handler = function iterator (event, context, callback) {
let index = event.iterator.index;
let step = event.iterator.step;
let count = event.iterator.count;
index = index + step;
callback(null, {
index,
step,
count,
continue: index &lt; count
})
}`
```
This code accepts input values for `count`, `index`, and `step`. It increments the `index` by the value of `step`
and returns these values, and the Boolean value of `continue`. The value of `continue` is `true` if `index` is less than
`count`.
5. Choose **Deploy** to deploy the code.
### Test the Iterate Lambda function
To see your `Iterate` function working, run it with numeric values. You can provide input values for your Lambda function that mimic an iteration to see what output
you get with specific input values.
#### To test your Lambda
function
1. In the **Configure test event** dialog box, choose **Create new test event**, and then type `TestIterator` for
**Event name**.
2. Replace the example data with the following.
```
`{
"Comment": "Test my Iterator function",
"iterator": {
"count": 10,
"index": 5,
"step": 1
}
}
`
```
These values mimic what would come from your state machine during an iteration. The Lambda function increments the index and returns `continue` as
`true`. When the index is not less than the `count`, it returns `continue` as `false`. For this test, the index has
already incremented to `5`. The results should increment the `index` to `6` and set `continue` to `true`.
3. Choose **Create**.
4. On the `**Iterator**` page in your Lambda console, be sure **TestIterator** is listed, and then choose
**Test**.
The results of the test are displayed at the top of the page. Choose **Details** and review the result.
```
`{
"index": 6,
"step": 1,
"count": 10,
"continue": true
}
`
```
###### Note
If you set `index` to `9` for this test, the `index` increments to `10`, and `continue` is
`false`.
## Step 2: Create a Restart Lambda function to start a new Step Functions execution
1. Open the [Lambda console](https://console.aws.amazon.com/lambda/home), and then choose **Create function**.
2. On the **Create function** page, choose **Author from scratch**.
3. In the **Basic information** section, configure your Lambda function, as follows:
1. For **Function name**, enter `Restart`.
2. For **Runtime**, choose **Node.js 16.x**.
3. Keep all the default selections on the page, and then choose **Create function**.
When your Lambda function is created, make a note of its Amazon Resource Name (ARN) in the upper-right corner of the page, for example:
```
`arn:aws:lambda:`region`:123456789012:function:Iterator`
```
4. Copy the following code for the Lambda function into the **Code source** section of the **`Restart`** page in the
Lambda console.
The following code decrements a count of the number of executions, and starts a new execution of your state machine, including the decremented value.
```
`var aws = require('aws-sdk');
var sfn = new aws.StepFunctions();
exports.restart = function(event, context, callback) {
let StateMachineArn = event.restart.StateMachineArn;
event.restart.executionCount -= 1;
event = JSON.stringify(event);
let params = {
input: event,
stateMachineArn: StateMachineArn
};
sfn.startExecution(params, function(err, data) {
if (err) callback(err);
else callback(null,event);
});
}`
```
5. Choose **Deploy** to deploy the code.
## Step 3: Create a state machine
Now that you've created your two Lambda functions, create a state machine. In this state machine, the `ShouldRestart` and `Restart` states are how you break
your work across multiple executions.
###### Example ShouldRestart Choice state
The following excerpt shows the `ShouldRestart``[Choice](./state-choice.html)` state. This state
determines whether or not you should restart the execution.
```
`"ShouldRestart": {
"Type": "Choice",
"Choices": [
{
"Variable": "$.restart.executionCount",
"NumericGreaterThan": 1,
"Next": "Restart"
}
],`
```
The `$.restart.executionCount` value is included in the input of the initial execution. It's decremented by one each time the `Restart` function is called, and
then placed into the input for each subsequent execution.
###### Example Restart Task state
The following excerpt shows the `Restart``[Task](./state-task.html)` state. This state uses the Lambda
function you created earlier to restart the execution, and to decrement the count to track the remaining number of executions to start.
```
`"Restart": {
"Type": "Task",
"Resource": "`arn:aws:lambda:`region`:123456789012:function:Restart`",
"Next": "Done"
},`
```
###### To create the state machine
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), choose **State machines** from the menu, then choose **Create state machine**.
###### Important
Make sure that your state machine is under the same AWS account and Region as the Lambda functions you created earlier in [Step 1](#tutorial-continue-new-step-1) and [Step 2](#tutorial-continue-new-step-3).
2. Choose **Create from blank**.
3. Name your state machine, then choose **Continue** to edit your state machine in Workflow Studio.
4. For this tutorial, you'll write the [Amazon States Language](./concepts-amazon-states-language.html) (ASL) definition of your state machine in the [Code editor](./workflow-studio.html#wfs-interface-code-editor). To do this, choose **Code**.
5. Remove the existing boilerplate code and paste the following code. Remember to replace the ARNs in this code with the ARNs of the Lambda functions you created.
```
`{
"Comment": "Continue-as-new State Machine Example",
"StartAt": "ConfigureCount",
"States": {
"ConfigureCount": {
"Type": "Pass",
"Result": {
"count": 100,
"index": -1,
"step": 1
},
"ResultPath": "$.iterator",
"Next": "Iterator"
},
"Iterator": {
"Type": "Task",
"Resource": "`arn:aws:lambda:`region`:123456789012:function:Iterator`",
"ResultPath": "$.iterator",
"Next": "IsCountReached"
},
"IsCountReached": {
"Type": "Choice",
"Choices": [
{
"Variable": "$.iterator.continue",
"BooleanEquals": true,
"Next": "ExampleWork"
}
],
"Default": "ShouldRestart"
},
"ExampleWork": {
"Comment": "Your application logic, to run a specific number of times",
"Type": "Pass",
"Result": {
"success": true
},
"ResultPath": "$.result",
"Next": "Iterator"
},
"ShouldRestart": {
"Type": "Choice",
"Choices": [
{
"Variable": "$.restart.executionCount",
"NumericGreaterThan": 0,
"Next": "Restart"
}
],
"Default": "Done"
},
"Restart": {
"Type": "Task",
"Resource": "`arn:aws:lambda:`region`:123456789012:function:Restart`",
"Next": "Done"
},
"Done": {
"Type": "Pass",
"End": true
}
}
}`
```
6. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
For this tutorial, enter the name `ContinueAsNew`.
7. (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
For this tutorial, keep all the default selections in **State machine settings**.
If you've [previously created an IAM role](./procedure-create-iam-role.html) with the correct permissions for your state machine and want to use it, in **Permissions**, select **Choose an existing role**, and then select a role from the list. Or select **Enter a role ARN** and then provide an ARN for that IAM role.
8. In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
9. Save the Amazon Resource Name (ARN) of this state machine in a text file. You'll need to provide the ARN while providing permission to the Lambda function to start a new Step Functions execution.
## Step 4: Update the IAM Policy
To make sure your Lambda function has permissions to start a new Step Functions execution, attach an inline policy to the IAM role you use for your `Restart` Lambda function. For
more information, see [Embedding Inline Policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html#embed-inline-policy-console) in the
*IAM User Guide*.
###### Note
You can update the `Resource` line in the previous example to reference the ARN of your `ContinueAsNew` state machine. This restricts the policy so that
it can only start an execution of that specific state machine.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Sid": "VisualEditor0",
"Effect": "Allow",
"Action": [
"states:StartExecution"
],
"Resource": "arn:aws:states:us-east-2:`123456789012`:stateMachine:ContinueAsNew"
}
]
}`
`
```
## Step 5: Run the state machine
To start an execution, provide input that includes the ARN of the state machine and an `executionCount` for how many times it should start a new execution.
1. On the **ContinueAsNew** page, choose **Start execution**.
The **Start execution** dialog box is displayed.
2. In the **Start execution** dialog box, do the following:
1. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
2. In the **Input** box, enter the following JSON input to run your workflow.
```
`{
"restart": {
"StateMachineArn": "`arn:aws:states:`region`:`account-id`:stateMachine:ContinueAsNew`",
"executionCount": `4`
}
}
`
```
3. Update the `StateMachineArn` field with the ARN for your `ContinueAsNew` state machine.
4. Choose **Start execution**.
5. The Step Functions console directs you to a page that's titled with your execution ID. This page is known as the *Execution Details* page. On this page, you can review the execution results as the execution progresses or after it's complete.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
The **Graph view** displays the first of the four executions. Before it completes, it will pass through the `Restart` state and start a new
execution.
![Execution diagram showing the first execution out of four.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/execution-test1.png)
As this execution completes, you can look at the next execution that's running. Select the **ContinueAsNew** link at the top to see the list of executions. You
should see both the recently closed execution, and an ongoing execution that the `Restart` Lambda function started.
When all the executions are complete, you should see four successful executions in the list. The first execution that was started displays the name you chose, and subsequent
executions have a generated name.
![Illustrative screenshot showing all executions have completed.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/execution-test1-complete.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Continue long-running workflows using Step Functions API (recommended)
Access cross-account resources
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.