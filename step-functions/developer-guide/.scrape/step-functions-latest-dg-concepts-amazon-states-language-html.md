---
url: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html
title: Using Amazon States Language to define Step Functions workflows
word_count: 353
filtered: true
elements_removed: 0
density_score: 0.84
---

Using Amazon States Language to define Step Functions workflows - AWS Step Functions
Using Amazon States Language to define Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#concepts-amazon-states-language)
[Example Amazon States Language
Specification (JSONata)](#example-amazon-states-language-specification)
# Using Amazon States Language to define Step Functions workflows
The Amazon States Language is a JSON-based, structured language used to define your state machine, a
collection of [states](./workflow-states.html), that can do work (`Task`
states), determine which states to transition to next (`Choice` states), stop an
execution with an error (`Fail` states), and so on.
For more information, see the [Amazon
States Language Specification](https://states-language.net/spec.html) and [Statelint](https://github.com/awslabs/statelint), a tool that validates Amazon States Language code.
To create a state machine on the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/)
using Amazon States Language, see [Getting Started](./getting-started.html).
###### Note
If you define your state machines outside the Step Functions' console, such as in an editor of your choice, you must save your state machine definitions with the extension *.asl.json*.
## Example Amazon States Language
Specification (JSONata)
```
`{
"Comment": "An example of the Amazon States Language using a choice state.",
"QueryLanguage": "JSONata",
"StartAt": "FirstState",
"States": {
"FirstState": {
"Type": "Task",
"Assign": {
"foo" : "{% $states.input.foo\_input %}"
},
"Resource": "arn:aws:lambda:`region`:123456789012:function:`FUNCTION\_NAME`",
"Next": "ChoiceState"
},
"ChoiceState": {
"Type": "Choice",
"Default": "DefaultState",
"Choices": [
{
"Next": "FirstMatchState",
"Condition": "{% $foo = 1 %}"
},
{
"Next": "SecondMatchState",
"Condition": "{% $foo = 2 %}"
}
]
},
"FirstMatchState": {
"Type" : "Task",
"Resource": "arn:aws:lambda:`region`:123456789012:function:`OnFirstMatch`",
"Next": "NextState"
},
"SecondMatchState": {
"Type" : "Task",
"Resource": "arn:aws:lambda:`region`:123456789012:function:`OnSecondMatch`",
"Next": "NextState"
},
"DefaultState": {
"Type": "Fail",
"Error": "DefaultStateError",
"Cause": "No Matches!"
},
"NextState": {
"Type": "Task",
"Resource": "arn:aws:lambda:`region`:123456789012:function:`FUNCTION\_NAME`",
"End": true
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Choosing workflow type
State machine structure
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.