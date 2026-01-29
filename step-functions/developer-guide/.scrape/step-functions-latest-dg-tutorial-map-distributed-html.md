---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-map-distributed.html
title: Copying large-scale CSV data using Distributed Map in Step Functions
word_count: 2165
filtered: true
elements_removed: 0
density_score: 0.81
---

Copying large-scale CSV data using Distributed Map in Step Functions - AWS Step Functions
Copying large-scale CSV data using Distributed Map in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-map-distributed)
[Prerequisites](#use-dist-map-prereqs)[Step 1: Create the workflow prototype](#use-dist-map-create-workflow)[Step 2: Configure the required fields for Map state](#use-dist-map-config-fields)[Step 3: Configure additional options](#use-dist-map-config-misc-fields)[Step 4: Configure the Lambda function](#use-dist-map-config-resource)[Step 5: Update the workflow prototype](#use-dist-map-update-workflow)[Step 6: Review the auto-generated Amazon States Language definition and save the workflow](#use-dist-map-review-asl)[Step 7: Run the state machine](#use-dist-map-sm-run)
# Copying large-scale CSV data using Distributed Map in Step Functions
This tutorial helps you start using the `Map` state in Distributed mode.
A `Map` state set to **Distributed** is known as a *Distributed Map state*. You use the *Distributed Map state* in your workflows to iterate over large-scale Amazon S3
data sources. The `Map` state runs each iteration as a child workflow execution,
which enables high concurrency. For more information about Distributed mode, see [Map state in Distributed
mode](./state-map-distributed.html).
In this tutorial, you use the *Distributed Map state* to iterate over a CSV file in an Amazon S3 bucket. You
then return its contents, along with the ARN of a child workflow execution, in another Amazon S3
bucket. You start by creating a workflow prototype in the Workflow Studio. Next, you set the [Map state's processing mode](./state-map.html#concepts-map-process-modes) to
Distributed, specify the CSV file as the dataset, and provide its location to the
`Map` state. You also specify the workflow type for the child workflow
executions that the *Distributed Map state* starts as **Express**.
In addition to these settings, you also specify other configurations, such as the maximum
number of concurrent child workflow executions and the location to export the
`Map` result, for the example workflow used in this tutorial.
## Prerequisites
* Upload a CSV file to an Amazon S3 bucket. You must define a header row within your CSV file. For
information about size limits imposed on the CSV file and how to specify the
header row, see [CSV file in an Amazon S3 bucket](./input-output-itemreader.html#itemsource-example-csv-data).
* Create another Amazon S3 bucket and a folder within that bucket to export the `Map`
state result to.
###### Requirements for account and region
Your Amazon S3 buckets must be in the same AWS account and AWS Region as your state machine.
Note that even though your state machine may be able to access files in buckets across different AWS accounts that are in the same AWS Region, Step Functions only supports listing objects in Amazon S3 buckets that are in *both* the same AWS account and the same AWS Region as the state machine.
## Step 1: Create the workflow prototype
In this step, you create the prototype for your workflow using Workflow Studio. Workflow Studio is a
visual workflow designer available in the Step Functions console. You choose the required state
and API action from the **Flow** and **Actions** tabs
respectively. You'll use the drag and drop feature of Workflow Studio to create the workflow
prototype.
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home), choose **State machines** from the menu, then choose **Create state machine**.
2. Choose **Create from blank**.
3. Name your state machine, then choose **Continue** to edit your state machine in Workflow Studio.
4. From the **Flow** tab, drag a **Map** state and drop it to the empty state labelled **Drag first state here**.
5. In the **Configuration** tab, for **State name**, enter `Process data`.
6. From the **Actions** tab, drag an **AWS Lambda Invoke** API action and drop it inside the **Process data** state.
7. Rename the **AWS Lambda Invoke** state to `Process CSV
data`.
## Step 2: Configure the required fields for Map state
In this step, you configure the following required fields of the *Distributed Map state*:
* [ItemReader](./input-output-itemreader.html) – Specifies the dataset and its location from which the `Map` state can read input.
* [ItemProcessor](./state-map-distributed.html#distitemprocessor) – Specifies the following values:
* `ProcessorConfig` – Set the `Mode` and `ExecutionType` to `DISTRIBUTED` and `EXPRESS` respectively. This sets the `Map` state's processing mode and the workflow type for child workflow executions that the *Distributed Map state* starts.
* `StartAt` – The first state in the Map workflow.
* `States` – Defines the Map workflow, which is a set of steps to repeat in each child workflow execution.
* [ResultWriter](./input-output-resultwriter.html) – Specifies the Amazon S3 location where Step Functions writes the *Distributed Map state* results.
###### Important
Make sure that the Amazon S3 bucket you use to export the results of a Map Run is under the same AWS account and AWS Region as your state machine. Otherwise, your state machine execution will fail with the `States.ResultWriterFailed` error.
###### To configure the required fields:
1. Choose the **Process data** state and, in the
**Configuration** tab, do the following:
1. For **Processing mode**, choose **Distributed**.
2. For **Item source**, choose **Amazon S3**, and then choose **CSV file in S3** from the **S3 item source** dropdown list.
3. Do the following to specify the Amazon S3 location of your CSV file:
1. For **S3 object**, select **Enter bucket and key** from the
dropdown list.
2. For **Bucket**, enter the name of the Amazon S3 bucket, which contains the CSV file. For example, `amzn-s3-demo-source-bucket`.
3. For **Key**, enter the name of the Amazon S3 object in which you saved the CSV file. You must also specify the name of the CSV file in this field. For example, `csvDataset/ratings.csv`.
4. For CSV files, you must also specify the location of the column header. To do this, choose
**Additional configuration**, and then for
**CSV header location** keep the default selection
of **First row** if the first row of your CSV file is
the header. Otherwise, choose **Given** to specify the
header within the state machine definition. For more information, see
`[ReaderConfig](./input-output-itemreader.html#readerconfig)`.
5. For
**Child
execution
type**, choose
**Express**.
6. In **Export location**, to export the Map Run results to a specific Amazon S3 location, choose **Export Map state's output to Amazon S3**.
7. Do the following:
1. For **S3 bucket**, choose **Enter bucket name and prefix** from the dropdown list.
2. For **Bucket**, enter the name of the Amazon S3 bucket where you want to export
the results to. For example, `mapOutputs`.
3. For **Prefix**, enter the folder name where you want to save the results to.
For example, `resultData`.
## Step 3: Configure additional options
In addition to the required settings for a *Distributed Map state*, you can also specify other
options. These can include the maximum number of concurrent child workflow executions
and the location to export the `Map` state result to.
1. Choose the **Process data** state. Then, in **Item source**,
choose **Additional configuration**.
2. Do the following:
1. Choose **Modify items with ItemSelector** to specify a custom JSON input for
each child workflow execution.
2. Enter the following JSON input:
```
`{
`"index.$":` `"$$.Map.Item.Index"`,
`"value.$":` `"$$.Map.Item.Value"`
}`
```
For information about how to create a custom input, see `[ItemSelector (Map)](./input-output-itemselector.html)`.
3. In **Runtime settings**, for **Concurrency limit**, specify
the number of concurrent child workflow executions that the *Distributed Map state* can
start. For example, enter `100`.
4. Open a new window or tab on your browser and complete the configuration of the
Lambda function you'll use in this workflow, as explained in [Step 4: Configure the Lambda function](#use-dist-map-config-resource).
###### Important
Ensure that your Lambda function is under the same AWS Region as your state machine.
1. Open the [Lambda
console](https://console.aws.amazon.com/lambda/home) and choose **Create function**.
2. On the **Create function** page, choose
**Author from scratch**.
3. In the **Basic information** section, configure your
Lambda function:
1. For **Function name**, enter
`distributedMapLambda`.
2. For **Runtime**, choose **Node.js**.
3. Keep all of the default selections and choose **Create
function**.
4. After you create your Lambda function, copy the function's
Amazon Resource Name (ARN) displayed in the upper-right corner of the page. You'll
need to provide this in your workflow prototype. The following is an example ARN:
```
`arn:aws:lambda:us-east-2:123456789012:function:distributedMapLambda`
```
5. Copy the following code for the Lambda function and paste it into the **Code source** section of the
**distributedMapLambda**
page.
```
`exports.handler = async function(event, context) {
console.log("Received Input:\\n", event);
return {
'statusCode' : 200,
'inputReceived' : event //returns the input that it received
}
};`
```
6. Choose **Deploy**. Once your function deploys, choose
**Test** to see the output of your Lambda function.
## Step 5: Update the workflow prototype
In the Step Functions console, you'll update your workflow to add the Lambda function's ARN.
1. Return to the tab or window where you created the workflow prototype.
2. Choose the **Process CSV data** step, and in the **Configuration** tab, do the following:
1. For **Integration type**, choose **Optimized**.
2. For **Function name**, start to enter the name of your Lambda function. Choose
the function from the dropdown list that appears, or choose
**Enter function name** and provide the Lambda
function ARN.
## Step 6: Review the auto-generated Amazon States Language definition and save the workflow
As you drag and drop states from the **Action** and **Flow** tabs onto the canvas, Workflow Studio automatically composes
the [Amazon States Language](./concepts-amazon-states-language.html) definition of your workflow in real-time. You can edit
this definition as required.
1. (Optional) Choose **Definition** on the [Inspector
panel](./workflow-studio.html#workflow-studio-components-formdefinition)
panel and view the state machine definition.
###### Tip
You can also view the ASL definition in the [Code editor](./workflow-studio.html#wfs-interface-code-editor) of Workflow Studio. In the code editor, you can also edit the ASL definition of your workflow.
The following example code shows the automatically generated Amazon States Language definition
for your workflow.
```
`{
"Comment": "Using Map state in Distributed mode",
"StartAt": "Process data",
"States": {
"Process data": {
"Type": "Map",
"MaxConcurrency": 100,
"ItemReader": {
"ReaderConfig": {
"InputType": "CSV",
"CSVHeaderLocation": "FIRST\_ROW"
},
"Resource": "arn:aws:states:::s3:getObject",
"Parameters": {
"Bucket": "amzn-s3-demo-source-bucket",
"Key": "csvDataset/ratings.csv"
}
},
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "DISTRIBUTED",
"ExecutionType": "EXPRESS"
},
"StartAt": "Process CSV data",
"States": {
"Process CSV data": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"Payload.$": "$",
"FunctionName": "arn:aws:lambda:us-east-2:`account-id`:function:distributedMapLambda"
},
"End": true
}
}
},
"Label": "Processdata",
"End": true,
"ResultWriter": {
"Resource": "arn:aws:states:::s3:putObject",
"Parameters": {
"Bucket": "mapOutputs",
"Prefix": "resultData"
}
},
"ItemSelector": {
"index.$": "$$.Map.Item.Index",
"value.$": "$$.Map.Item.Value"
}
}
}
}`
```
2. Specify a name for your state machine. To do this, choose the edit icon next to the default state machine name of **MyStateMachine**. Then, in **State machine configuration**, specify a name in the **State machine name** box.
For this tutorial, enter the name `DistributedMapDemo`.
3. (Optional) In **State machine configuration**, specify other workflow settings, such as state machine type and its execution role.
For this tutorial, keep all the default selections in **State machine configuration**.
4. In the **Confirm role creation** dialog box, choose **Confirm** to continue.
You can also choose **View role settings** to go back to **State machine configuration**.
###### Note
If you delete the IAM role that Step Functions creates, Step Functions can't
recreate it later. Similarly, if you modify the role (for example, by
removing Step Functions from the principals in the IAM policy), Step Functions can't
restore its original settings later.
## Step 7: Run the state machine
An *execution* is an instance of your state machine where you run
your workflow to perform tasks.
1. On the **DistributedMapDemo** page, choose **Start execution**.
2. In the **Start execution** dialog box, do the following:
1. (Optional) Enter a custom execution name to override the generated default.
###### Non-ASCII names and logging
Step Functions accepts names for state machines, executions, activities, and labels that contain non-ASCII characters. Because such characters will prevent Amazon CloudWatch from logging data, we recommend using only ASCII characters so you can track Step Functions metrics.
2. (Optional) In the **Input** box, enter input values in JSON format to run your workflow.
3. Choose **Start execution**.
4. The Step Functions console directs you to a page that's titled with your execution ID, known as the *Execution Details* page. You can review the execution results as the workflow progresses and after it completes.
To review the execution results, choose individual states on the **Graph view**, and then choose the individual tabs on the [Step details](./concepts-view-execution-details.html#exec-details-intf-step-details) pane to view each state's details including input, output, and definition respectively. For details about the execution information you can view on the *Execution Details* page, see [Execution details overview](./concepts-view-execution-details.html#exec-details-interface-overview).
For example, choose the `Map` state, and then choose **Map Run** to open the *Map Run Details* page. On this page, you can view all the execution details of the *Distributed Map state* and the child workflow executions that it started. For information about this page, see [Viewing Map Runs](./concepts-examine-map-run.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Repeat actions with Inline Map
Iterate a loop with Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.