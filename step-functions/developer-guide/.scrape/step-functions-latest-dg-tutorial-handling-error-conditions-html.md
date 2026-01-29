---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-handling-error-conditions.html
title: Handling error conditions in a Step Functions
word_count: 1013
filtered: true
elements_removed: 0
density_score: 0.83
---

Handling error conditions in a Step Functions state machine - AWS Step Functions
Handling error conditions in a Step Functions state machine - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-handling-error-conditions)
[Step 1: Create a Lambda
function that throws an error](#create-lambda-function-fail)[Step 2: Test your Lambda
function](#error-conditions-test)[Step 3: Create your state machine
machine](#state-machine-create-step)[Step 4: Configure your state machine](#state-machine-configure)[Step 5: Run the state machine](#error-conditions-execution)
# Handling error conditions in a Step Functions
state machine
In this tutorial, you create an AWS Step Functions state machine with a **Task** state that invokes an example Lambda function built to throw a custom error.
Tasks are one of the [Fallback states](./concepts-error-handling.html#error-handling-fallback-states), for which you can configure a `Catch` field. When errors are received by the integration, next steps are chosen by the catch field based on the error name.
## Step 1: Create a Lambda
function that throws an error
Use a Lambda function to simulate an error condition.
1. Open the AWS Lambda console at
[https://console.aws.amazon.com/lambda/](https://console.aws.amazon.com/lambda/).
2. Choose **Create function**.
3. Choose **Use a blueprint**, search for
`Step Functions`, and choose **Throw a custom error**.
4. For **Function name**, enter `ThrowErrorFunction`.
5. For **Role**, choose **Create a new role with basic Lambda permissions**.
6. Choose **Create function**.
The following code should be displayed in the **code** pane.
```
`export const handler = async () =&gt; {
function CustomError(message) {
this.name = 'CustomError';
this.message = message;
}
CustomError.prototype = new Error();
throw new CustomError('This is a custom error!');
};
`
```
## Step 2: Test your Lambda
function
Before creating a state machine, verify your Lambda function throws your `CustomError` when invoked.
1. Choose the **Test** tab.
2. Choose **Create a new event** and keep the default **Event JSON**
3. Choose **Test** to invoke your function with your test event.
4. Expand **Executing function** to review the details of the thrown error.
You now have a Lambda function ready to throw a custom error.
In the next step, you will set up a state machine to catch and retry on that error.
## Step 3: Create your state machine
machine
Use the Step Functions console to create a state machine that uses a [Task workflow state](./state-task.html)
with a `Catch` configuration. The state machine will invoke the Lambda function, which you've built to simulate throwing an error when the function is invoked. Step Functions retries the function using exponential backoff between retries.
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), choose **State machines** from the menu, then choose **Create state machine**.
2. Choose **Create from blank**, and for **State machine name**, enter `CatchErrorStateMachine`.
3. Accept the default type (Standard), then choose **Continue** to edit your state machine in Workflow Studio.
4. Choose **Code** to switch to the ASL editor, then replace the code with following state machine definition:
```
`{
"Comment": "Example state machine that can catch a custom error thrown by a function integration.",
"StartAt": "CreateAccount",
"States": {
"CreateAccount": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"Output": "{% $states.result.Payload %}",
"Arguments": {
"FunctionName": "arn:aws:lambda:`region`:`account-id`:function:ThrowErrorFunction:$LATEST",
"Payload": "{% $states.input %}"
},
"Catch": [
{
"ErrorEquals": [
"CustomError"
],
"Next": "CustomErrorFallback"
},
{
"ErrorEquals": [
"States.ALL"
],
"Next": "CatchAllFallback"
}
],
"End": true,
"Retry": [
{
"ErrorEquals": [
"CustomError",
"Lambda.ServiceException",
"Lambda.AWSLambdaException",
"Lambda.SdkClientException",
"Lambda.TooManyRequestsException"
],
"IntervalSeconds": 1,
"MaxAttempts": 3,
"BackoffRate": 2,
"JitterStrategy": "FULL"
}
]
},
"CustomErrorFallback": {
"Type": "Pass",
"End": true,
"Output": {
"Result": "Fallback from a custom error function."
}
},
"CatchAllFallback": {
"Type": "Pass",
"End": true,
"Output": {
"Result": "Fallback from all other error codes."
}
}
},
"QueryLanguage": "JSONata"
}`
```
## Step 4: Configure your state machine
Before you run your state machine, you must first connect to the Lambda function that you previously created.
1. Switch back to **Design** mode and select the **Lambda : Invoke** task state named **CreateAccount**.
2. On the **Configuration** tab, look for **API Arguments**. For **Function name** choose the Lambda function that you created earlier.
3. Choose **Create**, review the roles, then choose **Confirm** to create your state machine.
## Step 5: Run the state machine
After you create and configure your state machine, you can run it and examine the flow.
1. In the editor, choose **Execute**.
Alternatively, from the **State machines** list, choose **Start execution**.
2. In the **Start execution** dialog box, accept the generated ID, and for **Input**, enter the following JSON:
```
`{ "Cause" : "Custom Function Error" }`
```
3. Choose **Start execution**.
The Step Functions console directs you to a page that's titled with your execution ID, known as the *Execution Details* page. You can review the execution results as the workflow progresses and after it completes.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
Your state machine invokes the Lambda function, which throws a `CustomError`. Choose the **CreateAccount** step in the **Graph view** to see the state output. Your state machine output should look similar to the following illustration:
![Illustrative screenshot of the workflow catching the custom error.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/tutorial-console-retry-state-machine-error-output.png)
**Congratulations!**
You now have a state machine that can catch and handle error conditions thrown by a Lambda function. You can use this pattern to implement robust error handling in your workflows.
###### Note
You can also create state machines that [Retry](./concepts-error-handling.html#error-handling-retrying-after-an-error) on timeouts or those that
use `Catch` to transition to a specific state when an error or timeout
occurs. For examples of these error handling techniques, see [Examples Using Retry and Using
Catch](./concepts-error-handling.html#error-handling-examples).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorials and Workshops
Create a state machine using AWS SAM
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.