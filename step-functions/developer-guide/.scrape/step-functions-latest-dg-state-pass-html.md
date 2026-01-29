---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-pass.html
title: state pass.html
word_count: 517
filtered: true
elements_removed: 0
density_score: 0.84
---

Pass workflow state - AWS Step Functions
Pass workflow state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-pass)
[Pass State Example (JSONPath)](#pass-state-example)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
A `Pass` state (`"Type": "Pass"`) passes its input to its output,
without performing work. `Pass` states are useful when constructing and debugging
state machines.
You can also use a `Pass` state to transform JSON state input using filters, and then pass the transformed data to the next state in your workflows. For information about input transformation, see [Manipulate parameters in Step Functions workflows](./input-output-inputpath-params.html).
In addition to the [common state
fields](./statemachine-structure.html#amazon-states-language-common-fields), `Pass` states allow the following fields.
**
`Assign` (Optional, JSONata only)**
A collection of key-value pairs to assign data to variables. For
more information, see [Passing data between states with variables](./workflow-variables.html).
**`Output` (Optional, JSONata only)**
Used to specify and transform output from the state. When specified, the
value overrides the state output default.
The output field accepts any JSON value (object, array, string, number,
boolean, null). Any string value, including those inside objects or arrays,
will be evaluated as JSONata if surrounded by {% %} characters.
Output also accepts a JSONata expression directly, for example: "Output":
"{% jsonata expression %}"
For more information, see [Transforming data with JSONata in Step Functions](./transforming-data.html).
**
`Result` (Optional, JSONPath only)**
Refers to the output of a virtual task that is passed on to the next state. If you include the `ResultPath` field in your state machine definition,
`Result` is placed as specified by `ResultPath` and passed on to the next state.
**
`ResultPath` (Optional, JSONPath only)**
Specifies where to place the *output* (relative to the input) of the virtual task specified in `Result`. The input is further filtered
as specified by the `OutputPath` field (if present) before being used as the state's output. For more information, see
[Processing input and output in Step Functions](./concepts-input-output-filtering.html).
**
`Parameters` (Optional, JSONPath only)**
Creates a collection of key-value pairs that will be passed as input. You can specify `Parameters` as a static value or select from the input using a path. For
more information, see [Manipulate parameters in Step Functions workflows](./input-output-inputpath-params.html).
## Pass State Example (JSONPath)
Here is an example of a `Pass` state that injects some fixed data into the state machine,
probably for testing purposes.
```
`"No-op": {
"Type": "Pass",
"Result": {
"x-datum": 0.381018,
"y-datum": 622.2269926397355
},
"ResultPath": "$.coords",
"End": true
}`
```
Suppose the input to this state is the following.
```
`{
"georefOf": "Home"
}`
```
Then the output would be this.
```
`{
"georefOf": "Home",
"coords": {
"x-datum": 0.381018,
"y-datum": 622.2269926397355
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Distributed mode
Wait
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.