---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-creating-lambda-state-machine.html
title: Creating a Step Functions state machine that uses Lambda
word_count: 1314
filtered: true
elements_removed: 0
density_score: 0.87
---

Creating a Step Functions state machine that uses Lambda - AWS Step Functions
Creating a Step Functions state machine that uses Lambda - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-creating-lambda-state-machine)
[Step 1: Create a Lambda
function](#create-lambda-function)[Step 2: Test the Lambda
function](#test-lambda-function)[Step 3: Create a state
machine](#create-state-machine-step)[Step 4: Run the state machine](#start-lambda-function)
# Creating a Step Functions state machine that uses Lambda
In this tutorial, you will create a single-step workflow using AWS Step Functions to invoke an AWS Lambda function.
###### Note
Step Functions is based on *state machines* and *tasks*. In Step Functions, state machines are called *workflows*, which are a series of event-driven steps. Each step in a workflow is called a *state*. For example, a [Task state](./state-task.html) represents a unit of work that another AWS service performs, such as calling another AWS service or API. Instances of running workflows performing tasks are called *executions* in Step Functions.
For more information, see:
* [What is Step Functions?](./welcome.html)
* [Integrating services with Step Functions](./integrate-services.html)
Lambda is well-suited for `Task` states, because Lambda functions are *serverless* and easy to write. You can write code in the
AWS Management Console or your favorite editor. AWS handles the details of providing a computing environment for your function and running it.
## Step 1: Create a Lambda
function
Your Lambda function receives event data and returns a greeting message.
###### Important
Ensure that your Lambda function is under the same AWS account and AWS Region as your state machine.
1. Open the [Lambda
console](https://console.aws.amazon.com/lambda/home) and choose **Create function**.
2. On the **Create function** page, choose
**Author from scratch**.
3. For **Function name**, enter `HelloFunction`.
4. Keep the default selections for all other options, and then choose **Create function**.
5. After your Lambda function is created, copy the function's Amazon Resource Name (ARN) displayed in the upper-right corner of the page. The following is an example ARN:
```
`arn:aws:lambda:`region`:123456789012:function:`HelloFunction``
```
6. Copy the following code for the Lambda function into the **Code source** section of the
**`HelloFunction`**
page.
```
`export const handler = async(event, context, callback) =&gt; {
callback(null, "Hello from " + event.who + "!");
};`
```
This code assembles a greeting using the `who` field of the
input data, which is provided by the `event` object passed into
your function. You add input data for this function later, when you [start a new
execution](#start-lambda-function). The `callback` method returns the assembled
greeting from your function.
7. Choose **Deploy**.
## Step 2: Test the Lambda
function
Test your Lambda function to see it in operation.
1. Choose **Test**.
2. For **Event name**, enter `HelloEvent`.
3. Replace the **Event JSON** data with the following.
```
`{
"who": "AWS Step Functions"
}`
```
The `"who"` entry corresponds to the `event.who`
field in your Lambda function, completing the greeting. You will input the same
input data when you run your state machine.
4. Choose **Save** and then choose **Test**.
5. To review the test results, under **Execution result**, expand **Details**.
## Step 3: Create a state
machine
Use the Step Functions console to create a state machine that invokes
the Lambda function that you created in [Step 1](#create-lambda-function).
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), choose **State machines** from the menu, then choose **Create state machine**.
###### Important
Make sure that your state machine is under the same AWS account and Region as the Lambda function you created earlier.
2. Choose **Create from blank**.
3. Name your state machine, then choose **Continue** to edit your state machine in Workflow Studio.
4. In the [States browser](./workflow-studio.html#workflow-studio-components-states) on the left, make sure you've chosen the **Actions** tab. Then, drag and drop the **AWS Lambda Invoke** API into the empty state labelled **Drag first state here**.
5. In the [Inspector](./workflow-studio.html#workflow-studio-components-formdefinition) panel on the right, configure the Lambda function:
1. In the **API Parameters** section, choose [the Lambda
function that you created earlier](#create-lambda-function) in the **Function name** dropdown list.
2. Keep the default selection in the **Payload** dropdown list.
3. (Optional) Choose **Definition** to view the state machine's [Amazon States Language](./concepts-amazon-states-language.html) (ASL) definition, which is automatically generated
based on your selections in the **Actions** tab and ** Inspector** panel.
4. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
For example, enter the name `LambdaStateMachine`.
###### Note
Names of state machines, executions, and activity tasks must not exceed 80 characters in length. These names must be unique for your account and AWS Region,
and must not contain any of the following:
* Whitespace
* Wildcard characters (`? \*`)
* Bracket characters (`&lt; &gt; { } [ ]`)
* Special characters (`" # % \\ ^ | \~ ` $ &amp;&amp; , ; : /`)
* Control characters (`\\\\u0000` - `\\\\u001f` or `\\\\u007f` - `\\\\u009f`).
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
* (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
For this tutorial, keep all the default selections in **State machine settings**.
* Choose **Create**.
* In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
## Step 4: Run the state machine
After you create your state machine, you can run it.
1. On the **State machines** page, choose **LambdaStateMachine**.
2. Choose **Start execution**.
The **Start execution** dialog box is displayed.
3. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
4. In the **Input** area, replace the example execution data with the
following.
```
`{
"who" : "AWS Step Functions"
}`
```
`"who"` is the key name that your Lambda function uses to get the
name of the person to greet.
5. Choose **Start Execution**.
Your state machine's execution starts, and a new page showing your
running execution is displayed.
6. The Step Functions console directs you to a page that's titled with your execution ID. This page is known as the *Execution Details* page. On this page, you can review the execution results as the execution progresses or after it's complete.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
###### Note
You can also pass payloads while invoking Lambda from a state machine. For more information and examples about invoking Lambda by passing payload in the
`Parameters` field, see [Invoke an AWS Lambda function with Step Functions](./connect-lambda.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Examine executions
Wait for human approval
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.