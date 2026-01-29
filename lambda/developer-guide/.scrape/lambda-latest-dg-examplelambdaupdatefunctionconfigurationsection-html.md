---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_UpdateFunctionConfiguration_section.html
title: Use `UpdateFunctionConfiguration` with an AWS SDK or CLI
word_count: 1814
filtered: true
elements_removed: 0
density_score: 0.80
---

Use UpdateFunctionConfiguration with an AWS SDK or CLI - AWS Lambda
Use UpdateFunctionConfiguration with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_UpdateFunctionConfiguration_section)
# Use `UpdateFunctionConfiguration` with an AWS SDK or CLI
The following code examples show how to use `UpdateFunctionConfiguration`.
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
/// Update the code of a Lambda function.
/// &lt;/summary&gt;
/// &lt;param name="functionName"&gt;The name of the function to update.&lt;/param&gt;
/// &lt;param name="functionHandler"&gt;The code that performs the function's actions.&lt;/param&gt;
/// &lt;param name="environmentVariables"&gt;A dictionary of environment variables.&lt;/param&gt;
/// &lt;returns&gt;A Boolean value indicating the success of the action.&lt;/returns&gt;
public async Task&lt;bool&gt; UpdateFunctionConfigurationAsync(
string functionName,
string functionHandler,
Dictionary&lt;string, string&gt; environmentVariables)
{
var request = new UpdateFunctionConfigurationRequest
{
Handler = functionHandler,
FunctionName = functionName,
Environment = new Amazon.Lambda.Model.Environment { Variables = environmentVariables },
};
var response = await \_lambdaService.UpdateFunctionConfigurationAsync(request);
Console.WriteLine(response.LastModified);
return response.HttpStatusCode == System.Net.HttpStatusCode.OK;
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/UpdateFunctionConfiguration)
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
Aws::Lambda::Model::UpdateFunctionConfigurationRequest request;
request.SetFunctionName(LAMBDA\_NAME);
Aws::Lambda::Model::Environment environment;
environment.AddVariables("LOG\_LEVEL", "DEBUG");
request.SetEnvironment(environment);
Aws::Lambda::Model::UpdateFunctionConfigurationOutcome outcome = client.UpdateFunctionConfiguration(
request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt; "The lambda configuration was successfully updated."
&lt;&lt; std::endl;
break;
}
else {
std::cerr &lt;&lt; "Error with Lambda::UpdateFunctionConfiguration. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/UpdateFunctionConfiguration)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**To modify the configuration of a function**
The following `update-function-configuration` example modifies the memory size to be 256 MB for the unpublished ($LATEST) version of the `my-function` function.
```
``aws lambda update-function-configuration \\
--function-name `my-function` \\
--memory-size `256``
`
```
Output:
```
`{
"FunctionName": "my-function",
"LastModified": "2019-09-26T20:28:40.438+0000",
"RevisionId": "e52502d4-9320-4688-9cd6-152a6ab7490d",
"MemorySize": 256,
"Version": "$LATEST",
"Role": "arn:aws:iam::123456789012:role/service-role/my-function-role-uy3l9qyq",
"Timeout": 3,
"Runtime": "nodejs10.x",
"TracingConfig": {
"Mode": "PassThrough"
},
"CodeSha256": "5tT2qgzYUHaqwR716pZ2dpkn/0J1FrzJmlKidWoaCgk=",
"Description": "",
"VpcConfig": {
"SubnetIds": [],
"VpcId": "",
"SecurityGroupIds": []
},
"CodeSize": 304,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"Handler": "index.handler"
}`
```
For more information, see [AWS Lambda Function Configuration](https://docs.aws.amazon.com/lambda/latest/dg/resource-model.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[UpdateFunctionConfiguration](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-configuration.html)
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
// UpdateFunctionConfiguration updates a map of environment variables configured for
// the Lambda function specified by functionName.
func (wrapper FunctionWrapper) UpdateFunctionConfiguration(ctx context.Context, functionName string, envVars map[string]string) {
\_, err := wrapper.LambdaClient.UpdateFunctionConfiguration(ctx, &amp;&amp;lambda.UpdateFunctionConfigurationInput{
FunctionName: aws.String(functionName),
Environment: &amp;types.Environment{Variables: envVars},
})
if err != nil {
log.Panicf("Couldn't update configuration for %v. Here's why: %v", functionName, err)
}
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.UpdateFunctionConfiguration)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Updates the configuration of an AWS Lambda function.
\*
\* @param awsLambda the {@link LambdaClient} instance to use for the AWS Lambda operation
\* @param functionName the name of the AWS Lambda function to update
\* @param handler the new handler for the AWS Lambda function
\*
\* @throws LambdaException if there is an error while updating the function configuration
\*/
public static void updateFunctionConfiguration(LambdaClient awsLambda, String functionName, String handler) {
try {
UpdateFunctionConfigurationRequest configurationRequest = UpdateFunctionConfigurationRequest.builder()
.functionName(functionName)
.handler(handler)
.runtime(Runtime.JAVA17)
.build();
awsLambda.updateFunctionConfiguration(configurationRequest);
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/UpdateFunctionConfiguration)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`const updateFunctionConfiguration = (funcName) =&gt; {
const client = new LambdaClient({});
const config = readFileSync(`${dirname}../functions/config.json`).toString();
const command = new UpdateFunctionConfigurationCommand({
...JSON.parse(config),
FunctionName: funcName,
});
const result = client.send(command);
waitForFunctionUpdated({ FunctionName: funcName });
return result;
};
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/UpdateFunctionConfigurationCommand)
in *AWS SDK for JavaScript API Reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function updateFunctionConfiguration($functionName, $handler, $environment = '')
{
return $this-&gt;&gt;lambdaClient-&gt;&gt;updateFunctionConfiguration([
'FunctionName' =&gt;&gt; $functionName,
'Handler' =&gt;&gt; "$handler.lambda\_handler",
'Environment' =&gt;&gt; $environment,
]);
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/UpdateFunctionConfiguration)
in *AWS SDK for PHP API Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example updates the existing Lambda Function Configuration**
```
`Update-LMFunctionConfiguration -FunctionName "MylambdaFunction123" -Handler "lambda\_function.launch\_instance" -Timeout 600 -Environment\_Variable @{ "envvar1"="value";"envvar2"="value" } -Role arn:aws:iam::123456789101:role/service-role/lambda -DeadLetterConfig\_TargetArn arn:aws:sns:us-east-1: 123456789101:MyfirstTopic
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example updates the existing Lambda Function Configuration**
```
`Update-LMFunctionConfiguration -FunctionName "MylambdaFunction123" -Handler "lambda\_function.launch\_instance" -Timeout 600 -Environment\_Variable @{ "envvar1"="value";"envvar2"="value" } -Role arn:aws:iam::123456789101:role/service-role/lambda -DeadLetterConfig\_TargetArn arn:aws:sns:us-east-1: 123456789101:MyfirstTopic
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/powershell/v5/reference)
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
def update\_function\_configuration(self, function\_name, env\_vars):
"""
Updates the environment variables for a Lambda function.
:param function\_name: The name of the function to update.
:param env\_vars: A dict of environment variables to update.
:return: Data about the update, including the status.
"""
try:
response = self.lambda\_client.update\_function\_configuration(
FunctionName=function\_name, Environment={"Variables": env\_vars}
)
except ClientError as err:
logger.error(
"Couldn't update function configuration %s. Here's why: %s: %s",
function\_name,
err.response["Error"]["Code"],
err.response["Error"]["Message"],
)
raise
else:
return response
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/UpdateFunctionConfiguration)
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
# @param function\_name: The name of the function to update.
# @return: Data about the update, including the status.
def update\_function\_configuration(function\_name, log\_level)
@lambda\_client.update\_function\_configuration({
function\_name: function\_name,
environment: {
variables: {
'LOG\_LEVEL' =&gt;&gt; log\_level
}
}
})
@lambda\_client.wait\_until(:function\_updated\_v2, { function\_name: function\_name }) do |w|
w.max\_attempts = 5
w.delay = 5
end
rescue Aws::Lambda::Errors::ServiceException =&gt;&gt; e
@logger.error("There was an error updating configurations for #{function\_name}:\\n #{e.message}")
rescue Aws::Waiters::Errors::WaiterFailed =&gt; e
@logger.error("Failed waiting for #{function\_name} to activate:\\n #{e.message}")
end
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/UpdateFunctionConfiguration)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\* Update the environment for a function. \*/
pub async fn update\_function\_configuration(
&amp;&amp;self,
environment: Environment,
) -&gt;&gt; Result&lt;&lt;UpdateFunctionConfigurationOutput, anyhow::Error&gt;&gt; {
info!(
?environment,
"Updating environment for {}", self.lambda\_name
);
let updated = self
.lambda\_client
.update\_function\_configuration()
.function\_name(self.lambda\_name.clone())
.environment(environment)
.send()
.await
.map\_err(anyhow::Error::from)?;
self.wait\_for\_function\_ready().await?;
Ok(updated)
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.update_function_configuration)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
oo\_result = lo\_lmd-&gt;&gt;updatefunctionconfiguration( " oo\_result is returned for testing purposes. "
iv\_functionname = iv\_function\_name
iv\_runtime = iv\_runtime
iv\_description = 'Updated Lambda function'
iv\_memorysize = iv\_memory\_size ).
MESSAGE 'Lambda function configuration/settings updated.' TYPE 'I'.
CATCH /aws1/cx\_lmdcodesigningcfgno00.
MESSAGE 'Code signing configuration does not exist.' TYPE 'E'.
CATCH /aws1/cx\_lmdcodeverification00.
MESSAGE 'Code signature failed one or more validation checks for signature mismatch or expiration.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvalidcodesigex.
MESSAGE 'Code signature failed the integrity check.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourceconflictex.
MESSAGE 'Resource already exists or another operation is in progress.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
CATCH /aws1/cx\_lmdserviceexception.
MESSAGE 'An internal problem was encountered by the AWS Lambda service.' TYPE 'E'.
CATCH /aws1/cx\_lmdtoomanyrequestsex.
MESSAGE 'The maximum request throughput was reached.' TYPE 'E'.
ENDTRY.
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
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
/// Tell the server-side component to log debug output by setting its
/// environment's `LOG\_LEVEL` to `DEBUG`.
///
/// - Parameters:
/// - lambdaClient: The `LambdaClient` to use.
/// - functionName: The name of the AWS Lambda function to enable debug
/// logging for.
///
/// - Throws: `ExampleError.environmentResponseMissingError`,
/// `ExampleError.updateFunctionConfigurationError`,
/// `ExampleError.environmentVariablesMissingError`,
/// `ExampleError.logLevelIncorrectError`,
/// `ExampleError.updateFunctionConfigurationError`
func enableDebugLogging(lambdaClient: LambdaClient, functionName: String) async throws {
let envVariables = [
"LOG\_LEVEL": "DEBUG"
]
let environment = LambdaClientTypes.Environment(variables: envVariables)
do {
let output = try await lambdaClient.updateFunctionConfiguration(
input: UpdateFunctionConfigurationInput(
environment: environment,
functionName: functionName
)
)
guard let response = output.environment else {
throw ExampleError.environmentResponseMissingError
}
if response.error != nil {
throw ExampleError.updateFunctionConfigurationError
}
guard let retVariables = response.variables else {
throw ExampleError.environmentVariablesMissingError
}
for envVar in retVariables {
if envVar.key == "LOG\_LEVEL" &amp;&amp;&amp;&amp; envVar.value != "DEBUG" {
print("\*\*\* Log level is not set to DEBUG!")
throw ExampleError.logLevelIncorrectError
}
}
} catch {
throw ExampleError.updateFunctionConfigurationError
}
}
`
```
*
For API details, see
[UpdateFunctionConfiguration](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/updatefunctionconfiguration(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
UpdateFunctionCode
Scenarios
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.