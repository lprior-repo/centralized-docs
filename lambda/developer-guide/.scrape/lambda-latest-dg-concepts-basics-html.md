---
url: https://docs.aws.amazon.com/lambda/latest/dg/concepts-basics.html
title: How Lambda works
word_count: 1600
filtered: true
elements_removed: 0
density_score: 0.85
---

How Lambda works - AWS Lambda
How Lambda works - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#concepts-basics)
[Lambda functions and function handlers](#gettingstarted-concepts-function)[Lambda execution environment and runtimes](#gettingstarted-concepts-runtime)[Events and triggers](#gettingstarted-concepts-event)[Lambda permissions and roles](#gettingstarted-concepts-permissions)
# How Lambda works
Lambda functions are the basic building blocks you use to build Lambda applications. To write functions, it's essential to understand the core concepts and components that make up the Lambda programming model. This section will guide you through the fundamental elements you need to know to start building serverless applications with Lambda.
* **[Lambda functions and function handlers](#gettingstarted-concepts-function)** - A Lambda function is a small block of code that runs in response to events. Functions can be standard (up to 15 minutes) or [durable](./durable-functions.html) (up to one year). Functions are the basic building blocks you use to build applications. Function handlers are the entry point for event objects that your Lambda function code processes.
* **[Lambda execution environment and runtimes](#gettingstarted-concepts-runtime)** - Lambda execution environments manage the resources required to run your function. For [durable functions](./durable-functions.html), the execution environment includes automatic state management and checkpointing capabilities. Runtimes are the language-specific environments your functions run in.
* **[Events and triggers](#gettingstarted-concepts-event)** - Other AWS services can invoke your functions in response to specific events. For durable functions, events can also trigger resumption of paused workflows.
* **[Lambda permissions and roles](#gettingstarted-concepts-permissions)** - Control who can access your functions and what other AWS services your functions can interact with. durable functions require additional permissions for state management and extended execution.
###### Tip
If you want to start by understanding serverless development more generally, see [Understanding the difference between traditional and serverless development](https://docs.aws.amazon.com/serverless/latest/devguide/serverless-shift-mindset.html)
in the *AWS Serverless Developer Guide*.
## Lambda functions and function handlers
In Lambda, **functions** are the fundamental building blocks you use to create applications. A Lambda function is a piece
of code that runs in response to events, such as a user clicking a button on a website or a file being uploaded to an Amazon Simple Storage Service (Amazon S3) bucket. With durable functions, your code can pause execution between steps, maintaining state automatically, making them ideal for long-running workflows like order processing or content moderation
You can think of a function as a kind of self-contained program with the following properties.
A Lambda **function handler** is the method in your function code that processes events.
When a function runs in response to an event, Lambda runs the function handler. Data about the event that caused the function to run is
passed directly to the handler. While the code in a Lambda function can contain more than one method or function, Lambda functions can only have one handler.
To create a Lambda function, you bundle your function code and its dependencies in a deployment package. Lambda supports two types of deployment package,
[.zip file archives](./configuration-function-zip.html) and [container images](./images-create.html).
* A function has one specific job or purpose
* They run only when needed in response to specific events
* They automatically stop running when finished
## Lambda execution environment and runtimes
Lambda functions run inside a secure, isolated *[execution environment](./lambda-runtime-environment.html)* which Lambda manages for you. For [durable functions](./durable-functions.html), the execution environment includes additional components for state management and workflow coordination. The execution environment manages the processes and resources that are needed to run your function. When a function is first invoked, Lambda
creates a new execution environment for the function to run in. After the function has finished running, Lambda doesn't stop the execution environment right away;
if the function is invoked again, Lambda can re-use the existing execution environment.
The Lambda execution environment also contains a *runtime*, a language-specific environment that relays event information and responses between Lambda and your function.
Lambda provides a number of [managed runtimes](./lambda-runtimes.html#runtimes-supported) for the most popular programming languages, or you can create your own.
For managed runtimes, Lambda automatically applies security updates and patches to functions using the runtime.
## Events and triggers
You can also invoke a Lambda function directly by using the Lambda console, [AWS CLI](https://aws.amazon.com/cli/), or one of the [AWS Software Development Kits (SDKs)](https://aws.amazon.com/developer/tools/).
It's more usual in a production application for your function to be invoked by another AWS service in response to a
particular event. For example, you might want a function to run whenever an item is added to an Amazon DynamoDB table.
To make your function respond to events, you set up a **trigger**. A trigger connects your function
to an event source, and your function can have multiple triggers. When an event occurs, Lambda receives event data as
a JSON document and converts it into an object that your code can process. You might define the following JSON format for your event and the Lambda runtime converts this
JSON to an object before passing it to your function's handler.
###### Example custom Lambda event
```
`{
"Location": "SEA",
"WeatherData":{
"TemperaturesF":{
"MinTempF": 22,
"MaxTempF": 78
},
"PressuresHPa":{
"MinPressureHPa": 1015,
"MaxPressureHPa": 1027
}
}
}`
```
Stream and queue services like Amazon Kinesis or Amazon SQS, Lambda use an [event source mapping](./invocation-eventsourcemapping.html) instead
of a standard trigger. Event source mappings poll the source for new data, batch records together, and then invoke your function with the batched events. For more information, see [How event source mappings differ from direct triggers](./invocation-eventsourcemapping.html#eventsourcemapping-trigger-difference).
To understand how a trigger works, start by completing the [Use an Amazon S3 trigger](./with-s3-example.html) tutorial,
or for a general overview of using triggers and instructions on creating a trigger using the Lambda console, see [Integrating other services](./lambda-services.html).
## Lambda permissions and roles
For Lambda, there are two main types of [permissions](./permissions-granting-access.html) that you need to configure:
* Permissions that your function needs to access other AWS services
* Permissions that other users and AWS services need to access your function
The following sections describe both of these permission types and discuss best practices for applying least-privilege permissions.
### Permissions for functions to access other AWS resources
Lambda functions often need to access other AWS resources and perform actions on them. For example, a function might read items from a DynamoDB table, store an
object in an S3 bucket, or write to an Amazon SQS queue. To give functions the permissions they need to perform these actions, you use an *[execution role](./lambda-intro-execution-role.html)*.
A Lambda execution role is a special kind of AWS Identity and Access Management (IAM) [role](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html), an identity you create in your account
that has specific permissions associated with it defined in a *policy*.
Every Lambda function must have an execution role, and a single role can be used by more than one function. When a function is invoked, Lambda assumes the
function's execution role and is granted permission to take the actions defined in the role's policy.
When you create a function in the Lambda console, Lambda automatically creates an execution role for your function. The role's policy gives your function basic permissions to
write log outputs to Amazon CloudWatch Logs. To give your function permission to perform actions on other AWS resources, you need to edit the role to add the extra permissions. The easiest way to add
permissions is to use an AWS [managed policy](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html#aws-managed-policies). Managed policies
are created and administered by AWS and provide permissions for many common use cases. For example, if your function performs CRUD operations on a DynamoDB table, you can add the
[AmazonDynamoDBFullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AmazonDynamoDBFullAccess.html) policy to your role.
### Permissions for other users and resources to access your function
To grant other AWS service permission to access your Lambda function, you use a *[resource-based policy](./access-control-resource-based.html)*.
In IAM, resource-based policies are attached to a
resource (in this case, your Lambda function) and define who can access the resource and what actions they are allowed to take.
For another AWS service to invoke your function through a trigger, your function's resource-based policy must grant that service permission to use the `lambda:InvokeFunction` action.
If you create the trigger using the console, Lambda automatically adds this permission for you.
To grant permission to other AWS users to access your function, you can define this in your function's resource-based policy in exactly the same way as for another AWS service or resource.
You can also use an *[identity-based policy](./access-control-identity-based.html)* that's associated with the user.
### Best practices for Lambda permissions
When you set permissions using IAM policies, [security best practice](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html) is to grant only the permissions required to perform a task. This is known as the principle of *least privilege*.
To get started granting permissions for your function, you might choose to use an AWS managed policy. Managed policies can be the quickest and easiest way to grant permissions to perform a task, but
they might also include other permissions you don't need. As you move from early development through test and production, we recommend you reduce permissions to only those needed by defining your own
[customer-managed policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_managed-vs-inline.html#customer-managed-policies).
The same principle applies when granting permissions to access your function using a resource-based policy. For example, if you want to give permission to Amazon S3 to invoke your function,
best practice is to limit access to individual buckets, or buckets in particular AWS accounts, rather than giving blanket permissions to the S3 service.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
What is AWS Lambda?
Running code
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.