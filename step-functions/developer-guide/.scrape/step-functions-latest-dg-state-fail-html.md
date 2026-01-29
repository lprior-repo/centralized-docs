---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-fail.html
title: state fail.html
word_count: 583
filtered: true
elements_removed: 0
density_score: 0.91
---

Fail workflow state - AWS Step Functions
Fail workflow state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-fail)
[Fail state definition examples](#fail-state-examples)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
A `Fail` state (`"Type": "Fail"`) stops the execution of the state
machine and marks it as a failure, unless it is caught by a `Catch` block.
The `Fail` state only allows the use of `Type` and
`Comment` fields from the set of [common state fields](./statemachine-structure.html#amazon-states-language-common-fields). In addition,
the `Fail` state allows the following fields.
**
`Cause` (Optional)**
A custom string that describes the cause of the error. You can specify this field for operational or diagnostic purposes.
In JSONata states, you can also specify a JSONata expression.
**
`CausePath` (Optional, JSONPath only)
**
If you want to provide a detailed description about the cause of the error dynamically from the state input using a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths), use `CausePath`. When resolved, the reference path must select a field that contains a string value.
You can also specify `CausePath` using an [intrinsic function](./intrinsic-functions.html) that returns a string. These intrinsics are: [States.Format](./intrinsic-functions.html#asl-intrsc-func-generic), [States.JsonToString](./intrinsic-functions.html#jsontostring), [States.ArrayGetItem](./intrinsic-functions.html#arraygetitem), [States.Base64Encode](./intrinsic-functions.html#base64encode), [States.Base64Decode](./intrinsic-functions.html#base64decode), [States.Hash](./intrinsic-functions.html#asl-intrsc-func-uuid-generate), and [States.UUID](./intrinsic-functions.html#statesuuid).
###### Important
* You can specify either `Cause` or `CausePath`, but not both in your Fail state definition.
* As an information security best practice, we recommend that you remove any sensitive information or internal system details from the cause description.
**
`Error` (Optional)**
An error name that you can provide to perform error handling using [Retry](./concepts-error-handling.html#error-handling-retrying-after-an-error) or [Catch](./concepts-error-handling.html#error-handling-fallback-states) fields. You can also provide an error name for operational or diagnostic purposes.
In JSONata states, you can also specify a JSONata expression.
**
`ErrorPath` (Optional, JSONPath only)
**
If you want to provide a name for the error dynamically from the state input using a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths), use `ErrorPath`. When resolved, the reference path must select a field that contains a string value.
You can also specify `ErrorPath` using an [intrinsic function](./intrinsic-functions.html) that returns a string. These intrinsics are: [States.Format](./intrinsic-functions.html#asl-intrsc-func-generic), [States.JsonToString](./intrinsic-functions.html#jsontostring), [States.ArrayGetItem](./intrinsic-functions.html#arraygetitem), [States.Base64Encode](./intrinsic-functions.html#base64encode), [States.Base64Decode](./intrinsic-functions.html#base64decode), [States.Hash](./intrinsic-functions.html#asl-intrsc-func-uuid-generate), and [States.UUID](./intrinsic-functions.html#statesuuid).
###### Important
* You can specify either `Error` or `ErrorPath`, but not both in your Fail state definition.
* As an information security best practice, we recommend that you remove any sensitive information or internal system details from the error name.
Because `Fail` states always exit the state machine, they have no
`Next` field and don't require an `End` field.
## Fail state definition examples
The following Fail state definition example specifies static `Error` and `Cause` field values.
```
`"FailState": {
"Type": "Fail",
"Cause": "Invalid response.",
"Error": "ErrorA"
}`
```
The following Fail state definition example uses reference paths dynamically to resolve the `Error` and `Cause` field values.
```
`"FailState": {
"Type": "Fail",
"CausePath": "$.Cause",
"ErrorPath": "$.Error"
}`
```
The following Fail state definition example uses the [States.Format](./intrinsic-functions.html#asl-intrsc-func-generic) intrinsic function to specify the `Error` and `Cause` field values dynamically.
```
`"FailState": {
"Type": "Fail",
"CausePath": "States.Format('This is a custom error message for {}, caused by {}.', $.Error, $.Cause)",
"ErrorPath": "States.Format('{}', $.Error)"
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Succeed
Tutorials and Workshops
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.