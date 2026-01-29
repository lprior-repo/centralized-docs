---
url: https://docs.aws.amazon.com/lambda/latest/dg/dotnet-layers.html
title: Working with layers for .NET Lambda functions
word_count: 230
filtered: true
elements_removed: 0
density_score: 0.90
---

Working with layers for .NET Lambda functions - AWS Lambda
Working with layers for .NET Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#dotnet-layers)
# Working with layers for .NET Lambda functions
We don't recommend using [layers](./chapter-layers.html) to manage dependencies for Lambda functions written in .NET. This is because .NET is a compiled language, and your functions still have to manually load any shared assemblies into memory during the [Init](./lambda-runtime-environment.html#runtimes-lifecycle-ib) phase, which can increase cold start times. Using layers not only complicates the deployment process, but also prevents you from taking advantage of built-in compiler optimizations.
To use external dependencies with your .NET handlers, include them directly in your deployment package at compile time. By doing so, you simplify the deployment process and also take advantage of built-in .NET compiler optimizations. For an example of how to import and use dependencies like NuGet packages in your function, see [Define Lambda function handler in C#](./csharp-handler.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ASP.NET
Deploy container images
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.