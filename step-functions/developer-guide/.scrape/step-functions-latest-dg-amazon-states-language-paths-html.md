---
url: https://docs.aws.amazon.com/step-functions/latest/dg/amazon-states-language-paths.html
title: amazon states language paths.html
word_count: 522
filtered: true
elements_removed: 0
density_score: 0.88
---

Using JSONPath paths - AWS Step Functions
Using JSONPath paths - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#amazon-states-language-paths)
[Reference Paths](#amazon-states-language-reference-paths)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
In the Amazon States Language, a *path* is a string beginning with `$` that
you can use to identify components within JSON text. Paths follow [JsonPath](https://datatracker.ietf.org/wg/jsonpath/about/) syntax, which is only available when the `QueryLanguage` is set to JSONPath. You can specify a path
to access subsets of the input when specifying values for `InputPath`,
`ResultPath`, and `OutputPath`.
You must use square bracket notation if your field name contains any
character that is not included in the `member-name-shorthand`
definition of the [JsonPath ABNF](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#jsonpath-abnf) rule. Therefore, to encode special
characters, such as punctuation marks (excluding `\_`), you
must use square bracket notation. For example, `$.abc.['def ghi']`.
## Reference Paths
A *reference path* is a path whose syntax is limited in such a way that it can identify only a single node in a JSON structure:
* You can access object fields using only dot (`.`) and square bracket (`[ ]`) notation.
* Functions such as `length()` aren't supported.
* Lexical operators, which are non-symbolic, such as `subsetof` aren't supported.
* Filtering by regular expression or by referencing another value in the JSON structure is not supported.
* The operators `@`, `,`, `:`, and `?` are not supported
For example, if state input data contains the following values:
```
`{
"foo": 123,
"bar": ["a", "b", "c"],
"car": {
"cdr": true
}
}`
```
The following reference paths would return the following.
```
`$.foo =&gt; 123
$.bar =&gt; ["a", "b", "c"]
$.car.cdr =&gt; true
`
```
Certain states use paths and reference paths to control the flow of a state machine or configure a state's settings or options. For more information,
see [Modeling workflow input and output path processing with
data flow simulator](https://aws.amazon.com/blogs/compute/modeling-workflow-input-output-path-processing-with-data-flow-simulator/) and
[Using JSONPath effectively in AWS Step Functions](https://aws.amazon.com/blogs/compute/using-jsonpath-effectively-in-aws-step-functions/).
### Flattening an array of arrays
If the [Parallel workflow state](./state-parallel.html) or [Map workflow state](./state-map.html) state in your state machines return an array of arrays, you can transform them into a flat array with the [ResultSelector](./input-output-inputpath-params.html#input-output-resultselector) field. You can include this field inside the Parallel or Map state definition to manipulate the result of these states.
To flatten arrays, use the syntax: `[\*]` in the `ResultSelector` field as shown in the following example.
```
`"ResultSelector": {
"flattenArray.$": "$[\*][\*]"
}`
```
For examples that show how to flatten an array, see *Step 3* in the following tutorials:
* [Processing batch data with a Lambda function in Step Functions](./tutorial-itembatcher-param-task.html)
* [Processing individual items with a Lambda function in Step Functions](./tutorial-itembatcher-single-item-process.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Context object
Manipulate parameters with paths
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.