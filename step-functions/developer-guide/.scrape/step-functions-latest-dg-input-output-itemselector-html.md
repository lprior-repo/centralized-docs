---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-itemselector.html
title: input output itemselector.html
word_count: 449
filtered: true
elements_removed: 0
density_score: 0.83
---

ItemSelector (Map) - AWS Step Functions
ItemSelector (Map) - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-itemselector)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
By default, the effective input for the `Map` state is the set of individual
data items present in the raw state input. With the `ItemSelector` field, you can
override the data items’ values before they’re passed on to the `Map` state.
To override the values, specify a valid JSON input that contains a collection of key-value
pairs. The pairs can be static values provided in your state machine definition, values
selected from the state input using a [path](./amazon-states-language-paths.html), or values accessed from the [Context
object](./input-output-contextobject.html).
If you specify key-value pairs using a path or Context object, the key name must end in `.$`.
###### Note
The `ItemSelector` field replaces the `Parameters` field within the
`Map` state. If you use the `Parameters` field in your
`Map` state definitions to create custom input, we recommend that you replace
them with `ItemSelector`.
You can specify the `ItemSelector` field in both
an
*Inline Map state* and
a
*Distributed Map state*.
For example, consider the following JSON input that contains an array of three items
within the `imageData` node. For each *`Map` state iteration*, an array item is passed to the iteration as input.
```
`[
{
"resize": "true",
"format": "jpg"
},
{
"resize": "false",
"format": "png"
},
{
"resize": "true",
"format": "jpg"
}
]`
```
Using the `ItemSelector` field, you can define a custom JSON input to override
the original input as shown in the following example. Step Functions then passes this custom input to
each *`Map` state iteration*. The custom input contains a static value for `size` and the value of a Context object data for `Map` state. The
`$$.Map.Item.Value` Context object contains the value of each individual data
item.
```
`{
"ItemSelector": {
"size": 10,
"value.$": "$$.Map.Item.Value"
}
}`
```
The following example shows the input received by one
iteration
of the *Inline Map state*:
```
`{
"size": 10,
"value": {
"resize": "true",
"format": "jpg"
}
}`
```
###### Tip
For a complete example of a *Distributed Map state* that uses the `ItemSelector` field, see [Copy large-scale CSV using Distributed Map](./tutorial-map-distributed.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ItemsPath (JSONPath)
ItemBatcher
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.