---
url: https://docs.aws.amazon.com/lambda/latest/dg/lambda-typescript.html
title: Building Lambda functions with TypeScript
word_count: 747
filtered: true
elements_removed: 0
density_score: 0.87
---

Building Lambda functions with TypeScript - AWS Lambda
Building Lambda functions with TypeScript - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#lambda-typescript)
[Development environment](#typescript-dev)[Type definitions for Lambda](#typescript-type-definitions)
# Building Lambda functions with TypeScript
You can use the Node.js runtime to run TypeScript code in AWS Lambda. Because Node.js doesn't run TypeScript code natively, you must first
transpile your TypeScript code into JavaScript. Then, use the JavaScript files to deploy your function code to Lambda. Your code runs in an
environment that includes the AWS SDK for JavaScript, with credentials from an AWS Identity and Access Management (IAM) role that you manage. To learn more
about the SDK versions included with the Node.js runtimes, see [Runtime-included SDK versions](./lambda-nodejs.html#nodejs-sdk-included).
Lambda supports the following Node.js runtimes.
|Name|Identifier|Operating system|Deprecation date|Block function create|Block function update|
|
Node.js 24
|
`nodejs24.x`
|
Amazon Linux 2023
|
Apr 30, 2028
|
Jun 1, 2028
|
Jul 1, 2028
|
|
Node.js 22
|
`nodejs22.x`
|
Amazon Linux 2023
|
Apr 30, 2027
|
Jun 1, 2027
|
Jul 1, 2027
|
|
Node.js 20
|
`nodejs20.x`
|
Amazon Linux 2023
|
Apr 30, 2026
|
Aug 31, 2026
|
Sep 30, 2026
|
###### Topics
* [Setting up a TypeScript development environment](#typescript-dev)
* [Type definitions for Lambda](#typescript-type-definitions)
* [Define Lambda function handler in TypeScript](./typescript-handler.html)
* [Deploy transpiled TypeScript code in Lambda with .zip file archives](./typescript-package.html)
* [Deploy transpiled TypeScript code in Lambda with container images](./typescript-image.html)
* [Using the Lambda context object to retrieve TypeScript function information](./typescript-context.html)
* [Log and monitor TypeScript Lambda functions](./typescript-logging.html)
* [Tracing TypeScript code in AWS Lambda](./typescript-tracing.html)
## Setting up a TypeScript development environment
Use a local integrated development environment (IDE) or text editor to write your TypeScript function code. You can’t create TypeScript code on the Lambda console.
You can use either [esbuild](https://esbuild.github.io/) or Microsoft's TypeScript compiler (`tsc`) to transpile your TypeScript code into JavaScript. The [AWS Serverless Application Model (AWS SAM)](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-getting-started.html) and the [AWS Cloud Development Kit (AWS CDK)](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html) both use esbuild.
When using esbuild, consider the following:
* There are several [TypeScript caveats](https://esbuild.github.io/content-types/#typescript-caveats).
* You must configure your TypeScript transpilation settings to match the Node.js runtime that you plan to use. For more information, see [Target](https://esbuild.github.io/api/#target) in the esbuild documentation. For an example of a **tsconfig.json** file that demonstrates how to target a specific Node.js version supported by Lambda, refer to the [TypeScript GitHub repository](https://github.com/tsconfig/bases/blob/main/bases/node14.json).
* esbuild doesn’t perform type checks. To check types, use the `tsc` compiler. Run `tsc -noEmit` or add a `"noEmit"` parameter to your **tsconfig.json** file, as shown in the following example. This configures `tsc` to not emit JavaScript files. After checking types, use esbuild to convert the TypeScript files into JavaScript.
###### Example tsconfig.json
```
` {
"compilerOptions": {
"target": "es2020",
"strict": true,
"preserveConstEnums": true,
`"noEmit": true,`
"sourceMap": false,
"module":"commonjs",
"moduleResolution":"node",
"esModuleInterop": true,
"skipLibCheck": true,
"forceConsistentCasingInFileNames": true,
"isolatedModules": true,
},
"exclude": ["node\_modules", "\*\*/\*.test.ts"]
}`
```
## Type definitions for Lambda
The [@types/aws-lambda](https://www.npmjs.com/package/@types/aws-lambda) package provides type definitions for Lambda functions. Install this package when your function uses any of the following:
* Common AWS event sources, such as:
* `APIGatewayProxyEvent`: For [Amazon API Gateway proxy integrations](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html)
* `SNSEvent`: For [Amazon Simple Notification Service notifications](./with-sns.html)
* `SQSEvent`: For [Amazon Simple Queue Service messages](./with-sqs.html)
* `S3Event`: For [S3 trigger events](./with-s3.html)
* `DynamoDBStreamEvent`: For [Amazon DynamoDB Streams](./with-ddb.html)
* The Lambda [Context](./typescript-context.html) object
* The [callback](./typescript-handler.html#typescript-handler-callback) handler pattern
To add the Lambda type definitions to your function, install `@types/aws-lambda` as a development dependency:
```
`npm install -D @types/aws-lambda`
```
Then, import the types from `aws-lambda`:
```
`import { Context, S3Event, APIGatewayProxyEvent } from 'aws-lambda';
export const handler = async (event: S3Event, context: Context) =&gt; {
// Function code
};`
```
The `import ... from 'aws-lambda'` statement imports the type definitions. It does not import the `aws-lambda` npm package, which is an unrelated third-party tool. For more information, see [aws-lambda](https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/aws-lambda) in the DefinitelyTyped GitHub repository.
###### Note
You don't need [@types/aws-lambda](https://www.npmjs.com/package/@types/aws-lambda) when using your own custom type definitions. For an example function that defines its own type for an event object, see [Example TypeScript Lambda function code](./typescript-handler.html#typescript-example-code).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tracing
Handler
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.