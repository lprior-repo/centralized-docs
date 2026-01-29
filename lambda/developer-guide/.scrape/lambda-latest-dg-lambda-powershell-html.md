---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-powershell.html
title: Building Lambda functions with PowerShell
word_count: 335
filtered: true
elements_removed: 0
density_score: 0.85
---

Building Lambda functions with PowerShell - AWS Lambda
Building Lambda functions with PowerShell - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-powershell)
# Building Lambda functions with PowerShell
The following sections explain how common programming patterns and core concepts apply when you author Lambda
function code in PowerShell.
Lambda provides the following sample applications for PowerShell:
* [blank-powershell](https://github.com/awsdocs/aws-lambda-developer-guide/tree/main/sample-apps/blank-powershell) – A PowerShell function that shows the use of logging, environment variables, and the AWS SDK.
Before you get started, you must first set up a PowerShell development environment. For instructions on how to
do this, see [Setting Up a PowerShell Development Environment](./powershell-devenv.html).
To learn about how to use the AWSLambdaPSCore module to download sample PowerShell projects from templates,
create PowerShell deployment packages, and deploy PowerShell functions to the AWS Cloud, see [Deploy PowerShell Lambda functions with .zip file archives](./powershell-package.html).
Lambda provides the following runtimes for .NET languages:
|Name|Identifier|Operating system|Deprecation date|Block function create|Block function update|
|
.NET 10
|
`dotnet10`
|
Amazon Linux 2023
|
Nov 14, 2028
|
Dec 14, 2028
|
Jan 15, 2029
|
|
.NET 9 (container only)
|
`dotnet9`
|
Amazon Linux 2023
|
Nov 10, 2026
|
Not scheduled
|
Not scheduled
|
|
.NET 8
|
`dotnet8`
|
Amazon Linux 2023
|
Nov 10, 2026
|
Dec 10, 2026
|
Jan 11, 2027
|
###### Topics
* [Setting Up a PowerShell Development Environment](./powershell-devenv.html)
* [Deploy PowerShell Lambda functions with .zip file archives](./powershell-package.html)
* [Define Lambda function handler in PowerShell](./powershell-handler.html)
* [Using the Lambda context object to retrieve PowerShell function information](./powershell-context.html)
* [Log and monitor Powershell Lambda functions](./powershell-logging.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Testing
Development Environment
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.