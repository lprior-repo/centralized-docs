---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-contextobject.html
title: Accessing execution data from the Context object
word_count: 1059
filtered: true
elements_removed: 0
density_score: 0.80
---

Accessing execution data from the Context object in Step Functions - AWS Step Functions
Accessing execution data from the Context object in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-contextobject)
[Accessing the Context object](#contextobject-access)[Context object fields](#contextobject-format)[Context object data for Map states](#contextobject-map)
# Accessing execution data from the Context object
in Step Functions
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
The Context object is an internal JSON structure that is available during an execution,
and contains information about your state machine and execution. The context provides your
workflows information about their specific execution. Your workflows can reference the
Context object in a JSONata expression with `$states.context`.
## Accessing the Context object
**To access the Context object in JSONata**
To access the Context object in JSONata states, use `$states.context` in a
JSONata expression.
```
`{
"ExecutionID" : "{% $states.context.Execution.Id %}"
}`
```
**To access the Context object in JSONPath**
To access the Context object in JSONPath, you first append `.$` to the end of
the key to indicate the value is a path. Then, prepend the value with `$$.`
to select a node in the Context object.
```
`{
"ExecutionID**.$**": "**$$.Execution.Id**"
}`
```
JSONPath states can refer to the context (`$$.`) from the following JSONPath
fields:
* `InputPath`
* `OutputPath`
* `ItemsPath` (in Map states)
* `Variable` (in Choice states)
* `ResultSelector`
* `Parameters`
* Variable to variable comparison operators
## Context object fields
The Context object includes information about the state machine, state, execution, and
task. The Context JSON object includes nodes for each type of data in the following
format:
```
`{
"Execution": {
"Id": "`String`",
"Input": {},
"Name": "`String`",
"RoleArn": "`String`",
"StartTime": "`Format: ISO 8601`",
"RedriveCount": `Number`,
"RedriveTime": "`Format: ISO 8601`"
},
"State": {
"EnteredTime": "`Format: ISO 8601`",
"Name": "`String`",
"RetryCount": `Number`
},
"StateMachine": {
"Id": "`String`",
"Name": "`String`"
},
"Task": {
"Token": "`String`"
}
}`
```
During an execution, the Context object is populated with relevant data.
Occasionally, new fields are added to the context. If you are processing the JSON
context directly, we recommend crafting code that can gracefully handle new unknown
fields. For example, if using the Jackson library for unmarshalling JSON, we recommend
setting `FAIL\_ON\_UNKNOWN\_PROPERTIES` to `false` in your
`ObjectMapper` to prevent an
`UnrecognizedPropertyException`.
`RedriveTime` Context object is only available if you've
redriven an execution. If you've [redriven a Map Run](./redrive-map-run.html), the `RedriveTime` context
object is only available for child workflows of type Standard. For a
redriven Map Run with child workflows of type Express,
`RedriveTime` isn't available.
Content from a running execution includes specifics in the following format:
```
`{
"Execution": {
"Id": "arn:aws:states:`region`:123456789012:execution:stateMachineName:executionName",
"Input": {
"key": "value"
},
"Name": "executionName",
"RoleArn": "arn:aws:iam::123456789012:role...",
"StartTime": "2025-08-27T10:04:42Z"
},
"State": {
"EnteredTime": "2025-08-27T10:04:42.001Z",
"Name": "Test",
"RetryCount": 3
},
"StateMachine": {
"Id": "arn:aws:states:`region`:123456789012:stateMachine:stateMachineName",
"Name": "stateMachineName"
},
"Task": {
"Token": "h7XRiCdLtd/83p1E0dMccoxlzFhglsdkzpK9mBVKZsp7d9yrT1W"
}
}`
```
###### Timestamp format with fractional seconds
Step Functions follows the ISO8601 specification which states that output can be zero, three,
six or nine digits as necessary. When a timestamp has zero fractional seconds, Step Functions
removes the trailing zeros rather than pad the output.
If you create code that consumes Step Functions timestamps, your code must be able to process a variable number of fractional seconds.
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
When processing a [Map state](./state-map.html), the context
will also contain `Index`, `Value`, and `Source`.
For each `Map` state iteration, `Index` contains the index
number for the array item that is being currently processed, `Value` contains
the array item being processed, and `Source` will be the InputType of
`CSV`, `JSON`, `JSONL`, or `PARQUET`.
Within a `Map` state, the Context object includes the following
data:
```
`"Map": {
"Item": {
"Index" : `Number`,
"Key" : "`String`", // Only valid for JSON objects
"Value" : "`String`",
"Source": "`String`"
}
}`
```
These are available only in a `Map` state, and can be specified in the `[ItemSelector (Map)](./input-output-itemselector.html)` field.
###### Note
You must define parameters from the Context object in the `ItemSelector`
block of the main `Map` state, not within the states included in the
`ItemProcessor` section.
Given a state machine using a **JSONPath** `Map`
state, you can inject information from the Context object as follows.
```
`{
"StartAt": "ExampleMapState",
"States": {
"ExampleMapState": {
"Type": "Map",
**"ItemSelector": {
"ContextIndex.$": "$$.Map.Item.Index",
"ContextValue.$": "$$.Map.Item.Value",
"ContextSource.$": "$$.Map.Item.Source"
}**,
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "INLINE"
},
"StartAt": "TestPass",
"States": {
"TestPass": {
"Type": "Pass",
"End": true
}
}
},
"End": true
}
}
}`
```
For JSONata, the additional Map state context information can be accessed from the
`$states.context` variable:
```
`{
"StartAt": "ExampleMapState",
"States": {
"ExampleMapState": {
"Type": "Map",
**"ItemSelector": {
"ContextIndex": "{% $states.context.Map.Item.Index %}",
"ContextValue": "{% $states.context.Map.Item.Value %}",
"ContextSource": "{% $states.context.Map.Item.Source %}"
}**,
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "INLINE"
},
"StartAt": "TestPass",
"States": {
"TestPass": {
"Type": "Pass",
"End": true
}
}
},
"End": true
}
}
}`
```
If you execute the previous state machine with the following input, `Index`
and `Value` are inserted in the output.
```
`[
{
"who": "bob"
},
{
"who": "meg"
},
{
"who": "joe"
}
]`
```
The output for the execution returns the values of `Index` and `Value` items for each of the three iterations as follows:
```
`[
{
"ContextIndex": 0,
"ContextValue": {
"who": "bob"
},
"ContextSource" : "STATE\_DATA"
},
{
"ContextIndex": 1,
"ContextValue": {
"who": "meg"
},
"ContextSource" : "STATE\_DATA"
},
{
"ContextIndex": 2,
"ContextValue": {
"who": "joe"
},
"ContextSource" : "STATE\_DATA"
}
]`
```
Note that `$states.context.Map.Item.Source` will be one of the
following:
* For state input, the value will be : `STATE\_DATA`
* For `Amazon S3 LIST\_OBJECTS\_V2` with `Transformation=NONE`, the value will show the S3 URI for the bucket. For example: `S3://bucket-name`.
* For all the other input types, the value will be the Amazon S3 URI. For example:
`S3://bucket-name/object-key`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Transforming data with JSONata
Using JSONPath paths
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.