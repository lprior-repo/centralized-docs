---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-create-iterate-pattern-section.html
title: Iterate a loop with a Lambda function in Step Functions
word_count: 1379
filtered: true
elements_removed: 0
density_score: 0.80
---

Iterate a loop with a Lambda function in Step Functions - AWS Step Functions
Iterate a loop with a Lambda function in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-create-iterate-pattern-section)
[Step 1: Create a Lambda function to
iterate a count](#create-iterate-pattern-step-1)[Step 2: Test the Lambda Function](#create-iterate-pattern-step-2)[Step 3: Create a State Machine](#create-iterate-pattern-step-3)[Step 4: Start a New Execution](#create-iterate-pattern-step-4)
# Iterate a loop with a Lambda function in Step Functions
In this tutorial, you implement a design pattern that uses a state machine and an
AWS Lambda function to iterate a loop a specific number of times.
Use this design pattern any time you need to keep track of the number of loops in a state
machine. This implementation can help you break up large tasks or long-running executions
into smaller chunks, or to end an execution after a specific number of events. You can use a
similar implementation to periodically end and restart a long-running execution to avoid
exceeding service quotas for AWS Step Functions, AWS Lambda, or other AWS services.
Before you begin, go through the [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html) tutorial to ensure
you are familiar with using Lambda and Step Functions together.
## Step 1: Create a Lambda function to
iterate a count
By using a Lambda function you can track the number of iterations of a loop in your
state machine. The following Lambda function receives input values for
`count`, `index`, and `step`. It returns these values
with an updated `index` and a Boolean value named `continue`. The
Lambda function sets `continue` to `true` if the `index`
is less than `count`.
Your state machine then implements a `Choice` state that executes some
application logic if `continue` is `true`, or exits if it is
`false`.
### To create the Lambda
function
1. Sign in to the [Lambda
console](https://console.aws.amazon.com/lambda/home), and then choose **Create
function**.
2. On the **Create function** page, choose
**Author from scratch**.
3. In the **Basic information** section, configure your
Lambda function, as follows:
1. For **Function name**, enter
`Iterator`.
2. For **Runtime**, choose **Node.js**.
3. In **Change default execution role**, choose **Create a new role with basic Lambda permissions**.
4. Choose **Create function**.
5. Copy the following code for the Lambda function into the **Code source**.
```
`export const handler = function (event, context, callback) {
let index = event.iterator.index
let step = event.iterator.step
let count = event.iterator.count
index = index + step
callback(null, {
index,
step,
count,
continue: index &lt; count
})
}`
```
This code accepts input values for `count`, `index`, and `step`. It increments the `index` by the value of
`step` and returns these values, and the Boolean `continue`. The value of `continue` is `true` if
`index` is less than `count`.
6. Choose **Deploy**.
## Step 2: Test the Lambda Function
Run your Lambda function with numeric values to see it in operation. You can provide input values for your Lambda function that mimic an iteration.
### To test your Lambda function
1. Choose **Test**.
2. In the **Configure test event** dialog box, enter `TestIterator` in the **Event name** box.
3. Replace the example data with the following.
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
These values mimic what would come from your state machine during an
iteration. The Lambda function will increment the index and return
`true` for `continue` when the index is less
than `count`. For this test, the index has already incremented to
`5`. The test will increment `index` to
`6` and set `continue` to
`true`.
4. Choose **Create**.
5. Choose **Test** to test your Lambda function.
The results of the test are displayed in the **Execution results** tab.
6. Choose the **Execution results** tab to see the output.
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
If you set `index` to `9` and test again, the `index` increments to `10`, and `continue` will be `false`.
###### Before you leave the Lambda console…
Copy the Lambda function ARN. Paste it into a note. You'll need it in the next step.
Next, you will create a state machine with the following states:
* `ConfigureCount` – Sets default values for `count`, `index`, and `step`.
* `Iterator` – Refers to the Lambda function you created earlier, passing in the values configured in `ConfigureCount`.
* `IsCountReached` – A choice state that continues the loop or proceeds to `Done` state, based on the value returned from your `Iterator` function.
* `ExampleWork` – A stub for work that needs to be done. In this example, the workflow has a `Pass` state, but in a real solution, you would likely use a `Task`.
* `Done` – End state of your workflow.
To create the state machine in the console:
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), and then choose **Create a state machine**.
###### Important
Your state machine must be in the same AWS account and Region as your Lambda function.
2. Select the **Blank** template.
3. In the **Code** pane, paste the following JSON which defines the state machine.
For more information about the Amazon States Language,
see [State Machine Structure](./statemachine-structure.html).
```
`{
"Comment": "Iterator State Machine Example",
"StartAt": "ConfigureCount",
"States": {
"ConfigureCount": {
"Type": "Pass",
"Result": {
"count": 10,
"index": 0,
"step": 1
},
"ResultPath": "$.iterator",
"Next": "Iterator"
},
"Iterator": {
"Type": "Task",
"Resource": "`arn:aws:lambda:`region`:123456789012:function:Iterate`",
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
"Default": "Done"
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
"Done": {
"Type": "Pass",
"End": true
}
}
}
`
```
4. Replace the `Iterator Resource` field with the ARN for your `Iterator` Lambda function that you created earlier.
5. Select **Config**, and enter a **Name** for your state machine, such as ``IterateCount``.
###### Note
Names of state machines, executions, and activity tasks must not exceed 80 characters in length. These names must be unique for your account and AWS Region,
and must not contain any of the following:
* Whitespace
* Wildcard characters (`? \*`)
* Bracket characters (`&lt; &gt; { } [ ]`)
* Special characters (`" # % \\ ^ | \~ ` $ &amp;&amp; , ; : /`)
* Control characters (`\\\\u0000` - `\\\\u001f` or `\\\\u007f` - `\\\\u009f`).
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
* For **Type**, accept default value of **Standard**. For **Permissions**, choose **Create new role**.
* Choose **Create**, and then **Confirm** the role creations.
## Step 4: Start a New Execution
After you create your state machine, you can start an execution.
1. On the **IterateCount** page, choose **Start execution**.
2. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
3. Choose **Start Execution**.
A new execution of your state machine starts, showing your running
execution.
![State machine graph showing blue iterator state indicating in progress status.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/tutorial-create-iterate-running.png)
The execution increments in steps, tracking the count using your Lambda
function. On each iteration, it performs the example work referenced in the `ExampleWork` state in your state machine.
When the count reaches the number specified in the `ConfigureCount` state in your state machine, the execution
quits iterating and ends.
![State machine graph showing Iterator and Done state in green to indicate both have succeeded.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/tutorial-create-iterate-done.png)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Copy large-scale CSV using Distributed Map
Process batch data with Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.