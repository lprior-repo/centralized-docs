---
url: https://docs.aws.amazon.com/step-functions/latest/dg/statemachine-structure.html
title: State machine structure in Amazon States Language for Step Functions workflows
word_count: 1092
filtered: true
elements_removed: 0
density_score: 0.84
---

State machine structure in Amazon States Language for Step Functions workflows - AWS Step Functions
State machine structure in Amazon States Language for Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#statemachine-structure)
[Common state fields](#amazon-states-language-common-fields)
# State machine structure in Amazon States Language for Step Functions workflows
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
State machines are defined using JSON text that represents a structure containing the following fields.
**
`Comment` (Optional)**
A human-readable description of the state machine.
**
`QueryLanguage` (Optional; when omitted, defaults to `JSONPath`)**
* The name of the query language used by the state machine. Allowed
values are `JSONPath` and `JSONata`.
* If not provided for the state machine, the default value for each
state is JSONPath.
* When the top-level state machine query language is
`JSONPath`, individual states can override the query language
by setting QueryLanguage to `JSONata`. Given this approach,
you can incrementally convert a state machine from JSONPath to JSONata
one state at a time.
* **Note: **You cannot revert a top-level
JSONata-based state machine to a mix of JSONata and JSONPath
states.
**
`StartAt` (Required)**
A string that must exactly match (is case sensitive) the name of one of
the state objects.
**
`TimeoutSeconds` (Optional)**
The maximum number of seconds an execution of the state machine can run.
If it runs longer than the specified time, the execution fails with a
`States.Timeout`
[Error Name](./concepts-error-handling.html#error-handling-error-representation).
**
`Version` (Optional)**
The version of the Amazon States Language used in the state machine (default is "1.0").
**
`States` (Required)**
An object containing a comma-delimited set of states.
The `States` field contains [States](./workflow-states.html).
```
`{
"State1" : {
},
"State2" : {
},
...
}`
```
A state machine is defined by the states it contains and the relationships between them.
The following is an example.
```
`{
"Comment": "A Hello World example of the Amazon States Language using a Pass state",
"StartAt": "HelloWorld",
"States": {
"HelloWorld": {
"Type": "Pass",
"Result": "Hello World!",
"End": true
}
}
}`
```
When an execution of this state machine is launched, the system begins with the state
referenced in the `StartAt` field (`"HelloWorld"`). If this state has
an `"End": true` field, the execution stops and returns a result. Otherwise,
the system looks for a `"Next":` field and continues with that state
next. This process repeats until the system reaches a terminal state (a state with
`"Type": "Succeed"`, `"Type": "Fail"`, or `"End": true`), or a runtime
error occurs.
The following rules apply to states within a state machine:
* States can occur in any order within the enclosing block, but the order in
which they're listed doesn't affect the order in which they're run. The contents of
the states determines this order.
* Within a state machine, there can be only one state that's designated as the
`start` state, designated by the value of the `StartAt` field in the
top-level structure. This state is the one that is executed first when the
execution starts.
* Any state for which the `End` field is `true` is
considered an `end` (or `terminal`) state. Depending on your
state machine logic—for example, if your state machine has multiple branches
of execution—you might have more than one `end` state.
* If your state machine consists of only one state, it can be both the `start`
state and the `end` state.
## Common state fields in workflows
The following fields are common to all state elements.
**
`Type` (Required)**
The state's type: Task, Choice, Parallel, Map, Pass, Wait, Succeed, Fail.
**`QueryLanguage` (Optional; when omitted, defaults to
`JSONPath`)**
* The name of the query language used by the state. Allowed values
are `JSONPath` and `JSONata`.
* When the top-level state machine query language is
`JSONPath`, individual states can override the query
language by setting QueryLanguage to `JSONata`. Given
this approach, you can incrementally convert a state machine from
JSONPath to JSONata one state at a time.
**
`Next`
**
The name of the next state that is run when the current state finishes. Some state types, such as `Choice`, allow multiple transition states.
If the current state is the last state in your workflow, or a terminal state, such as [Succeed workflow state](./state-succeed.html) or [Fail workflow state](./state-fail.html), you don't need to specify the `Next` field.
**
`End`
**
Designates this state as a terminal state (ends the execution) if set to `true`. There can be any number of terminal states per state machine. Only one of `Next` or `End` can be used in a state. Some state types, such as `Choice`, or terminal states, such as [Succeed workflow state](./state-succeed.html) and [Fail workflow state](./state-fail.html), don't support or use the `End` field.
**
`Comment` (Optional)**
Holds a human-readable description of the state.
** `Assign` (Optional)**
Used to store variables. The `Assign` field accepts a JSON object with
key/value pairs that define variable names and their assigned values. Any
string value, including those inside objects or arrays, will be evaluated as
JSONata when surrounded by `{% %}` characters
For more information, see [Passing data between states with variables](./workflow-variables.html).
**
`Output` (Optional, JSONata only)**
Used to specify and transform output from the state. When specified, the value overrides the state output default.
The output field accepts any JSON value (object, array, string, number, boolean, null). Any string value, including those inside objects or arrays, will be evaluated as JSONata if surrounded by {% %} characters.
Output also accepts a JSONata expression directly, for example: "Output": "{% jsonata expression %}"
For more information, see [Input and Output Processing](./concepts-input-output-filtering.html).
**
`InputPath` (Optional, JSONPath only)**
A [path](./concepts-input-output-filtering.html) that selects a portion of the state's input to be passed
to the state's task for processing. If omitted, it has the value `$` which designates
the entire input. For more information, see [Input and Output Processing](./concepts-input-output-filtering.html).
**
`OutputPath` (Optional, JSONPath only)**
A [path](./concepts-input-output-filtering.html) that selects a portion of the state's output to be passed to the next state. If omitted, it has the value
`$` which designates the entire output. For more information, see [Input and Output Processing](./concepts-input-output-filtering.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Amazon States Language
Intrinsic functions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.