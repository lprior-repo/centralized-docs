---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_CreateFunction_section.html
title: Use `CreateFunction` with an AWS SDK or CLI
word_count: 3017
filtered: true
elements_removed: 0
density_score: 0.76
---

Use CreateFunction with an AWS SDK or CLI - AWS Lambda
Use CreateFunction with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_CreateFunction_section)
# Use `CreateFunction` with an AWS SDK or CLI
The following code examples show how to use `CreateFunction`.
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
/// Creates a new Lambda function.
/// &lt;/summary&gt;
/// &lt;param name="functionName"&gt;The name of the function.&lt;/param&gt;
/// &lt;param name="s3Bucket"&gt;The Amazon Simple Storage Service (Amazon S3)
/// bucket where the zip file containing the code is located.&lt;/param&gt;
/// &lt;param name="s3Key"&gt;The Amazon S3 key of the zip file.&lt;/param&gt;
/// &lt;param name="role"&gt;The Amazon Resource Name (ARN) of a role with the
/// appropriate Lambda permissions.&lt;/param&gt;
/// &lt;param name="handler"&gt;The name of the handler function.&lt;/param&gt;
/// &lt;returns&gt;The Amazon Resource Name (ARN) of the newly created
/// Lambda function.&lt;/returns&gt;
public async Task&lt;string&gt; CreateLambdaFunctionAsync(
string functionName,
string s3Bucket,
string s3Key,
string role,
string handler)
{
// Defines the location for the function code.
// S3Bucket - The S3 bucket where the file containing
// the source code is stored.
// S3Key - The name of the file containing the code.
var functionCode = new FunctionCode
{
S3Bucket = s3Bucket,
S3Key = s3Key,
};
var createFunctionRequest = new CreateFunctionRequest
{
FunctionName = functionName,
Description = "Created by the Lambda .NET API",
Code = functionCode,
Handler = handler,
Runtime = Runtime.Dotnet6,
Role = role,
};
var reponse = await \_lambdaService.CreateFunctionAsync(createFunctionRequest);
return reponse.FunctionArn;
}
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/CreateFunction)
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
Aws::Lambda::Model::CreateFunctionRequest request;
request.SetFunctionName(LAMBDA\_NAME);
request.SetDescription(LAMBDA\_DESCRIPTION); // Optional.
#if USE\_CPP\_LAMBDA\_FUNCTION
request.SetRuntime(Aws::Lambda::Model::Runtime::provided\_al2);
request.SetTimeout(15);
request.SetMemorySize(128);
// Assume the AWS Lambda function was built in Docker with same architecture
// as this code.
#endif
request.SetRole(roleArn);
request.SetHandler(LAMBDA\_HANDLER\_NAME);
request.SetPublish(true);
Aws::Lambda::Model::FunctionCode code;
std::ifstream ifstream(INCREMENT\_LAMBDA\_CODE.c\_str(),
std::ios\_base::in | std::ios\_base::binary);
if (!ifstream.is\_open()) {
std::cerr &lt;&lt;&lt;&lt; "Error opening file " &lt;&lt;&lt;&lt; INCREMENT\_LAMBDA\_CODE &lt;&lt;&lt;&lt; "." &lt;&lt;&lt;&lt; std::endl;
#if USE\_CPP\_LAMBDA\_FUNCTION
std::cerr
&lt;&lt;&lt;&lt; "The cpp Lambda function must be built following the instructions in the cpp\_lambda/README.md file. "
&lt;&lt;&lt;&lt; std::endl;
#endif
deleteIamRole(clientConfig);
return false;
}
Aws::StringStream buffer;
buffer &lt;&lt;&lt;&lt; ifstream.rdbuf();
code.SetZipFile(Aws::Utils::ByteBuffer((unsigned char \*) buffer.str().c\_str(),
buffer.str().length()));
request.SetCode(code);
Aws::Lambda::Model::CreateFunctionOutcome outcome = client.CreateFunction(
request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt; "The lambda function was successfully created. " &lt;&lt; seconds
&lt;&lt; " seconds elapsed." &lt;&lt; std::endl;
break;
}
else {
std::cerr &lt;&lt; "Error with CreateFunction. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
deleteIamRole(clientConfig);
return false;
}
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/CreateFunction)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**To create a Lambda function**
The following `create-function` example creates a Lambda function named `my-function`.
```
``aws lambda create-function \\
--function-name `my-function` \\
--runtime `nodejs22.x` \\
--zip-file `fileb://my-function.zip` \\
--handler `my-function.handler` \\
--role `arn:aws:iam::123456789012:role/service-role/MyTestFunction-role-tges6bf4``
`
```
Contents of `my-function.zip`:
```
`This file is a deployment package that contains your function code and any dependencies.`
```
Output:
```
`{
"TracingConfig": {
"Mode": "PassThrough"
},
"CodeSha256": "PFn4S+er27qk+UuZSTKEQfNKG/XNn7QJs90mJgq6oH8=",
"FunctionName": "my-function",
"CodeSize": 308,
"RevisionId": "873282ed-4cd3-4dc8-a069-d0c647e470c6",
"MemorySize": 128,
"FunctionArn": "arn:aws:lambda:us-west-2:123456789012:function:my-function",
"Version": "$LATEST",
"Role": "arn:aws:iam::123456789012:role/service-role/MyTestFunction-role-zgur6bf4",
"Timeout": 3,
"LastModified": "2025-10-14T22:26:11.234+0000",
"Handler": "my-function.handler",
"Runtime": "nodejs22.x",
"Description": ""
}`
```
For more information, see [Configure Lambda function memory](https://docs.aws.amazon.com/lambda/latest/dg/configuration-memory.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[CreateFunction](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/create-function.html)
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
// CreateFunction creates a new Lambda function from code contained in the zipPackage
// buffer. The specified handlerName must match the name of the file and function
// contained in the uploaded code. The role specified by iamRoleArn is assumed by
// Lambda and grants specific permissions.
// When the function already exists, types.StateActive is returned.
// When the function is created, a lambda.FunctionActiveV2Waiter is used to wait until the
// function is active.
func (wrapper FunctionWrapper) CreateFunction(ctx context.Context, functionName string, handlerName string,
iamRoleArn \*string, zipPackage \*bytes.Buffer) types.State {
var state types.State
\_, err := wrapper.LambdaClient.CreateFunction(ctx, &amp;&amp;lambda.CreateFunctionInput{
Code: &amp;types.FunctionCode{ZipFile: zipPackage.Bytes()},
FunctionName: aws.String(functionName),
Role: iamRoleArn,
Handler: aws.String(handlerName),
Publish: true,
Runtime: types.RuntimePython39,
})
if err != nil {
var resConflict \*types.ResourceConflictException
if errors.As(err, &amp;&amp;resConflict) {
log.Printf("Function %v already exists.\\n", functionName)
state = types.StateActive
} else {
log.Panicf("Couldn't create function %v. Here's why: %v\\n", functionName, err)
}
} else {
waiter := lambda.NewFunctionActiveV2Waiter(wrapper.LambdaClient)
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
[CreateFunction](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.CreateFunction)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Creates a new Lambda function in AWS using the AWS Lambda Java API.
\*
\* @param awsLambda the AWS Lambda client used to interact with the AWS Lambda service
\* @param functionName the name of the Lambda function to create
\* @param key the S3 key of the function code
\* @param bucketName the name of the S3 bucket containing the function code
\* @param role the IAM role to assign to the Lambda function
\* @param handler the fully qualified class name of the function handler
\* @return the Amazon Resource Name (ARN) of the created Lambda function
\*/
public static String createLambdaFunction(LambdaClient awsLambda,
String functionName,
String key,
String bucketName,
String role,
String handler) {
try {
LambdaWaiter waiter = awsLambda.waiter();
FunctionCode code = FunctionCode.builder()
.s3Key(key)
.s3Bucket(bucketName)
.build();
CreateFunctionRequest functionRequest = CreateFunctionRequest.builder()
.functionName(functionName)
.description("Created by the Lambda Java API")
.code(code)
.handler(handler)
.runtime(Runtime.JAVA17)
.role(role)
.build();
// Create a Lambda function using a waiter
CreateFunctionResponse functionResponse = awsLambda.createFunction(functionRequest);
GetFunctionRequest getFunctionRequest = GetFunctionRequest.builder()
.functionName(functionName)
.build();
WaiterResponse&lt;GetFunctionResponse&gt; waiterResponse = waiter.waitUntilFunctionExists(getFunctionRequest);
waiterResponse.matched().response().ifPresent(System.out::println);
return functionResponse.functionArn();
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
return "";
}
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/CreateFunction)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`const createFunction = async (funcName, roleArn) =&gt; {
const client = new LambdaClient({});
const code = await readFile(`${dirname}../functions/${funcName}.zip`);
const command = new CreateFunctionCommand({
Code: { ZipFile: code },
FunctionName: funcName,
Role: roleArn,
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
[CreateFunction](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/CreateFunctionCommand)
in *AWS SDK for JavaScript API Reference*.
Kotlin
**SDK for Kotlin**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/kotlin/services/lambda#code-examples).
```
`suspend fun createNewFunction(
myFunctionName: String,
s3BucketName: String,
myS3Key: String,
myHandler: String,
myRole: String,
): String? {
val functionCode =
FunctionCode {
s3Bucket = s3BucketName
s3Key = myS3Key
}
val request =
CreateFunctionRequest {
functionName = myFunctionName
code = functionCode
description = "Created by the Lambda Kotlin API"
handler = myHandler
role = myRole
runtime = Runtime.Java17
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
val functionResponse = awsLambda.createFunction(request)
awsLambda.waitUntilFunctionActive {
functionName = myFunctionName
}
return functionResponse.functionArn
}
}
`
```
*
For API details, see
[CreateFunction](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
in *AWS SDK for Kotlin API reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function createFunction($functionName, $role, $bucketName, $handler)
{
//This assumes the Lambda function is in an S3 bucket.
return $this-&gt;customWaiter(function () use ($functionName, $role, $bucketName, $handler) {
return $this-&gt;&gt;lambdaClient-&gt;&gt;createFunction([
'Code' =&gt;&gt; [
'S3Bucket' =&gt;&gt; $bucketName,
'S3Key' =&gt;&gt; $functionName,
],
'FunctionName' =&gt;&gt; $functionName,
'Role' =&gt;&gt; $role['Arn'],
'Runtime' =&gt;&gt; 'python3.9',
'Handler' =&gt;&gt; "$handler.lambda\_handler",
]);
});
}
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/CreateFunction)
in *AWS SDK for PHP API Reference*.
PowerShell
**Tools for PowerShell V4**
**Example 1: This example creates a new C# (dotnetcore1.0 runtime) function named MyFunction in AWS Lambda, providing the compiled binaries for the function from a zip file on the local file system (relative or absolute paths may be used). C# Lambda functions specify the handler for the function using the designation AssemblyName::Namespace.ClassName::MethodName. You should replace the assembly name (without .dll suffix), namespace, class name and method name parts of the handler spec appropriately. The new function will have environment variables 'envvar1' and 'envvar2' set up from the provided values.**
```
`Publish-LMFunction -Description "My C# Lambda Function" `
-FunctionName MyFunction `
-ZipFilename .\\MyFunctionBinaries.zip `
-Handler "AssemblyName::Namespace.ClassName::MethodName" `
-Role "arn:aws:iam::123456789012:role/LambdaFullExecRole" `
-Runtime dotnetcore1.0 `
-Environment\_Variable @{ "envvar1"="value";"envvar2"="value" }
`
```
**Output:**
```
`CodeSha256 : /NgBMd...gq71I=
CodeSize : 214784
DeadLetterConfig :
Description : My C# Lambda Function
Environment : Amazon.Lambda.Model.EnvironmentResponse
FunctionArn : arn:aws:lambda:us-west-2:123456789012:function:ToUpper
FunctionName : MyFunction
Handler : AssemblyName::Namespace.ClassName::MethodName
KMSKeyArn :
LastModified : 2016-12-29T23:50:14.207+0000
MemorySize : 128
Role : arn:aws:iam::123456789012:role/LambdaFullExecRole
Runtime : dotnetcore1.0
Timeout : 3
Version : $LATEST
VpcConfig :`
```
**Example 2: This example is similar to the previous one except the function binaries are first uploaded to an Amazon S3 bucket (which must be in the same region as the intended Lambda function) and the resulting S3 object is then referenced when creating the function.**
```
`Write-S3Object -BucketName amzn-s3-demo-bucket -Key MyFunctionBinaries.zip -File .\\MyFunctionBinaries.zip
Publish-LMFunction -Description "My C# Lambda Function" `
-FunctionName MyFunction `
-BucketName amzn-s3-demo-bucket `
-Key MyFunctionBinaries.zip `
-Handler "AssemblyName::Namespace.ClassName::MethodName" `
-Role "arn:aws:iam::123456789012:role/LambdaFullExecRole" `
-Runtime dotnetcore1.0 `
-Environment\_Variable @{ "envvar1"="value";"envvar2"="value" }
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/powershell/v4/reference)
in *AWS Tools for PowerShell Cmdlet Reference (V4)*.
**Tools for PowerShell V5**
**Example 1: This example creates a new C# (dotnetcore1.0 runtime) function named MyFunction in AWS Lambda, providing the compiled binaries for the function from a zip file on the local file system (relative or absolute paths may be used). C# Lambda functions specify the handler for the function using the designation AssemblyName::Namespace.ClassName::MethodName. You should replace the assembly name (without .dll suffix), namespace, class name and method name parts of the handler spec appropriately. The new function will have environment variables 'envvar1' and 'envvar2' set up from the provided values.**
```
`Publish-LMFunction -Description "My C# Lambda Function" `
-FunctionName MyFunction `
-ZipFilename .\\MyFunctionBinaries.zip `
-Handler "AssemblyName::Namespace.ClassName::MethodName" `
-Role "arn:aws:iam::123456789012:role/LambdaFullExecRole" `
-Runtime dotnetcore1.0 `
-Environment\_Variable @{ "envvar1"="value";"envvar2"="value" }
`
```
**Output:**
```
`CodeSha256 : /NgBMd...gq71I=
CodeSize : 214784
DeadLetterConfig :
Description : My C# Lambda Function
Environment : Amazon.Lambda.Model.EnvironmentResponse
FunctionArn : arn:aws:lambda:us-west-2:123456789012:function:ToUpper
FunctionName : MyFunction
Handler : AssemblyName::Namespace.ClassName::MethodName
KMSKeyArn :
LastModified : 2016-12-29T23:50:14.207+0000
MemorySize : 128
Role : arn:aws:iam::123456789012:role/LambdaFullExecRole
Runtime : dotnetcore1.0
Timeout : 3
Version : $LATEST
VpcConfig :`
```
**Example 2: This example is similar to the previous one except the function binaries are first uploaded to an Amazon S3 bucket (which must be in the same region as the intended Lambda function) and the resulting S3 object is then referenced when creating the function.**
```
`Write-S3Object -BucketName amzn-s3-demo-bucket -Key MyFunctionBinaries.zip -File .\\MyFunctionBinaries.zip
Publish-LMFunction -Description "My C# Lambda Function" `
-FunctionName MyFunction `
-BucketName amzn-s3-demo-bucket `
-Key MyFunctionBinaries.zip `
-Handler "AssemblyName::Namespace.ClassName::MethodName" `
-Role "arn:aws:iam::123456789012:role/LambdaFullExecRole" `
-Runtime dotnetcore1.0 `
-Environment\_Variable @{ "envvar1"="value";"envvar2"="value" }
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/powershell/v5/reference)
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
def create\_function(
self, function\_name, handler\_name, iam\_role, deployment\_package
):
"""
Deploys a Lambda function.
:param function\_name: The name of the Lambda function.
:param handler\_name: The fully qualified name of the handler function. This
must include the file name and the function name.
:param iam\_role: The IAM role to use for the function.
:param deployment\_package: The deployment package that contains the function
code in .zip format.
:return: The Amazon Resource Name (ARN) of the newly created function.
"""
try:
response = self.lambda\_client.create\_function(
FunctionName=function\_name,
Description="AWS Lambda doc example",
Runtime="python3.9",
Role=iam\_role.arn,
Handler=handler\_name,
Code={"ZipFile": deployment\_package},
Publish=True,
)
function\_arn = response["FunctionArn"]
waiter = self.lambda\_client.get\_waiter("function\_active\_v2")
waiter.wait(FunctionName=function\_name)
logger.info(
"Created function '%s' with ARN: '%s'.",
function\_name,
response["FunctionArn"],
)
except ClientError:
logger.error("Couldn't create function %s.", function\_name)
raise
else:
return function\_arn
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/CreateFunction)
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
# @param handler\_name: The fully qualified name of the handler function.
# @param role\_arn: The IAM role to use for the function.
# @param deployment\_package: The deployment package that contains the function code in .zip format.
# @return: The Amazon Resource Name (ARN) of the newly created function.
def create\_function(function\_name, handler\_name, role\_arn, deployment\_package)
response = @lambda\_client.create\_function({
role: role\_arn.to\_s,
function\_name: function\_name,
handler: handler\_name,
runtime: 'ruby2.7',
code: {
zip\_file: deployment\_package
},
environment: {
variables: {
'LOG\_LEVEL' =&gt;&gt; 'info'
}
}
})
@lambda\_client.wait\_until(:function\_active\_v2, { function\_name: function\_name }) do |w|
w.max\_attempts = 5
w.delay = 5
end
response
rescue Aws::Lambda::Errors::ServiceException =&gt;&gt; e
@logger.error("There was an error creating #{function\_name}:\\n #{e.message}")
rescue Aws::Waiters::Errors::WaiterFailed =&gt; e
@logger.error("Failed waiting for #{function\_name} to activate:\\n #{e.message}")
end
`
```
*
For API details, see
[CreateFunction](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/CreateFunction)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\*
\* Create a function, uploading from a zip file.
\*/
pub async fn create\_function(&amp;&amp;self, zip\_file: PathBuf) -&gt;&gt; Result&lt;&lt;String, anyhow::Error&gt;&gt; {
let code = self.prepare\_function(zip\_file, None).await?;
let key = code.s3\_key().unwrap().to\_string();
let role = self.create\_role().await.map\_err(|e| anyhow!(e))?;
info!("Created iam role, waiting 15s for it to become active");
tokio::time::sleep(Duration::from\_secs(15)).await;
info!("Creating lambda function {}", self.lambda\_name);
let \_ = self
.lambda\_client
.create\_function()
.function\_name(self.lambda\_name.clone())
.code(code)
.role(role.arn())
.runtime(aws\_sdk\_lambda::types::Runtime::Providedal2)
.handler("\_unused")
.send()
.await
.map\_err(anyhow::Error::from)?;
self.wait\_for\_function\_ready().await?;
self.lambda\_client
.publish\_version()
.function\_name(self.lambda\_name.clone())
.send()
.await?;
Ok(key)
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
[CreateFunction](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.create_function)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
lo\_lmd-&gt;&gt;createfunction(
iv\_functionname = iv\_function\_name
iv\_runtime = `python3.9`
iv\_role = iv\_role\_arn
iv\_handler = iv\_handler
io\_code = io\_zip\_file
iv\_description = 'AWS Lambda code example' ).
MESSAGE 'Lambda function created.' TYPE 'I'.
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
[CreateFunction](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
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
// Read the Zip archive containing the AWS Lambda function.
let zipUrl = URL(fileURLWithPath: path)
let zipData = try Data(contentsOf: zipUrl)
// Create the AWS Lambda function that runs the specified code,
// using the name given on the command line. The Lambda function
// will run using the Amazon Linux 2 runtime.
\_ = try await lambdaClient.createFunction(
input: CreateFunctionInput(
code: LambdaClientTypes.FunctionCode(zipFile: zipData),
functionName: functionName,
handler: "handle",
role: roleArn,
runtime: .providedal2
)
)
} catch {
print("\*\*\* Error creating Lambda function:")
dump(error)
return false
}
`
```
*
For API details, see
[CreateFunction](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/createfunction(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
CreateAlias
DeleteAlias
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.