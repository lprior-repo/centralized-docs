---
url: https://docs.aws.amazon.com/step-functions/latest/dg/getting-started.html
title: Learn how to get started with Step Functions
word_count: 3384
filtered: true
elements_removed: 0
density_score: 0.85
---

Learn how to get started with Step Functions - AWS Step Functions
Learn how to get started with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#getting-started)
[What you will build](#what-you-will-build)[Step 1 - Create your state machine](#step-1-create-your-state-machine)[Step 2 - Start your state machine](#step-2-start-your-state-machine)[Step 3 - Process external input](#step-3-process-external-input)[Step 4 - Integrate a service](#step-4-integrate-a-service)[Clean up resources](#clean-up-resources)
# Learn how to get started with Step Functions
With the Step Functions service, you can orchestrate complex application workflows. To get started, you'll use Workflow Studio to create and run a built-in **Hello World** workflow. You'll review the auto-generated [Amazon States Language](./concepts-amazon-states-language.html) (ASL) definition in code. Finally, you'll drag-and-drop a service integration to do sentiment analysis.
After you complete this tutorial, you'll know how to use Workflow Studio to create, configure, run, and update a
workflow using both the **Design** and **Code** modes.
Estimated duration: **20-30 minutes**
## What you will build
Your first state machine will start with *flow states*. Flow states are
used to direct and control your workflow. After you learn how to run the workflow, you will add an **Action** to integrate the Amazon Comprehend service with a **Task state**.
The following diagram shows a visual of the complete state machine that you will build.
When you first create the Hello World state machine, it will not need additional resources to run. The
Step Functions console will create all the states and an IAM role in a single click. Later, when you add the service integration, you will need to create a role with a custom permission policy.
![Visual representation of the Hello World workflow.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello-world-arch.png)
## Step 1 - Create your state machine
In Step Functions, *workflows* are called **state machines**. We'll use both terms interchangeably. Your workflows will contain *states* that either **take action** or **control the flow** of your state machines.
1. Go to the **Step Functions console.**
2. In the Step Functions console, choose "**Step Functions**" from the upper left navigation, or the breadcrumbs, then choose **Get started**:
![Illustrative screenshot showing how to get started with the Hello World workflow](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/get-started-hello-world.png)
3. From the options, choose **Run Hello World**:
![Illustrative screenshot showing how to choose the Hello World workflow](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/create-hello-world.png)
###### Tip
We recommend stepping through the short in-console walk through to become familiar with the UI.
### Overview of Workflow Studio
With Workflow Studio for Step Functions, you can visually drag-and-drop states onto a canvas to build workflows.
You can add and edit states, configure steps, transform results, and set up error handling. The following screenshot shows four important areas of the interface that you will use to build your state machines.
![Illustrative screenshot of the four important areas of the Workflow Studio interface](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/wfs-panel-overview.png)
**Modes** - Workflow Studio provides three modes of operation and defaults to the visual design mode.
* **Design** - a visual editing mode, where you can drag-and-drop states into your workflow.
* **Code** - a mode that focuses on the Amazon States Language code, also known as ASL code. You can edit ASL code directly and see changes reflected in the visual design.
* **Config** - configuration options including the name and type of the state machine (Standard or Express), assigned role when the workflow runs, logging, tracing, versioning, encryption, and tags.
**States browser** contains the following three tabs:
* **Actions** - a list of AWS APIs that you can drag-and-drop into your workflow. Each action represents a Task workflow state.
* **Flow** - flow states to control the order of steps in your workflow.
* **Patterns** - ready-to-use, reusable building blocks, such as iteratively processing data in an Amazon S3 bucket.
**Canvas and workflow graph** is where you drag-and-drop states on to your workflow graph, change the order of states, and select states to configure and test.
**Inspector panel** is where you view and edit the properties of any state selected on the canvas. You can turn on the *Definition* toggle to show the code for the currently selected state.
### Overview of the state machine
The Hello World workflow starts with a **Pass state** which *passes* its input to its output, without performing work. Pass states can be used to generate static JSON output or transform JSON input before passing the data to the next state. Pass states are useful when constructing and debugging state machines.
The next state, a **Choice state**, uses the data in `IsHelloWorldExample` to choose the next branch of the workflow. If the first rule matches, the workflow pauses in a **Wait state**, then runs two tasks in a **Parallel state**, before moving on to a checkpoint and the successful end of the workflow. When there is no match, the workflow defaults to the **Fail state** before stopping the state machine.
Wait states can be useful when you want to delay before performing more work. Perhaps your workflow will wait 30 seconds after an order entry, so your customer has time to notice and fix an incorrect shipping address.
Parallel states can run multiple processes on your data. Perhaps the workflow will print an order ticket, update inventory, and increase a daily sales report simultaneously.
![Illustrative image of the getting started Hello World workflow](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/workflow-1-hello-world.png)
### View the workflow code (ASL)
Your first state machine is in fact quite detailed, so explore further by reviewing the code.
State machines are defined using [Amazon States Language (ASL)](https://states-language.net/), an open source specification that describes a JSON-based language to describe state machines declaratively.
**To view the entire state machine definition
**
1. Choose the **{ } Code** button to view the ASL code.
2. View the code on the left and compare with the state machine graph on the right.
3. Select some states on the canvas to review. For example, pick the **Choice state**.
![Illustrative image of code view](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello-code-view.png)
Did you notice how the state's definition is highlighted in the code view?
**To view code in the Inspector **
1. Switch back to **Design** mode.
2. Expand the **Inspector panel** on the right.
3. Select the **Choice state** from the workflow graph on the Canvas.
4. In the **Inspector** panel, choose the **Definition** toggle.
Try choosing other states. See how the ASL code for each state you select is scrolled into view and highlighted?
###### Warning: name your state machine now!
You **cannot rename** a state machine after you create it. Choose a name **before** you save your state machine.
Until now, you've been working on a draft of your state machine. No resources have been created yet.
**To rename and create your state machine**
1. Choose **Config mode**.
2. For state machine name, enter `MyFirstStateMachine`
3. For permissions, accept the default to *Create a new role*.
4. Choose the **Create** button to **actually** create your state machine.
You should see notifications that your state machine and a new IAM role have been created.
You will be automatically presented with the option to start the state machine. You'll do that in the next step!
![Illustrative image of the getting started Hello World workflow](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/workflow-created.png)
###### Workflow creation achieved!
Step Functions created your workflow and IAM role. Now, you are ready to start your state machine.
## Step 2 - Start your state machine
After your state machine has been created, you can start your workflow running.
Workflows optionally take **Input** that can be used in the state, sent to integrated services, and passed to the next state.
The **Hello World** state machine is self-contained and does not need input.
![Illustrative image of getting started](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/start_283486381.jpg)
**To start the state machine**
1. Enter `hello001` for the name of the execution.
2. Leave the input field *empty*.
3. Choose the **Start execution** button.
![Illustrative image for starting a workflow.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello001.png)
### Review the execution details
Immediately after starting, you should see the first two states have **succeeded**.
After a short wait, the rest of the state transitions will run to complete the workflow.
Are you wondering how the **Choice state** (*Is Hello World Example?*) decided to branch to the **Wait for X Seconds** state?
1. Hint: the first step in the state machine contains the data needed for the branch decision
2. In the **Graph View**, you can monitor progress during execution and explore details for each state.
3. Select the first **Pass state** (named *Set Variables and State Output*), then review the **Input/Output** tab.
You should see that **State input** is blank, but **State output** contains JSON that sets the value of `IsHelloWorldExample` to `true`.
![Execution 001](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello001-graph-view.png)
Switch from the **Graph view** to the **Table view** to see a list of states by name, type, and status.
![Execution 001 table view](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello001-table.png)
###### Tip
Take note of the **Duration** and **Timeline** fields in the previous screenshot. At a glance, you can see which states take more time than others.
There are two more views to explore on this Executions Details page: **Event view** and **State view**.
The **Event view** is a detailed granular view of the flow from state to state.
Expand the first **PassStateEntered** and **PassStateExited** events in the **Event View** table to see how the state takes no input, assigns a variable called `CheckpointCount` the value of zero, and produces the output you saw previously.
![Execution 001 event view](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello001-event-view.png)
Lastly, you have the **State view** which is similar to the **Table view**. In the **State view** table, you can selectively expand **states** to see just the Inputs and Outputs for each state:
![Execution 001 state view](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello001-state-view.png)
###### Congratulations! You've run your first Step Functions state machine!
Using a Pass state to add **static data** into a workflow is a common pattern, especially for troubleshooting.
In the next step, you'll update the workflow so you can *dynamically* set your state machine input.
## Step 3 - Process external input
Setting the value of `IsHelloWorldExample` to a constant value inside the workflow is not realistic. You should expect your state machine to respond to varying input data.
In this step, we'll show you how external JSON data can be used as input to your workflow:
![Process external input](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/process-external-input.png)
### Remove the hard-coded input
First, replace the hard-coded value in the **Output** of the first Pass state.
1. Edit your Hello World state machine by selecting the **Edit state machine** button located at the top right of the page.
2. Select the first **Pass state** after **Start** (named *Set Variables and State Output*), then select the **Output** tab.
3. Replace the **Output** with following JSON:
```
`{
"IsHelloWorldExample": "{% $states.input.hello\_world %}",
"ExecutionWaitTimeInSeconds": "{% $states.input.wait %}"
}`
```
4. Save the state machine.
The updated state output will pull input data from the reserved **[$states](./transforming-data.html#transforming-reserved-variable-states)** variable using a JSONata expression. Those values will be passed to the next state as output to become the input for the next state.
### Run the updated workflow, with input data
Next, run the workflow and provide external input data as JSON.
1. Choose the **Execute** button to run the workflow.
2. For the **Name**, use the randomly generated ID.
3. Use the following JSON for the input field:
```
`{
"wait" : 20,
"hello\_world": true
}`
```
4. Choose the **Start execution** button.
Your state machine execution should wait a lot longer (20 seconds), but eventually it should succeed using the input you provided.
In the Graph view, review the **Input/Output** for the first Pass State. Notice how the input you provided was converted into outputs. Also, take a look at the **Execution input and output** at the top of the execution details page. Both locations show the input that you used to start the execution.
###### Tip
What do you expect if you run a new execution with *hello\_world* set to **false**? Try it!
### Review workflow executions
Now that you've run your workflow a few times, review the execution details to review runs of your workflow.
**To review execution details**
1. Choose **State machines** from the navigation breadcrumbs or left-hand menu.
2. Choose your state machine.
In the **Executions** tab, you should see a list of executions, similar to the following screenshot:
![Illustrative screenshot that shows a sample list of hello workflow executions.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/hello-executions.png)
One final note: workflow execution names must be unique and **cannot** be reused. Although we suggested a short name (`hello001`) in this tutorial, we recommend using a naming convention that will always be unique for your production workloads.
###### Tip
**Congratulations!** You've modified your workflow to process *external input* that can vary every time you run your workflow.
## Step 4 - Integrate a service
Step Functions state machines can call over 220 AWS services using [AWS SDK integrations](https://docs.aws.amazon.com/step-functions/latest/dg/supported-services-awssdk.html). AWS services provide over 10,000 potential API actions for your state machines.
In this step, you will integrate an Amazon Comprehend task for **sentiment analysis** to process your state machine input.
Service integrations use one of three *service integration patterns*:
1. **Request a Response** (default) - wait for HTTP response, then *immediately* proceed to the next state.
2. **Run a Job** (*.sync*) - wait for a job to complete before moving to the next step.
3. **Wait for Callback** (*.waitForTaskToken*) - pause a workflow until a task token is returned by an external process.
![Illustrative screenshot depicting a service integration.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/AdobeStock_513621530_integration_1000.jpg)
For your first integration, you will use the **Request Response** (default) integration pattern.
### How do integrations work?
A *Task state* represents a single unit of work performed by a state machine. All work in your state machine is done by tasks.
A task typically performs work by passing input to the API actions of other services which then perform their own work. You can specify how a Task performs, using a number of fields including: `Credentials`, `Retry`, `Catch`, `TimeoutSeconds`, and more. You can learn more about Tasks in [Task workflow state](./state-task.html).
To use AWS SDK integrations, you specify the **service name** and **API** to call. Some integrations also require parameters.
You can use Amazon States Language to specify an AWS API action in the **Resource** field of a task state. You may optionally add a service integration type to the service name.
To specify an API action, you will use the following resource name template:
```
`arn:aws:states:::aws-sdk:serviceName:apiAction.[serviceIntegrationPattern]`
```
###### Parameter name case
Note that API actions will be *camelCase* (lowercase initial), but *ParameterNames* will be Pascal case (Uppercase initial).
**Examples of resource names**
* `arn:aws:states:::aws-sdk:ec2:describeInstances` will return the results from calling the Amazon EC2 describeInstances API.
* `arn:aws:states:::aws-sdk:s3:listBuckets` will return the results from calling the Amazon S3 listBuckets API.
* `arn:aws:states:::aws-sdk:sfn:startExecution` will start a nested Step Functions state machine execution and return the results of that workflow.
When Step Functions calls another service using the `Task` state, the default pattern is [Request Response](https://docs.aws.amazon.com/step-functions/latest/dg/connect-to-resource.html#connect-default). With the **Request Response** integration pattern, Step Functions calls a service, receives a response, and ***immediately*** proceeds to the next state.
![Sentiment detection integration](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/integrate_sentiment_592194331.png)
### Step 4.1 - Add sentiment analysis state
1. **Edit** your **MyFirstStateMachine** state machine.
2. From the **Actions** panel in the **States browser**, search for `DetectSentiment`.
3. Drag &amp; drop **Comprehend DetectSentiment** onto the **Default** branch of the **Choice state**.
4. Select and delete the **Fail** state.
5. From the **Flow tab** in the **States browser**, drag the **Success** state after **DetectSentiment**.
### Step 4.2 - Configure the sentiment analysis state
1. Select the **Comprehend** step to configure it in the Inspector panel.
2. Select the **Arguments &amp; Output** tab, then replace the **Arguments** with the following JSON:
```
`{
"LanguageCode": "en",
"Text": "{% %}"
}`
```
3. Place your cursor **between** the percent signs: `{% %}` and type: `$`
4. Use **auto-complete** in the editor to choose `states`,
then type `.` and choose `context`,
then type `.` and choose `Execution`,
then type `.` and choose `Input`,
finally, type `.feedback\_comment` to retrieve initial input from the **Context Object**.
After choosing those auto-complete options, you should have the following JSON for your states **Arguments**:
```
`{
"LanguageCode": "en",
"Text": "{% $states.context.Execution.Input.feedback\_comment %}"
}
`
```
###### Using editor auto-complete
With editor auto-complete, you can explore your options.
Auto-complete will list your variables, the reserved **[$states](./transforming-data.html#transforming-reserved-variable-states)** variable which contains the context object, and available functions with their definitions!
### Step 4.3 - Configure an identity policy
Before you can run the workflow, you need to create a **role** and **policy** to allow the state machine to perform API calls to the external service.
**To create an IAM role for Step Functions**
1. Go to the IAM console in a new tab and select **Roles**.
2. Choose **Create a new role**.
3. For **Trusted entity type** choose `AWS Service`.
4. For **Use case** choose `Step Functions`.
5. For **Add permissions** choose **Next** to accept the default policy. You will add a policy for Comprehend after creating the role.
6. For **Name**, enter `HelloWorldWorkflowRole`.
7. Choose **Create role**.
**To add a policy to the **HelloWorldWorkflowRole** for Amazon Comprehend**
1. Select and edit the **HelloWorldWorkflowRole** role.
2. Choose **Add permission** then **Create inline policy**.
3. Select **Comprehend** for the service.
4. In **Read** choose **DetectSentiment**, then **Next**
5. For **Policy name** enter `DetectSentimentPolicy`, then **Create policy**.
If you review the policy, you'll see it allows all resources to take the **Action** `"comprehend:DetectSentiment"`.
**To attach the IAM role to the Step Functions state machine**
1. Return to editing your state machine and select the **Config** tab.
2. From the **Execution role** dropdown, choose `HelloWorldWorkflowRole`.
3. Save your state machine.
### Step 4.4 - Run your state machine
Start executing your state machine with the following JSON for input:
```
`{
"hello\_world": false,
"wait": 42,
"feedback\_comment" : "This getting started with Step Functions workshop is a challenge!"
}
`
```
Without the correct policy, you will receive a **permissions error**, similar to the following:
```
`User: arn:aws:sts::`account-id`:assumed-role/StepFunctions-MyStateMachine-role is not authorized
to perform: comprehend:DetectSentiment because no identity-based policy allows the comprehend:DetectSentiment
action (Service: Comprehend, Status Code: 400, Request ID: a1b2c3d4-5678-90ab-cdef-EXAMPLE11111)
`
```
The previous error message is telling you that your state machine is not authorized to use the external service. Go back a step and make sure you have configured an identity policy.
**Practice what you've learned!**
Before you dive into more complex workflows, practice what you've learned with the following tasks:
*
Review the **DetectSentiment** step. Take a look at the input/output in the various views to see the results of sentiment detection.
* Find the **duration** of the DetectSentiment state in the table view.
* Change the comment in the **JSON input**, then re-run your state machine.
To learn more about sentiment analysis results, see [Amazon Comprehend - Sentiment](https://docs.aws.amazon.com/comprehend/latest/dg/how-sentiment.html).
One way to think about Request Response integration is the response generally represents only an *acknowledgement* of the request. However, in some integrations, such as sentiment analysis, the acknowledgement actually represents *completion* of the task.
The key learning is the `Task` state does **not wait** for the underlying job in Request Response integrations. To wait for a response, you'll need to explore the *Run a Job (.sync)* service integration pattern.
###### Congratulations!
You created your first state machine and integrated a sentiment analysis task using the **Request Response** pattern.
###### We value your feedback!
If you found this getting started tutorial helpful, or you have suggestions to improve the tutorial, let us know by using the feedback options on this page.
## Clean up resources
Take the following steps to clean up the resources you created:
1. Navigate to the [Step Functions](https://console.aws.amazon.com/states/home) page in the AWS Console.
2. Select **State machines** from the navigation pane on the left.
3. Choose the **MyFirstStateMachine**
4. To delete the IAM roles
1 - Follow the link for the **IAM role** to go to the IAM role page in a new tab. Delete the custom related role.
2 - In IAM Roles, search for the auto-generated role containing `MyFirstStateMachine`. Delete the auto-generated role.
5. Return to your Step Functions console tab and select the **Actions** drop down, then select **Delete** to delete the state machine.
Your state machine and related role should now be deleted successfully.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use cases
State machines
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.