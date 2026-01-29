---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_Invoke_section.html
title: Use `Invoke` with an AWS SDK or CLI
word_count: 1978
filtered: true
elements_removed: 0
density_score: 0.78
---

Use Invoke with an AWS SDK or CLI - AWS Lambda
Use Invoke with an AWS SDK or CLI - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_Invoke_section)
# Use `Invoke` with an AWS SDK or CLI
The following code examples show how to use `Invoke`.
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
/// Invoke a Lambda function.
/// &lt;/summary&gt;
/// &lt;param name="functionName"&gt;The name of the Lambda function to
/// invoke.&lt;/param
/// &lt;param name="parameters"&gt;The parameter values that will be passed to the function.&lt;/param&gt;
/// &lt;returns&gt;A System Threading Task.&lt;/returns&gt;
public async Task&lt;string&gt; InvokeFunctionAsync(
string functionName,
string parameters)
{
var payload = parameters;
var request = new InvokeRequest
{
FunctionName = functionName,
Payload = payload,
};
var response = await \_lambdaService.InvokeAsync(request);
MemoryStream stream = response.Payload;
string returnValue = System.Text.Encoding.UTF8.GetString(stream.ToArray());
return returnValue;
}
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/Invoke)
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
Aws::Lambda::Model::InvokeRequest request;
request.SetFunctionName(LAMBDA\_NAME);
request.SetLogType(logType);
std::shared\_ptr&lt;&lt;Aws::IOStream&gt;&gt; payload = Aws::MakeShared&lt;&lt;Aws::StringStream&gt;&gt;(
"FunctionTest");
\*payload &lt;&lt;&lt;&lt; jsonPayload.View().WriteReadable();
request.SetBody(payload);
request.SetContentType("application/json");
Aws::Lambda::Model::InvokeOutcome outcome = client.Invoke(request);
if (outcome.IsSuccess()) {
invokeResult = std::move(outcome.GetResult());
result = true;
break;
}
else {
std::cerr &lt;&lt; "Error with Lambda::InvokeRequest. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
break;
}
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/Invoke)
in *AWS SDK for C++ API Reference*.
CLI
**AWS CLI**
**Example 1: To invoke a Lambda function synchronously**
The following `invoke` example invokes the `my-function` function synchronously. The `cli-binary-format` option is required if you're using AWS CLI version 2. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide*.
```
``aws lambda invoke \\
--function-name `my-function` \\
--cli-binary-format `raw-in-base64-out` \\
--payload '`{ "name": "Bob" }`' \\
`response.json``
`
```
Output:
```
`{
"ExecutedVersion": "$LATEST",
"StatusCode": 200
}`
```
For more information, see [Invoke a Lambda function synchronously](https://docs.aws.amazon.com/lambda/latest/dg/invocation-sync.html) in the *AWS Lambda Developer Guide*.
**Example 2: To invoke a Lambda function asynchronously**
The following `invoke` example invokes the `my-function` function asynchronously. The `cli-binary-format` option is required if you're using AWS CLI version 2. For more information, see [AWS CLI supported global command line options](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-options.html#cli-configure-options-list) in the *AWS Command Line Interface User Guide*.
```
``aws lambda invoke \\
--function-name `my-function` \\
--invocation-type `Event` \\
--cli-binary-format `raw-in-base64-out` \\
--payload '`{ "name": "Bob" }`' \\
`response.json``
`
```
Output:
```
`{
"StatusCode": 202
}`
```
For more information, see [Invoking a Lambda function asynchronously](https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html) in the *AWS Lambda Developer Guide*.
*
For API details, see
[Invoke](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lambda/invoke.html)
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
// Invoke invokes the Lambda function specified by functionName, passing the parameters
// as a JSON payload. When getLog is true, types.LogTypeTail is specified, which tells
// Lambda to include the last few log lines in the returned result.
func (wrapper FunctionWrapper) Invoke(ctx context.Context, functionName string, parameters any, getLog bool) \*lambda.InvokeOutput {
logType := types.LogTypeNone
if getLog {
logType = types.LogTypeTail
}
payload, err := json.Marshal(parameters)
if err != nil {
log.Panicf("Couldn't marshal parameters to JSON. Here's why %v\\n", err)
}
invokeOutput, err := wrapper.LambdaClient.Invoke(ctx, &amp;&amp;lambda.InvokeInput{
FunctionName: aws.String(functionName),
LogType: logType,
Payload: payload,
})
if err != nil {
log.Panicf("Couldn't invoke function %v. Here's why: %v\\n", functionName, err)
}
return invokeOutput
}
`
```
*
For API details, see
[Invoke](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.Invoke)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Invokes a specific AWS Lambda function.
\*
\* @param awsLambda an instance of {@link LambdaClient} to interact with the AWS Lambda service
\* @param functionName the name of the AWS Lambda function to be invoked
\*/
public static void invokeFunction(LambdaClient awsLambda, String functionName) {
InvokeResponse res;
try {
// Need a SdkBytes instance for the payload.
JSONObject jsonObj = new JSONObject();
jsonObj.put("inputValue", "2000");
String json = jsonObj.toString();
SdkBytes payload = SdkBytes.fromUtf8String(json);
InvokeRequest request = InvokeRequest.builder()
.functionName(functionName)
.payload(payload)
.build();
res = awsLambda.invoke(request);
String value = res.payload().asUtf8String();
System.out.println(value);
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
}
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/Invoke)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`const invoke = async (funcName, payload) =&gt; {
const client = new LambdaClient({});
const command = new InvokeCommand({
FunctionName: funcName,
Payload: JSON.stringify(payload),
LogType: LogType.Tail,
});
const { Payload, LogResult } = await client.send(command);
const result = Buffer.from(Payload).toString();
const logs = Buffer.from(LogResult, "base64").toString();
return { logs, result };
};
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/InvokeCommand)
in *AWS SDK for JavaScript API Reference*.
Kotlin
**SDK for Kotlin**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/kotlin/services/lambda#code-examples).
```
`suspend fun invokeFunction(functionNameVal: String) {
val json = """{"inputValue":"1000"}"""
val byteArray = json.trimIndent().encodeToByteArray()
val request =
InvokeRequest {
functionName = functionNameVal
logType = LogType.Tail
payload = byteArray
}
LambdaClient { region = "us-west-2" }.use { awsLambda -&gt;
val res = awsLambda.invoke(request)
println("${res.payload?.toString(Charsets.UTF\_8)}")
println("The log result is ${res.logResult}")
}
}
`
```
*
For API details, see
[Invoke](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
in *AWS SDK for Kotlin API reference*.
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
` public function invoke($functionName, $params, $logType = 'None')
{
return $this-&gt;&gt;lambdaClient-&gt;&gt;invoke([
'FunctionName' =&gt;&gt; $functionName,
'Payload' =&gt;&gt; json\_encode($params),
'LogType' =&gt;&gt; $logType,
]);
}
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/Invoke)
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
def invoke\_function(self, function\_name, function\_params, get\_log=False):
"""
Invokes a Lambda function.
:param function\_name: The name of the function to invoke.
:param function\_params: The parameters of the function as a dict. This dict
is serialized to JSON before it is sent to Lambda.
:param get\_log: When true, the last 4 KB of the execution log are included in
the response.
:return: The response from the function invocation.
"""
try:
response = self.lambda\_client.invoke(
FunctionName=function\_name,
Payload=json.dumps(function\_params),
LogType="Tail" if get\_log else "None",
)
logger.info("Invoked function %s.", function\_name)
except ClientError:
logger.exception("Couldn't invoke function %s.", function\_name)
raise
return response
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/Invoke)
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
# @param function\_name [String] The name of the function to invoke.
# @return [Object] The response from the function invocation.
def invoke\_function(function\_name, payload = nil)
params = { function\_name: function\_name }
params[:payload] = payload unless payload.nil?
@lambda\_client.invoke(params)
rescue Aws::Lambda::Errors::ServiceException =&gt;&gt; e
@logger.error("There was an error executing #{function\_name}:\\n #{e.message}")
end
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/Invoke)
in *AWS SDK for Ruby API Reference*.
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
```
` /\*\* Invoke the lambda function using calculator InvokeArgs. \*/
pub async fn invoke(&amp;&amp;self, args: InvokeArgs) -&gt;&gt; Result&lt;&lt;InvokeOutput, anyhow::Error&gt;&gt; {
info!(?args, "Invoking {}", self.lambda\_name);
let payload = serde\_json::to\_string(&amp;&amp;args)?;
debug!(?payload, "Sending payload");
self.lambda\_client
.invoke()
.function\_name(self.lambda\_name.clone())
.payload(Blob::new(payload))
.send()
.await
.map\_err(anyhow::Error::from)
}
fn log\_invoke\_output(invoke: &amp;&amp;InvokeOutput, message: &amp;&amp;str) {
if let Some(payload) = invoke.payload().cloned() {
let payload = String::from\_utf8(payload.into\_inner());
info!(?payload, message);
} else {
info!("Could not extract payload")
}
if let Some(logs) = invoke.log\_result() {
debug!(?logs, "Invoked function logs")
} else {
debug!("Invoked function had no logs")
}
}
`
```
*
For API details, see
[Invoke](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.invoke)
in *AWS SDK for Rust API reference*.
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
` TRY.
DATA(lv\_json) = /aws1/cl\_rt\_util=&gt;&gt;string\_to\_xstring(
`{` &amp;&amp;&amp;&amp;
`"action": "increment",` &amp;&amp;&amp;&amp;
`"number": 10` &amp;&amp;&amp;&amp;
`}` ).
oo\_result = lo\_lmd-&gt;&gt;invoke( " oo\_result is returned for testing purposes. "
iv\_functionname = iv\_function\_name
iv\_payload = lv\_json ).
MESSAGE 'Lambda function invoked.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvrequestcontex.
MESSAGE 'Unable to parse request body as JSON.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvalidzipfileex.
MESSAGE 'The deployment package could not be unzipped.' TYPE 'E'.
CATCH /aws1/cx\_lmdrequesttoolargeex.
MESSAGE 'Invoke request body JSON input limit was exceeded by the request payload.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourceconflictex.
MESSAGE 'Resource already exists or another operation is in progress.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
CATCH /aws1/cx\_lmdserviceexception.
MESSAGE 'An internal problem was encountered by the AWS Lambda service.' TYPE 'E'.
CATCH /aws1/cx\_lmdtoomanyrequestsex.
MESSAGE 'The maximum request throughput was reached.' TYPE 'E'.
CATCH /aws1/cx\_lmdunsuppedmediatyp00.
MESSAGE 'Invoke request body does not have JSON as its content type.' TYPE 'E'.
ENDTRY.
`
```
*
For API details, see
[Invoke](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
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
/// Invoke the Lambda function to increment a value.
///
/// - Parameters:
/// - lambdaClient: The `IAMClient` to use.
/// - number: The number to increment.
///
/// - Throws: `ExampleError.noAnswerReceived`, `ExampleError.invokeError`
///
/// - Returns: An integer number containing the incremented value.
func invokeIncrement(lambdaClient: LambdaClient, number: Int) async throws -&gt; Int {
do {
let incRequest = IncrementRequest(action: "increment", number: number)
let incData = try! JSONEncoder().encode(incRequest)
// Invoke the lambda function.
let invokeOutput = try await lambdaClient.invoke(
input: InvokeInput(
functionName: "lambda-basics-function",
payload: incData
)
)
let response = try! JSONDecoder().decode(Response.self, from:invokeOutput.payload!)
guard let answer = response.answer else {
throw ExampleError.noAnswerReceived
}
return answer
} catch {
throw ExampleError.invokeError
}
}
`
```
*
For API details, see
[Invoke](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/invoke(input:))
in *AWS SDK for Swift API reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
GetProvisionedConcurrencyConfig
ListFunctions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.