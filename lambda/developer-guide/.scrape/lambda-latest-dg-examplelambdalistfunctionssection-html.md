---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_ListFunctions_section.html
title: Use `ListFunctions` with an AWS SDK or CLI
word_count: 1601
filtered: true
elements_removed: 0
density_score: 0.74
---

Use ListFunctions with an AWS SDK or CLI - AWS Lambda
Use ListFunctions with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_ListFunctions_section)
# Use `ListFunctions` with an AWS SDK or CLI
The following code examples show how to use `ListFunctions`.
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
/// Get a list of Lambda functions.
/// &lt;/summary&gt;
/// &lt;returns&gt;A list of FunctionConfiguration objects.&lt;/returns&gt;
public async Task&lt;List&lt;FunctionConfiguration&gt;&gt; ListFunctionsAsync()
{
var functionList = new List&lt;&lt;FunctionConfiguration&gt;&gt;();
var functionPaginator =
\_lambdaService.Paginators.ListFunctions(new ListFunctionsRequest());
await foreach (var function in functionPaginator.Functions)
{
functionList.Add(function);
}
return functionList;
}
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/ListFunctions)
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
std::vector&lt;Aws::String&gt; functions;
Aws::String marker;
do {
Aws::Lambda::Model::ListFunctionsRequest request;
if (!marker.empty()) {
request.SetMarker(marker);
}
Aws::Lambda::Model::ListFunctionsOutcome outcome = client.ListFunctions(
request);
if (outcome.IsSuccess()) {
const Aws::Lambda::Model::ListFunctionsResult &amp;result = outcome.GetResult();
std::cout &lt;&lt; result.GetFunctions().size()
&lt;&lt; " lambda functions were retrieved." &lt;&lt; std::endl;
for (const Aws::Lambda::Model::FunctionConfiguration &amp;functionConfiguration: result.GetFunctions()) {
functions.push\_back(functionConfiguration.GetFunctionName());
std::cout &lt;&lt;&lt;&lt; functions.size() &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; functionConfiguration.GetDescription() &lt;&lt;&lt;&lt; std::endl;
std::cout &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; Aws::Lambda::Model::RuntimeMapper::GetNameForRuntime(
functionConfiguration.GetRuntime()) &lt;&lt;&lt;&lt; ": "
&lt;&lt;&lt;&lt; functionConfiguration.GetHandler()
&lt;&lt;&lt;&lt; std::endl;
}
marker = result.GetNextMarker();
}
else {
std::cerr &lt;&lt; "Error with Lambda::ListFunctions. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
}
} while (!marker.empty());
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/ListFunctions)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**To retrieve a list of Lambda functions**
The following `list-functions` example displays a list of all of the functions for the current user.
```
``aws lambda list-functions`
`
```
Output:
```
`{
"Functions": [
{
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "$LATEST",
"CodeSha256": "dBG9m8SGdmlEjw/JYXlhhvCrAv5TxvXsbL/RMr0fT/I=",
"FunctionName": "helloworld",
"MemorySize": 128,
"RevisionId": "1718e831-badf-4253-9518-d0644210af7b",
"CodeSize": 294,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:helloworld",
"Handler": "helloworld.handler",
"Role": "arn:aws:iam::123456789012:role/service-role/MyTestFunction-role-zgur6bf4",
"Timeout": 3,
"LastModified": "2025-09-23T18:32:33.857+0000",
"Runtime": "nodejs22.x",
"Description": ""
},
{
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "$LATEST",
"CodeSha256": "sU0cJ2/hOZevwV/lTxCuQqK3gDZP3i8gUoqUUVRmY6E=",
"FunctionName": "my-function",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"MemorySize": 256,
"RevisionId": "93017fc9-59cb-41dc-901b-4845ce4bf668",
"CodeSize": 266,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"Handler": "index.handler",
"Role": "arn:aws:iam::123456789012:role/service-role/helloWorldPython-role-uy3l9qyq",
"Timeout": 3,
"LastModified": "2025-10-01T16:47:28.490+0000",
"Runtime": "nodejs22.x",
"Description": ""
},
{
"Layers": [
{
"CodeSize": 41784542,
"Arn": "arn:aws:lambda:us-west-2:420165488524:layer:AWSLambda-Python37-SciPy1x:2"
},
{
"CodeSize": 4121,
"Arn": "arn:aws:lambda:us-west-2:123456789012:layer:pythonLayer:1"
}
],
"TracingConfig": {
"Mode": "PassThrough"
},
"Version": "$LATEST",
"CodeSha256": "ZQukCqxtkqFgyF2cU41Avj99TKQ/hNihPtDtRcc08mI=",
"FunctionName": "my-python-function",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"MemorySize": 128,
"RevisionId": "80b4eabc-acf7-4ea8-919a-e874c213707d",
"CodeSize": 299,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-python-function",
"Handler": "lambda\_function.lambda\_handler",
"Role": "arn:aws:iam::123456789012:role/service-role/my-python-function-role-z5g7dr6n",
"Timeout": 3,
"LastModified": "2025-10-01T19:40:41.643+0000",
"Runtime": "python3.11",
"Description": ""
}
]
}`
```
For more information, see [Configure Lambda function memory](https://docs.aws.amazon.com/lambda/latest/dg/configuration-memory.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[ListFunctions](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/list-functions.html)
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
// ListFunctions lists up to maxItems functions for the account. This function uses a
// lambda.ListFunctionsPaginator to paginate the results.
func (wrapper FunctionWrapper) ListFunctions(ctx context.Context, maxItems int) []types.FunctionConfiguration {
var functions []types.FunctionConfiguration
paginator := lambda.NewListFunctionsPaginator(wrapper.LambdaClient, &amp;lambda.ListFunctionsInput{
MaxItems: aws.Int32(int32(maxItems)),
})
for paginator.HasMorePages() &amp;&amp; len(functions) &lt; maxItems {
pageOutput, err := paginator.NextPage(ctx)
if err != nil {
log.Panicf("Couldn't list functions for your account. Here's why: %v\\n", err)
}
functions = append(functions, pageOutput.Functions...)
}
return functions
}
`
```
*
For API details, see
[ListFunctions](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.ListFunctions)
in *AWS SDK for Go API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`const listFunctions = () =&gt; {
const client = new LambdaClient({});
const command = new ListFunctionsCommand({});
return client.send(command);
};
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/ListFunctionsCommand)
in *AWS SDK for JavaScript API Reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function listFunctions($maxItems = 50, $marker = null)
{
if (is\_null($marker)) {
return $this-&gt;lambdaClient-&gt;listFunctions([
'MaxItems' =&gt; $maxItems,
]);
}
return $this-&gt;lambdaClient-&gt;listFunctions([
'Marker' =&gt; $marker,
'MaxItems' =&gt; $maxItems,
]);
}
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/ListFunctions)
in *AWS SDK for PHP API Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This sample displays all the Lambda functions with sorted code size**
```
`Get-LMFunctionList | Sort-Object -Property CodeSize | Select-Object FunctionName, RunTime, Timeout, CodeSize
`
```
**Output:**
```
`FunctionName Runtime Timeout CodeSize
------------ ------- ------- --------
test python2.7 3 243
MylambdaFunction123 python3.8 600 659
myfuncpython1 python3.8 303 675`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This sample displays all the Lambda functions with sorted code size**
```
`Get-LMFunctionList | Sort-Object -Property CodeSize | Select-Object FunctionName, RunTime, Timeout, CodeSize
`
```
**Output:**
```
`FunctionName Runtime Timeout CodeSize
------------ ------- ------- --------
test python2.7 3 243
MylambdaFunction123 python3.8 600 659
myfuncpython1 python3.8 303 675`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/powershell/v5/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V5)*.
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
def list\_functions(self):
"""
Lists the Lambda functions for the current account.
"""
try:
func\_paginator = self.lambda\_client.get\_paginator("list\_functions")
for func\_page in func\_paginator.paginate():
for func in func\_page["Functions"]:
print(func["FunctionName"])
desc = func.get("Description")
if desc:
print(f"\\t{desc}")
print(f"\\t{func['Runtime']}: {func['Handler']}")
except ClientError as err:
logger.error(
"Couldn't list functions. Here's why: %s: %s",
err.response["Error"]["Code"],
err.response["Error"]["Message"],
)
raise
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/ListFunctions)
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
# Lists the Lambda functions for the current account.
def list\_functions
functions = []
@lambda\_client.list\_functions.each do |response|
response['functions'].each do |function|
functions.append(function['function\_name'])
end
end
functions
rescue Aws::Lambda::Errors::ServiceException =&gt;&gt; e
@logger.error("There was an error listing functions:\\n #{e.message}")
end
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/ListFunctions)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\* List all Lambda functions in the current Region. \*/
pub async fn list\_functions(&amp;&amp;self) -&gt;&gt; Result&lt;&lt;ListFunctionsOutput, anyhow::Error&gt;&gt; {
info!("Listing lambda functions");
self.lambda\_client
.list\_functions()
.send()
.await
.map\_err(anyhow::Error::from)
}
`
```
*
For API details, see
[ListFunctions](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.list_functions)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
oo\_result = lo\_lmd-&gt;&gt;listfunctions( ). " oo\_result is returned for testing purposes. "
DATA(lt\_functions) = oo\_result-&gt;&gt;get\_functions( ).
MESSAGE 'Retrieved list of Lambda functions.' TYPE 'I'.
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
[ListFunctions](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
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
/// Returns an array containing the names of all AWS Lambda functions
/// available to the user.
///
/// - Parameter lambdaClient: The `IAMClient` to use.
///
/// - Throws: `ExampleError.listFunctionsError`
///
/// - Returns: An array of lambda function name strings.
func getFunctionNames(lambdaClient: LambdaClient) async throws -&gt; [String] {
let pages = lambdaClient.listFunctionsPaginated(
input: ListFunctionsInput()
)
var functionNames: [String] = []
for try await page in pages {
guard let functions = page.functions else {
throw ExampleError.listFunctionsError
}
for function in functions {
functionNames.append(function.functionName ?? "&lt;unknown&gt;")
}
}
return functionNames
}
`
```
*
For API details, see
[ListFunctions](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/listfunctions(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoke
ListProvisionedConcurrencyConfigs
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.