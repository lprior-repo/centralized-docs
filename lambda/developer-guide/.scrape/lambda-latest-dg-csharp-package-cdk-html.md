---
url: https://docs.aws.amazon.com/lambda/latest/dg/csharp-package-cdk.html
title: Deploy C# Lambda functions using AWS CDK
word_count: 773
filtered: true
elements_removed: 0
density_score: 0.77
---

Deploy C# Lambda functions using AWS CDK - AWS Lambda
Deploy C# Lambda functions using AWS CDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#csharp-package-cdk)
[Prerequisites](#csharp-package-cdk-prereqs)[Deploy a sample AWS CDK application](#csharp-package-cdk-deploy)[Next steps](#csharp-package-cdk-next)
# Deploy C# Lambda functions using AWS CDK
The AWS Cloud Development Kit (AWS CDK) is an open-source software development framework for defining cloud infrastructure as code with modern programming
languages and frameworks like .NET. AWS CDK projects are executed to generate CloudFormation templates which are then used to deploy your code.
To build and deploy an example Hello world .NET application using the AWS CDK, follow the instructions in the following sections. The sample
application implements a basic API backend consisting of an API Gateway endpoint and a Lambda function. API Gateway invokes the Lambda function when
you send an HTTP GET request to the endpoint. The function returns a Hello world message, along with the IP address of the Lambda instance that
processes your request.
## Prerequisites
**.NET 8 SDK**
Install the [.NET 8](https://dotnet.microsoft.com/en-us/download/dotnet/8.0) SDK and
Runtime.
**AWS CDK version 2**
To learn how to install the latest version of the AWS CDK see
[Getting started with the AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html) in the
*AWS Cloud Development Kit (AWS CDK) v2 Developer Guide*.
## Deploy a sample AWS CDK application
1. Create a project directory for the sample application and navigate into it.
```
``mkdir hello-world
cd hello-world``
```
2. Initialize a new AWS CDK application by running the following command.
```
``cdk init app --language csharp``
```
The command creates the following files and directories in your project directory
```
`├── README.md
├── cdk.json
└── src
├── HelloWorld
│ ├── GlobalSuppressions.cs
│ ├── HelloWorld.csproj
│ ├── HelloWorldStack.cs
│ └── Program.cs
└── HelloWorld.sln`
```
3. Open the `src` directory and create a new Lambda function using the .NET CLI. This is the function you will deploy
using the AWS CDK. In this example, you create a Hello world function named `HelloWorldLambda`using the
`lambda.EmptyFunction` template.
```
``cd src
dotnet new lambda.EmptyFunction -n HelloWorldLambda``
```
After this step, your directory structure inside your project directory should look like the following.
```
`├── README.md
├── cdk.json
└── src
├── HelloWorld
│ ├── GlobalSuppressions.cs
│ ├── HelloWorld.csproj
│ ├── HelloWorldStack.cs
│ └── Program.cs
├── HelloWorld.sln
└── HelloWorldLambda
├── src
│ └── HelloWorldLambda
│ ├── Function.cs
│ ├── HelloWorldLambda.csproj
│ ├── Readme.md
│ └── aws-lambda-tools-defaults.json
└── test
└── HelloWorldLambda.Tests
├── FunctionTest.cs
└── HelloWorldLambda.Tests.csproj`
```
4. Open the `HelloWorldStack.cs` file from the `src/HelloWorld` directory. Replace the contents of the file with
the following code.
```
`using Amazon.CDK;
using Amazon.CDK.AWS.Lambda;
using Amazon.CDK.AWS.Logs;
using Constructs;
namespace CdkTest
{
public class HelloWorldStack : Stack
{
internal HelloWorldStack(Construct scope, string id, IStackProps props = null) : base(scope, id, props)
{
var buildOption = new BundlingOptions()
{
Image = Runtime.DOTNET\_8.BundlingImage,
User = "root",
OutputType = BundlingOutput.ARCHIVED,
Command = new string[]{
"/bin/sh",
"-c",
" dotnet tool install -g Amazon.Lambda.Tools"+
" &amp;&amp; dotnet build"+
" &amp;&amp; dotnet lambda package --output-package /asset-output/function.zip"
}
};
var helloWorldLambdaFunction = new Function(this, "HelloWorldFunction", new FunctionProps
{
Runtime = Runtime.DOTNET\_8,
MemorySize = 1024,
LogRetention = RetentionDays.ONE\_DAY,
Handler = "HelloWorldLambda::HelloWorldLambda.Function::FunctionHandler",
Code = Code.FromAsset("./src/HelloWorldLambda/src/HelloWorldLambda", new Amazon.CDK.AWS.S3.Assets.AssetOptions
{
Bundling = buildOption
}),
});
}
}
}`
```
This is the code to compile and bundle the application code, as well as the definition of the Lambda function itself. the
`BundlingOptions` object allows a zip file to be created, along with a set of commands that are used to generate the
contents of the zip file. In this instance, the `dotnet lambda package` command is used to compile and generate the zip
file.
5. To deploy your application, run the following command.
```
``cdk deploy``
```
6. Invoke your deployed Lambda function using the .NET Lambda CLI.
```
``dotnet lambda invoke-function HelloWorldFunction -p "hello world"``
```
7. After you've finished testing, you can delete the resources you created, unless you want to retain them. Run the following
command to delete your resources.
```
``cdk destroy``
```
## Next steps
To learn more about using AWS CDK to build and deploy Lambda functions using .NET, see the following resources:
* [Working with the AWS CDK in C#](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-csharp.html)
* [Build, package, and
publish .NET C# Lambda functions with the AWS CDK](https://aws.amazon.com/blogs/modernizing-with-aws/build-package-publish-dotnet-csharp-lambda-functions-aws-cdk/)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
AWS SAM
ASP.NET
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.