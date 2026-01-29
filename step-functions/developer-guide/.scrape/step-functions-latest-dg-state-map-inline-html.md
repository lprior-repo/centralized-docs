---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-map-inline.html
title: Using Map state in Inline mode in Step Functions workflows
word_count: 2314
filtered: true
elements_removed: 0
density_score: 0.79
---

Using Map state in Inline mode in Step Functions workflows - AWS Step Functions
Using Map state in Inline mode in Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-map-inline)
[Key concepts in this topic](#key-concepts-inline-map)[Inline Map state fields](#map-state-inline-additional-fields)[Deprecated fields](#map-state-inline-deprecated-fields)[Inline Map state example (JSONPath)](#inline-map-state-examples)[Inline Map state example with
ItemSelector](#inline-map-state-example-params)[Inline Map state
input and output processing](#inline-map-state-output)
# Using Map state in Inline mode in Step Functions workflows
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
By default, `Map` states runs in **Inline** mode. In Inline
mode, the Map state accepts only a JSON array as input. It receives this array from a
previous step in the workflow. In this mode, each iteration of the `Map` state runs in the context of the workflow that contains the `Map` state. Step Functions adds the execution history of these iterations to the parent workflow's execution history.
In this mode, the `Map` state supports up to 40 concurrent iterations.
A `Map` state set to **Inline** is known as an *Inline Map state*.
Use the `Map` state in Inline mode if your workflow's execution history won't exceed 25,000 entries, or if you don't require more than 40 concurrent iterations.
For an introduction to using the *Inline Map state*, see the tutorial [Repeat actions with Inline Map](./tutorial-map-inline.html).
###### Contents
* [Key concepts in this topic](#key-concepts-inline-map)
* [Inline Map state fields](#map-state-inline-additional-fields)
* [Deprecated fields](#map-state-inline-deprecated-fields)
* [Inline Map state example (JSONPath)](#inline-map-state-examples)
* [Inline Map state example with
ItemSelector](#inline-map-state-example-params)
* [Inline Map state
input and output processing](#inline-map-state-output)
## Key concepts in this topic
**Inline mode**
A limited-concurrency mode of the `Map` state. In this mode, each iteration of the `Map` state runs in the context of the workflow that contains the `Map` state. Step Functions adds the execution history of these iterations to the parent workflow's execution history.
`Map` states run in the Inline mode by default.
This mode accepts only a JSON array as input and supports up to 40 concurrent iterations.
**Inline Map state**
A `Map` state set to the **Inline** mode.
**
Map workflow**
The set of steps that the `Map` state runs for each iteration.
**Map state iteration**
A repetition of the workflow defined inside of the `Map` state.
## Inline Map state fields
To use the *Inline Map state* in your workflows, specify one or more of these fields. You
specify these fields in addition to the [common state fields](./statemachine-structure.html#amazon-states-language-common-fields).
**`Type`
(Required)**
Sets the type of state, such as `Map`.
**`
ItemProcessor` (Required)**
Contains the following JSON objects that specify the `Map` state processing mode and definition.
The definition contains the set of steps to repeat for processing each
array item.
* `ProcessorConfig` – An optional JSON object that specifies the processing mode for the `Map` state. This object contains the `Mode` sub-field. This field defaults to `INLINE`, which uses the `Map` state in Inline mode.
In this mode, the failure of any iteration causes the
`Map` state to fail. All iterations stop when the
`Map` state fails.
* `StartAt` – Specifies a string that indicates the first state in a workflow. This string is case-sensitive and must match the name of one of the state objects. This state runs first for each item in the dataset. Any execution input that you provide to the `Map` state passes to the `StartAt` state first.
* `States` – A JSON object containing a comma-delimited set of [states](./workflow-states.html). In this object, you define the [Map workflow](#mapwflow).
###### Note
* States within the `ItemProcessor` field can
only transition to each other. No state outside the
`ItemProcessor` field can transition to a
state within it.
* The `ItemProcessor` field replaces the now
deprecated `[Iterator](#iterator)` field.
Although you can continue to include `Map`
states that use the `Iterator` field, we
highly recommend that you replace this field with
`ItemProcessor`.
[Step Functions Local](./sfn-local.html) doesn't currently support the `ItemProcessor` field. We recommend that you use the `Iterator` field with Step Functions Local.
**`Items` (Optional, JSONata only)**
A JSON array or a JSONata expression that must evaluate to an array.
**`ItemsPath` (Optional, JSONPath only)**
Specifies a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) using the [JsonPath](https://datatracker.ietf.org/wg/jsonpath/about/) syntax.
This path selects the JSON node that contains the array of items inside the
state input. For more information, see [ItemsPath (Map, JSONPath only)](./input-output-itemspath.html).
**`
ItemSelector`
(Optional)**
Overrides the values of the input array items before they're passed on to
each `Map` state iteration.
In this field, you specify a valid JSON that contains a collection of
key-value pairs. These pairs can contain any of the following:
* Static values you define in your state machine definition.
* Values selected from the state input using a [path](./amazon-states-language-paths.html).
* Values accessed from the [context
object](./input-output-contextobject.html).
For more information, see [ItemSelector (Map)](./input-output-itemselector.html).
The `ItemSelector` field replaces the now deprecated
`[Parameters](#mapparameters)` field. Although you can continue to include `Map`
states that use the `Parameters` field, we highly recommend that
you replace this field with `ItemSelector`.
**`MaxConcurrency` (Optional)**
Specifies an integer value that provides the upper bound on the number of
`Map` state iterations that can run in parallel. For example, a
`MaxConcurrency` value of 10 limits the `Map` state to
10 concurrent iterations running at one time.
In JSONata states, you can specify a JSONata expression that evaluates to an integer.
###### Note
Concurrent iterations may be limited. When this occurs, some
iterations won't begin until previous iterations are complete. The
likelihood of this occurring increases when your input array has more
than 40 items.
To achieve a higher concurrency, consider [Distributed mode](./state-map-distributed.html).
The default value is `0`, which places no limit on concurrency.
Step Functions invokes iterations as concurrently as possible.
A `MaxConcurrency` value of `1` invokes the
`ItemProcessor` once for each array element. Items in the array
are processed in the order of their appearance in the input. Step Functions doesn't
start a new iteration until it completes the previous iteration.
**`MaxConcurrencyPath` (Optional, JSONPath only)**
If you want to provide a maximum concurrency value dynamically from the state input using a reference path, use `MaxConcurrencyPath`. When resolved, the reference path must select a field whose value is a non-negative integer.
###### Note
A `Map` state cannot include both `MaxConcurrency` and `MaxConcurrencyPath`.
**`ResultPath` (Optional, JSONPath only)**
Specifies where in the input to store the output of the `Map`
state's iterations. The Map state then filters the input as specified by the
[OutputPath](./input-output-example.html#input-output-outputpath)
field, if specified. Then, it uses the filtered input as the state's
output. For more information, see [Input and Output
Processing](./concepts-input-output-filtering.html).
**`ResultSelector` (Optional, JSONPath only)**
Pass a collection of key value pairs, where the values are either static
or selected from the result. For more information, see [ResultSelector](./input-output-inputpath-params.html#input-output-resultselector).
###### Tip
If the Parallel or Map state you use in your state machines returns an array of arrays, you can transform them into a flat array with the [ResultSelector](./input-output-inputpath-params.html#input-output-resultselector) field. For more information, see [Flattening an array of arrays](./input-output-inputpath-params.html#flatten-array-of-arrays-result-selector).
**`Retry` (Optional)**
An array of objects, called Retriers, that define a retry policy. States
use a retry policy when they encounter runtime errors. For more information,
see [State machine examples using Retry and Catch](./concepts-error-handling.html#error-handling-examples).
###### Note
If you define Retriers for the *Inline Map state*, the retry policy applies
to all `Map` state iterations, instead of only failed
iterations. For example, your `Map` state contains two
successful iterations and one failed iteration. If you have defined
the `Retry` field for the `Map` state, the retry
policy applies to all three `Map` state iterations instead of
only the failed iteration.
**`Catch` (Optional)**
An array of objects, called Catchers, that define a fallback state. States
run a catcher if they encounter runtime errors and either don't have a retry
policy, or their retry policy is exhausted. For more information, see [Fallback States](./concepts-error-handling.html#error-handling-fallback-states).
**`Output` (Optional, JSONata only)**
Used to specify and transform output from the state. When specified, the
value overrides the state output default.
The output field accepts any JSON value (object, array, string, number,
boolean, null). Any string value, including those inside objects or arrays,
will be evaluated as JSONata if surrounded by {% %} characters.
Output also accepts a JSONata expression directly, for example: "Output":
"{% jsonata expression %}"
For more information, see [Transforming data with JSONata in Step Functions](./transforming-data.html).
** `Assign` (Optional)**
Used to store variables. The `Assign` field accepts a JSON object with
key/value pairs that define variable names and their assigned values. Any
string value, including those inside objects or arrays, will be evaluated as
JSONata when surrounded by `{% %}` characters
For more information, see [Passing data between states with variables](./workflow-variables.html).
###### Note
Although you can continue to include `Map` states that use the following fields, we highly recommend that you replace `Iterator` with `[ItemProcessor](#itemprocessor)` and `Parameters` with `[ItemSelector](#itemselector)`.
**
`Iterator`**
Specifies a JSON object that defines a set of steps that process each
element of the array.
**
`Parameters`**
Specifies a collection of key-value pairs, where the values can contain
any of the following:
* Static values that you define in your state machine
definition.
* Values selected from the input using a [path](./amazon-states-language-paths.html).
## Inline Map state example (JSONPath)
Consider the following input data for a `Map` state running in **Inline** mode.
```
`{
"ship-date": "2016-03-14T01:59:00Z",
"detail": {
"delivery-partner": "UQS",
"shipped": [
{ "prod": "R31", "dest-code": 9511, "quantity": 1344 },
{ "prod": "S39", "dest-code": 9511, "quantity": 40 },
{ "prod": "R31", "dest-code": 9833, "quantity": 12 },
{ "prod": "R40", "dest-code": 9860, "quantity": 887 },
{ "prod": "R40", "dest-code": 9511, "quantity": 1220 }
]
}
}`
```
Given the previous input, the `Map` state in the following example invokes
an AWS Lambda function named `ship-val` once for each item of the array in the
`shipped` field.
```
`"Validate All": {
"Type": "Map",
"InputPath": "$.detail",
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "INLINE"
},
"StartAt": "Validate",
"States": {
"Validate": {
"Type": "Task",
"Resource": "arn:aws:states:::lambda:invoke",
"OutputPath": "$.Payload",
"Parameters": {
"FunctionName": "arn:aws:lambda:us-east-2:`account-id`:function:`ship-val:$LATEST`"
},
"End": true
}
}
},
"End": true,
"ResultPath": "$.detail.shipped",
"ItemsPath": "$.shipped"
}`
```
Each iteration of the `Map` state sends an item in the array, selected with
the [ItemsPath](./input-output-itemspath.html) field, as input
to the `ship-val` Lambda function. The following values are an example of input the `Map` state sends to an invocation of the Lambda function:
```
`{
"prod": "R31",
"dest-code": 9511,
"quantity": 1344
}`
```
When complete, the output of the `Map` state is a JSON array, where each
item is the output of an iteration. In this case, this array contains the output of the
`ship-val` Lambda function.
## Inline Map state example with
`ItemSelector`
Suppose that the `ship-val` Lambda function in the previous example also
needs information about the shipment's courier. This information is in addition to the
items in the array for each iteration. You can include information from the input, along
with information specific to the current iteration of the `Map` state. Note
the `ItemSelector` field in the following example:
```
`"Validate-All": {
"Type": "Map",
"InputPath": "$.detail",
"ItemsPath": "$.shipped",
"MaxConcurrency": 0,
"ResultPath": "$.detail.shipped",
**"ItemSelector": {
"parcel.$": "$$.Map.Item.Value",
"courier.$": "$.delivery-partner"
},**
"ItemProcessor": {
"StartAt": "Validate",
"States": {
"Validate": {
"Type": "Task",
"Resource": "arn:aws:lambda:`region`:`account-id`:function:ship-val",
"End": true
}
}
},
"End": true
}`
```
The `ItemSelector` block replaces the input to the iterations with a JSON
node. This node contains both the current item data from the [Context object](./input-output-contextobject.html#contextobject-map) and the courier information from
the `Map` state input's `delivery-partner` field. The following is
an example of input to a single iteration. The `Map` state passes this input
to an invocation of the `ship-val` Lambda function.
```
`{
"parcel": {
"prod": "R31",
"dest-code": 9511,
"quantity": 1344
},
"courier": "UQS"
}`
```
In the previous *Inline Map state* example, the `ResultPath`
field produces output in the same format as the input. However, it overwrites the
`detail.shipped` field with an array in which each element is the output
of each iteration's `ship-val` Lambda
invocation.
For more information about using the *Inline Map state* state and
its fields, see the following.
* [Repeat actions with Inline Map](./tutorial-map-inline.html)
* [Processing input and output in Step Functions](./concepts-input-output-filtering.html)
* [ItemsPath (Map, JSONPath only)](./input-output-itemspath.html)
* [Context object data for Map states](./input-output-contextobject.html#contextobject-map)
## Inline `Map` state
input and output processing
For a given `Map` state, [InputPath](./input-output-inputpath-params.html#input-output-inputpath) selects a subset of the state's input.
The input of a `Map` state must include a JSON array. The `Map`
state runs the `ItemProcessor` section once for each item in the array. If
you specify the [ItemsPath](./input-output-itemspath.html)
field, the `Map` state selects where in the input to find the array to
iterate over. If not specified, the value of `ItemsPath` is `$`,
and the `ItemProcessor` section expects that the array is the only input. If
you specify the `ItemsPath` field, its value must be a [Reference Path](./amazon-states-language-paths.html#amazon-states-language-reference-paths). The
`Map` state applies this path to the effective input after it applies the
`InputPath`. The `ItemsPath` must identify a field whose value
is a JSON array.
The input to each iteration, by default, is a single element of the array field
identified by the `ItemsPath` value. You can override this value with the
`[ItemSelector (Map)](./input-output-itemselector.html)` field.
When complete, the output of the `Map` state is a JSON array, where each
item is the output of an iteration.
For more information about Inline Map state inputs and outputs, see the
following:
* [Repeat actions with Inline Map](./tutorial-map-inline.html)
* [Inline Map state example with
ItemSelector](#inline-map-state-example-params)
* [Processing input and output in Step Functions](./concepts-input-output-filtering.html)
* [Context object data for Map states](./input-output-contextobject.html#contextobject-map)
* [Process data from a queue with a Map state in Step Functions](./sample-map-state.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Map
Distributed mode
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.