---
url: https://docs.aws.amazon.com/lambda/latest/dg/permissions-executionrole-update.html
title: Viewing and updating permissions in the execution role
word_count: 502
filtered: true
elements_removed: 0
density_score: 0.83
---

Viewing and updating permissions in the execution role - AWS Lambda
Viewing and updating permissions in the execution role - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#permissions-executionrole-update)
[Viewing a function's execution role](#view-execution-role)[Updating a function's execution role](#update-execution-role)
# Viewing and updating permissions in the execution role
This topic covers how you can view and update your function's
[execution role](./lambda-intro-execution-role.html).
###### Topics
* [Viewing a function's execution role](#view-execution-role)
* [Updating a function's execution role](#update-execution-role)
## Viewing a function's execution role
To view a function's execution role, use the Lambda console.
###### To view a function's execution role (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the name of a function.
3. Choose **Configuration**, and then choose **Permissions**.
4. Under **Execution role**, you can view the role that's currently being used
as the function's execution role. For convenience, you can view all the resources and actions
that the function can access under the **Resource summary** section. You can
also choose a service from the dropdown list to see all permissions related to that service.
## Updating a function's execution role
You can add or remove permissions from a function's execution role at any time, or
configure your function to use a different role. If your function needs access to
any other services or resources, you must add the necessary permissions to the
execution role.
When you add permissions to your function, perform a trivial update to its code or
configuration as well. This forces running instances of your function, which have outdated credentials,
to stop and be replaced.
To update a function's execution role, you can use the Lambda console.
###### To update a function's execution role (console)
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose the name of a function.
3. Choose **Configuration**, and then choose **Permissions**.
4. Under **Execution role**, choose **Edit**.
5. If you want to update your function to use a different role as the execution role,
choose the new role in the dropdown menu under **Existing role**.
###### Note
If you want to update the permissions within an existing execution role, you can
only do so in the AWS Identity and Access Management (IAM) console.
If you want to create a new role to use as the execution role, choose
**Create a new role from AWS policy templates** under
**Execution role**. Then, enter a name for your new role under
**Role name**, and specify any policies you want to attach to the
new role under **Policy templates**.
6. Choose **Save**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Execution role (permissions for functions to access other resources)
AWS managed policies
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.