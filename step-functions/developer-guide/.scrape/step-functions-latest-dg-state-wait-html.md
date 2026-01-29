---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-wait.html
title: state wait.html
word_count: 529
filtered: true
elements_removed: 0
density_score: 0.85
---

Wait workflow state - AWS Step Functions
Wait workflow state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-wait)
[Wait State Examples](#wait-state-example)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
A `Wait` state (`"Type": "Wait"`) delays the state machine from
continuing for a specified time. You can choose either a relative time, specified in seconds
from when the state begins, or an absolute end time, specified as a timestamp.
In addition to the [common state
fields](./statemachine-structure.html#amazon-states-language-common-fields), `Wait` states have one of the following fields.
**
`Seconds`
**
A time, in seconds, to wait before beginning the state specified in the `Next` field. You must specify time as an integer value from 0 to 99999999. In JSONata states, you can alternatively specify a JSONata expression which must evaluate to an integer in the stated range.
**
`Timestamp`
**
An absolute time to wait until beginning the state specified in the
`Next` field.
Timestamps must conform to the RFC3339 profile of ISO 8601, with the further
restrictions that an uppercase `T` must separate the date and time portions,
and an uppercase `Z` must denote that a numeric time zone offset is not
present, for example, `2024-08-18T17:33:00Z`.
In JSONata states, you can specify a JSONata expression which results in a string that conforms to the previous requirements.
###### Note
Currently, if you specify the wait time as a timestamp, Step Functions considers the time value up to seconds and truncates milliseconds.
**
`SecondsPath` (Optional, JSONPath only)
**
A [path](./concepts-input-output-filtering.html) in the states input data to an integer value that specifies the time to wait, in seconds, before proceeding to the next state.
**
`TimestampPath` (Optional, JSONPath only)
**
A [path](./concepts-input-output-filtering.html) in the states input data to an absolute date and time (timestamp) to wait before proceeding to the next state.
###### Note
You must specify exactly one of `Seconds`, `Timestamp`, `SecondsPath`, or `TimestampPath`. In addition, the maximum wait time that
you can specify for Standard Workflows and Express workflows is one year and five minutes respectively.
## Wait State Examples
The following `Wait` state introduces a 10-second delay into a
state machine.
```
`"wait\_ten\_seconds": {
"Type": "Wait",
"Seconds": 10,
"Next": "NextState"
}`
```
In the next example, the `Wait` state waits until an absolute time: March 14, 2024,
at 1:59 AM UTC.
```
`"wait\_until" : {
"Type": "Wait",
"Timestamp": "2024-03-14T01:59:00Z",
"Next": "NextState"
}`
```
You don't have to hard-code the wait duration. For example, given the following input
data:
```
`{
"expirydate": "2024-03-14T01:59:00Z"
}`
```
You can select the value of "expirydate" from the input using a reference [path](./concepts-input-output-filtering.html) to select it from
the input data.
```
`"wait\_until" : {
"Type": "Wait",
"TimestampPath": "$.expirydate",
"Next": "NextState"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Pass
Succeed
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.