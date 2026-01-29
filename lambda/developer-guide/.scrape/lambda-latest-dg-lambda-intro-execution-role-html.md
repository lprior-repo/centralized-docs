---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-intro-execution-role.html
title: Defining Lambda function permissions with an execution role
word_count: 1031
filtered: true
elements_removed: 0
density_score: 0.86
---

Defining Lambda function permissions with an execution role - AWS Lambda
Defining Lambda function permissions with an execution role - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-intro-execution-role)
[Creating an execution role in the IAM console](#permissions-executionrole-console)[Creating and managing roles with the AWS CLI](#permissions-executionrole-api)[Grant least privilege access to your Lambda execution role](#permissions-executionrole-least-privilege)
# Defining Lambda function permissions with an execution role
A Lambda function's execution role is an AWS Identity and Access Management (IAM) role that grants the function permission to access
AWS services and resources. For example, you might create an execution role that has permission to send logs to
Amazon CloudWatch and upload trace data to AWS X-Ray. This page provides information on how to create, view, and manage a
Lambda function's execution role.
Lambda automatically assumes your execution role when you invoke your function. You should avoid manually
calling `sts:AssumeRole` to assume the execution role in your function code. If your use case requires
that the role assumes itself, you must include the role itself as a trusted principal in your role's trust policy.
For more information on how to modify a role trust policy, see [
Modifying a role trust policy (console)](https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-managingrole-editing-console.html#roles-managingrole_edit-trust-policy) in the IAM User Guide.
In order for Lambda to properly assume your execution role, the role's
[trust policy](#permissions-executionrole-api) must specify the Lambda service principal
(`lambda.amazonaws.com`) as a trusted service.
###### Topics
* [Creating an execution role in the IAM console](#permissions-executionrole-console)
* [Creating and managing roles with the AWS CLI](#permissions-executionrole-api)
* [Grant least privilege access to your Lambda execution role](#permissions-executionrole-least-privilege)
* [Viewing and updating permissions in the execution role](./permissions-executionrole-update.html)
* [Working with AWS managed policies in the execution role](./permissions-managed-policies.html)
* [Using source function ARN to control function access behavior](./permissions-source-function-arn.html)
## Creating an execution role in the IAM console
By default, Lambda creates an execution role with minimal permissions when you [create a function in the Lambda console](./getting-started.html#getting-started-create-function). Specifically,
this execution role includes the [`AWSLambdaBasicExecutionRole` managed policy](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html), which gives your function
basic permissions to log events to Amazon CloudWatch Logs.
Your functions typically need additional permissions to perform more meaningful
tasks. For example, you might have a Lambda function that responds to an event by updating
entries in an Amazon DynamoDB database. You can create an execution role with the necessary
permissions using the IAM console.
###### To create an execution role in the IAM console
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) in the IAM console.
2. Choose **Create role**.
3. Under **Trusted entity type**, choose **AWS service**.
4. Under **Use case**, choose **Lambda**.
5. Choose **Next**.
6. Select the AWS managed policies that you want to attach to your role.
For example, if your function needs to access DynamoDB, select the
**AWSLambdaDynamoDBExecutionRole** managed policy.
7. Choose **Next**.
8. Enter a **Role name** and then choose **Create role**.
For detailed instructions, see [Creating a role
for an AWS service (console)](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-service.html#roles-creatingrole-service-console) in the *IAM User Guide*.
After you create your execution role, attach it to your function. When you
[create a function in the Lambda console](./getting-started.html#getting-started-create-function),
you can attach any execution role that you previously created to the function. If you want
to attach a new execution role to an existing function, follow the steps in [Updating a function's execution role](./permissions-executionrole-update.html#update-execution-role).
## Creating and managing roles with the AWS CLI
To create an execution role with the AWS Command Line Interface (AWS CLI), use the **create-role**
command. When using this command, you can specify the trust policy inline.
A role's trust policy gives the specified principals permission to assume the role. In the
following example, you grant the Lambda service principal permission to assume your role. Note that
requirements for escaping quotes in the JSON string may vary depending on your shell.
```
``aws iam create-role \\
--role-name lambda-ex \\
--assume-role-policy-document '{"Version": "2012-10-17", "Statement": [{ "Effect": "Allow", "Principal": {"Service": "lambda.amazonaws.com"}, "Action": "sts:AssumeRole"}]}'``
```
You can also define the trust policy for the role using a separate JSON file. In the following example,
`trust-policy.json` is a file in the current directory.
###### Example trust-policy.json
JSON
****
```
``{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {
"Service": "lambda.amazonaws.com"
},
"Action": "sts:AssumeRole"
}
]
}`
`
```
```
``aws iam create-role \\
--role-name lambda-ex \\
--assume-role-policy-document file://trust-policy.json``
```
To add permissions to the role, use the **attach-policy-to-role** command. The following
command adds the `AWSLambdaBasicExecutionRole` managed policy to the `lambda-ex`
execution role.
```
``aws iam attach-role-policy --role-name lambda-ex --policy-arn arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole``
```
After you create your execution role, attach it to your function. When you
[create a function in the Lambda console](./getting-started.html#getting-started-create-function),
you can attach any execution role that you previously created to the function. If you want
to attach a new execution role to an existing function, follow the steps in [Updating a function's execution role](./permissions-executionrole-update.html#update-execution-role).
## Grant least privilege access to your Lambda execution role
When you first create an IAM role for your Lambda function during the development phase, you might sometimes
grant permissions beyond what is required. Before publishing your function in the production environment, as a
best practice, adjust the policy to include only the required permissions. For more information, see [Apply least-privilege
permissions](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#grant-least-privilege) in the *IAM User Guide*.
Use IAM Access Analyzer to help identify the required permissions for the IAM execution role policy. IAM Access Analyzer
reviews your AWS CloudTrail logs over the date range that you specify and generates a policy template with only the
permissions that the function used during that time. You can use the template to create a managed policy with
fine-grained permissions, and then attach it to the IAM role. That way, you grant only the permissions that the
role needs to interact with AWS resources for your specific use case.
For more information, see [Generate policies based on access
activity](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_generate-policy.html) in the *IAM User Guide*.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda permissions
Update execution role
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.