---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-provided.html
title: When to use Lambda's OS-only runtimes
word_count: 482
filtered: true
elements_removed: 0
density_score: 0.88
---

When to use Lambda's OS-only runtimes - AWS Lambda
When to use Lambda's OS-only runtimes - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtimes-provided)
# When to use Lambda's OS-only runtimes
Lambda provides [managed runtimes](./lambda-runtimes.html) for Java, Python, Node.js, .NET, and Ruby. To create Lambda functions in a programming language that is not available as a managed runtime, use an OS-only runtime (the `provided` runtime family). There are three primary use cases for OS-only runtimes:
* **Native ahead-of-time (AOT) compilation**: Languages such as Go, Rust, Swift, and C++ compile natively to an executable binary, which doesn't require a dedicated language runtime. These languages only need an OS environment in which the compiled binary can run. You can also use Lambda OS-only runtimes to deploy binaries compiled with .NET Native AOT and Java GraalVM Native Image.
You must include a runtime interface client in your binary. The runtime interface client calls the [Using the Lambda runtime API for custom runtimes](./runtimes-api.html) to retrieve function invocations and then calls your function handler. Lambda provides runtime interface clients for [Rust](./lambda-rust.html), [Go](./golang-package.html#golang-package-mac-linux), [.NET Native AOT](./dotnet-native-aot.html), [Swift](https://github.com/awslabs/swift-aws-lambda-runtime) (experimental), and [C++](https://github.com/awslabs/aws-lambda-cpp) (experimental).
You must compile your binary for a Linux environment and for the same instruction set architecture that you plan to use for the function (x86\_64 or arm64).
* **Third-party runtimes**: You can run Lambda functions using off-the-shelf runtimes such as [Bref](https://bref.sh/docs/news/01-bref-1.0.html#amazon-linux-2) for PHP.
* **Custom runtimes**: You can build your own runtime for a language or language version that Lambda doesn't provide a managed runtime for, such as Node.js 19. For more information, see [Building a custom runtime for AWS Lambda](./runtimes-custom.html). This is the least common use case for OS-only runtimes.
Lambda supports the following OS-only runtimes:
|Name|Identifier|Operating system|Deprecation date|Block function create|Block function update|
|
OS-only Runtime
|
`provided.al2023`
|
Amazon Linux 2023
|
Jun 30, 2029
|
Jul 31, 2029
|
Aug 31, 2029
|
|
OS-only Runtime
|
`provided.al2`
|
Amazon Linux 2
|
Jul 31, 2026
|
Aug 31, 2026
|
Sep 30, 2026
|
The Amazon Linux 2023 (`provided.al2023`) runtime provides several advantages over Amazon Linux 2, including a smaller deployment footprint and updated versions of libraries such as `glibc`.
The `provided.al2023` runtime uses `dnf` as the package manager instead of `yum`, which is the default package manager in Amazon Linux 2. For more information about the differences between `provided.al2023` and `provided.al2`, see [Introducing the Amazon Linux 2023 runtime for AWS Lambda](https://aws.amazon.com/blogs/compute/introducing-the-amazon-linux-2023-runtime-for-aws-lambda/) on the AWS Compute Blog.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Runtime API
Building a custom runtime
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.