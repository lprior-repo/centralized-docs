---
url: https://docs.aws.amazon.com/lambda/latest/dg/rust-layers.html
title: Working with layers for Rust Lambda functions
word_count: 247
filtered: true
elements_removed: 0
density_score: 0.84
---

Working with layers for Rust Lambda functions - AWS Lambda
Working with layers for Rust Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#rust-layers)
# Working with layers for Rust Lambda functions
We don't recommend using [layers](./chapter-layers.html) to manage dependencies for Lambda functions written in Rust.
This is because Lambda functions in Rust compile into a single executable, which you provide to
Lambda when you deploy your function. This executable contains your compiled function code,
along with all of its dependencies. Using layers not only complicates this process, but
also leads to increased cold start times because your functions need to manually load extra
assemblies into memory during the init phase.
To use external dependencies with your Rust handlers, include them directly in your deployment
package. By doing so, you simplify the deployment process and also take advantage of built-in
Rust compiler optimizations. For an example of how to import and use a dependency like the
AWS SDK for Rust in your function, see [Define Lambda function handlers in Rust](./rust-handler.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Deploy .zip file archives
Logging
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.