---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_GetFunction_section.html
title: Use `GetFunction` with an AWS SDK or CLI
word_count: 1427
filtered: true
elements_removed: 0
density_score: 0.78
---

Use GetFunction with an AWS SDK or CLI - AWS Lambda
Use GetFunction with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_GetFunction_section)
# Use `GetFunction` with an AWS SDK or CLI
The following code examples show how to use `GetFunction`.
Action examples are code excerpts from larger programs and must be run in context. You can see this action in
context in the following code example:
* [Learn the basics](./example_lambda_Scenario_GettingStartedFunctions_section.html)
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/dotnetv3/Lambda#code-examples).
```
` /// &lt;summary&gt;
/// Gets information about a Lambda function.
/// &lt;/summary&gt;
/// &lt;param name="functionName"&gt;The name of the Lambda function for
/// which to retrieve information.&lt;/param&gt;
/// &lt;returns&gt;Async Task.&lt;/returns&gt;
public async Task&lt;FunctionConfiguration&gt; GetFunctionAsync(string functionName)
{
var functionRequest = new GetFunctionRequest
{
FunctionName = functionName,
};
var response = await \_lambdaService.GetFunctionAsync(functionRequest);
return response.Configuration;
}
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/GetFunction)
in *AWS SDK for .NET API Reference*.
C++
**SDK for C++**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/cpp/example_code/lambda#code-examples).
```
` Aws::Client::ClientConfiguration clientConfig;
// Optional: Set to the AWS Region in which the bucket was created (overrides config file).
// clientConfig.region = "us-east-1";
Aws::Lambda::LambdaClient client(clientConfig);
Aws::Lambda::Model::GetFunctionRequest request;
request.SetFunctionName(functionName);
Aws::Lambda::Model::GetFunctionOutcome outcome = client.GetFunction(request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt;&lt;&lt; "Function retrieve.\\n" &lt;&lt;&lt;&lt;
outcome.GetResult().GetConfiguration().Jsonize().View().WriteReadable()
&lt;&lt;&lt;&lt; std::endl;
}
else {
std::cerr &lt;&lt; "Error with Lambda::GetFunction. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
}
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/GetFunction)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**To retrieve information about a function**
The following `get-function` example displays information about the `my-function` function.
```
``aws lambda get-function \\
--function-name `my-function``
`
```
Output:
```
`{
"Concurrency": {
"ReservedConcurrentExecutions": 100
},
"Code": {
"RepositoryType": "S3",
"Location": "https://awslambda-us-west-2-tasks.s3.us-west-2.amazonaws.com/snapshots/123456789012/my-function..."
},
"Configuration": {
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "$LATEST",
"CodeSha256": "5tT2qgzYUHoqwR616pZ2dpkn/0J1FrzJmlKidWaaCgk=",
"FunctionName": "my-function",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"MemorySize": 128,
"RevisionId": "28f0fb31-5c5c-43d3-8955-03e76c5c1075",
"CodeSize": 304,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"Handler": "index.handler",
"Role": "arn:aws:iam::123456789012:role/service-role/helloWorldPython-role-uy3l9qyq",
"Timeout": 3,
"LastModified": "2025-09-24T18:20:35.054+0000",
"Runtime": "nodejs22.x",
"Description": ""
}
}`
```
For more information, see [Configure Lambda function memory](https://docs.aws.amazon.com/lambda/latest/dg/configuration-memory.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[GetFunction](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/get-function.html)
in *AWS CLI Command Reference*.
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/gov2/lambda#code-examples).
```
`
import (
"bytes"
"context"
"encoding/json"
"errors"
"log"
"time"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/lambda"
"github.com/aws/aws-sdk-go-v2/service/lambda/types"
)
// FunctionWrapper encapsulates function actions used in the examples.
// It contains an AWS Lambda service client that is used to perform user actions.
type FunctionWrapper struct {
LambdaClient \*lambda.Client
}
// GetFunction gets data about the Lambda function specified by functionName.
func (wrapper FunctionWrapper) GetFunction(ctx context.Context, functionName string) types.State {
var state types.State
funcOutput, err := wrapper.LambdaClient.GetFunction(ctx, &amp;lambda.GetFunctionInput{
FunctionName: aws.String(functionName),
})
if err != nil {
log.Panicf("Couldn't get function %v. Here's why: %v\\n", functionName, err)
} else {
state = funcOutput.Configuration.State
}
return state
}
`
```
*
For API details, see
[GetFunction](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.GetFunction)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Retrieves information about an AWS Lambda function.
\*
\* @param awsLambda an instance of the {@link LambdaClient} class, which is used to interact with the AWS Lambda service
\* @param functionName the name of the AWS Lambda function to retrieve information about
\*/
public static void getFunction(LambdaClient awsLambda, String functionName) {
try {
GetFunctionRequest functionRequest = GetFunctionRequest.builder()
.functionName(functionName)
.build();
GetFunctionResponse response = awsLambda.getFunction(functionRequest);
System.out.println("The runtime of this Lambda function is " + response.configuration().runtime());
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
}
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/GetFunction)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`const getFunction = (funcName) =&gt; {
const client = new LambdaClient({});
const command = new GetFunctionCommand({ FunctionName: funcName });
return client.send(command);
};
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/GetFunctionCommand)
in *AWS SDK for JavaScript API Reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function getFunction($functionName)
{
return $this-&gt;lambdaClient-&gt;getFunction([
'FunctionName' =&gt; $functionName,
]);
}
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/GetFunction)
in *AWS SDK for PHP API Reference*.
Python
**SDK for Python (Boto3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/example_code/lambda#code-examples).
```
`class LambdaWrapper:
def \_\_init\_\_(self, lambda\_client, iam\_resource):
self.lambda\_client = lambda\_client
self.iam\_resource = iam\_resource
def get\_function(self, function\_name):
"""
Gets data about a Lambda function.
:param function\_name: The name of the function.
:return: The function data.
"""
response = None
try:
response = self.lambda\_client.get\_function(FunctionName=function\_name)
except ClientError as err:
if err.response["Error"]["Code"] == "ResourceNotFoundException":
logger.info("Function %s does not exist.", function\_name)
else:
logger.error(
"Couldn't get function %s. Here's why: %s: %s",
function\_name,
err.response["Error"]["Code"],
err.response["Error"]["Message"],
)
raise
return response
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/GetFunction)
in *AWS SDK for Python (Boto3) API Reference*.
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/ruby/example_code/lambda#code-examples).
```
`class LambdaWrapper
attr\_accessor :lambda\_client, :cloudwatch\_client, :iam\_client
def initialize
@lambda\_client = Aws::Lambda::Client.new
@cloudwatch\_client = Aws::CloudWatchLogs::Client.new(region: 'us-east-1')
@iam\_client = Aws::IAM::Client.new(region: 'us-east-1')
@logger = Logger.new($stdout)
@logger.level = Logger::WARN
end
# @return response: The function data, or nil if no such function exists.
def get\_function(function\_name)
@lambda\_client.get\_function(
{
function\_name: function\_name
}
)
rescue Aws::Lambda::Errors::ResourceNotFoundException =&gt;&gt; e
@logger.debug("Could not find function: #{function\_name}:\\n #{e.message}")
nil
end
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/GetFunction)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\* Get the Lambda function with this Manager's name. \*/
pub async fn get\_function(&amp;&amp;self) -&gt;&gt; Result&lt;&lt;GetFunctionOutput, anyhow::Error&gt;&gt; {
info!("Getting lambda function");
self.lambda\_client
.get\_function()
.function\_name(self.lambda\_name.clone())
.send()
.await
.map\_err(anyhow::Error::from)
}
`
```
*
For API details, see
[GetFunction](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.get_function)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
oo\_result = lo\_lmd-&gt;&gt;getfunction( iv\_functionname = iv\_function\_name ). " oo\_result is returned for testing purposes. "
MESSAGE 'Lambda function information retrieved.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdserviceexception.
MESSAGE 'An internal problem was encountered by the AWS Lambda service.' TYPE 'E'.
CATCH /aws1/cx\_lmdtoomanyrequestsex.
MESSAGE 'The maximum request throughput was reached.' TYPE 'E'.
ENDTRY.
`
```
*
For API details, see
[GetFunction](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
in *AWS SDK for SAP ABAP API reference*.
Swift
**SDK for Swift**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/swift/example_code/lambda/basics#code-examples).
```
`import AWSClientRuntime
import AWSLambda
import Foundation
/// Detect whether or not the AWS Lambda function with the specified name
/// exists, by requesting its function information.
///
/// - Parameters:
/// - lambdaClient: The `LambdaClient` to use.
/// - name: The name of the AWS Lambda function to find.
///
/// - Returns: `true` if the Lambda function exists. Otherwise `false`.
func doesLambdaFunctionExist(lambdaClient: LambdaClient, name: String) async -&gt; Bool {
do {
\_ = try await lambdaClient.getFunction(
input: GetFunctionInput(functionName: name)
)
} catch {
return false
}
return true
}
`
```
*
For API details, see
[GetFunction](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/getfunction(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
GetAlias
GetFunctionConcurrency
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.