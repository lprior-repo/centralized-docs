---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-csharp.html
title: Building Lambda functions with C#
word_count: 378
filtered: true
elements_removed: 0
density_score: 0.84
---

Building Lambda functions with C# - AWS Lambda
Building Lambda functions with C# - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-csharp)
[Development environment](#csharp-dev-env)
# Building Lambda functions with C#
You can run your .NET application in Lambda using the managed .NET 8 runtime, a custom runtime, or a container image. After your application
code is compiled, you can deploy it to Lambda either as a .zip file or a container image. Lambda provides the following runtimes for .NET languages:
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
## Setting up your .NET development environment
To develop and build your Lambda functions, you can use any of the commonly available .NET integrated development environments (IDEs),
including Microsoft Visual Studio, Visual Studio Code, and JetBrains Rider. To simplify your development experience, AWS provides a set of
.NET project templates, as well as the `Amazon.Lambda.Tools` command line interface (CLI).
Run the following .NET CLI commands to install these project templates and command line tools.
### Installing the .NET project templates
To install the project templates, run the following command:
```
``dotnet new install Amazon.Lambda.Templates``
```
### Installing and updating the CLI tools
Run the following commands to install, update, and uninstall the `Amazon.Lambda.Tools` CLI.
To install the command line tools:
```
``dotnet tool install -g Amazon.Lambda.Tools``
```
To update the command line tools:
```
``dotnet tool update -g Amazon.Lambda.Tools``
```
To uninstall the command line tools:
```
``dotnet tool uninstall -g Amazon.Lambda.Tools``
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tracing
Handler
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.