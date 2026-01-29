---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/simple-calc-nodejs-lambda-function.html
title: Simple calculator Lambda function
word_count: 378
filtered: true
elements_removed: 0
density_score: 0.81
---

Simple calculator Lambda function - Amazon API Gateway
Simple calculator Lambda function - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#simple-calc-nodejs-lambda-function)
[Simple calculator Lambda function input format](#simple-calc-lambda-function-input-format)[Simple calculator Lambda function output format](#simple-calc-lambda-function-output-format)[Simple calculator Lambda function implementation](#simple-calc-lambda-function-implementation)
# Simple calculator Lambda function
As an illustration, we will use a Node.js Lambda function that performs the binary
operations of addition, subtraction, multiplication and division.
###### Topics
* [Simple calculator Lambda function input format](#simple-calc-lambda-function-input-format)
* [Simple calculator Lambda function output format](#simple-calc-lambda-function-output-format)
* [Simple calculator Lambda function implementation](#simple-calc-lambda-function-implementation)
## Simple calculator Lambda function input format
This function takes an input of the following format:
```
`{ "a": "Number", "b": "Number", "op": "string"}`
```
where `op` can be any of `(+, -, \*, /, add, sub, mul, div)`.
## Simple calculator Lambda function output format
When an operation succeeds, it returns the result of the following format:
```
`{ "a": "Number", "b": "Number", "op": "string", "c": "Number"}`
```
where `c` holds the result of the calculation.
## Simple calculator Lambda function implementation
The implementation of the Lambda function is as follows:
```
`export const handler = async function (event, context) {
console.log("Received event:", JSON.stringify(event));
if (
event.a === undefined ||
event.b === undefined ||
event.op === undefined
) {
return "400 Invalid Input";
}
const res = {};
res.a = Number(event.a);
res.b = Number(event.b);
res.op = event.op;
if (isNaN(event.a) || isNaN(event.b)) {
return "400 Invalid Operand";
}
switch (event.op) {
case "+":
case "add":
res.c = res.a + res.b;
break;
case "-":
case "sub":
res.c = res.a - res.b;
break;
case "\*":
case "mul":
res.c = res.a \* res.b;
break;
case "/":
case "div":
if (res.b == 0) {
return "400 Divide by Zero";
} else {
res.c = res.a / res.b;
}
break;
default:
return "400 Invalid Operator";
}
return res;
};`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
SDK generation
Simple calculator API in API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.