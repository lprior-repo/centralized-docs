---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_UpdateFunctionCode_section.html
title: Use `UpdateFunctionCode` with an AWS SDK or CLI
word_count: 2153
filtered: true
elements_removed: 0
density_score: 0.80
---

Use UpdateFunctionCode with an AWS SDK or CLI - AWS Lambda
Use UpdateFunctionCode with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_UpdateFunctionCode_section)
# Use `UpdateFunctionCode` with an AWS SDK or CLI
The following code examples show how to use `UpdateFunctionCode`.
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
/// Update an existing Lambda function.
/// &lt;/summary&gt;
/// &lt;param name="functionName"&gt;The name of the Lambda function to update.&lt;/param&gt;
/// &lt;param name="bucketName"&gt;The bucket where the zip file containing
/// the Lambda function code is stored.&lt;/param&gt;
/// &lt;param name="key"&gt;The key name of the source code file.&lt;/param&gt;
/// &lt;returns&gt;Async Task.&lt;/returns&gt;
public async Task UpdateFunctionCodeAsync(
string functionName,
string bucketName,
string key)
{
var functionCodeRequest = new UpdateFunctionCodeRequest
{
FunctionName = functionName,
Publish = true,
S3Bucket = bucketName,
S3Key = key,
};
var response = await \_lambdaService.UpdateFunctionCodeAsync(functionCodeRequest);
Console.WriteLine($"The Function was last modified at {response.LastModified}.");
}
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/UpdateFunctionCode)
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
Aws::Lambda::Model::UpdateFunctionCodeRequest request;
request.SetFunctionName(LAMBDA\_NAME);
std::ifstream ifstream(CALCULATOR\_LAMBDA\_CODE.c\_str(),
std::ios\_base::in | std::ios\_base::binary);
if (!ifstream.is\_open()) {
std::cerr &lt;&lt;&lt;&lt; "Error opening file " &lt;&lt;&lt;&lt; INCREMENT\_LAMBDA\_CODE &lt;&lt;&lt;&lt; "." &lt;&lt;&lt;&lt; std::endl;
#if USE\_CPP\_LAMBDA\_FUNCTION
std::cerr
&lt;&lt;&lt;&lt; "The cpp Lambda function must be built following the instructions in the cpp\_lambda/README.md file. "
&lt;&lt;&lt;&lt; std::endl;
#endif
deleteLambdaFunction(client);
deleteIamRole(clientConfig);
return false;
}
Aws::StringStream buffer;
buffer &lt;&lt;&lt;&lt; ifstream.rdbuf();
request.SetZipFile(
Aws::Utils::ByteBuffer((unsigned char \*) buffer.str().c\_str(),
buffer.str().length()));
request.SetPublish(true);
Aws::Lambda::Model::UpdateFunctionCodeOutcome outcome = client.UpdateFunctionCode(
request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt; "The lambda code was successfully updated." &lt;&lt; std::endl;
}
else {
std::cerr &lt;&lt; "Error with Lambda::UpdateFunctionCode. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
}
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/UpdateFunctionCode)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**To update the code of a Lambda function**
The following `update-function-code` example replaces the code of the unpublished ($LATEST) version of the `my-function` function with the contents of the specified zip file.
```
``aws lambda update-function-code \\
--function-name `my-function` \\
--zip-file `fileb://my-function.zip``
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
[UpdateFunctionCode](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/update-function-code.html)
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
// UpdateFunctionCode updates the code for the Lambda function specified by functionName.
// The existing code for the Lambda function is entirely replaced by the code in the
// zipPackage buffer. After the update action is called, a lambda.FunctionUpdatedV2Waiter
// is used to wait until the update is successful.
func (wrapper FunctionWrapper) UpdateFunctionCode(ctx context.Context, functionName string, zipPackage \*bytes.Buffer) types.State {
var state types.State
\_, err := wrapper.LambdaClient.UpdateFunctionCode(ctx, &amp;&amp;lambda.UpdateFunctionCodeInput{
FunctionName: aws.String(functionName), ZipFile: zipPackage.Bytes(),
})
if err != nil {
log.Panicf("Couldn't update code for function %v. Here's why: %v\\n", functionName, err)
} else {
waiter := lambda.NewFunctionUpdatedV2Waiter(wrapper.LambdaClient)
funcOutput, err := waiter.WaitForOutput(ctx, &amp;lambda.GetFunctionInput{
FunctionName: aws.String(functionName)}, 1\*time.Minute)
if err != nil {
log.Panicf("Couldn't wait for function %v to be active. Here's why: %v\\n", functionName, err)
} else {
state = funcOutput.Configuration.State
}
}
return state
}
`
```
*
For API details, see
[UpdateFunctionCode](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.UpdateFunctionCode)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Updates the code for an AWS Lambda function.
\*
\* @param awsLambda the AWS Lambda client
\* @param functionName the name of the Lambda function to update
\* @param bucketName the name of the S3 bucket where the function code is located
\* @param key the key (file name) of the function code in the S3 bucket
\* @throws LambdaException if there is an error updating the function code
\*/
public static void updateFunctionCode(LambdaClient awsLambda, String functionName, String bucketName, String key) {
try {
LambdaWaiter waiter = awsLambda.waiter();
UpdateFunctionCodeRequest functionCodeRequest = UpdateFunctionCodeRequest.builder()
.functionName(functionName)
.publish(true)
.s3Bucket(bucketName)
.s3Key(key)
.build();
UpdateFunctionCodeResponse response = awsLambda.updateFunctionCode(functionCodeRequest);
GetFunctionConfigurationRequest getFunctionConfigRequest = GetFunctionConfigurationRequest.builder()
.functionName(functionName)
.build();
WaiterResponse&lt;GetFunctionConfigurationResponse&gt; waiterResponse = waiter
.waitUntilFunctionUpdated(getFunctionConfigRequest);
waiterResponse.matched().response().ifPresent(System.out::println);
System.out.println("The last modified value is " + response.lastModified());
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
}
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/UpdateFunctionCode)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`const updateFunctionCode = async (funcName, newFunc) =&gt; {
const client = new LambdaClient({});
const code = await readFile(`${dirname}../functions/${newFunc}.zip`);
const command = new UpdateFunctionCodeCommand({
ZipFile: code,
FunctionName: funcName,
Architectures: [Architecture.arm64],
Handler: "index.handler", // Required when sending a .zip file
PackageType: PackageType.Zip, // Required when sending a .zip file
Runtime: Runtime.nodejs16x, // Required when sending a .zip file
});
return client.send(command);
};
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/UpdateFunctionCodeCommand)
in *AWS SDK for JavaScript API Reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function updateFunctionCode($functionName, $s3Bucket, $s3Key)
{
return $this-&gt;lambdaClient-&gt;updateFunctionCode([
'FunctionName' =&gt; $functionName,
'S3Bucket' =&gt; $s3Bucket,
'S3Key' =&gt; $s3Key,
]);
}
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/UpdateFunctionCode)
in *AWS SDK for PHP API Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: Updates the function named 'MyFunction' with new content contained in the specified zip file. For a C# .NET Core Lambda function the zip file should contain the compiled assembly.**
```
`Update-LMFunctionCode -FunctionName MyFunction -ZipFilename .\\UpdatedCode.zip
`
```
**Example 2: This example is similar to the previous one but uses an Amazon S3 object containing the updated code to update the function.**
```
`Update-LMFunctionCode -FunctionName MyFunction -BucketName amzn-s3-demo-bucket -Key UpdatedCode.zip
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: Updates the function named 'MyFunction' with new content contained in the specified zip file. For a C# .NET Core Lambda function the zip file should contain the compiled assembly.**
```
`Update-LMFunctionCode -FunctionName MyFunction -ZipFilename .\\UpdatedCode.zip
`
```
**Example 2: This example is similar to the previous one but uses an Amazon S3 object containing the updated code to update the function.**
```
`Update-LMFunctionCode -FunctionName MyFunction -BucketName amzn-s3-demo-bucket -Key UpdatedCode.zip
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/powershell/v5/reference)
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
def update\_function\_code(self, function\_name, deployment\_package):
"""
Updates the code for a Lambda function by submitting a .zip archive that contains
the code for the function.
:param function\_name: The name of the function to update.
:param deployment\_package: The function code to update, packaged as bytes in
.zip format.
:return: Data about the update, including the status.
"""
try:
response = self.lambda\_client.update\_function\_code(
FunctionName=function\_name, ZipFile=deployment\_package
)
except ClientError as err:
logger.error(
"Couldn't update function %s. Here's why: %s: %s",
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
[UpdateFunctionCode](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/UpdateFunctionCode)
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
# Updates the code for a Lambda function by submitting a .zip archive that contains
# @param function\_name: The name of the function to update.
# @param deployment\_package: The function code to update, packaged as bytes in
# @return: Data about the update, including the status.
def update\_function\_code(function\_name, deployment\_package)
@lambda\_client.update\_function\_code(
function\_name: function\_name,
zip\_file: deployment\_package
)
@lambda\_client.wait\_until(:function\_updated\_v2, { function\_name: function\_name }) do |w|
w.max\_attempts = 5
w.delay = 5
end
rescue Aws::Lambda::Errors::ServiceException =&gt;&gt; e
@logger.error("There was an error updating function code for: #{function\_name}:\\n #{e.message}")
nil
rescue Aws::Waiters::Errors::WaiterFailed =&gt; e
@logger.error("Failed waiting for #{function\_name} to update:\\n #{e.message}")
end
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/UpdateFunctionCode)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\* Given a Path to a zip file, update the function's code and wait for the update to finish. \*/
pub async fn update\_function\_code(
&amp;&amp;self,
zip\_file: PathBuf,
key: String,
) -&gt;&gt; Result&lt;&lt;UpdateFunctionCodeOutput, anyhow::Error&gt;&gt; {
let function\_code = self.prepare\_function(zip\_file, Some(key)).await?;
info!("Updating code for {}", self.lambda\_name);
let update = self
.lambda\_client
.update\_function\_code()
.function\_name(self.lambda\_name.clone())
.s3\_bucket(self.bucket.clone())
.s3\_key(function\_code.s3\_key().unwrap().to\_string())
.send()
.await
.map\_err(anyhow::Error::from)?;
self.wait\_for\_function\_ready().await?;
Ok(update)
}
/\*\*
\* Upload function code from a path to a zip file.
\* The zip file must have an AL2 Linux-compatible binary called `bootstrap`.
\* The easiest way to create such a zip is to use `cargo lambda build --output-format Zip`.
\*/
async fn prepare\_function(
&amp;&amp;self,
zip\_file: PathBuf,
key: Option&lt;&lt;String&gt;&gt;,
) -&gt;&gt; Result&lt;&lt;FunctionCode, anyhow::Error&gt;&gt; {
let body = ByteStream::from\_path(zip\_file).await?;
let key = key.unwrap\_or\_else(|| format!("{}\_code", self.lambda\_name));
info!("Uploading function code to s3://{}/{}", self.bucket, key);
let \_ = self
.s3\_client
.put\_object()
.bucket(self.bucket.clone())
.key(key.clone())
.body(body)
.send()
.await?;
Ok(FunctionCode::builder()
.s3\_bucket(self.bucket.clone())
.s3\_key(key)
.build())
}
`
```
*
For API details, see
[UpdateFunctionCode](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.update_function_code)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
oo\_result = lo\_lmd-&gt;&gt;updatefunctioncode( " oo\_result is returned for testing purposes. "
iv\_functionname = iv\_function\_name
iv\_zipfile = io\_zip\_file ).
MESSAGE 'Lambda function code updated.' TYPE 'I'.
CATCH /aws1/cx\_lmdcodesigningcfgno00.
MESSAGE 'Code signing configuration does not exist.' TYPE 'E'.
CATCH /aws1/cx\_lmdcodestorageexcdex.
MESSAGE 'Maximum total code size per account exceeded.' TYPE 'E'.
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
[UpdateFunctionCode](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
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
let zipUrl = URL(fileURLWithPath: path)
let zipData: Data
// Read the function's Zip file.
do {
zipData = try Data(contentsOf: zipUrl)
} catch {
throw ExampleError.zipFileReadError
}
// Update the function's code and wait for the updated version to be
// ready for use.
do {
\_ = try await lambdaClient.updateFunctionCode(
input: UpdateFunctionCodeInput(
functionName: functionName,
zipFile: zipData
)
)
} catch {
return false
}
`
```
*
For API details, see
[UpdateFunctionCode](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/updatefunctioncode(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
UpdateAlias
UpdateFunctionConfiguration
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.