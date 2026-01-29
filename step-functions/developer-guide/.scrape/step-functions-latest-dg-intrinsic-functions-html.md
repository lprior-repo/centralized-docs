---
url: https://docs.aws.amazon.com/step-functions/latest/dg/intrinsic-functions.html
title: intrinsic functions.html
word_count: 3235
filtered: true
elements_removed: 0
density_score: 0.77
---

Intrinsic functions for JSONPath states in Step Functions - AWS Step Functions
Intrinsic functions for JSONPath states in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#intrinsic-functions)
[Fields that support intrinsic functions](#intrinsic-functions-states)[Intrinsics for arrays](#asl-intrsc-func-arrays)[Intrinsics for data encoding and decoding](#asl-intrsc-func-data-encode-decode)[Intrinsic for hash calculation](#asl-intrsc-func-hash-calc)[Intrinsics for JSON data manipulation](#asl-intrsc-func-json-manipulate)[Intrinsics for Math operations](#asl-intrsc-func-math-operation)[Intrinsic for String operation](#asl-intrsc-func-string-operation)[Intrinsic for unique identifier generation](#asl-intrsc-func-uuid-generate)[Intrinsic for generic operation](#asl-intrsc-func-generic)[Reserved characters in intrinsic functions](#intrinsic-functions-escapes)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
###### Warning
Intrinsic functions are only available to states that use the **JSONPath** query language. For JSONata, see [Transforming data with JSONata in Step Functions](./transforming-data.html).
The Amazon States Language provides several intrinsic functions, also known as
*intrinsics*, for use in fields that accept JSONPath. With intrinsics,
you can perform basic data processing operations without using a `Task` state.
Intrinsics look similar to functions in programming languages. They can be used to help
payload builders process the data going to and from the `Resource` field of a
`Task` state that uses the JSONPath query language.
In Amazon States Language, intrinsic functions are grouped into the following categories, based on the type
of data processing task that you want to perform:
* [Intrinsics for arrays](#asl-intrsc-func-arrays)
* [Intrinsics for data encoding and decoding](#asl-intrsc-func-data-encode-decode)
* [Intrinsic for hash calculation](#asl-intrsc-func-hash-calc)
* [Intrinsics for JSON data manipulation](#asl-intrsc-func-json-manipulate)
* [Intrinsics for Math operations](#asl-intrsc-func-math-operation)
* [Intrinsic for String operation](#asl-intrsc-func-string-operation)
* [Intrinsic for unique identifier generation](#asl-intrsc-func-uuid-generate)
* [Intrinsic for generic operation](#asl-intrsc-func-generic)
To use intrinsic functions, you must specify `.$` in the key value in your state
machine definitions, as shown in the following example:
```
``"KeyId.$": "`States.Array`($.Id)"``
```
You can nest up to 10 intrinsic functions within a field in your workflows. The following example shows a field named ``myArn`` that includes nine nested intrinsic functions:
```
`"`myArn.$`": "States.Format('{}.{}.{}', States.ArrayGetItem(States.StringSplit(States.ArrayGetItem(States.StringSplit(`$.ImageRecipe.Arn`, '/'), 2), '.'), 0), States.ArrayGetItem(States.StringSplit(States.ArrayGetItem(States.StringSplit(`$.ImageRecipe.Arn`, '/'), 2), '.'), 1))"`
```
###### QueryLanguage required for intrinsic functions
To use intrinsic functions, the state machine must use the **JSONPath
query language**.
States that use JSONata cannot use intrinsic functions; however, JSONata and Step Functions
provide equivalent options.
## Fields that support intrinsic functions
The following states support intrinsic functions in the following fields:
* **Pass state** : Parameters
* **Task state** : Parameters, ResultSelector,
Credentials
* **Parallel state**: Parameters,
ResultSelector
* **Map state**: Parameters, ResultSelector
## Intrinsics for arrays
Use the following intrinsics for performing array manipulations.
**`States.Array`**
The `States.Array` intrinsic function takes zero or more
arguments. The interpreter returns a JSON array containing the values of the
arguments in the order provided. For example, given the following
input:
```
`{
"Id": 123456
}`
```
You could use
```
`"BuildId.$": "States.Array($.Id)"`
```
Which would return the following result:``
```
`“BuildId”: [123456]`
```
**`States.ArrayPartition`**
Use the `States.ArrayPartition` intrinsic function to partition
a large array. You can also use this intrinsic to slice the data and then
send the payload in smaller chunks.
This intrinsic function takes two arguments. The first argument is an
array, while the second argument defines the chunk size. The interpreter
chunks the input array into multiple arrays of the size specified by chunk
size. The length of the last array chunk may be less than the length of the
previous array chunks if the number of remaining items in the array is
smaller than the chunk size.
###### Input validation
* You must specify an array as the input value for the function's
first argument.
* You must specify a non-zero, positive integer for the second argument representing the chunk size value.
If you specify a non-integer value for the second argument, Step Functions will round it off to the nearest integer.
* The input array can't exceed Step Functions' payload size limit of 256 KiB.
For example, given the following input array:
```
`{"inputArray": [1,2,3,4,5,6,7,8,9] }`
```
You could use the `States.ArrayPartition` function to divide
the array into chunks of four values:
```
`"inputArray.$": "States.ArrayPartition($.inputArray,4)"`
```
Which would return the following array chunks:
```
`{"inputArray": [ [1,2,3,4], [5,6,7,8], [9]] }`
```
In the previous example, the `States.ArrayPartition` function
outputs three arrays. The first two arrays each contain four values, as
defined by the chunk size. A third array contains the remaining value and is
smaller than the defined chunk
size.
**`States.ArrayContains`**
Use the `States.ArrayContains` intrinsic function to determine
if a specific value is present in an array. For example, you can use this
function to detect if there was an error in a `Map` state
iteration.
This intrinsic function takes two arguments. The first argument is an
array, while the second argument is the value to be searched for within the
array.
###### Input validation
* You must specify an array as the input value for function's first
argument.
* You must specify a valid JSON object as the second argument.
* The input array can't exceed Step Functions' payload size limit of 256 KiB.
For example, given the following input array:
```
`{
"inputArray": [1,2,3,4,5,6,7,8,9],
"lookingFor": 5
}`
```
You could use the `States.ArrayContains` function to find the
`lookingFor` value within the `inputArray`:
```
`"contains.$": "States.ArrayContains($.inputArray, $.lookingFor)"`
```
Because the value stored in `lookingFor` is included in the
`inputArray`, `States.ArrayContains` returns the
following result:
```
`{"contains": true }`
```
**`States.ArrayRange`**
Use the `States.ArrayRange` intrinsic function to create a new array
containing a specific range of elements. The new array can contain up to
1000 elements.
This function takes three arguments. The first argument is the first
element of the new array, the second argument is the final element of the
new array, and the third argument is the increment value between the
elements in the new array.
###### Input validation
* You must specify integer values for all of the arguments.
If you specify a non-integer value for any of the arguments, Step Functions will round it off to the nearest integer.
* You must specify a non-zero value for the third argument.
* The newly generated array can't contain more than 1000
items.
For example, the following use of the `States.ArrayRange`
function will create an array with a first value of 1, a final value of
9, and values in between the first and final values increase by two for each
item:
```
`"array.$": "States.ArrayRange(1, 9, 2)"`
```
Which would return the following array:
```
`{"array": [1,3,5,7,9] }`
```
**
`States.ArrayGetItem`**
This intrinsic function returns a specified index's value. This
function takes two arguments. The first argument is an array of values and
the second argument is the array index of the value to return.
For example, use the following `inputArray` and
`index` values:
```
`{
"inputArray": [1,2,3,4,5,6,7,8,9],
"index": 5
}`
```
From these values, you can use the `States.ArrayGetItem`
function to return the value in the `index` position 5 within the
array:
```
`"item.$": "States.ArrayGetItem($.inputArray, $.index)"`
```
In this example, `States.ArrayGetItem` would return the
following result:
```
`{ "item": 6 }`
```
**`States.ArrayLength`**
The `States.ArrayLength` intrinsic function returns the length
of an array. It has one argument, the array to return the length of.
For example, given the following input array:
```
`{
"inputArray": [1,2,3,4,5,6,7,8,9]
}`
```
You can use `States.ArrayLength` to return the length of
`inputArray`:
```
`"length.$": "States.ArrayLength($.inputArray)"`
```
In this example, `States.ArrayLength` would return the
following JSON object that represents the array length:
```
`{ "length": 9 }`
```
**`States.ArrayUnique`**
The `States.ArrayUnique` intrinsic function removes duplicate
values from an array and returns an array containing only unique elements.
This function takes an array, which can be unsorted, as its sole
argument.
For example, the following `inputArray` contains a series of
duplicate values:
```
`{"inputArray": [1,2,3,3,3,3,3,3,4] }`
```
You could use the `States.ArrayUnique` function as and specify
the array you want to remove duplicate values from:
```
`"array.$": "States.ArrayUnique($.inputArray)"`
```
The `States.ArrayUnique` function would return the following
array containing only unique elements, removing all duplicate values:
```
`{"array": [1,2,3,4] }`
```
## Intrinsics for data encoding and decoding
Use the following intrinsic functions to encode or decode data based on the Base64 encoding scheme.
**
`States.Base64Encode`**
Use the `States.Base64Encode` intrinsic function to encode data
based on MIME Base64 encoding scheme. You can use this function to pass data
to other AWS services without using an AWS Lambda function.
This function takes a data string of up to 10,000 characters to encode as
its only argument.
For example, consider the following `input` string:
```
`{"input": "Data to encode" }`
```
You can use the `States.Base64Encode` function to encode the
`input` string as a MIME Base64 string:
```
`"base64.$": "States.Base64Encode($.input)"`
```
The `States.Base64Encode` function returns the following
encoded data in response:
```
`{"base64": "RGF0YSB0byBlbmNvZGU=" }`
```
**
`States.Base64Decode`**
Use the `States.Base64Decode` intrinsic function to decode data
based on MIME Base64 decoding scheme. You can use this function to pass data
to other AWS services without using a Lambda function.
This function takes a Base64 encoded data string of up to 10,000
characters to decode as its only argument.
For example, given the following input:
```
`{"base64": "RGF0YSB0byBlbmNvZGU=" }`
```
You can use the `States.Base64Decode` function to decode the
base64 string to a human-readable string:
```
`"data.$": "States.Base64Decode($.base64)"`
```
The `States.Base64Decode function` would return the following
decoded data in response:
```
`{"data": "Decoded data" }`
```
## Intrinsic for hash calculation
**`States.Hash`**
Use the `States.Hash` intrinsic function to calculate the hash
value of a given input. You can use this function to pass data to other
AWS services without using a Lambda function.
This function takes two arguments. The first argument is the data you want
to calculate the hash value of. The second argument is the hashing algorithm
to use to perform the hash calculation. The data you provide must be an
object string containing 10,000 characters or less.
The hashing algorithm you specify can be any of the following
algorithms:
* `MD5`
* `SHA-1`
* `SHA-256`
* `SHA-384`
* `SHA-512`
For example, you can use this function to calculate the hash value of the
`Data` string using the specified
`Algorithm`:
```
`{
"Data": "input data",
"Algorithm": "SHA-1"
}`
```
You can use the `States.Hash` function to calculate the hash
value:
```
`"output.$": "States.Hash($.Data, $.Algorithm)"`
```
The `States.Hash` function returns the following hash value in
response:
```
`{"output": "aaff4a450a104cd177d28d18d7485e8cae074b7" }`
```
## Intrinsics for JSON data manipulation
Use these functions to perform basic data processing operations on JSON objects.
**`States.JsonMerge`**
Use the `States.JsonMerge` intrinsic function to merge two JSON objects into
a single object. This function takes three arguments. The first two
arguments are the JSON objects that you want to merge. The third argument is
a boolean value of `false`. This boolean value determines if the
deep merging mode is enabled.
Currently, Step Functions only supports the shallow merging mode; therefore, you
must specify the boolean value as `false`. In the shallow mode,
if the same key exists in both JSON objects, the latter object's key
overrides the same key in the first object. Additionally, objects nested
within a JSON object are not merged when you use shallow merging.
For example, you can use the `States.JsonMerge` function to
merge the following JSON objects that share the key `a`.
```
`{
"json1": { "a": {"a1": 1, "a2": 2}, "b": 2 },
"json2": { "a": {"a3": 1, "a4": 2}, "c": 3 }
}`
```
You can specify the json1 and json2 objects as inputs in the
`States.JsonMerge` function to merge them together:
```
`"output.$": "States.JsonMerge($.json1, $.json2, false)"`
```
The `States.JsonMerge` returns the following merged JSON object as result.
In the merged JSON object `output`, the `json2`
object's key `a` replaces the `json1`
object's key `a`. Also, the nested object in
`json1` object's key `a` is discarded because
shallow mode doesn't support merging nested objects.
```
`{
"output": {
"a": {"a3": 1, "a4": 2},
"b": 2,
"c": 3
}
}`
```
**
`States.StringToJson`
**
The `States.StringToJson` function takes a reference path to an
escaped JSON string as its only argument.
The interpreter applies a JSON parser and returns the input's parsed JSON
form. For example, you can use this function to escape the following input
string:
```
`{
"escapedJsonString": "{\\"foo\\": \\"bar\\"}"
}
`
```
Use the `States.StringToJson` function and specify the
`escapedJsonString` as the input argument:
```
`States.StringToJson($.escapedJsonString)`
```
The `States.StringToJson` function returns the following result:
```
``{ "foo": "bar" }``
```
**
`States.JsonToString`
**
The `States.JsonToString` function takes only one argument,
which is the path that contains the JSON data to return as an unescaped
string. The interpreter returns a string that contains JSON text
representing the data specified by the Path. For example, you can provide
the following JSON Path containing an escaped value:
```
`{
"unescapedJson": {
"foo": "bar"
}
}`
```
Provide the `States.JsonToString` function with the data
contained within `unescapedJson`:
```
`States.JsonToString($.unescapedJson)
`
```
The `States.JsonToString` function returns the following response:
```
``{\\"foo\\": \\"bar\\"}``
```
## Intrinsics for Math operations
Use these functions to perform Math operations.
**`States.MathRandom`**
Use the `States.MathRandom` intrinsic function to return a random number between the specified start number (inclusive) and end number (exclusive).
You can use this function to distribute a specific task between two or more resources.
This function takes three arguments. The first argument is the start
number, the second argument is the end number, and the last argument
controls the optional seed value, Note that if you use this function with
the same seed value, it will return identical numbers.
###### Important
Because the `States.MathRandom` function does not
return cryptographically secure random numbers, we recommend that you
don't use it for security
sensitive
applications.
###### Input validation
* You must specify integer values for the start number and end number arguments.
If you specify a non-integer value for the start number or end number argument, Step Functions will round it off to the nearest integer.
For example, to generate a random number between one and 999, you can
use the following input values:
```
`{
"start": 1,
"end": 999
}`
```
To generate the random number, provide the `start` and
`end` values to the `States.MathRandom` function:
```
`"random.$": "States.MathRandom($.start, $.end)"`
```
The `States.MathRandom` function returns the following random
number as a response:
```
`{"random": 456 }`
```
**`States.MathAdd`**
Use the `States.MathAdd` intrinsic function to return the sum
of two numbers. For example, you can use this function to increment values
inside a loop without invoking a Lambda function.
###### Input validation
* You must specify integer values for all the arguments.
If you specify a non-integer value for one or both the arguments, Step Functions will round it off to the nearest integer.
* You must specify integer values in the range of -2147483648 and 2147483647.
For example, you can use the following values to subtract one from
111:
```
`{
"value1": 111,
"step": -1
}`
```
Then, use the `States.MathAdd` function defining
`value1` as the starting value, and `step` as the
value to increment `value1` by:
```
`"value1.$": "States.MathAdd($.value1, $.step)"`
```
The `States.MathAdd` function would return the following number
in response:
```
`{"value1": 110 }`
```
## Intrinsic for String operation
**`States.StringSplit`**
Use the `States.StringSplit` intrinsic function to split a
string into an array of values. This function takes two arguments. The first
argument is a string and the second argument is the delimiting character
that the function will use to divide the string.
###### Example - Split an input string using a single delimiting character
For this example, use `States.StringSplit` to divide the
following `inputString`, which contains a series of comma
separated values:
```
`{
"inputString": "1,2,3,4,5",
"splitter": ","
}`
```
Use the `States.StringSplit` function and define
`inputString` as the first argument, and the delimiting
character `splitter` as the second argument:
```
`"array.$": "States.StringSplit($.inputString, $.splitter)"`
```
The `States.StringSplit` function returns the following string
array as result:
```
`{"array": ["1","2","3","4","5"] }`
```
###### Example - Split an input string using multiple delimiting characters
For this example, use `States.StringSplit` to divide the following `inputString`, which contains multiple delimiting characters:
```
`{
"inputString": "This.is+a,test=string",
"splitter": ".+,="
}`
```
Use the `States.StringSplit` function as follows:
```
`{
"myStringArray.$": "States.StringSplit($.inputString, $.splitter)"
}`
```
The `States.StringSplit` function returns the following string array as result:
```
`{"myStringArray": [
"This",
"is",
"a",
"test",
"string"
]}`
```
## Intrinsic for unique identifier generation
**
`States.UUID`**
Use the `States.UUID` intrinsic function to return a version 4
universally unique identifier (v4 UUID) generated using random numbers. For
example, you can use this function to call other AWS services or resources
that need a UUID parameter or insert items in a DynamoDB table.
The `States.UUID` function is called with no arguments
specified:
```
`"uuid.$": "States.UUID()"`
```
The function returns a randomly generated UUID, as in the following
example:
```
`{"uuid": "ca4c1140-dcc1-40cd-ad05-7b4aa23df4a8" }`
```
## Intrinsic for generic operation
**`States.Format`**
Use the `States.Format` intrinsic function to construct a
string from both literal and interpolated values. This function takes one or
more arguments. The value of the first argument must be a string, and may
include zero or more instances of the character sequence `{}`.
There must be as many remaining arguments in the intrinsic function invocation as
there are occurrences of `{}`. The interpreter returns the string
defined in the first argument with each `{}` replaced by the
value of the positionally-corresponding argument in the Intrinsic
invocation.
For
example, you can use the following inputs of an individual's
`name`, and a `template` sentence to have their
name inserted into:
```
`{
"name": "Arnav",
"template": "Hello, my name is {}."
}`
```
Use the `States.Format` function and specify the
`template` string and the string to insert in place of the
`{}` characters:
```
`States.Format('Hello, my name is {}.', $.name)`
```
or
```
`States.Format($.template, $.name)`
```
With either of the previous inputs, the `States.Format`
function returns the completed string in response:
```
`Hello, my name is Arnav.`
```
## Reserved characters in intrinsic functions
The following characters are reserved for intrinsic functions, and must be escaped
with a backslash ('\\') if you want them to appear in the Value:
**'****{****}**, and **\\**.
If the character `\\` needs to appear as part of the value without serving
as an escape character, you must escape it with a backslash. The following escaped
character sequences are used with intrinsic functions:
* The literal string `\\'` represents `'`.
* The literal string `\\{` represents `{`.
* The literal string `\\}` represents `}`.
* The literal string `\\\\` represents `\\`.
In JSON, backslashes contained in a string literal value must be escaped with another
backslash. The equivalent list for JSON is:
* The escaped string
`\\\\\\'`
represents
`\\'`.
* The escaped string
`\\\\\\{`
represents
`\\{`.
* The escaped string
`\\\\\\}`
represents
`\\}`.
* The escaped string `\\\\\\\\` represents
`\\\\`.
###### Note
If an open escape backslash `\\` is found in the intrinsic invocation
string, the interpreter will return a runtime error.
You must use square bracket notation for a **Path** passed as an argument
to an Intrinsic Function if the field name contains any character that
is not included in the `member-name-shorthand` definition of
the [JsonPath ABNF](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#jsonpath-abnf)
rule. If your **Path** contains non-alphanumeric characters,
besides `\_`, you must use square bracket notation. For
example, `$.abc.['def ghi']`.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
State machine structure
Workflow states
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.