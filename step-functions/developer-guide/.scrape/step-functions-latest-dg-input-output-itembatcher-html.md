---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-itembatcher.html
title: input output itembatcher.html
word_count: 1029
filtered: true
elements_removed: 0
density_score: 0.82
---

ItemBatcher (Map) - AWS Step Functions
ItemBatcher (Map) - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-itembatcher)
[Fields to specify item batching](#input-output-itembatcher-subfields)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
The `ItemBatcher` field is a JSON object, which specifies to process a group of
items in a single child workflow execution. Use batching when processing large CSV files or JSON arrays, or large sets of Amazon S3 objects.
The following example shows the syntax of the `ItemBatcher` field. In the
following syntax, the maximum number of items that each child workflow execution should process
is set to 100.
```
`{
"ItemBatcher": {
"MaxItemsPerBatch": `100`
}
}`
```
By default, each item in a dataset is passed as input to individual child workflow
executions. For example, assume you specify a JSON file as input that contains the following
array:
```
`[
{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
},
{
"verdict": "false",
"statement\_date": "6/7/2022",
"statement\_source": "television"
},
{
"verdict": "true",
"statement\_date": "5/18/2016",
"statement\_source": "news"
},
...
]`
```
For the given input, each child workflow execution receives an array item as its input. The
following example shows the input of a child workflow execution:
```
`{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
}`
```
To
help optimize the performance and cost of your processing job, select a batch
size that balances the number of items against the items processing time. If you use batching,
Step Functions adds the items to an **Items** array. It then passes the array as input to
each child workflow execution. The following example shows a batch of two items passed as input
to a child workflow execution:
```
`{
"Items": [
{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
},
{
"verdict": "false",
"statement\_date": "6/7/2022",
"statement\_source": "television"
}
]
}`
```
###### Tip
To learn more about using the `ItemBatcher` field in your workflows, try the following tutorials and workshop:
* [Process an entire batch of data within a Lambda function](./tutorial-itembatcher-param-task.html)
* [Iterate over items in a batch inside child workflow executions](./tutorial-itembatcher-single-item-process.html)
* [Distributed map and related resources](https://catalog.workshops.aws/stepfunctions/use-cases/distributed-map) in *The AWS Step Functions Workshop*
## Fields to specify item batching
To batch items, specify the maximum number of items to batch, the maximum batch size, or
both. You must specify one of these values to batch items.
**Max items per batch**
Specifies the maximum number of items that each child workflow execution processes. The
interpreter limits the number of items batched in the `Items` array to this
value. If you specify both a batch number and size, the interpreter reduces the number of
items in a batch to avoid exceeding the specified batch size limit.
If you don't specify this value but provide a value for maximum batch size,
Step Functions processes as many items as possible in each child workflow execution without
exceeding the maximum batch size in bytes.
For example, imagine you run an execution with an input JSON file that
contains 1130 nodes. If you specify a maximum items value for each batch of 100, Step Functions
creates 12 batches. Of these, 11 batches contain 100 items each, while the twelfth batch
contains the remaining 30 items.
Alternatively, you can specify the maximum items for each batch as a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) to an existing key-value pair in your *Distributed Map state* input. This path must resolve to a positive integer.
For example, given the following input:
```
`{
`"maxBatchItems"`: `500`
}`
```
You can specify the maximum number of items to batch using a reference path
(**JSONPath only**) as follows:
```
`{
...
"Map": {
"Type": "Map",
"MaxConcurrency": 2000,
**"ItemBatcher": {
`"MaxItemsPerBatchPath"`: `"$.maxBatchItems"`
}**
...
...
}
}`
```
For **JSONata-based** states, you can also provide
a JSONata expression that evaluates to a positive integer.
###### Important
You can specify either the `MaxItemsPerBatch` or the `MaxItemsPerBatchPath (JSONPath only)` sub-field, but not both.
**Max KiB per batch**
Specifies the maximum size of a batch in bytes, up to 256 KiB. If you specify both a maximum batch number and size, Step Functions reduces the number of items in a batch to avoid exceeding the specified batch size limit.
Alternatively, you can specify the maximum batch size as a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) to an existing key-value pair in your *Distributed Map state* input. This path must resolve to a positive integer.
###### Note
If you use batching and don't specify a maximum batch size, the interpreter processes as many
items it can process up to 256 KiB in each child workflow execution.
For example, given the following input:
```
`{
`"batchSize"`: `131072`
}`
```
You can specify the maximum batch size using a reference path as
follows:
```
`{
...
"Map": {
"Type": "Map",
"MaxConcurrency": 2000,
**"ItemBatcher": {
`"MaxInputBytesPerBatchPath"`: `"$.batchSize"`
}**
...
...
}
}`
```
For **JSONata-based** states, you can also
provide a JSONata expression that evaluates to a positive integer.
###### Important
You can specify either the `MaxInputBytesPerBatch` or the
`MaxInputBytesPerBatchPath` (JSONPath only) sub-field, but not
both.
**Batch input**
Optionally, you can also specify a fixed JSON input to include in each batch passed to each
child workflow execution. Step Functions merges this input with the input for each individual
child workflow executions. For example, given the following fixed input of a fact check
date on an array of items:
```
`"ItemBatcher": {
**`"BatchInput": {`
`"factCheck"`: `"December 2022"`
}**
}`
```
Each child workflow execution receives the following as input:
```
`{
"BatchInput": {
"factCheck": "December 2022"
},
"Items": [
{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
},
{
"verdict": "false",
"statement\_date": "6/7/2022",
"statement\_source": "television"
},
...
]
}`
```
For **JSONata-based** states, you can provide
JSONata expressions directly to BatchInput, or use JSONata expressions inside
JSON objects or arrays.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ItemSelector
ResultWriter
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.