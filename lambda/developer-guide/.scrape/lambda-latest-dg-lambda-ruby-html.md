---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-ruby.html
title: Building Lambda functions with Ruby
word_count: 778
filtered: true
elements_removed: 0
density_score: 0.86
---

Building Lambda functions with Ruby - AWS Lambda
Building Lambda functions with Ruby - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-ruby)
[Runtime-included SDK versions](#ruby-sdk-included)[Enabling Yet Another Ruby JIT (YJIT)](#ruby-yjit)
# Building Lambda functions with Ruby
You can run Ruby code in AWS Lambda. Lambda provides [runtimes](./lambda-runtimes.html) for Ruby that run your code to process events. Your code runs
in an environment that includes the AWS SDK for Ruby, with credentials from an AWS Identity and Access Management (IAM) role that you manage. To learn more
about the SDK versions included with the Ruby runtimes, see [Runtime-included SDK versions](#ruby-sdk-included).
Lambda supports the following Ruby runtimes.
|Name|Identifier|Operating system|Deprecation date|Block function create|Block function update|
|
Ruby 3.4
|
`ruby3.4`
|
Amazon Linux 2023
|
Mar 31, 2028
|
Apr 30, 2028
|
May 31, 2028
|
|
Ruby 3.3
|
`ruby3.3`
|
Amazon Linux 2023
|
Mar 31, 2027
|
Apr 30, 2027
|
May 31, 2027
|
|
Ruby 3.2
|
`ruby3.2`
|
Amazon Linux 2
|
Mar 31, 2026
|
Aug 31, 2026
|
Sep 30, 2026
|
###### To create a Ruby function
1. Open the [Lambda console](https://console.aws.amazon.com/lambda).
2. Choose **Create function**.
3. Configure the following settings:
* **Function name**: Enter a name for the function.
* **Runtime**: Choose **Ruby 3.4**.
* Choose **Create function**.
The console creates a Lambda function with a single source file named `lambda\_function.rb`. You can edit this file and add more files in the built-in code editor. In the **DEPLOY** section, choose **Deploy** to update your function's code. Then, to run your code, choose **Create test event** in the **TEST EVENTS** section.
The `lambda\_function.rb` file exports a function named `lambda\_handler` that takes an event object and a
context object. This is the [handler function](./ruby-handler.html) that Lambda calls when the
function is invoked. The Ruby function runtime gets invocation events from Lambda and passes them to the
handler. In the function configuration, the handler value is `lambda\_function.lambda\_handler`.
When you save your function code, the Lambda console creates a .zip file archive deployment package.
When you develop your function code outside of the console (using an IDE) you need to [create a
deployment package](./ruby-package.html) to upload your code to the Lambda function.
The function runtime passes a context object to the handler, in addition to the invocation event. The [context object](./ruby-context.html) contains additional information about the invocation, the
function, and the execution environment. More information is available from environment variables.
Your Lambda function comes with a CloudWatch Logs log group. The function runtime sends details about each invocation to
CloudWatch Logs. It relays any [logs that your function outputs](./ruby-logging.html) during invocation. If
your function returns an error, Lambda formats the error and returns it to the
invoker.
###### Topics
* [Runtime-included SDK versions](#ruby-sdk-included)
* [Enabling Yet Another Ruby JIT (YJIT)](#ruby-yjit)
* [Define Lambda function handler in Ruby](./ruby-handler.html)
* [Deploy Ruby Lambda functions with .zip file archives](./ruby-package.html)
* [Deploy Ruby Lambda functions with container images](./ruby-image.html)
* [Working with layers for Ruby Lambda functions](./ruby-layers.html)
* [Using the Lambda context object to retrieve Ruby function information](./ruby-context.html)
* [Log and monitor Ruby Lambda functions](./ruby-logging.html)
* [Instrumenting Ruby code in AWS Lambda](./ruby-tracing.html)
## Runtime-included SDK versions
The version of the AWS SDK included in the Ruby runtime depends on the runtime version and your AWS Region. The AWS SDK for Ruby is designed to be modular and is
separated by AWS service. To find the version number of a particular service gem included in the runtime you're using, create a Lambda function with code in the following
format. Replace `aws-sdk-s3` and `Aws::S3`with the name of the service gems your code uses.
```
`require 'aws-sdk-s3'
def lambda\_handler(event:, context:)
puts "Service gem version: #{Aws::S3::GEM\_VERSION}"
puts "Core version: #{Aws::CORE\_GEM\_VERSION}"
end`
```
## Enabling Yet Another Ruby JIT (YJIT)
The Ruby 3.2 runtime supports [YJIT](https://github.com/ruby/ruby/blob/master/doc/yjit/yjit.md), a lightweight, minimalistic Ruby JIT compiler. YJIT provides significantly higher performance, but also uses more memory than the Ruby interpreter. YJIT is recommended for Ruby on Rails workloads.
YJIT is not enabled by default. To enable YJIT for a Ruby 3.2 function, set the `RUBY\_YJIT\_ENABLE` environment variable to `1`. To confirm that YJIT is enabled, print the result of the `RubyVM::YJIT.enabled?` method.
###### Example — Confirm that YJIT is enabled
```
`puts(RubyVM::YJIT.enabled?())
# =&gt; true`
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