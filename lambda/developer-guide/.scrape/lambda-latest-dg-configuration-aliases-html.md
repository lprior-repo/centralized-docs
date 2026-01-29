---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html
title: Create an alias for a Lambda function
word_count: 368
filtered: true
elements_removed: 0
density_score: 0.91
---

Create an alias for a Lambda function - AWS Lambda
Create an alias for a Lambda function - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-aliases)
# Create an alias for a Lambda function
You can create aliases for your Lambda function. A Lambda alias is a pointer to a
function version that you can update. The function's users can access the function version
using the alias Amazon Resource Name (ARN). When you deploy a new version, you can update the
alias to use the new version, or split traffic between two versions.
Console
###### To create an alias using the console
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose **Aliases** and then choose **Create alias**.
4. On the **Create alias** page, do the following:
1. Enter a **Name** for the alias.
2. (Optional) Enter a **Description** for the alias.
3. For **Version**, choose a function version that you want the alias to point to.
4. (Optional) To configure routing on the alias, expand **Weighted alias**. For more
information, see [Implement Lambda canary deployments using a weighted alias](./configuring-alias-routing.html).
5. Choose **Save**.
AWS CLI
To create an alias using the AWS Command Line Interface (AWS CLI), use the [create-alias](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-alias.html) command.
```
``aws lambda create-alias \\
--function-name `my-function` \\
--name `alias-name` \\
--function-version `version-number` \\
--description " "``
```
To change an alias to point a new version of the function, use the [update-alias](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-alias.html) command.
```
``aws lambda update-alias \\
--function-name `my-function` \\
--name `alias-name` \\
--function-version `version-number```
```
To delete an alias, use the [delete-alias](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/delete-alias.html) command.
```
``aws lambda delete-alias \\
--function-name `my-function` \\
--name `alias-name```
```
The AWS CLI commands in the preceding steps correspond to the following Lambda API operations:
* [CreateAlias](https://docs.aws.amazon.com/lambda/latest/api/API_CreateAlias.html)
* [UpdateAlias](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateAlias.html)
* [DeleteAlias](https://docs.aws.amazon.com/lambda/latest/api/API_DeleteAlias.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
File system
Using aliases
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.