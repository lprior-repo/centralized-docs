---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_DeleteFunction_section.html
title: Use `DeleteFunction` with an AWS SDK or CLI
word_count: 1642
filtered: true
elements_removed: 0
density_score: 0.81
---

Use DeleteFunction with an AWS SDK or CLI - AWS Lambda
Use DeleteFunction with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_DeleteFunction_section)
# Use `DeleteFunction` with an AWS SDK or CLI
The following code examples show how to use `DeleteFunction`.
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
/// Delete an AWS Lambda function.
/// &lt;/summary&gt;
/// &lt;param name="functionName"&gt;The name of the Lambda function to
/// delete.&lt;/param&gt;
/// &lt;returns&gt;A Boolean value that indicates the success of the action.&lt;/returns&gt;
public async Task&lt;bool&gt; DeleteFunctionAsync(string functionName)
{
var request = new DeleteFunctionRequest
{
FunctionName = functionName,
};
var response = await \_lambdaService.DeleteFunctionAsync(request);
// A return value of NoContent means that the request was processed.
// In this case, the function was deleted, and the return value
// is intentionally blank.
return response.HttpStatusCode == System.Net.HttpStatusCode.NoContent;
}
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/DeleteFunction)
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
Aws::Lambda::Model::DeleteFunctionRequest request;
request.SetFunctionName(LAMBDA\_NAME);
Aws::Lambda::Model::DeleteFunctionOutcome outcome = client.DeleteFunction(
request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt; "The lambda function was successfully deleted." &lt;&lt; std::endl;
}
else {
std::cerr &lt;&lt; "Error with Lambda::DeleteFunction. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
}
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/DeleteFunction)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**Example 1: To delete a Lambda function by function name**
The following `delete-function` example deletes the Lambda function named `my-function` by specifying the function's name.
```
``aws lambda delete-function \\
--function-name `my-function``
`
```
This command produces no output.
**Example 2: To delete a Lambda function by function ARN**
The following `delete-function` example deletes the Lambda function named `my-function` by specifying the function's ARN.
```
``aws lambda delete-function \\
--function-name `arn:aws:lambda:us-west-2:123456789012:function:my-function``
`
```
This command produces no output.
**Example 3: To delete a Lambda function by partial function ARN**
The following `delete-function` example deletes the Lambda function named `my-function` by specifying the function's partial ARN.
```
``aws lambda delete-function \\
--function-name `123456789012:function:my-function``
`
```
This command produces no output.
For more information, see [AWS Lambda Function Configuration](https://docs.aws.amazon.com/lambda/latest/dg/resource-model.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[DeleteFunction](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/delete-function.html)
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
// DeleteFunction deletes the Lambda function specified by functionName.
func (wrapper FunctionWrapper) DeleteFunction(ctx context.Context, functionName string) {
\_, err := wrapper.LambdaClient.DeleteFunction(ctx, &amp;&amp;lambda.DeleteFunctionInput{
FunctionName: aws.String(functionName),
})
if err != nil {
log.Panicf("Couldn't delete function %v. Here's why: %v\\n", functionName, err)
}
}
`
```
*
For API details, see
[DeleteFunction](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.DeleteFunction)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Deletes an AWS Lambda function.
\*
\* @param awsLambda an instance of the {@link LambdaClient} class, which is used to interact with the AWS Lambda service
\* @param functionName the name of the Lambda function to be deleted
\*
\* @throws LambdaException if an error occurs while deleting the Lambda function
\*/
public static void deleteLambdaFunction(LambdaClient awsLambda, String functionName) {
try {
DeleteFunctionRequest request = DeleteFunctionRequest.builder()
.functionName(functionName)
.build();
awsLambda.deleteFunction(request);
System.out.println("The " + functionName + " function was deleted");
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
}
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/DeleteFunction)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`/\*\*
\* @param {string} funcName
\*/
const deleteFunction = (funcName) =&gt;&gt; {
const client = new LambdaClient({});
const command = new DeleteFunctionCommand({ FunctionName: funcName });
return client.send(command);
};
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/DeleteFunctionCommand)
in *AWS SDK for JavaScript API Reference*.
Kotlin
**SDK for Kotlin**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/kotlin/services/lambda#code-examples).
```
`suspend fun delLambdaFunction(myFunctionName: String) {
val request =
DeleteFunctionRequest {
functionName = myFunctionName
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
awsLambda.deleteFunction(request)
println("$myFunctionName was deleted")
}
}
`
```
*
For API details, see
[DeleteFunction](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
in *AWS SDK for Kotlin API reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function deleteFunction($functionName)
{
return $this-&gt;lambdaClient-&gt;deleteFunction([
'FunctionName' =&gt; $functionName,
]);
}
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/DeleteFunction)
in *AWS SDK for PHP API Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example deletes a specific version of a Lambda function**
```
`Remove-LMFunction -FunctionName "MylambdaFunction123" -Qualifier '3'
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example deletes a specific version of a Lambda function**
```
`Remove-LMFunction -FunctionName "MylambdaFunction123" -Qualifier '3'
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/powershell/v5/reference)
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
def delete\_function(self, function\_name):
"""
Deletes a Lambda function.
:param function\_name: The name of the function to delete.
"""
try:
self.lambda\_client.delete\_function(FunctionName=function\_name)
except ClientError:
logger.exception("Couldn't delete function %s.", function\_name)
raise
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/DeleteFunction)
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
# @param function\_name: The name of the function to delete.
def delete\_function(function\_name)
print "Deleting function: #{function\_name}..."
@lambda\_client.delete\_function(
function\_name: function\_name
)
print 'Done!'.green
rescue Aws::Lambda::Errors::ServiceException =&gt;&gt; e
@logger.error("There was an error deleting #{function\_name}:\\n #{e.message}")
end
`
```
*
For API details, see
[DeleteFunction](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/DeleteFunction)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\* Delete a function and its role, and if possible or necessary, its associated code object and bucket. \*/
pub async fn delete\_function(
&amp;&amp;self,
location: Option&lt;&lt;String&gt;&gt;,
) -&gt;&gt; (
Result&lt;&lt;DeleteFunctionOutput, anyhow::Error&gt;&gt;,
Result&lt;&lt;DeleteRoleOutput, anyhow::Error&gt;&gt;,
Option&lt;&lt;Result&lt;&lt;DeleteObjectOutput, anyhow::Error&gt;&gt;&gt;&gt;,
) {
info!("Deleting lambda function {}", self.lambda\_name);
let delete\_function = self
.lambda\_client
.delete\_function()
.function\_name(self.lambda\_name.clone())
.send()
.await
.map\_err(anyhow::Error::from);
info!("Deleting iam role {}", self.role\_name);
let delete\_role = self
.iam\_client
.delete\_role()
.role\_name(self.role\_name.clone())
.send()
.await
.map\_err(anyhow::Error::from);
let delete\_object: Option&lt;&lt;Result&lt;&lt;DeleteObjectOutput, anyhow::Error&gt;&gt;&gt;&gt; =
if let Some(location) = location {
info!("Deleting object {location}");
Some(
self.s3\_client
.delete\_object()
.bucket(self.bucket.clone())
.key(location)
.send()
.await
.map\_err(anyhow::Error::from),
)
} else {
info!(?location, "Skipping delete object");
None
};
(delete\_function, delete\_role, delete\_object)
}
`
```
*
For API details, see
[DeleteFunction](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.delete_function)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
lo\_lmd-&gt;&gt;deletefunction( iv\_functionname = iv\_function\_name ).
MESSAGE 'Lambda function deleted.' TYPE 'I'.
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
[DeleteFunction](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
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
do {
\_ = try await lambdaClient.deleteFunction(
input: DeleteFunctionInput(
functionName: "lambda-basics-function"
)
)
} catch {
print("Error: Unable to delete the function.")
}
`
```
*
For API details, see
[DeleteFunction](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/deletefunction(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
DeleteAlias
DeleteFunctionConcurrency
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.