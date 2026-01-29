---
url: https://docs.aws.amazon.com/step-functions/latest/dg/workflow-variables.html
title: Ending values: $x=6, $nextX=3`
word_count: 2003
filtered: true
elements_removed: 0
density_score: 0.83
---

Passing data between states with variables - AWS Step Functions
Passing data between states with variables - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#workflow-variables)
[Conceptual overview of variables](#conceptual-overview-of-variables)[Reserved variable : $states](#reserved-variable-states)[Variable name syntax](#variable-name-syntax)[Variable scope](#variable-scope)[Assign field in ASL](#assign-field-in-asl)[Evaluation order in an assign field](#evaluation-order-in-an-assign-field)[Limits](#limits)[Using variables in JSONPath states](#using-variables-in-jsonpath-states)
###### Managing state with variables and JSONata
Step Functions recently added variables and JSONata to manage state and transform data.
Learn more in the blog post [Simplifying developer experience with variables and JSONata in AWS Step Functions](https://aws.amazon.com/blogs/compute/simplifying-developer-experience-with-variables-and-jsonata-in-aws-step-functions/)
The following video describes variables and JSONata in Step Functions with a DynamoDB example:
With variables and state output, you can pass data between the steps of your workflow.
Using workflow variables, you can store data in a step and retrieve that data in future steps. For example, you could store an API response that contains data you might need later. Conversely, state output can only be used as input to the very next step.
## Conceptual overview of variables
With workflow variables, you can store data to reference later. For example, Step 1 might store the result from an API request so a part of that request can be re-used later in Step 5.
In the following scenario, the state machine fetches data from an API once. In Step 1, the workflow stores the returned API data (up to 256 KiB per state) in a variable ‘x’ to use in later steps.
Without variables, you would need to pass the data through output from Step 1 to Step 2 to Step 3 to Step 4 to use it in Step 5. What if those intermediate steps do not need the data? Passing data from state to state through outputs and input would be unnecessary effort.
With variables, you can store data and use it in any future step. You can also modify, rearrange, or add steps without disrupting the flow of your data. Given the flexibility of variables, you might only need to use **Output** to return data from Parallel and Map sub-workflows, and at the end of your state machine execution.
![Diagram showing step 1 assigning a value to $x, used in step 5.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/vars-diag-opt1.png)
**States that support variables **
The following state types support `Assign` to declare and assign values to variables: *Pass, Task, Map, Parallel, Choice, Wait.*
To set a variable, provide a JSON object with variable names and values:
```
`"Assign": {
"productName": "product1",
"count" : 42,
"available" : true
}
`
```
To reference a variable, prepend the name with a dollar sign (`$`), for example, `$productName`.
## Reserved variable : $states
Step Functions defines a single reserved variable called **`$states`**. In JSONata states, the following structures are assigned to `$states` for use in JSONata expressions:
```
`# Reserved $states variable in JSONata states
$states = {
"input": // Original input to the state
"result": // API or sub-workflow's result (if successful)
"errorOutput": // Error Output (only available in a Catch)
"context": // Context object
}
`
```
On state entry, Step Functions assigns the state input to **`$states.input`**. The value of `$states.input` can be used in all fields that accept JSONata expressions. `$states.input` always refers to the original state input.
For `Task`, `Parallel`, and `Map` states:
* **`$states.result`** refers to the
API or sub-workflow’s raw result if successful.
* **`$states.errorOutput`** refers to
the Error Output if the API or sub-workflow failed.
`$states.errorOutput` can be used in the `Catch` field’s
`Assign` or `Output`.
Attempting to access `$states.result` or `$states.errorOutput` in fields
and states where they are not accessible will be caught at creation, update, or
validation of the state machine.
The `$states.context` object provides your workflows information about
their specific execution, such as `StartTime`, task token, and initial
workflow input. To learn more, see [Accessing execution data from the Context object
in Step Functions ](./input-output-contextobject.html).
## Variable name syntax
Variable names follow the rules for Unicode Identifiers as described in [Unicode® Standard Annex #31](https://unicode.org/reports/tr31/). The first character of a variable name must be a Unicode ID\_Start character, and the second and subsequent characters must be Unicode ID\_Continue characters. The maximum length of a variable name is 80.
The variable name convention is similar to rules for JavaScript and other programming languages.
## Variable scope
Step Functions workflows avoid race conditions with variables by using a *workflow-local scope*.
Workflow-local scope includes all states inside a state machine's **States** field, but not states inside Parallel or Map states. States inside Parallel or Map states can refer to outer scope variables, but they create and maintain their own separate workflow-local variables and values.
`Parallel` branches and `Map` iterations can access variable values from **outer scopes**, but they do not have access to variable values from other concurrent branches or iterations. When handling errors, the `Assign` field in a `Catch` can assign values to variables in the outer scope, that is, the scope in which the Parallel/Map state exists.
Exception: **Distributed Map states** cannot currently reference variables in outer scopes.
A variable exists in a scope if any state in the scope assigns a value to it. To help avoid common errors, a variable assigned in an inner scope cannot have the same name as one assigned in an outer scope. For example, if the top-level scope assigns a value to a variable called `myVariable`, then no other scope (inside a `Map`, `Parallel`) can assign to `myVariable` as well.
Access to variables depends on the current scope. Parallel and Map states have their own scope, but can access variables in outer scopes.
When a Parallel or Map state completes, all of their variables will go out of scope and stop being accessible. Use the **Output field** to pass data out of Parallel branches and Map iterations.
## Assign field in ASL
The `Assign` field in ASL is used to assign values to one or more variables. The `Assign` field is available at the top level of each state (except `Succeed` and `Fail`), inside `Choice` state rules, and inside `Catch` fields. For example:
```
`# Example of Assign with JSONata
"Store inputs": {
"Type": "Pass",
"Next": "Get Current Price",
"Comment": "Store the input desired price into a variable: $desiredPrice",
"Assign": {
"desiredPrice": "{% $states.input.desired\_price %}",
"maximumWait": "{% $states.input.max\_days %}"
}
},
`
```
The `Assign` field takes a JSON object. Each top-level field names a variable to assign. In the previous examples, the variable names are `desiredPrice` and `maximumWait`. When using JSONata, `{% ... %}` indicates a JSONata expression which might contain variables or more complex expressions. For more information about JSONata expressions, refer to the [JSONata.org documentation](https://docs.jsonata.org/overview.html).
When using **JSONata** as the query language, the following diagram shows how **Assign** and **Output** fields are processed in parallel. Note the implication: *assigning variable values will not affect state Output. *
![Diagram showing a comparison of JSONPath and JSONata flow.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/vars-jsonata.png)
The following JSONata example retrieves `order.product` from the state input. The variable `currentPrice` is set to a value from the result of the task.
```
`# Example of Task with JSONata assignment from result
{
"Type": "Task",
...
"Assign": {
"product": "{% $states.input.order.product %}",
"currentPrice": "{% $states.result.Payload.current\_price %}"
},
"Next": "the next state"
}
`
```
Note: You **cannot** assign a value to a part of a variable. For example, you can `"Assign":{"x":42}`, but you cannot `"Assign":{"x.y":42}` or `"Assign":{"x[2]":42}`.
## Evaluation order in an assign field
All variable references in Step Functions states use the values as they were on **state entry**.
The previous fact is important to understand how the `Assign` field assigns
values to one or more variables. First, new values are calculated, then Step Functions assigns
the new values to the variables. The new variable values will be available starting with
the **next **state. For example, consider the following
`Assign` field:
```
`# Starting values: $x=3, $a=6
"Assign": {
"x": "{% $a %}",
"nextX": "{% $x %}"
}
# Ending values: $x=6, $nextX=3`
```
In the preceding example, the variable `x` is both assigned and referenced.
Remember, all expressions are ***evaluated
first***, then assignments are made. And newly assigned values will
be available in the **next** state.
Let's go through the example in detail. Assume that in a previous state,
`$x` was assigned a value of three (3) and `$a` was assigned a
value of six (6). The following steps describe the process:
1. All expressions are evaluated, using **current** values of all
variables.
The expression `"{% $a %}"` will evaluate to 6, and `"{% $x
%}"` will evaluate to 3.
2. Next, assignments are made:
`$x` will be assigned the value six (6)
`$nextX` will be assigned three (3)
Note: If `$x` had not been previously assigned, the example would **fail** because `$x` would be
*undefined*.
In summary, Step Functions evaluates **all** expressions and then makes
assignments. The order in which the variables occur in the `Assign` field
does **not** matter.
## Limits
The maximum size of a single variable is 256Kib, for both Standard and Express workflows.
The maximum combined size for all variables in a single `Assign` field is also 256Kib. For example, you could assign X and Y to 128KiB, but you could not assign both X and Y to 256KiB in the same `Assign` field.
The total size of all stored variables cannot exceed 10MiB per execution.
## Using variables in JSONPath states
Variables are also available in states that use JSONPath for their query language.
You can reference a variable in any field that accepts a JSONpath expression (
`$.` or `$$.` syntax), with the exception of
`ResultPath`, which specifies a location in state input to inject the state's
result. Variables cannot be used in `ResultPath`.
In JSONPath, the `$` symbol refers to the ‘current’ value and `$$` represents the states Context object. JSONPath expressions can start with `$.` as in `$.customer.name`. You can access context with `$$.` as in `$$.Execution.Id`.
To reference a variable, you also use the `$` symbol before a variable name, for example, `$x` or `$order.numItems`.
In** JSONPath** fields that accept intrinsic functions, variables can be used in the arguments, for example `States.Format('The order number is {}', $order.number)`.
The following digram illustrates how the assign step in a **JSONPath** task occurs in at the same time as the ResultSelector:
![Logical diagram of a state that uses JSONPath query language.](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/vars-jsonpath.png)
**Assigning variables in JSONPath**
JSONPath variable assignments behave similarly to payload templates. Fields that end with `.$` indicate the value is a JSONPath expression which Step Functions evaluates to a value during state machine execution (for example: `$.order..product` and `$.order.total`).
```
`# Example of Assign with JSONPath
{
"Type": "Task",
...
"Assign": {
"products.$": "$.order..product",
"orderTotal.$": "$.order.total"
},
"Next": "the next state"
}
`
```
For JSONPath states, the value of `$` in an `Assign` field depends on the state type. In `Task,` `Map`, `Parallel` states, the `$` refers to the API/sub-workflow result. In `Choice` and `Wait` state, `$` refers to the *effective input*, which is the value after `InputPath` has been applied to the state input. For `Pass`, `$` refers to the result, whether generated by the `Result` field or the `InputPath`/`Parameters` fields.
The following JSONPath example assigns a JSON object to the `details` variable, the result of the JSONPath expression `$.result.code` to `resultCode`, and the result of the JSONPath expression `States.Format('Hello {}', $customer.name)` to `message`. If this was in a `Task` state, then `$` in `$.order.items` and `$.result.code` refers to the API result. The `startTime` variable is assigned with a value from the Context object, `$$.Execution.StartTime`.
```
`"Assign": {
"details": {
"status": "SUCCESS",
"lineItems.$": "$.order.items"
},
"resultCode.$": "$.result.code",
"message.$": "States.Format('Hello {}', $customer.name)",
"startTime.$": "$$.Execution.StartTime"
}
`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Processing input and output
Transforming data with JSONata
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.