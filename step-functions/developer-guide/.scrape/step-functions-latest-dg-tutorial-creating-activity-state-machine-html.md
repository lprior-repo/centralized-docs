---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-creating-activity-state-machine.html
title: Creating an Activity state
word_count: 1578
filtered: true
elements_removed: 0
density_score: 0.79
---

Creating an Activity state machine using Step Functions - AWS Step Functions
Creating an Activity state machine using Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-creating-activity-state-machine)
[Step 1: Create an
Activity](#create-activity-state-machine-step-1)[Step 2: Create a state
machine](#create-activity-state-machine-step-2)[Step 3: Implement a
Worker](#create-activity-state-machine-step-3)[Step 4: Run the state machine](#create-activity-state-machine-step-4)[Step 5: Run and Stop the
Worker](#create-activity-state-machine-step-5)
# Creating an Activity state
machine using Step Functions
This tutorial shows you how to create an activity-based state machine using Java and
AWS Step Functions. Activities allow you to control worker code that runs somewhere else from your state machine. For an overview, see [Learn about Activities in Step Functions](./concepts-activities.html) in [Learn about state machines in Step Functions](./concepts-statemachines.html).
To complete this tutorial, you need the following:
* The [SDK for Java](https://aws.amazon.com/sdk-for-java/). The example activity in this tutorial is a Java application that uses
the AWS SDK for Java to communicate with AWS.
* AWS credentials in the environment or in the standard AWS configuration file.
For more information, see [Set
Up Your AWS Credentials](https://docs.aws.amazon.com/AWSSdkDocsJava/latest/DeveloperGuide/set-up-creds.html) in the
*AWS SDK for Java Developer Guide*.
## Step 1: Create an
Activity
You must make Step Functions aware of the *activity* whose
*worker* (a program) you want to create. Step Functions responds with an
Amazon Resource Name(ARN) that establishes an identity for the activity. Use this identity to
coordinate the information passed between your state machine and worker.
###### Important
Ensure that your activity task is under the same AWS account as your state machine.
1. In the [Step Functions console](https://console.aws.amazon.com/states/home),
in the navigation pane on the left, choose
**Activities**.
2. Choose **Create activity**.
3. Enter a **Name** for the activity, for example,
``get-greeting``, and then
choose **Create activity**.
4. When your activity task is created, make a note of its ARN, as shown in
the following example.
```
`arn:aws:states:`region`:123456789012:activity:get-greeting`
```
## Step 2: Create a state
machine
Create a state machine that determines when your activity is invoked and when your
worker should perform its primary work, collect its results, and return them. To create the state machine, you'll use the [Code editor](./workflow-studio.html#wfs-interface-code-editor) of Workflow Studio.
1. In the [Step Functions console](https://console.aws.amazon.com/states/home),
in the navigation pane on the left, choose **State
machines**.
2. On the **State machines** page, choose **Create
state machine**.
3. Choose **Create from blank**.
4. Name your state machine, then choose **Continue** to edit your state machine in Workflow Studio.
5. For this tutorial, you'll write the [Amazon States Language](./concepts-amazon-states-language.html) (ASL) definition of your state machine in the code editor. To do this, choose **Code**.
6. Remove the existing boilerplate code and paste the following code. Remember to replace the example ARN in the `Resource`
field with the ARN of the activity task that you created earlier in [Step 1: Create an
Activity](#create-activity-state-machine-step-1).
```
`{
"Comment": "An example using a Task state.",
"StartAt": "getGreeting",
"Version": "1.0",
"TimeoutSeconds": 300,
"States":
{
"getGreeting": {
"Type": "Task",
"Resource": "`arn:aws:states:`region`:123456789012:activity:get-greeting`",
"End": true
}
}
}`
```
This is a description of your state machine using the [Amazon States Language](./concepts-amazon-states-language.html) (ASL). It defines a
single `Task` state named `getGreeting`. For more
information, see [State Machine
Structure](./statemachine-structure.html).
7. On the [Graph visualization](./workflow-studio.html#wfs-interface-code-graph-viz), make sure the workflow graph for the ASL definition you added looks similar to the following graph.
![Graph visualization of state machine with RunActivity task state.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/tutorial-create-state-machine-custom-preview.png)
8. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
For this tutorial, enter the name `ActivityStateMachine`.
9. (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
For this tutorial, keep all the default selections in **State machine settings**.
If you've [previously created an IAM role](./procedure-create-iam-role.html) with the correct permissions for your state machine and want to use it, in **Permissions**, select **Choose an existing role**, and then select a role from the list. Or select **Enter a role ARN** and then provide an ARN for that IAM role.
10. In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
## Step 3: Implement a
Worker
Create
a *worker*. A worker is a program that is responsible for:
* Polling Step Functions for activities using the `GetActivityTask` API
action.
* Performing the work of the activity using your code, (for example, the
`getGreeting()` method in the following code).
* Returning the results using the `SendTaskSuccess`,
`SendTaskFailure`, and `SendTaskHeartbeat` API
actions.
###### Note
For a more complete example of an activity worker, see [Example: Activity Worker in Ruby](./concepts-activities.html#example-ruby-activity-worker). This example provides an
implementation based on best practices, which you can use as a reference for your
activity worker. The code implements a consumer-producer pattern with a configurable
number of threads for pollers and activity workers.
### To implement the
worker
1. Create a file named `GreeterActivities.java`.
2. Add the following code to it.
```
`import com.amazonaws.ClientConfiguration;
import com.amazonaws.auth.EnvironmentVariableCredentialsProvider;
import com.amazonaws.regions.Regions;
import com.amazonaws.services.stepfunctions.AWSStepFunctions;
import com.amazonaws.services.stepfunctions.AWSStepFunctionsClientBuilder;
import com.amazonaws.services.stepfunctions.model.GetActivityTaskRequest;
import com.amazonaws.services.stepfunctions.model.GetActivityTaskResult;
import com.amazonaws.services.stepfunctions.model.SendTaskFailureRequest;
import com.amazonaws.services.stepfunctions.model.SendTaskSuccessRequest;
import com.amazonaws.util.json.Jackson;
import com.fasterxml.jackson.databind.JsonNode;
import java.util.concurrent.TimeUnit;
public class GreeterActivities {
public String getGreeting(String who) throws Exception {
return "{\\"Hello\\": \\"" + who + "\\"}";
}
public static void main(final String[] args) throws Exception {
GreeterActivities greeterActivities = new GreeterActivities();
ClientConfiguration clientConfiguration = new ClientConfiguration();
clientConfiguration.setSocketTimeout((int)TimeUnit.SECONDS.toMillis(70));
AWSStepFunctions client = AWSStepFunctionsClientBuilder.standard()
.withRegion(Regions.US\_EAST\_1)
.withCredentials(new EnvironmentVariableCredentialsProvider())
.withClientConfiguration(clientConfiguration)
.build();
while (true) {
GetActivityTaskResult getActivityTaskResult =
client.getActivityTask(
new GetActivityTaskRequest().withActivityArn(ACTIVITY\_ARN));
if (getActivityTaskResult.getTaskToken() != null) {
try {
JsonNode json = Jackson.jsonNodeOf(getActivityTaskResult.getInput());
String greetingResult =
greeterActivities.getGreeting(json.get("who").textValue());
client.sendTaskSuccess(
new SendTaskSuccessRequest().withOutput(
greetingResult).withTaskToken(getActivityTaskResult.getTaskToken()));
} catch (Exception e) {
client.sendTaskFailure(new SendTaskFailureRequest().withTaskToken(
getActivityTaskResult.getTaskToken()));
}
} else {
Thread.sleep(1000);
}
}
}
}`
```
###### Note
The `EnvironmentVariableCredentialsProvider` class in this
example assumes that the `AWS\_ACCESS\_KEY\_ID` (or
`AWS\_ACCESS\_KEY`) and `AWS\_SECRET\_KEY` (or
`AWS\_SECRET\_ACCESS\_KEY`) environment variables are set.
For more information about providing the required credentials to the
factory, see [AWSCredentialsProvider](https://docs.aws.amazon.com/AWSJavaSDK/latest/javadoc/com/amazonaws/auth/AWSCredentialsProvider.html) in the
*AWS SDK for Java API Reference* and [Set Up AWS Credentials and Region for Development](https://docs.aws.amazon.com/sdk-for-java/v1/developer-guide/setup-credentials.html) in the
*AWS SDK for Java Developer Guide*.
By default the AWS SDK will wait up to 50 seconds to receive data from
the server for any operation. The `GetActivityTask` operation
is a long-poll operation that will wait up to 60 seconds for the next
available task. To prevent receiving a
`SocketTimeoutException` error, set SocketTimeout to 70
seconds.
3. In the parameter list of the
`GetActivityTaskRequest().withActivityArn()` constructor,
replace the `ACTIVITY\_ARN` value with the ARN of the activity task that you created earlier in
[Step 1: Create an
Activity](#create-activity-state-machine-step-1).
## Step 4: Run the state machine
When you start the execution of the state machine, your worker polls Step Functions for
activities, performs its work (using the input that you provide), and returns its
results.
1. On the
**`ActivityStateMachine`**
page, choose **Start execution**.
The **Start execution** dialog box is displayed.
2. In the **Start execution** dialog box, do the following:
1. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
2. In the **Input** box, enter the following JSON input to run your workflow.
```
`{
"who": "AWS Step Functions"
}`
```
3. Choose **Start execution**.
4. The Step Functions console directs you to a page that's titled with your execution ID. This page is known as the *Execution Details* page. On this page, you can review the execution results as the execution progresses or after it's complete.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
## Step 5: Run and Stop the
Worker
To have the worker poll your state machine for activities, you must run the
worker.
1. On the command line, navigate to the directory in which you created
`GreeterActivities.java`.
2. To use the AWS SDK, add the full path of the `lib` and
`third-party` directories to the dependencies of your build
file and to your Java `CLASSPATH`. For more information, see
[Downloading and Extracting the SDK](https://docs.aws.amazon.com/sdk-for-java/v1/developer-guide/setup-install.html#download-and-extract-sdk) in the
*AWS SDK for Java Developer Guide*.
3. Compile the file.
```
`$ javac GreeterActivities.java`
```
4. Run the file.
```
`$ java GreeterActivities`
```
5. On the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/), navigate to the *Execution
Details* page.
6. When the execution completes, examine
the results of your execution.
7. Stop the worker.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create an API using API Gateway
View X-Ray traces
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.