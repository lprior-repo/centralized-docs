---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-succeed.html
title: Succeed workflow state
word_count: 276
filtered: true
elements_removed: 0
density_score: 0.86
---

Succeed workflow state - AWS Step Functions
Succeed workflow state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-succeed)
# Succeed workflow state
A `Succeed` state (`"Type": "Succeed"`) either terminates a state
machine successfully, ends a branch of a [Parallel workflow state](./state-parallel.html), or ends an
iteration of a [Map workflow state](./state-map.html). The `Succeed` state is a useful
target for `Choice` state branches that don't do anything except terminate the
state machine.
Because `Succeed` states are terminal states, they have no `Next`
field, and don't need an `End` field, as shown in the following example.
```
`"SuccessState": {
"Type": "Succeed"
}`
```
**`Output` (Optional, JSONata only)**
In addition to the [common state
fields](./statemachine-structure.html#amazon-states-language-common-fields), `Succeed` states that use JSONata can include an Output field to specify and
transform output from the state. When specified, the `Output` value overrides the
state output default.
The output field accepts any JSON value (object, array, string, number,
boolean, null). Any string value, including those inside objects or arrays,
will be evaluated as JSONata if surrounded by `{% %}` characters.
Output also accepts a JSONata expression directly, for example:
```
`"Output" : "{% jsonata expression %}"`
```
For more information on JSONata, see [Transforming data with JSONata in Step Functions](./transforming-data.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Wait
Fail
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.