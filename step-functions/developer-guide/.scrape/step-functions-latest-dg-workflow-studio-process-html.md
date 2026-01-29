---
url: https://docs.aws.amazon.com/step-functions/latest/dg/workflow-studio-process.html
title: Configure states inputs and outputs with Workflow Studio in Step Functions
word_count: 1564
filtered: true
elements_removed: 0
density_score: 0.81
---

Configure states inputs and outputs with Workflow Studio in Step Functions - AWS Step Functions
Configure states inputs and outputs with Workflow Studio in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#workflow-studio-process)
[Configure input to a state](#workflow-studio-process-input)[Configure output of a state](#workflow-studio-process-output)
# Configure states inputs and outputs with Workflow Studio in Step Functions
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
Each state makes a decision or performs an action based on input that it receives. In most
cases, it then passes output to other states. In Workflow Studio, you can configure how a state filters
and manipulates its input and output data in the **Input** and
**Output** tabs of the [Inspector
panel](./workflow-studio.html#workflow-studio-components-formdefinition) panel.
Use the **Info** links to access contextual help when configuring inputs and
outputs.
![Illustrative screenshot showing state inputs, outputs, and the info help panel](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/wfs_input_output_01.png)
For
detailed information about how Step Functions processes input and output, see [Processing input and output in Step Functions](./concepts-input-output-filtering.html).
## Configure input to a state
Each state receives input from the previous state as JSON. If you want to filter the
input, you can use the `[InputPath](./input-output-inputpath-params.html#input-output-inputpath)` filter under the **Input** tab
in the [Inspector
panel](./workflow-studio.html#workflow-studio-components-formdefinition) panel. The `InputPath` is a
string, beginning with `$`, that identifies a specific JSON node. These are
called [reference paths](./amazon-states-language-paths.html), and they follow
JsonPath syntax.
To filter the input:
* Choose **Filter input with InputPath**.
* Enter a valid [JsonPath](https://datatracker.ietf.org/wg/jsonpath/about/) for the `InputPath` filter. For example, `$.data`.
Your `InputPath` filter will be added to your workflow.
###### Example 1: Use InputPath filter in Workflow Studio
Say the input to your state includes the following JSON data.
```
`{
"comment": "Example for InputPath",
"dataset1": {
"val1": 1,
"val2": 2,
"val3": 3
},
"dataset2": {
"val1": "a",
"val2": "b",
"val3": "c"
}
}`
```
To apply the `InputPath` filter, choose **Filter input with
InputPath**, then enter an appropriate reference path. If you enter
`$.dataset2.val1`, the following JSON is passed as input to the state.
```
`{"a"}`
```
A reference path can also have a selection of values. If the data you reference is `{ "a":
[1, 2, 3, 4] }` and you apply the reference path `$.a[0:2]`as the
`InputPath` filter, the following is the result.
```
`[ 1, 2 ]`
```
[Parallel workflow state](./state-parallel.html), [Map workflow state](./state-map.html), and [Pass workflow state](./state-pass.html) flow states have an
additional input filtering option called `Parameters` under their
**Input** tab. This filter takes effect after the InputPath filter and
can be used to construct a custom JSON object consisting of one or more key-value pairs. The
values of each pair can either be static values, can be selected from the input, or can be
selected from the [Accessing execution data from the Context object
in Step Functions ](./input-output-contextobject.html) with a path.
###### Note
To specify that a parameter uses a reference path to point to a JSON node in the
input, the parameter name must end with `.$`.
###### Example 2: Create custom JSON input for Parallel state
Say the following JSON data is the input to a Parallel state.
```
`{
"comment": "Example for Parameters",
"product": {
"details": {
"color": "blue",
"size": "small",
"material": "cotton"
},
"availability": "in stock",
"sku": "2317",
"cost": "$23"
}
}`
```
To select part of this input and pass additional key-value pairs with a static
value, you can specify the following in the **Parameters** field, under the
**Parallel** state’s **Input** tab.
```
`{
"comment": "Selecting what I care about.",
"MyDetails": {
"size.$": "$.product.details.size",
"exists.$": "$.product.availability",
"StaticValue": "foo"
}
}`
```
The following JSON data will be the result.
```
`{
"comment": "Selecting what I care about.",
"MyDetails": {
"size": "small",
"exists": "in stock",
"StaticValue": "foo"
}
}`
```
## Configure output of a state
Each state produces JSON output that can be filtered before it is passed to the next
state. There are several filters available, and each affects the output in a different way.
Output filters available for each state are listed under the **Output** tab
in the **Inspector** panel. For [Task workflow state](./state-task.html) states, any output filters you select are
processed in this order:
1. `[ResultSelector](./input-output-inputpath-params.html#input-output-resultselector)`: Use this filter to
manipulate the state’s result. You can construct a new JSON object with
parts of the result.
2. `[Specifying state output using ResultPath in Step Functions](./input-output-resultpath.html)`: Use this filter to select
a combination of the state input and the task result to pass to the output.
3. `[Filtering state output using OutputPath](./input-output-example.html#input-output-outputpath)`: Use this filter to filter
the JSON output to choose which information from the result will be passed
to the next state.
### Use ResultSelector
`ResultSelector` is an optional output filter for the following states:
* [Task workflow state](./state-task.html) states, which are all states listed in the
**Actions** tab of the [States browser](./workflow-studio.html#workflow-studio-components-states).
* [Map workflow state](./state-map.html) states, in the **Flow** tab of the States
browser.
* [Parallel workflow state](./state-parallel.html) states, in the **Flow** tab of the
States browser.
`ResultSelector` can be used to construct a custom JSON object consisting
of one or more key-value pairs. The values of each pair can either be static values or
selected from the state's result with a path.
###### Note
To specify that a parameter uses a path to reference a JSON node in the result, the
parameter name must end with `.$`.
###### Example to use ResultSelector filter
In this example, you use `ResultSelector` to manipulate the response from
the Amazon EMR CreateCluster API call for an Amazon EMR `CreateCluster` state. The
following is the result from the Amazon EMR `CreateCluster` API call.
```
`{
"resourceType": "elasticmapreduce",
"resource": "createCluster.sync",
"output": {
"SdkHttpMetadata": {
"HttpHeaders": {
"Content-Length": "1112",
"Content-Type": "application/x-amz-JSON-1.1",
"Date": "Mon, 25 Nov 2019 19:41:29 GMT",
"x-amzn-RequestId": "1234-5678-9012"
},
"HttpStatusCode": 200
},
"SdkResponseMetadata": {
"RequestId": "1234-5678-9012"
},
"ClusterId": "AKIAIOSFODNN7EXAMPLE"
}
}`
```
To select part of this information and pass an additional key-value pair with a static
value, specify the following in the **ResultSelector** field, under the state’s
**Output** tab.
```
`{
"result": "found",
"ClusterId.$": "$.output.ClusterId",
"ResourceType.$": "$.resourceType"
}`
```
Using `ResultSelector` produces the following result.
```
`{
"result": "found",
"ClusterId": "AKIAIOSFODNN7EXAMPLE",
"ResourceType": "elasticmapreduce"
}`
```
### Use ResultPath
The output of a state can be a copy of its input, the result it produces, or a combination of its
input and result. Use `ResultPath` to control which combination of these is
passed to the state output. For more use cases of `ResultPath`, see [Specifying state output using ResultPath in Step Functions](./input-output-resultpath.html).
`ResultPath` is an optional output filter for the following states:
* [Task workflow state](./state-task.html) states, which are all states listed in the
**Actions** tab of the States browser.
* [Map workflow state](./state-map.html) states, in the **Flow** tab of the States
browser.
* [Parallel workflow state](./state-parallel.html) states, in the **Flow** tab of the
States browser.
* [Pass workflow state](./state-pass.html) states, in the **Flow** tab of the States
browser.
`ResultPath` can be used to add the result into the original state input.
The specified path indicates where to add the result.
###### Example to use ResultPath filter
Say the following is the input to a Task state.
```
`{
"details": "Default example",
"who": "AWS Step Functions"
}`
```
The result of the Task state is the following.
```
`Hello, AWS Step Functions`
```
You can add this result to the state’s input by applying `ResultPath` and
entering a reference [path](./amazon-states-language-paths.html) that
indicates where to add the result, such as `$.taskresult`:
With this `ResultPath`, the following is the JSON that is passed as the state’s
output.
```
`{
"details": "Default example",
"who": "AWS Step Functions",
"taskresult": "Hello, AWS Step Functions!"
}`
```
### Use OutputPath
The `OutputPath` filter lets you filter out unwanted information, and pass
only the portion of JSON that you need. The `OutputPath` is a string,
beginning with `$`, that identifies nodes within JSON text.
###### Example to use OutputPath filter
Imagine a Lambda Invoke API call returns metadata in addition to the Lambda function’s
result.
```
`{
"ExecutedVersion": "$LATEST",
"Payload": {
"foo": "bar",
"colors": [
"red",
"blue",
"green"
],
"car": {
"year": 2008,
"make": "Toyota",
"model": "Matrix"
}
},
"SdkHttpMetadata": {
"AllHttpHeaders": {
"X-Amz-Executed-Version": ["$LATEST"]
...`
```
You can use `OutputPath` to filter out the additional metadata. By default, the value of **OutputPath** filter for Lambda Invoke states created through the Workflow Studio is `$.Payload`. This default value removes the additional metadata and returns an output equivalent to
running the Lambda function directly.
The Lambda Invoke task result example and the value of `$.Payload` for the
**Output** filter pass the following JSON data as the output.
```
`{
"foo": "bar",
"colors": [
"red",
"blue",
"green"
],
"car": {
"year": 2008,
"make": "Toyota",
"model": "Matrix"
}
}`
```
###### Note
The `OutputPath` filter is the last output filter to take effect, so if
you use additional output filters such as `ResultSelector` or
`ResultPath`, you should modify the default value of
`$.Payload` for the `OutputPath` filter
accordingly.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a workflow
Set up execution roles
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.