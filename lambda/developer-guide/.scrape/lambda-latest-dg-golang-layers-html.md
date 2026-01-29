---
url: https://docs.aws.amazon.com/lambda/latest/dg/golang-layers.html
title: Working with layers for Go Lambda functions
word_count: 246
filtered: true
elements_removed: 0
density_score: 0.84
---

Working with layers for Go Lambda functions - AWS Lambda
Working with layers for Go Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#golang-layers)
# Working with layers for Go Lambda functions
We don't recommend using [layers](./chapter-layers.html) to manage dependencies for Lambda functions written in Go.
This is because Lambda functions in Go compile into a single executable, which you provide to
Lambda when you deploy your function. This executable contains your compiled function code,
along with all of its dependencies. Using layers not only complicates this process, but
also leads to increased cold start times because your functions need to manually load extra
assemblies into memory during the init phase.
To use external dependencies with your Go handlers, include them directly in your deployment
package. By doing so, you simplify the deployment process and also take advantage of built-in
Go compiler optimizations. For an example of how to import and use a dependency like the
AWS SDK for Go in your function, see [Define Lambda function handlers in Go](./golang-handler.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy container images
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.