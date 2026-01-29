---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_Hello_section.html
title: Hello Lambda
word_count: 1276
filtered: true
elements_removed: 0
density_score: 0.79
---

Hello Lambda - AWS Lambda
Hello Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_Hello_section)
# Hello Lambda
The following code examples show how to get started using Lambda.
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/dotnetv3/Lambda#code-examples).
```
`namespace LambdaActions;
using Amazon.Lambda;
public class HelloLambda
{
static async Task Main(string[] args)
{
var lambdaClient = new AmazonLambdaClient();
Console.WriteLine("Hello AWS Lambda");
Console.WriteLine("Let's get started with AWS Lambda by listing your existing Lambda functions:");
var response = await lambdaClient.ListFunctionsAsync();
response.Functions.ForEach(function =&gt;
{
Console.WriteLine($"{function.FunctionName}\\t{function.Description}");
});
}
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
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/cpp/example_code/lambda/hello_lambda#code-examples).
Code for the CMakeLists.txt CMake file.
```
`# Set the minimum required version of CMake for this project.
cmake\_minimum\_required(VERSION 3.13)
# Set the AWS service components used by this project.
set(SERVICE\_COMPONENTS lambda)
# Set the C++ standard to use to build this target.
# At least C++ 11 is required for the AWS SDK for C++.
set(CMAKE\_CXX\_STANDARD 11)
# Use the MSVC variable to determine if this is a Windows build.
set(WINDOWS\_BUILD ${MSVC})
if (WINDOWS\_BUILD) # Set the location where CMake can find the installed libraries for the AWS SDK.
string(REPLACE ";" "/aws-cpp-sdk-all;" SYSTEM\_MODULE\_PATH "${CMAKE\_SYSTEM\_PREFIX\_PATH}/aws-cpp-sdk-all")
list(APPEND CMAKE\_PREFIX\_PATH ${SYSTEM\_MODULE\_PATH})
endif ()
# Find the AWS SDK for C++ package.
find\_package(AWSSDK REQUIRED COMPONENTS ${SERVICE\_COMPONENTS})
if (WINDOWS\_BUILD AND AWSSDK\_INSTALL\_AS\_SHARED\_LIBS)
# Copy relevant AWS SDK for C++ libraries into the current binary directory for running and debugging.
# set(BIN\_SUB\_DIR "/Debug") # if you are building from the command line you may need to uncomment this
# and set the proper subdirectory to the executables' location.
AWSSDK\_CPY\_DYN\_LIBS(SERVICE\_COMPONENTS "" ${CMAKE\_CURRENT\_BINARY\_DIR}${BIN\_SUB\_DIR})
endif ()
add\_executable(${PROJECT\_NAME}
hello\_lambda.cpp)
target\_link\_libraries(${PROJECT\_NAME}
${AWSSDK\_LINK\_LIBRARIES})
`
```
Code for the hello\_lambda.cpp source file.
```
`#include &lt;&lt;aws/core/Aws.h&gt;&gt;
#include &lt;&lt;iostream&gt;&gt;
/\*
\* A "Hello Lambda" starter application which initializes an AWS Lambda (Lambda) client and lists the Lambda functions.
\*
\* main function
\*
\* Usage: 'hello\_lambda'
\*
\*/
int main(int argc, char \*\*argv) {
Aws::SDKOptions options;
// Optionally change the log level for debugging.
// options.loggingOptions.logLevel = Utils::Logging::LogLevel::Debug;
Aws::InitAPI(options); // Should only be called once.
int result = 0;
{
Aws::Client::ClientConfiguration clientConfig;
// Optional: Set to the AWS Region (overrides config file).
// clientConfig.region = "us-east-1";
Aws::Lambda::LambdaClient lambdaClient(clientConfig);
std::vector&lt;Aws::String&gt; functions;
Aws::String marker; // Used for pagination.
do {
Aws::Lambda::Model::ListFunctionsRequest request;
if (!marker.empty()) {
request.SetMarker(marker);
}
Aws::Lambda::Model::ListFunctionsOutcome outcome = lambdaClient.ListFunctions(
request);
if (outcome.IsSuccess()) {
const Aws::Lambda::Model::ListFunctionsResult &amp;listFunctionsResult = outcome.GetResult();
std::cout &lt;&lt; listFunctionsResult.GetFunctions().size()
&lt;&lt; " lambda functions were retrieved." &lt;&lt; std::endl;
for (const Aws::Lambda::Model::FunctionConfiguration &amp;functionConfiguration: listFunctionsResult.GetFunctions()) {
functions.push\_back(functionConfiguration.GetFunctionName());
std::cout &lt;&lt;&lt;&lt; functions.size() &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; functionConfiguration.GetDescription() &lt;&lt;&lt;&lt; std::endl;
std::cout &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; Aws::Lambda::Model::RuntimeMapper::GetNameForRuntime(
functionConfiguration.GetRuntime()) &lt;&lt;&lt;&lt; ": "
&lt;&lt;&lt;&lt; functionConfiguration.GetHandler()
&lt;&lt;&lt;&lt; std::endl;
}
marker = listFunctionsResult.GetNextMarker();
} else {
std::cerr &lt;&lt; "Error with Lambda::ListFunctions. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
result = 1;
break;
}
} while (!marker.empty());
}
Aws::ShutdownAPI(options); // Should only be called once.
return result;
}
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/ListFunctions)
in *AWS SDK for C++ API Reference*.
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/gov2/lambda#code-examples).
```
`
package main
import (
"context"
"fmt"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/config"
"github.com/aws/aws-sdk-go-v2/service/lambda"
)
// main uses the AWS SDK for Go (v2) to create an AWS Lambda client and list up to 10
// functions in your account.
// This example uses the default settings specified in your shared credentials
// and config files.
func main() {
ctx := context.Background()
sdkConfig, err := config.LoadDefaultConfig(ctx)
if err != nil {
fmt.Println("Couldn't load default configuration. Have you set up your AWS account?")
fmt.Println(err)
return
}
lambdaClient := lambda.NewFromConfig(sdkConfig)
maxItems := 10
fmt.Printf("Let's list up to %v functions for your account.\\n", maxItems)
result, err := lambdaClient.ListFunctions(ctx, &amp;&amp;lambda.ListFunctionsInput{
MaxItems: aws.Int32(int32(maxItems)),
})
if err != nil {
fmt.Printf("Couldn't list functions for your account. Here's why: %v\\n", err)
return
}
if len(result.Functions) == 0 {
fmt.Println("You don't have any functions!")
} else {
for \_, function := range result.Functions {
fmt.Printf("\\t%v\\n", \*function.FunctionName)
}
}
}
`
```
*
For API details, see
[ListFunctions](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.ListFunctions)
in *AWS SDK for Go API Reference*.
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
` /\*\*
\* Lists the AWS Lambda functions associated with the current AWS account.
\*
\* @param awsLambda an instance of the {@link LambdaClient} class, which is used to interact with the AWS Lambda service
\*
\* @throws LambdaException if an error occurs while interacting with the AWS Lambda service
\*/
public static void listFunctions(LambdaClient awsLambda) {
try {
ListFunctionsResponse functionResult = awsLambda.listFunctions();
List&lt;FunctionConfiguration&gt; list = functionResult.functions();
for (FunctionConfiguration config : list) {
System.out.println("The function name is " + config.functionName());
}
} catch (LambdaException e) {
System.err.println(e.getMessage());
System.exit(1);
}
}
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/ListFunctions)
in *AWS SDK for Java 2.x API Reference*.
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda#code-examples).
```
`import { LambdaClient, paginateListFunctions } from "@aws-sdk/client-lambda";
const client = new LambdaClient({});
export const helloLambda = async () =&gt; {
const paginator = paginateListFunctions({ client }, {});
const functions = [];
for await (const page of paginator) {
const funcNames = page.Functions.map((f) =&gt;&gt; f.FunctionName);
functions.push(...funcNames);
}
console.log("Functions:");
console.log(functions.join("\\n"));
return functions;
};
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/ListFunctionsCommand)
in *AWS SDK for JavaScript API Reference*.
Python
**SDK for Python (Boto3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/example_code/lambda#code-examples).
```
`
import boto3
def main():
"""
List the Lambda functions in your AWS account.
"""
# Use the paginator to list the functions
paginator = lambda\_client.get\_paginator("list\_functions")
response\_iterator = paginator.paginate()
print("Here are the Lambda functions in your account:")
for page in response\_iterator:
for function in page["Functions"]:
print(f" {function['FunctionName']}")
if \_\_name\_\_ == "\_\_main\_\_":
main()
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
`
require 'aws-sdk-lambda'
# Creates an AWS Lambda client using the default credentials and configuration
def lambda\_client
Aws::Lambda::Client.new
end
# Lists the Lambda functions in your AWS account, paginating the results if necessary
def list\_lambda\_functions
lambda = lambda\_client
# Print the name and ARN of each function
functions.each do |function|
puts "Function name: #{function.function\_name}"
puts "Function ARN: #{function.function\_arn}"
puts
end
puts "Total functions: #{functions.count}"
end
list\_lambda\_functions if \_\_FILE\_\_ == $PROGRAM\_NAME
`
```
*
For API details, see
[ListFunctions](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/ListFunctions)
in *AWS SDK for Ruby API Reference*.
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Basics
Learn the basics
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.