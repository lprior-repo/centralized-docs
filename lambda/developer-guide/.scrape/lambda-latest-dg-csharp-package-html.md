---
url: https://docs.aws.amazon.com/lambda/latest/dg/csharp-package.html
title: Build and deploy C# Lambda functions with .zip file archives
word_count: 460
filtered: true
elements_removed: 0
density_score: 0.86
---

Build and deploy C# Lambda functions with .zip file archives - AWS Lambda
Build and deploy C# Lambda functions with .zip file archives - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#csharp-package)
# Build and deploy C# Lambda functions with .zip file archives
A .NET deployment package (.zip file archive) contains your function's compiled assembly along with all of
its assembly dependencies. The package also contains a ``proj`.deps.json` file.
This signals to the .NET runtime all of your function's dependencies and a
``proj`.runtimeconfig.json` file, which is used to configure the runtime.
To deploy individual Lambda functions, you can use the `Amazon.Lambda.Tools` .NET Lambda Global CLI. Using the
`dotnet lambda deploy-function` command automatically creates a .zip deployment package and deploys it to Lambda. However, we
recommend that you use frameworks like the AWS Serverless Application Model (AWS SAM) or the AWS Cloud Development Kit (AWS CDK) to deploy your .NET applications to AWS.
Serverless applications usually comprise a combination of Lambda functions and other managed AWS services working together to
perform a particular business task. AWS SAM and AWS CDK simplify building and deploying Lambda functions with other AWS services at scale. The
[AWS SAM template specification](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-specification.html)
provides a simple and clean syntax to describe Lambda functions, APIs, permissions, configurations, and other AWS resources that make up your
serverless application. With the [AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/home.html) you define cloud infrastructure as code
to help you build reliable, scalable, cost-effective applications in the cloud using modern programming languages and frameworks like .NET. Both
the AWS CDK and the AWS SAM use the .NET Lambda Global CLI to package your functions.
While it's possible to use [Lambda layers](./chapter-layers.html)
with functions in C# by [using the .NET Core CLI](./csharp-package-cli.html#csharp-layers), we
recommend against it. Functions in C# that use layers manually load the shared
assemblies into memory during the [Init phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib),
which can increase cold start times. Instead, include all shared code at compile
time to avoid the performance impact of loading assemblies at runtime.
You can find instructions for building and deploying .NET Lambda functions using the AWS SAM, the AWS CDK, and the .NET Lambda Global CLI in
the following sections.
###### Topics
* [Using the .NET Lambda Global CLI](./csharp-package-cli.html)
* [Deploy C# Lambda functions using AWS SAM](./csharp-package-sam.html)
* [Deploy C# Lambda functions using AWS CDK](./csharp-package-cdk.html)
* [Deploy ASP.NET applications](./csharp-package-asp.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Handler
NET Lambda Global CLI
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.