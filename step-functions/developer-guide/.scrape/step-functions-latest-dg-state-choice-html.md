---
url: https://docs.aws.amazon.com/step-functions/latest/dg/state-choice.html
title: state choice.html
word_count: 1084
filtered: true
elements_removed: 0
density_score: 0.84
---

Choice workflow state - AWS Step Functions
Choice workflow state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#state-choice)
[Choice Rules (JSONata)](#state-choice-rules)[Choice Rules (JSONPath)](#state-choice-rules-jsonpath)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
A `Choice` state (`"Type": "Choice"`) adds conditional logic to a
state machine.
In addition to most of the [common
state fields](./statemachine-structure.html#amazon-states-language-common-fields), `Choice` states contains the following additional
fields.
**`Choices` (Required)**
An array of [Choice Rules](#state-choice-rules) that
determines which state the state machine transitions to next. You must define at
least one rule in the `Choice` state.
When a `Choice` state is run, Step Functions evaluates each **Choice Rule** to true or false. Based on the result, Step Functions
transitions to the next state in the workflow.
**`Default` (Optional, Recommended)**
The name of the state to transition to if no **Choice
Rule** evaluates to true.
###### Important
`Choice` states do not support the `End` field. In addition,
they use `Next` only inside their `Choices` field.
If no **Choices** evaluate to true when the workflow runs, and no **Default** is provided, the
state machine will throw an **error** due to a *failure to transition out of the state*.
## Choice Rules (JSONata)
A `Choice` state must have a `Choices` field whose value is a
non-empty array of Choice Rules, which contain the following fields when using
JSONata:
* **`Condition` field** – a JSONata expression that
evaluates to true/false.
* **`Next` field** – a value that
must match a state name in the state machine.
The following example checks whether the numerical value is equal to
`1`.
```
`{
"Condition": "{% $foo = 1 %}",
"Next": "NumericMatchState"
}`
```
The following example checks whether the `type`variable is equal to
`local`.
```
`{
"Condition": "{% $type = 'local' %}",
"Next": "StringMatchState"
}`
```
The following example checks whether the string is greater than
`MyStringABC`.
```
`{
"Condition": "{% $foo &gt; 'MyStringABC' %}",
"Next": "StringGreaterMatchState"
}`
```
The following example checks whether the string is not null.
```
`{
"Condition" : "{% $possiblyNullValue != null and $possiblyNullValue = 42 %}",
"Next": "NotNullAnd42"
}`
```
## Choice Rules (JSONPath)
A `Choice` state must have a `Choices` field whose value is a
non-empty array of Choice Rules, which contain the following fields when using
JSONPath:
* A **comparison** – Two fields that specify
an input variable to compare, the type of comparison, and the value to compare
the variable to. Choice Rules support comparison between two variables. Within a
Choice Rule, the value of variable can be compared with another value from the
state input by appending `Path` to name of supported comparison
operators. The values of `Variable` and Path fields in a comparison
must be valid [Reference
Paths](./amazon-states-language-paths.html#amazon-states-language-reference-paths).
* A **`Next` field** – The value
of this field must match a state name in the state machine.
The following example checks whether the numerical value is equal to
`1`.
```
`{
"Variable": "$.foo",
"NumericEquals": 1,
"Next": "FirstMatchState"
}`
```
The following example checks whether the string is equal to
`MyString`.
```
`{
"Variable": "$.foo",
"StringEquals": "MyString",
"Next": "FirstMatchState"
}`
```
The following example checks whether the string is greater than
`MyStringABC`.
```
`{
"Variable": "$.foo",
"StringGreaterThan": "MyStringABC",
"Next": "FirstMatchState"
}`
```
The following example checks whether the string is null.
```
`{
"Variable": "$.possiblyNullValue",
"IsNull": true
}`
```
The following example shows how the StringEquals rule is only evaluated when
`$.keyThatMightNotExist` exists because of the preceding
`IsPresent` Choice Rule.
```
`"And": [
{
"Variable": "$.keyThatMightNotExist",
"IsPresent": true
},
{
"Variable": "$.keyThatMightNotExist",
"StringEquals": "foo"
}
]`
```
The following example checks whether a pattern with a wildcard matches.
```
`{
"Variable": "$.foo",
"StringMatches": "log-\*.txt"
}`
```
The following example checks whether the timestamp is equal to
`2001-01-01T12:00:00Z`.
```
`{
"Variable": "$.foo",
"TimestampEquals": "2001-01-01T12:00:00Z",
"Next": "FirstMatchState"
}`
```
The following example compares a variable with another value from the state
input.
```
`{
"Variable": "$.foo",
"StringEqualsPath": "$.bar"
}`
```
Step Functions examines each of the Choice Rules in the order listed in the
`Choices` field. Then it transitions to the state specified in the
`Next` field of the first Choice Rule in which the variable matches the value
according to the comparison operator.
The following comparison operators are supported:
* `And`
* `BooleanEquals`,`BooleanEqualsPath`
* `IsBoolean`
* `IsNull`
* `IsNumeric`
* `IsPresent`
* `IsString`
* `IsTimestamp`
* `Not`
* `NumericEquals`,`NumericEqualsPath`
* `NumericGreaterThan`,`NumericGreaterThanPath`
* `NumericGreaterThanEquals`,`NumericGreaterThanEqualsPath`
* `NumericLessThan`,`NumericLessThanPath`
* `NumericLessThanEquals`,`NumericLessThanEqualsPath`
* `Or`
* `StringEquals`,`StringEqualsPath`
* `StringGreaterThan`,`StringGreaterThanPath`
* `StringGreaterThanEquals`,`StringGreaterThanEqualsPath`
* `StringLessThan`,`StringLessThanPath`
* `StringLessThanEquals`,`StringLessThanEqualsPath`
* `StringMatches`
* `TimestampEquals`,`TimestampEqualsPath`
* `TimestampGreaterThan`,`TimestampGreaterThanPath`
* `TimestampGreaterThanEquals`,`TimestampGreaterThanEqualsPath`
* `TimestampLessThan`,`TimestampLessThanPath`
* `TimestampLessThanEquals`,`TimestampLessThanEqualsPath`
For each of these operators, the corresponding value must be of the appropriate type:
string, number, Boolean, or timestamp. Step Functions doesn't attempt to match a numeric field to
a string value. However, because timestamp fields are logically strings, it's possible
that a field considered to be a timestamp can be matched by a `StringEquals`
comparator.
###### Note
For interoperability, don't assume that numeric comparisons work with values
outside the magnitude or precision that the [IEEE
754-2008 `binary64` data type](https://en.wikipedia.org/wiki/IEEE_754#Basic_and_interchange_formats) represents. In particular, integers
outside of the range `[-253+1,
253-1]` might fail to compare in the expected
way.
Timestamps (for example, `2016-08-18T17:33:00Z`) must conform to [RFC3339 profile ISO 8601](https://www.ietf.org/rfc/rfc3339.txt), with
further restrictions:
* An uppercase `T` must separate the date and time
portions.
* An uppercase `Z` must denote that a numeric time zone offset
isn't present.
To understand the behavior of string comparisons, see the [Java `compareTo` documentation](https://docs.oracle.com/javase/8/docs/api/java/lang/String.html#compareTo-java.lang.String-).
The values of the `And` and `Or` operators must be non-empty
arrays of Choice Rules that must not themselves contain `Next` fields.
Likewise, the value of a `Not` operator must be a single Choice Rule that
must not contain `Next` fields.
You can create complex, nested Choice Rules using `And`,
`Not`, and `Or`. However, the `Next` field can
appear only in a top-level Choice Rule.
String comparison against patterns with one or more wildcards (“\*”) can be
performed with the StringMatches comparison operator. The wildcard character is
escaped by using the standard `\\\\ (Ex: “\\\\\*”)`. No characters other than
“\*” have any special meaning during matching.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Task
Parallel
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.