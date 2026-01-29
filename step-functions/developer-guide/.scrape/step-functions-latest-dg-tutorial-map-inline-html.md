---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-map-inline.html
title: Using Inline Map state to repeat an action in Step Functions
word_count: 1263
filtered: true
elements_removed: 0
density_score: 0.81
---

Using Inline Map state to repeat an action in Step Functions - AWS Step Functions
Using Inline Map state to repeat an action in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-map-inline)
[Step 1: Create the workflow prototype](#use-inline-map-create-workflow)[Step 2: Configure input and output](#use-inline-map-configure-io)[Step 3: Review and save auto-generated definition](#use-inline-map-review-asl-def)[Step 4: Run the state machine](#use-inline-map-sm-run)
# Using Inline Map state to repeat an action in Step Functions
This tutorial helps you get started with using the `Map` state in Inline mode.
You use the *Inline Map state* in your workflows to repeatedly perform an action. For more
information about Inline mode, see [Map
state in Inline mode](./state-map-inline.html).
In this tutorial, you use the *Inline Map state* to repeatedly generate version 4 universally
unique identifiers (v4 UUID). You start by creating a workflow that contains two [Pass workflow state](./state-pass.html) states and an *Inline Map state* in the
Workflow Studio. Then, you configure the input and output, including the input JSON array for the
`Map` state. The `Map` state returns an output array that contains
the v4 UUIDs generated for each item in the input array.
## Step 1: Create the workflow prototype
In this step, you create the prototype for your workflow using Workflow Studio. Workflow Studio is a
visual workflow designer available in the Step Functions console. You’ll choose the required
states from the **Flow** tab and use the drag and drop feature of Workflow Studio
to create the workflow prototype.
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), choose **State machines** from the menu, then choose **Create state machine**.
2. Choose **Create from blank**.
3. Name your state machine, then choose **Continue** to edit your state machine in Workflow Studio.
4. From the **Flow** tab, drag a **Pass** state and drop it to the empty state labelled **Drag first state here**.
5. Drag a **Map** state and drop it below the **Pass** state. Rename the **Map** state to `Map demo`.
6. Drag a second **Pass** state and drop it inside of the **Map demo** state.
7. Rename the second **Pass** state to `Generate UUID`.
## Step 2: Configure input and output
In this step, you configure input and output for all the states in your workflow
prototype. First, you inject some fixed data into the workflow using the first
**Pass** state. This **Pass** state passes on this
data as input to the **Map demo** state. Within this input, you specify
the node that contains the input array the **Map demo** state should
iterate over. Then you define the step that the **Map demo** state
should repeat to generate the v4 UUIDs. Finally, you configure the output to return for
each repetition.
1. Choose the first **Pass** state in your workflow prototype. In the
**Output** tab, enter the following under
**Result**:
```
`{
"foo": "bar",
"colors": [
"red",
"green",
"blue",
"yellow",
"white"
]
}`
```
2. Choose the **Map demo** state and in the **Configuration** tab, do the following:
1. Choose **Provide a path to items array**.
2. Specify the following [reference
path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) to select the node that contains the input array:
```
`$.colors`
```
3. Choose the **Generate UUID** state and in the **Input** tab, do the following:
1. Choose **Transform input with Parameters**.
2. Enter the following JSON input to generate the v4 UUIDs for each of the input array items. You
use the `[States.UUID](./intrinsic-functions.html#statesuuid)` intrinsic function to generate the UUIDs.
```
`{
"uuid.$": "States.UUID()"
}`
```
3. For the **Generate UUID** state, choose the **Output** tab and do the following:
1. Choose **Filter output with OutputPath**.
2. Enter the following reference path to select the JSON node that contains the output array
items:
```
`$.uuid`
```
## Step 3: Review and save auto-generated definition
As you drag and drop states from the **Flow** panel onto the canvas,
Workflow Studio automatically composes the [Amazon States Language](./concepts-amazon-states-language.html) (ASL) definition of your workflow in real-time. You can edit this definition
as required.
1. (Optional) Choose **Definition** on the [Inspector
panel](./workflow-studio.html#workflow-studio-components-formdefinition) panel to view the automatically-generated Amazon States Language definition of your workflow.
###### Tip
You can also view the ASL definition in the [Code editor](./workflow-studio.html#wfs-interface-code-editor) of Workflow Studio. In the code editor, you can also edit the ASL definition of your workflow.
The following example shows the automatically generated Amazon States Language definition for
your workflow.
```
`{
"Comment": "Using Map state in Inline mode",
"StartAt": "Pass",
"States": {
"Pass": {
"Type": "Pass",
"Next": "Map demo",
"Result": {
"foo": "bar",
"colors": [
"red",
"green",
"blue",
"yellow",
"white"
]
}
},
"Map demo": {
"Type": "Map",
"ItemsPath": "$.colors",
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "INLINE"
},
"StartAt": "Generate UUID",
"States": {
"Generate UUID": {
"Type": "Pass",
"End": true,
"Parameters": {
"uuid.$": "States.UUID()"
},
"OutputPath": "$.uuid"
}
}
},
"End": true
}
}
}`
```
2. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
For this tutorial, enter the name `InlineMapDemo`.
3. (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
For this tutorial, keep all the default selections in **State machine configuration**.
4. In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
## Step 4: Run the state machine
State machine executions are instances where you run your workflow to perform tasks.
1. On the **InlineMapDemo** page, choose **Start execution**.
2. In the **Start execution** dialog box, do the following:
1. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
2. (Optional) In the **Input** box, enter input values in JSON format to run your workflow.
3. Choose **Start execution**.
4. The Step Functions console directs you to a page that's titled with your execution ID, known as the *Execution Details* page. You can review the execution results as the workflow progresses and after it completes.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
To view the execution input and output side-by-side, choose
**Execution input and output**. Under **Output**, view the output array returned by the `Map` state. The following is an example of the output array:
```
`[
"a85cbc7b-4e65-4ac2-97af-80ed504adc1d",
"b05bca11-d481-414e-aa9a-88285ec6590d",
"f42d59f7-bd32-480f-b270-caddb518ce2a",
"15f18616-517d-4b69-b7c3-bf22222d2efd",
"690bcfee-6d58-408c-a6b4-1995ccafdbd2"
]`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Wait for human approval
Copy large-scale CSV using Distributed Map
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.