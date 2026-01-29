---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-resultwriter.html
title: input output resultwriter.html
word_count: 1963
filtered: true
elements_removed: 0
density_score: 0.83
---

ResultWriter (Map) - AWS Step Functions
ResultWriter (Map) - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-resultwriter)
[Contents of the ResultWriter field](#input-output-resultwriter-field-contents)[Examples](#input-output-resultwriter-examples)[Exporting to Amazon S3](#input-output-resultwriter-exporting-to-S3)[IAM policies for ResultWriter](#resultwriter-iam-policies)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
The `ResultWriter` field is a JSON object that provides options for the output results of the child workflow executions started by a
Distributed Map state. You can specify different formatting options for the output results along with the Amazon S3 location to store them if you choose to
export them. Step Functions doesn't export these results by default.
###### Contents
* [Contents of the ResultWriter field](#input-output-resultwriter-field-contents)
* [Examples](#input-output-resultwriter-examples)
* [Exporting to Amazon S3](#input-output-resultwriter-exporting-to-S3)
* [IAM policies for ResultWriter](#resultwriter-iam-policies)
## Contents of the ResultWriter field
The `ResultWriter` field contains the following sub-fields. The choice of fields determines how the output is formatted and whether it's
exported to Amazon S3.
**`ResultWriter`**
A JSON object that specifies the following details:
* `Resource`
The Amazon S3 API action that Step Functions invokes to export the execution results.
* `Parameters`
A JSON object that specifies the Amazon S3 bucket name and prefix that stores the execution output.
* `WriterConfig`
This field enables you to configure the following options.
* `Transformation`
* `NONE` - returns the output of the child workflow executions unchanged, in addition to the workflow metadata. Default
when exporting the child workflow execution results to Amazon S3 and `WriterConfig` is not specified.
* `COMPACT` - returns the output of the child workflow executions. Default when `ResultWriter` is not specified.
* `FLATTEN` - returns the output of the child workflow executions. If a child workflow execution returns an array, this
option flattens the array, prior to returning the result to a state output or writing the result to an Amazon S3 object.
###### Note
If a child workflow execution fails, Step Functions returns its execution result unchanged. The results would be equivalent to having set
`Transformation` to `NONE`.
* `OutputType`
* `JSON` - formats the results as a JSON array.
* `JSONL` - formats the results as JSON Lines.
###### Required field combinations
The `ResultWriter` field cannot be empty. You must specify one of these sets of sub-fields.
* `WriterConfig` - to preview the formatted output, without saving the results to Amazon S3.
* `Resource` and `Parameters` - to save the results to Amazon S3 without additional formatting.
* All three fields: `WriterConfig`, `Resource` and `Parameters` - to format the output and save it to Amazon S3.
## Example configurations and transformation output
The following topics demonstrate the possible configuration settings for `ResultWriter` and examples of processed results from the
different transformation options.
* [ResultWriter configurations](#input-output-resultwriter-example-configurations)
* [Transformations](#input-output-resultwriter-example-transformations)
The following examples demonstrate configurations with the possible combinations of the three fields: `WriterConfig`,
`Resources` and `Parameters`.
###### Only *WriterConfig*
This example configures how the state output is presented in preview, with the output format and transformation specified in the
`WriterConfig` field. Non-existent `Resource` and `Parameters` fields, which would have provided the Amazon S3 bucket
specifications, imply the *state output* resource. The results are passed on to the next state.
```
`"ResultWriter": {
"WriterConfig": {
"Transformation": "`FLATTEN`",
"OutputType": "`JSON`"
}
}`
```
###### Only *Resources* and *Parameters*
This example exports the state output to the specified Amazon S3 bucket, without the additional formatting and transformation that the non-existent
`WriterConfig` field would have specified.
```
`"ResultWriter": {
"Resource": "arn:aws:states:::s3:putObject",
"Parameters": {
"Bucket": "`amzn-s3-demo-destination-bucket`",
"Prefix": "`csvProcessJobs`"
}`
```
###### All three fields: *WriterConfig*, *Resources* and *Parameters*
This example formats the state output according the specifications in the `WriterConfig` field. It also exports it to an Amazon S3 bucket
according to the specifications in the `Resource` and `Parameters` fields.
```
`"ResultWriter": {
"WriterConfig": {
"Transformation": "`FLATTEN`",
"OutputType": "`JSON`"
},
"Resource": "arn:aws:states:::s3:putObject",
"Parameters": {
"Bucket": "`amzn-s3-demo-destination-bucket`",
"Prefix": "`csvProcessJobs`"
}
}`
```
For these examples assume that each child workflow execution returns an output, which is an array of objects.
```
`[
{
"customer\_id": "145538",
"order\_id": "100000"
},
{
"customer\_id": "898037",
"order\_id": "100001"
}
]`
```
These examples demonstrate the formatted output for different `Transformation` values, with `OutputType` of
`JSON`.
###### Transformation NONE
This is an example of the processed result when you use the `NONE` transformation. The output is unchanged, and it includes the
workflow metadata.
```
`[
{
"ExecutionArn": "arn:aws:states:`region`:`account-id`:execution:orderProcessing/getOrders:da4e9fc7-abab-3b27-9a77-a277e463b709",
"Input": ...,
"InputDetails": {
"Included": true
},
"Name": "da4e9fc7-abab-3b27-9a77-a277e463b709",
"Output": "[{\\"customer\_id\\":\\"145538\\",\\"order\_id\\":\\"100000\\"},{\\"customer\_id\\":\\"898037\\",\\"order\_id\\":\\"100001\\"}]",
"OutputDetails": {
"Included": true
},
"RedriveCount": 0,
"RedriveStatus": "NOT\_REDRIVABLE",
"RedriveStatusReason": "Execution is SUCCEEDED and cannot be redriven",
"StartDate": "2025-02-04T01:49:50.099Z",
"StateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:orderProcessing/getOrders",
"Status": "SUCCEEDED",
"StopDate": "2025-02-04T01:49:50.163Z"
},
...
{
"ExecutionArn": "arn:aws:states:`region`:`account-id`:execution:orderProcessing/getOrders:f43a56f7-d21e-3fe9-a40c-9b9b8d0adf5a",
"Input": ...,
"InputDetails": {
"Included": true
},
"Name": "f43a56f7-d21e-3fe9-a40c-9b9b8d0adf5a",
"Output": "[{\\"customer\_id\\":\\"169881\\",\\"order\_id\\":\\"100005\\"},{\\"customer\_id\\":\\"797471\\",\\"order\_id\\":\\"100006\\"}]",
"OutputDetails": {
"Included": true
},
"RedriveCount": 0,
"RedriveStatus": "NOT\_REDRIVABLE",
"RedriveStatusReason": "Execution is SUCCEEDED and cannot be redriven",
"StartDate": "2025-02-04T01:49:50.135Z",
"StateMachineArn": "arn:aws:states:`region`:`account-id`:stateMachine:orderProcessing/getOrders",
"Status": "SUCCEEDED",
"StopDate": "2025-02-04T01:49:50.227Z"
}
]`
```
###### Transformation COMPACT
This is an example of the processed result when you use the `COMPACT` transformation. Note that it’s the combined output of the child
workflow executions with the original array structure.
```
`[
[
{
"customer\_id": "145538",
"order\_id": "100000"
},
{
"customer\_id": "898037",
"order\_id": "100001"
}
],
...,
[
{
"customer\_id": "169881",
"order\_id": "100005"
},
{
"customer\_id": "797471",
"order\_id": "100006"
}
]
]`
```
###### Transformation FLATTEN
This is an example of the processed result when you use the `FLATTEN` transformation. Note that it’s the combined output of the child
workflow executions arrays flattened into one array.
```
`[
{
"customer\_id": "145538",
"order\_id": "100000"
},
{
"customer\_id": "898037",
"order\_id": "100001"
},
...
{
"customer\_id": "169881",
"order\_id": "100005"
},
{
"customer\_id": "797471",
"order\_id": "100006"
}
]`
```
###### Important
Make sure that the Amazon S3 bucket you use to export the results of a Map Run is under the same AWS account and AWS Region as your state machine. Otherwise, your state machine execution will fail with the `States.ResultWriterFailed` error.
Exporting the results to an Amazon S3 bucket is helpful if your output payload size exceeds 256 KiB. Step Functions consolidates all child workflow execution data,
such as execution input and output, ARN, and execution status. It then exports executions with the same status to their respective files in the specified
Amazon S3 location.
The following example, using **JSONPath**, shows the syntax of the `ResultWriter` field with
`Parameters` to export the child workflow execution results. In this example, you store the results in a bucket named
`amzn-s3-demo-destination-bucket` within a prefix called `csvProcessJobs`.
```
`{
"ResultWriter": {
"Resource": "arn:aws:states:::s3:putObject",
"Parameters": {
"Bucket": "`amzn-s3-demo-destination-bucket`",
"Prefix": "`csvProcessJobs`"
}
}
}`
```
For **JSONata** states, `Parameters` will be replaced with `Arguments`.
```
`{
"ResultWriter": {
"Resource": "arn:aws:states:::s3:putObject",
"Arguments": {
"Bucket": "`amzn-s3-demo-destination-bucket`",
"Prefix": "`csvProcessJobs`"
}
}
}`
```
###### Tip
In Workflow Studio, you can export the child workflow execution results by selecting **Export Map state results to Amazon S3**. Then, provide the
name of the Amazon S3 bucket and prefix where you want to export the results to.
Step Functions needs appropriate permissions to access the bucket and folder where you want to export the results. For information about the required IAM
policy, see [IAM policies for ResultWriter](#resultwriter-iam-policies).
If you export the child workflow execution results, the *Distributed Map state* execution returns the Map Run ARN and data about the Amazon S3 export location in the
following format:
```
`{
"MapRunArn": "arn:aws:states:us-east-2:`account-id`:mapRun:`csvProcess`/`Map:ad9b5f27-090b-3ac6-9beb-243cd77144a7`",
"ResultWriterDetails": {
"Bucket": "`amzn-s3-demo-destination-bucket`",
"Key": "`csvProcessJobs`/ad9b5f27-090b-3ac6-9beb-243cd77144a7/manifest.json"
}
}`
```
Step Functions exports executions with the same status to their respective files. For example, if your child workflow executions resulted in 500 success and
200 failure results, Step Functions creates two files in the specified Amazon S3 location for the success and failure results. In this example, the success results file
contains the 500 success results, while the failure results file contains the 200 failure results.
For a given execution attempt, Step Functions creates the following files in the specified Amazon S3 location depending on your execution output:
* `manifest.json` – Contains Map Run metadata, such as export location, Map Run ARN, and information about the result
files.
If you've [redriven](./redrive-map-run.html) a Map Run, the `manifest.json` file, contains references to
all the successful child workflow executions across all the attempts of a Map Run. However, this file contains references to the failed and pending
executions for a specific redrive.
* `SUCCEEDED\_n.json` – Contains the consolidated data for all successful child workflow executions. *n*
represents the index number of the file. The index number starts from 0. For example, `SUCCEEDED\_1.json`.
* `FAILED\_n.json` – Contains the consolidated data for all failed, timed out, and aborted child workflow executions. Use this file
to recover from failed executions. *n* represents the index of the file. The index number starts from 0. For example,
`FAILED\_1.json`.
* `PENDING\_n.json` – Contains the consolidated data for all child workflow executions that weren’t started because the Map Run
failed or aborted. *n* represents the index of the file. The index number starts from 0. For example,
`PENDING\_1.json`.
Step Functions supports individual result files of up to 5 GB. If a file size exceeds 5 GB, Step Functions creates another file to write the remaining execution results
and appends an index number to the file name. For example, if size of the `SUCCEEDED\_0.json` file exceeds 5 GB, Step Functions creates
`SUCCEEDED\_1.json` file to record the remaining results.
If you didn’t specify to export the child workflow execution results, the state machine execution returns an array of child workflow execution results
as shown in the following example:
```
`[
{
"statusCode": 200,
"inputReceived": {
"show\_id": "s1",
"release\_year": "2020",
"rating": "PG-13",
"type": "Movie"
}
},
{
"statusCode": 200,
"inputReceived": {
"show\_id": "s2",
"release\_year": "2021",
"rating": "TV-MA",
"type": "TV Show"
}
},
...
]`
```
###### Note
If the returned output size exceeds 256 KiB, the state machine execution fails and returns a `[States.DataLimitExceeded](./concepts-error-handling.html#error-data-limit-exceed)` error.
## IAM policies for ResultWriter
When you create workflows with the Step Functions console, Step Functions can automatically generate IAM policies based on the resources in your workflow definition. Generated policies include the least privileges necessary to allow the state machine role to invoke the `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API action for the *Distributed Map state* and access AWS resources, such as Amazon S3 buckets and objects, and Lambda functions.
We recommend including only the necessary permissiosn in your IAM policies. For example, if your workflow includes a `Map` state in Distributed mode, scope your policies down to the specific Amazon S3 bucket and folder that contains your data.
###### Important
If you specify an Amazon S3 bucket and object, or prefix, with a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) to an existing key-value pair in your *Distributed Map state* input, make sure that you update the IAM policies for your workflow. Scope the policies down to the bucket and object names the path resolves to at runtime.
The following IAM policy example grants the least privileges required to write your
child workflow execution results to a folder named `csvJobs` in an
Amazon S3 bucket using the `[PutObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)` API
action.
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Action": [
"s3:PutObject",
"s3:GetObject",
"s3:ListMultipartUploadParts",
"s3:AbortMultipartUpload"
],
"Resource": [
"arn:aws:s3:::`amzn-s3-demo-destination-bucket`/`csvJobs`/\*"
]
}
]
}`
`
```
If the Amazon S3 bucket to which you're writing the child workflow execution result is encrypted using an AWS Key Management Service
(AWS KMS) key, you must include the necessary AWS KMS permissions in your IAM policy. For more information, see [IAM permissions for AWS KMS key encrypted Amazon S3 bucket](./iam-policies-eg-dist-map.html#multiupload-dmap-result-policy).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ItemBatcher
Parsing input CSV files
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.