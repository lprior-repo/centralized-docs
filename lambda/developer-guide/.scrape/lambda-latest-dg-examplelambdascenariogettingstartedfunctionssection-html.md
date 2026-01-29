---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_lambda_Scenario_GettingStartedFunctions_section.html
title: Learn the basics of Lambda with an AWS SDK
word_count: 19395
filtered: true
elements_removed: 0
density_score: 0.74
---

Learn the basics of Lambda with an AWS SDK - AWS Lambda
Learn the basics of Lambda with an AWS SDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_lambda_Scenario_GettingStartedFunctions_section)
# Learn the basics of Lambda with an AWS SDK
The following code examples show how to:
* Create an IAM role and Lambda function, then upload handler code.
* Invoke the function with a single parameter and get results.
* Update the function code and configure with an environment variable.
* Invoke the function with new parameters and get results. Display the returned execution log.
* List the functions for your account, then clean up resources.
For more information, see [Create a Lambda function with the console](https://docs.aws.amazon.com/lambda/latest/dg/getting-started-create-function.html).
.NET
**SDK for .NET**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/dotnetv3/Lambda#code-examples).
Create methods that perform Lambda actions.
```
`namespace LambdaActions;
using Amazon.Lambda;
using Amazon.Lambda.Model;
/// &lt;summary&gt;
/// A class that implements AWS Lambda methods.
/// &lt;/summary&gt;
public class LambdaWrapper
{
private readonly IAmazonLambda \_lambdaService;
/// &lt;&lt;summary&gt;&gt;
/// Constructor for the LambdaWrapper class.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="lambdaService"&gt;&gt;An initialized Lambda service client.&lt;&lt;/param&gt;&gt;
public LambdaWrapper(IAmazonLambda lambdaService)
{
\_lambdaService = lambdaService;
}
/// &lt;&lt;summary&gt;&gt;
/// Creates a new Lambda function.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="functionName"&gt;&gt;The name of the function.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;param name="s3Bucket"&gt;&gt;The Amazon Simple Storage Service (Amazon S3)
/// bucket where the zip file containing the code is located.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;param name="s3Key"&gt;&gt;The Amazon S3 key of the zip file.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;param name="role"&gt;&gt;The Amazon Resource Name (ARN) of a role with the
/// appropriate Lambda permissions.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;param name="handler"&gt;&gt;The name of the handler function.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;The Amazon Resource Name (ARN) of the newly created
/// Lambda function.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;string&gt;&gt; CreateLambdaFunctionAsync(
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
/// &lt;&lt;summary&gt;&gt;
/// Delete an AWS Lambda function.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="functionName"&gt;&gt;The name of the Lambda function to
/// delete.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;A Boolean value that indicates the success of the action.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;bool&gt;&gt; DeleteFunctionAsync(string functionName)
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
/// &lt;&lt;summary&gt;&gt;
/// Gets information about a Lambda function.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="functionName"&gt;&gt;The name of the Lambda function for
/// which to retrieve information.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;Async Task.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;FunctionConfiguration&gt;&gt; GetFunctionAsync(string functionName)
{
var functionRequest = new GetFunctionRequest
{
FunctionName = functionName,
};
var response = await \_lambdaService.GetFunctionAsync(functionRequest);
return response.Configuration;
}
/// &lt;&lt;summary&gt;&gt;
/// Invoke a Lambda function.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="functionName"&gt;&gt;The name of the Lambda function to
/// invoke.&lt;&lt;/param
/// &lt;&lt;param name="parameters"&gt;&gt;The parameter values that will be passed to the function.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;A System Threading Task.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;string&gt;&gt; InvokeFunctionAsync(
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
/// &lt;&lt;summary&gt;&gt;
/// Get a list of Lambda functions.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;returns&gt;&gt;A list of FunctionConfiguration objects.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;List&lt;&lt;FunctionConfiguration&gt;&gt;&gt;&gt; ListFunctionsAsync()
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
/// &lt;summary&gt;
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
/// &lt;summary&gt;
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
}
`
```
Create a function that runs the scenario.
```
`global using System.Threading.Tasks;
global using Amazon.IdentityManagement;
global using Amazon.Lambda;
global using LambdaActions;
global using LambdaScenarioCommon;
global using Microsoft.Extensions.DependencyInjection;
global using Microsoft.Extensions.Hosting;
global using Microsoft.Extensions.Logging;
global using Microsoft.Extensions.Logging.Console;
global using Microsoft.Extensions.Logging.Debug;
using Amazon.Lambda.Model;
using Microsoft.Extensions.Configuration;
namespace LambdaBasics;
public class LambdaBasics
{
private static ILogger logger = null!;
static async Task Main(string[] args)
{
// Set up dependency injection for the Amazon service.
using var host = Host.CreateDefaultBuilder(args)
.ConfigureLogging(logging =&gt;&gt;
logging.AddFilter("System", LogLevel.Debug)
.AddFilter&lt;&lt;DebugLoggerProvider&gt;&gt;("Microsoft", LogLevel.Information)
.AddFilter&lt;&lt;ConsoleLoggerProvider&gt;&gt;("Microsoft", LogLevel.Trace))
.ConfigureServices((\_, services) =&gt;&gt;
services.AddAWSService&lt;&lt;IAmazonLambda&gt;&gt;()
.AddAWSService&lt;&lt;IAmazonIdentityManagementService&gt;&gt;()
.AddTransient&lt;&lt;LambdaWrapper&gt;&gt;()
.AddTransient&lt;&lt;LambdaRoleWrapper&gt;&gt;()
.AddTransient&lt;&lt;UIWrapper&gt;&gt;()
)
.Build();
var configuration = new ConfigurationBuilder()
.SetBasePath(Directory.GetCurrentDirectory())
.AddJsonFile("settings.json") // Load test settings from .json file.
.AddJsonFile("settings.local.json",
true) // Optionally load local settings.
.Build();
logger = LoggerFactory.Create(builder =&gt;&gt; { builder.AddConsole(); })
.CreateLogger&lt;LambdaBasics&gt;();
var lambdaWrapper = host.Services.GetRequiredService&lt;LambdaWrapper&gt;();
var lambdaRoleWrapper = host.Services.GetRequiredService&lt;LambdaRoleWrapper&gt;();
var uiWrapper = host.Services.GetRequiredService&lt;UIWrapper&gt;();
string functionName = configuration["FunctionName"]!;
string roleName = configuration["RoleName"]!;
string policyDocument = "{" +
" \\"Version\\": \\"2012-10-17\\"," +
" \\"Statement\\": [ " +
" {" +
" \\"Effect\\": \\"Allow\\"," +
" \\"Principal\\": {" +
" \\"Service\\": \\"lambda.amazonaws.com\\" " +
" }," +
" \\"Action\\": \\"sts:AssumeRole\\" " +
" }" +
"]" +
"}";
var incrementHandler = configuration["IncrementHandler"];
var calculatorHandler = configuration["CalculatorHandler"];
var bucketName = configuration["BucketName"];
var incrementKey = configuration["IncrementKey"];
var calculatorKey = configuration["CalculatorKey"];
var policyArn = configuration["PolicyArn"];
uiWrapper.DisplayLambdaBasicsOverview();
// Create the policy to use with the AWS Lambda functions and then attach the
// policy to a new role.
var roleArn = await lambdaRoleWrapper.CreateLambdaRoleAsync(roleName, policyDocument);
Console.WriteLine("Waiting for role to become active.");
uiWrapper.WaitABit(15, "Wait until the role is active before trying to use it.");
// Attach the appropriate AWS Identity and Access Management (IAM) role policy to the new role.
var success = await lambdaRoleWrapper.AttachLambdaRolePolicyAsync(policyArn, roleName);
uiWrapper.WaitABit(10, "Allow time for the IAM policy to be attached to the role.");
// Create the Lambda function using a zip file stored in an Amazon Simple Storage Service
// (Amazon S3) bucket.
uiWrapper.DisplayTitle("Create Lambda Function");
Console.WriteLine($"Creating the AWS Lambda function: {functionName}.");
var lambdaArn = await lambdaWrapper.CreateLambdaFunctionAsync(
functionName,
bucketName,
incrementKey,
roleArn,
incrementHandler);
Console.WriteLine("Waiting for the new function to be available.");
Console.WriteLine($"The AWS Lambda ARN is {lambdaArn}");
// Get the Lambda function.
Console.WriteLine($"Getting the {functionName} AWS Lambda function.");
FunctionConfiguration config;
do
{
config = await lambdaWrapper.GetFunctionAsync(functionName);
Console.Write(".");
}
while (config.State != State.Active);
Console.WriteLine($"\\nThe function, {functionName} has been created.");
Console.WriteLine($"The runtime of this Lambda function is {config.Runtime}.");
uiWrapper.PressEnter();
// List the Lambda functions.
uiWrapper.DisplayTitle("Listing all Lambda functions.");
var functions = await lambdaWrapper.ListFunctionsAsync();
DisplayFunctionList(functions);
uiWrapper.DisplayTitle("Invoke increment function");
Console.WriteLine("Now that it has been created, invoke the Lambda increment function.");
string? value;
do
{
Console.Write("Enter a value to increment: ");
value = Console.ReadLine();
}
while (string.IsNullOrEmpty(value));
string functionParameters = "{" +
"\\"action\\": \\"increment\\", " +
"\\"x\\": \\"" + value + "\\"" +
"}";
var answer = await lambdaWrapper.InvokeFunctionAsync(functionName, functionParameters);
Console.WriteLine($"{value} + 1 = {answer}.");
uiWrapper.DisplayTitle("Update function");
Console.WriteLine("Now update the Lambda function code.");
await lambdaWrapper.UpdateFunctionCodeAsync(functionName, bucketName, calculatorKey);
do
{
config = await lambdaWrapper.GetFunctionAsync(functionName);
Console.Write(".");
}
while (config.LastUpdateStatus == LastUpdateStatus.InProgress);
await lambdaWrapper.UpdateFunctionConfigurationAsync(
functionName,
calculatorHandler,
new Dictionary&lt;string, string&gt; { { "LOG\_LEVEL", "DEBUG" } });
do
{
config = await lambdaWrapper.GetFunctionAsync(functionName);
Console.Write(".");
}
while (config.LastUpdateStatus == LastUpdateStatus.InProgress);
uiWrapper.DisplayTitle("Call updated function");
Console.WriteLine("Now call the updated function...");
bool done = false;
do
{
string? opSelected;
Console.WriteLine("Select the operation to perform:");
Console.WriteLine("\\t1. add");
Console.WriteLine("\\t2. subtract");
Console.WriteLine("\\t3. multiply");
Console.WriteLine("\\t4. divide");
Console.WriteLine("\\tOr enter \\"q\\" to quit.");
Console.WriteLine("Enter the number (1, 2, 3, 4, or q) of the operation you want to perform: ");
do
{
Console.Write("Your choice? ");
opSelected = Console.ReadLine();
}
while (opSelected == string.Empty);
var operation = (opSelected) switch
{
"1" =&gt;&gt; "add",
"2" =&gt;&gt; "subtract",
"3" =&gt;&gt; "multiply",
"4" =&gt;&gt; "divide",
"q" =&gt;&gt; "quit",
\_ =&gt;&gt; "add",
};
if (operation == "quit")
{
done = true;
}
else
{
// Get two numbers and an action from the user.
value = string.Empty;
do
{
Console.Write("Enter the first value: ");
value = Console.ReadLine();
}
while (value == string.Empty);
string? value2;
do
{
Console.Write("Enter a second value: ");
value2 = Console.ReadLine();
}
while (value2 == string.Empty);
functionParameters = "{" +
"\\"action\\": \\"" + operation + "\\", " +
"\\"x\\": \\"" + value + "\\"," +
"\\"y\\": \\"" + value2 + "\\"" +
"}";
answer = await lambdaWrapper.InvokeFunctionAsync(functionName, functionParameters);
Console.WriteLine($"The answer when we {operation} the two numbers is: {answer}.");
}
uiWrapper.PressEnter();
} while (!done);
// Delete the function created earlier.
uiWrapper.DisplayTitle("Clean up resources");
// Detach the IAM policy from the IAM role.
Console.WriteLine("First detach the IAM policy from the role.");
success = await lambdaRoleWrapper.DetachLambdaRolePolicyAsync(policyArn, roleName);
uiWrapper.WaitABit(15, "Let's wait for the policy to be fully detached from the role.");
Console.WriteLine("Delete the AWS Lambda function.");
success = await lambdaWrapper.DeleteFunctionAsync(functionName);
if (success)
{
Console.WriteLine($"The {functionName} function was deleted.");
}
else
{
Console.WriteLine($"Could not remove the function {functionName}");
}
// Now delete the IAM role created for use with the functions
// created by the application.
Console.WriteLine("Now we can delete the role that we created.");
success = await lambdaRoleWrapper.DeleteLambdaRoleAsync(roleName);
if (success)
{
Console.WriteLine("The role has been successfully removed.");
}
else
{
Console.WriteLine("Couldn't delete the role.");
}
Console.WriteLine("The Lambda Scenario is now complete.");
uiWrapper.PressEnter();
// Displays a formatted list of existing functions returned by the
// LambdaMethods.ListFunctions.
void DisplayFunctionList(List&lt;FunctionConfiguration&gt; functions)
{
functions.ForEach(functionConfig =&gt;
{
Console.WriteLine($"{functionConfig.FunctionName}\\t{functionConfig.Description}");
});
}
}
}
namespace LambdaActions;
using Amazon.IdentityManagement;
using Amazon.IdentityManagement.Model;
public class LambdaRoleWrapper
{
private readonly IAmazonIdentityManagementService \_lambdaRoleService;
public LambdaRoleWrapper(IAmazonIdentityManagementService lambdaRoleService)
{
\_lambdaRoleService = lambdaRoleService;
}
/// &lt;&lt;summary&gt;&gt;
/// Attach an AWS Identity and Access Management (IAM) role policy to the
/// IAM role to be assumed by the AWS Lambda functions created for the scenario.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="policyArn"&gt;&gt;The Amazon Resource Name (ARN) of the IAM policy.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;param name="roleName"&gt;&gt;The name of the IAM role to attach the IAM policy to.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;A Boolean value indicating the success of the action.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;bool&gt;&gt; AttachLambdaRolePolicyAsync(string policyArn, string roleName)
{
var response = await \_lambdaRoleService.AttachRolePolicyAsync(new AttachRolePolicyRequest { PolicyArn = policyArn, RoleName = roleName });
return response.HttpStatusCode == System.Net.HttpStatusCode.OK;
}
/// &lt;summary&gt;
/// Create a new IAM role.
/// &lt;/summary&gt;
/// &lt;param name="roleName"&gt;The name of the IAM role to create.&lt;/param&gt;
/// &lt;param name="policyDocument"&gt;The policy document for the new IAM role.&lt;/param&gt;
/// &lt;returns&gt;A string representing the ARN for newly created role.&lt;/returns&gt;
public async Task&lt;string&gt; CreateLambdaRoleAsync(string roleName, string policyDocument)
{
var request = new CreateRoleRequest
{
AssumeRolePolicyDocument = policyDocument,
RoleName = roleName,
};
var response = await \_lambdaRoleService.CreateRoleAsync(request);
return response.Role.Arn;
}
/// &lt;&lt;summary&gt;&gt;
/// Deletes an IAM role.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="roleName"&gt;&gt;The name of the role to delete.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;A Boolean value indicating the success of the operation.&lt;&lt;/returns&gt;&gt;
public async Task&lt;&lt;bool&gt;&gt; DeleteLambdaRoleAsync(string roleName)
{
var request = new DeleteRoleRequest
{
RoleName = roleName,
};
var response = await \_lambdaRoleService.DeleteRoleAsync(request);
return response.HttpStatusCode == System.Net.HttpStatusCode.OK;
}
public async Task&lt;&lt;bool&gt;&gt; DetachLambdaRolePolicyAsync(string policyArn, string roleName)
{
var response = await \_lambdaRoleService.DetachRolePolicyAsync(new DetachRolePolicyRequest { PolicyArn = policyArn, RoleName = roleName });
return response.HttpStatusCode == System.Net.HttpStatusCode.OK;
}
}
namespace LambdaScenarioCommon;
public class UIWrapper
{
public readonly string SepBar = new('-', Console.WindowWidth);
/// &lt;summary&gt;
/// Show information about the AWS Lambda Basics scenario.
/// &lt;/summary&gt;
public void DisplayLambdaBasicsOverview()
{
Console.Clear();
DisplayTitle("Welcome to AWS Lambda Basics");
Console.WriteLine("This example application does the following:");
Console.WriteLine("\\t1. Creates an AWS Identity and Access Management (IAM) role that will be assumed by the functions we create.");
Console.WriteLine("\\t2. Attaches an IAM role policy that has Lambda permissions.");
Console.WriteLine("\\t3. Creates a Lambda function that increments the value passed to it.");
Console.WriteLine("\\t4. Calls the increment function and passes a value.");
Console.WriteLine("\\t5. Updates the code so that the function is a simple calculator.");
Console.WriteLine("\\t6. Calls the calculator function with the values entered.");
Console.WriteLine("\\t7. Deletes the Lambda function.");
Console.WriteLine("\\t7. Detaches the IAM role policy.");
Console.WriteLine("\\t8. Deletes the IAM role.");
PressEnter();
}
/// &lt;&lt;summary&gt;&gt;
/// Display a message and wait until the user presses enter.
/// &lt;&lt;/summary&gt;&gt;
public void PressEnter()
{
Console.Write("\\nPress &lt;&lt;Enter&gt;&gt; to continue. ");
\_ = Console.ReadLine();
Console.WriteLine();
}
/// &lt;&lt;summary&gt;&gt;
/// Pad a string with spaces to center it on the console display.
/// &lt;&lt;/summary&gt;&gt;
/// &lt;&lt;param name="strToCenter"&gt;&gt;The string to be centered.&lt;&lt;/param&gt;&gt;
/// &lt;&lt;returns&gt;&gt;The padded string.&lt;&lt;/returns&gt;&gt;
public string CenterString(string strToCenter)
{
var padAmount = (Console.WindowWidth - strToCenter.Length) / 2;
var leftPad = new string(' ', padAmount);
return $"{leftPad}{strToCenter}";
}
/// &lt;summary&gt;
/// Display a line of hyphens, the centered text of the title and another
/// line of hyphens.
/// &lt;/summary&gt;
/// &lt;param name="strTitle"&gt;The string to be displayed.&lt;/param&gt;
public void DisplayTitle(string strTitle)
{
Console.WriteLine(SepBar);
Console.WriteLine(CenterString(strTitle));
Console.WriteLine(SepBar);
}
/// &lt;summary&gt;
/// Display a countdown and wait for a number of seconds.
/// &lt;/summary&gt;
/// &lt;param name="numSeconds"&gt;The number of seconds to wait.&lt;/param&gt;
public void WaitABit(int numSeconds, string msg)
{
Console.WriteLine(msg);
// Wait for the requested number of seconds.
for (int i = numSeconds; i &gt; 0; i--)
{
System.Threading.Thread.Sleep(1000);
Console.Write($"{i}...");
}
PressEnter();
}
}
`
```
Define a Lambda handler that increments a number.
```
`using Amazon.Lambda.Core;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace LambdaIncrement;
public class Function
{
/// &lt;summary&gt;
/// A simple function increments the integer parameter.
/// &lt;/summary&gt;
/// &lt;param name="input"&gt;A JSON string containing an action, which must be
/// "increment" and a string representing the value to increment.&lt;/param&gt;
/// &lt;param name="context"&gt;The context object passed by Lambda containing
/// information about invocation, function, and execution environment.&lt;/param&gt;
/// &lt;returns&gt;A string representing the incremented value of the parameter.&lt;/returns&gt;
public int FunctionHandler(Dictionary&lt;string, string&gt; input, ILambdaContext context)
{
if (input["action"] == "increment")
{
int inputValue = Convert.ToInt32(input["x"]);
return inputValue + 1;
}
else
{
return 0;
}
}
}
`
```
Define a second Lambda handler that performs arithmetic operations.
```
`using Amazon.Lambda.Core;
// Assembly attribute to enable the Lambda function's JSON input to be converted into a .NET class.
[assembly: LambdaSerializer(typeof(Amazon.Lambda.Serialization.SystemTextJson.DefaultLambdaJsonSerializer))]
namespace LambdaCalculator;
public class Function
{
/// &lt;summary&gt;
/// A simple function that takes two number in string format and performs
/// the requested arithmetic function.
/// &lt;/summary&gt;
/// &lt;param name="input"&gt;JSON data containing an action, and x and y values.
/// Valid actions include: add, subtract, multiply, and divide.&lt;/param&gt;
/// &lt;param name="context"&gt;The context object passed by Lambda containing
/// information about invocation, function, and execution environment.&lt;/param&gt;
/// &lt;returns&gt;A string representing the results of the calculation.&lt;/returns&gt;
public int FunctionHandler(Dictionary&lt;string, string&gt; input, ILambdaContext context)
{
var action = input["action"];
int x = Convert.ToInt32(input["x"]);
int y = Convert.ToInt32(input["y"]);
int result;
switch (action)
{
case "add":
result = x + y;
break;
case "subtract":
result = x - y;
break;
case "multiply":
result = x \* y;
break;
case "divide":
if (y == 0)
{
Console.Error.WriteLine("Divide by zero error.");
result = 0;
}
else
result = x / y;
break;
default:
Console.Error.WriteLine($"{action} is not a valid operation.");
result = 0;
break;
}
return result;
}
}
`
```
* For API details, see the following topics in *AWS SDK for .NET API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/CreateFunction)
* [DeleteFunction](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/DeleteFunction)
* [GetFunction](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/GetFunction)
* [Invoke](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/Invoke)
* [ListFunctions](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/ListFunctions)
* [UpdateFunctionCode](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/DotNetSDKV3/lambda-2015-03-31/UpdateFunctionConfiguration)
C++
**SDK for C++**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/cpp/example_code/lambda#code-examples).
```
`//! Get started with functions scenario.
/\*!
\\param clientConfig: AWS client configuration.
\\return bool: Successful completion.
\*/
bool AwsDoc::Lambda::getStartedWithFunctionsScenario(
const Aws::Client::ClientConfiguration &amp;&amp;clientConfig) {
Aws::Lambda::LambdaClient client(clientConfig);
// 1. Create an AWS Identity and Access Management (IAM) role for Lambda function.
Aws::String roleArn;
if (!getIamRoleArn(roleArn, clientConfig)) {
return false;
}
// 2. Create a Lambda function.
int seconds = 0;
do {
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
std::cout &lt;&lt;&lt;&lt; "The lambda function was successfully created. " &lt;&lt;&lt;&lt; seconds
&lt;&lt;&lt;&lt; " seconds elapsed." &lt;&lt;&lt;&lt; std::endl;
break;
}
else if (outcome.GetError().GetErrorType() ==
Aws::Lambda::LambdaErrors::INVALID\_PARAMETER\_VALUE &amp;&amp;&amp;&amp;
outcome.GetError().GetMessage().find("role") &gt;&gt;= 0) {
if ((seconds % 5) == 0) { // Log status every 10 seconds.
std::cout
&lt;&lt; "Waiting for the IAM role to become available as a CreateFunction parameter. "
&lt;&lt; seconds
&lt;&lt; " seconds elapsed." &lt;&lt; std::endl;
std::cout &lt;&lt; outcome.GetError().GetMessage() &lt;&lt; std::endl;
}
}
else {
std::cerr &lt;&lt;&lt;&lt; "Error with CreateFunction. "
&lt;&lt;&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt;&lt;&lt; std::endl;
deleteIamRole(clientConfig);
return false;
}
++seconds;
std::this\_thread::sleep\_for(std::chrono::seconds(1));
} while (60 &gt;&gt; seconds);
std::cout &lt;&lt;&lt;&lt; "The current Lambda function increments 1 by an input." &lt;&lt;&lt;&lt; std::endl;
// 3. Invoke the Lambda function.
{
int increment = askQuestionForInt("Enter an increment integer: ");
Aws::Lambda::Model::InvokeResult invokeResult;
Aws::Utils::Json::JsonValue jsonPayload;
jsonPayload.WithString("action", "increment");
jsonPayload.WithInteger("number", increment);
if (invokeLambdaFunction(jsonPayload, Aws::Lambda::Model::LogType::Tail,
invokeResult, client)) {
Aws::Utils::Json::JsonValue jsonValue(invokeResult.GetPayload());
Aws::Map&lt;Aws::String, Aws::Utils::Json::JsonView&gt; values =
jsonValue.View().GetAllObjects();
auto iter = values.find("result");
if (iter != values.end() &amp;&amp; iter-&gt;second.IsIntegerType()) {
{
std::cout &lt;&lt;&lt;&lt; INCREMENT\_RESUlT\_PREFIX
&lt;&lt;&lt;&lt; iter-&gt;&gt;second.AsInteger() &lt;&lt;&lt;&lt; std::endl;
}
}
else {
std::cout &lt;&lt; "There was an error in execution. Here is the log."
&lt;&lt; std::endl;
Aws::Utils::ByteBuffer buffer = Aws::Utils::HashingUtils::Base64Decode(
invokeResult.GetLogResult());
std::cout &lt;&lt; "With log " &lt;&lt; buffer.GetUnderlyingData() &lt;&lt; std::endl;
}
}
}
std::cout
&lt;&lt; "The Lambda function will now be updated with new code. Press return to continue, ";
Aws::String answer;
std::getline(std::cin, answer);
// 4. Update the Lambda function code.
{
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
std::cerr &lt;&lt;&lt;&lt; "Error with Lambda::UpdateFunctionCode. "
&lt;&lt;&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt;&lt;&lt; std::endl;
}
}
std::cout
&lt;&lt;&lt;&lt; "This function uses an environment variable to control the logging level."
&lt;&lt;&lt;&lt; std::endl;
std::cout
&lt;&lt;&lt;&lt; "UpdateFunctionConfiguration will be used to set the LOG\_LEVEL to DEBUG."
&lt;&lt;&lt;&lt; std::endl;
seconds = 0;
// 5. Update the Lambda function configuration.
do {
++seconds;
std::this\_thread::sleep\_for(std::chrono::seconds(1));
Aws::Lambda::Model::UpdateFunctionConfigurationRequest request;
request.SetFunctionName(LAMBDA\_NAME);
Aws::Lambda::Model::Environment environment;
environment.AddVariables("LOG\_LEVEL", "DEBUG");
request.SetEnvironment(environment);
Aws::Lambda::Model::UpdateFunctionConfigurationOutcome outcome = client.UpdateFunctionConfiguration(
request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt;&lt;&lt; "The lambda configuration was successfully updated."
&lt;&lt;&lt;&lt; std::endl;
break;
}
// RESOURCE\_IN\_USE: function code update not completed.
else if (outcome.GetError().GetErrorType() !=
Aws::Lambda::LambdaErrors::RESOURCE\_IN\_USE) {
if ((seconds % 10) == 0) { // Log status every 10 seconds.
std::cout &lt;&lt; "Lambda function update in progress . After " &lt;&lt; seconds
&lt;&lt; " seconds elapsed." &lt;&lt; std::endl;
}
}
else {
std::cerr &lt;&lt; "Error with Lambda::UpdateFunctionConfiguration. "
&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt; std::endl;
}
} while (0 &lt; seconds);
if (0 &gt; seconds) {
std::cerr &lt;&lt; "Function failed to become active." &lt;&lt; std::endl;
}
else {
std::cout &lt;&lt;&lt;&lt; "Updated function active after " &lt;&lt;&lt;&lt; seconds &lt;&lt;&lt;&lt; " seconds."
&lt;&lt;&lt;&lt; std::endl;
}
std::cout
&lt;&lt;&lt;&lt; "\\nThe new code applies an arithmetic operator to two variables, x an y."
&lt;&lt;&lt;&lt; std::endl;
std::vector&lt;&lt;Aws::String&gt;&gt; operators = {"plus", "minus", "times", "divided-by"};
for (size\_t i = 0; i &lt;&lt; operators.size(); ++i) {
std::cout &lt;&lt; " " &lt;&lt; i + 1 &lt;&lt; " " &lt;&lt; operators[i] &lt;&lt; std::endl;
}
// 6. Invoke the updated Lambda function.
do {
int operatorIndex = askQuestionForIntRange("Select an operator index 1 - 4 ", 1,
4);
int x = askQuestionForInt("Enter an integer for the x value ");
int y = askQuestionForInt("Enter an integer for the y value ");
Aws::Utils::Json::JsonValue calculateJsonPayload;
calculateJsonPayload.WithString("action", operators[operatorIndex - 1]);
calculateJsonPayload.WithInteger("x", x);
calculateJsonPayload.WithInteger("y", y);
Aws::Lambda::Model::InvokeResult calculatedResult;
if (invokeLambdaFunction(calculateJsonPayload,
Aws::Lambda::Model::LogType::Tail,
calculatedResult, client)) {
Aws::Utils::Json::JsonValue jsonValue(calculatedResult.GetPayload());
Aws::Map&lt;Aws::String, Aws::Utils::Json::JsonView&gt; values =
jsonValue.View().GetAllObjects();
auto iter = values.find("result");
if (iter != values.end() &amp;&amp; iter-&gt;second.IsIntegerType()) {
std::cout &lt;&lt;&lt;&lt; ARITHMETIC\_RESUlT\_PREFIX &lt;&lt;&lt;&lt; x &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; operators[operatorIndex - 1] &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; y &lt;&lt;&lt;&lt; " is " &lt;&lt;&lt;&lt; iter-&gt;&gt;second.AsInteger() &lt;&lt;&lt;&lt; std::endl;
}
else if (iter != values.end() &amp;&amp;&amp;&amp; iter-&gt;&gt;second.IsFloatingPointType()) {
std::cout &lt;&lt;&lt;&lt; ARITHMETIC\_RESUlT\_PREFIX &lt;&lt;&lt;&lt; x &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; operators[operatorIndex - 1] &lt;&lt;&lt;&lt; " "
&lt;&lt;&lt;&lt; y &lt;&lt;&lt;&lt; " is " &lt;&lt;&lt;&lt; iter-&gt;&gt;second.AsDouble() &lt;&lt;&lt;&lt; std::endl;
}
else {
std::cout &lt;&lt; "There was an error in execution. Here is the log."
&lt;&lt; std::endl;
Aws::Utils::ByteBuffer buffer = Aws::Utils::HashingUtils::Base64Decode(
calculatedResult.GetLogResult());
std::cout &lt;&lt; "With log " &lt;&lt; buffer.GetUnderlyingData() &lt;&lt; std::endl;
}
}
answer = askQuestion("Would you like to try another operation? (y/n) ");
} while (answer == "y");
std::cout
&lt;&lt; "A list of the lambda functions will be retrieved. Press return to continue, ";
std::getline(std::cin, answer);
// 7. List the Lambda functions.
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
// 8. Get a Lambda function.
if (!functions.empty()) {
std::stringstream question;
question &lt;&lt;&lt;&lt; "Choose a function to retrieve between 1 and " &lt;&lt;&lt;&lt; functions.size()
&lt;&lt;&lt;&lt; " ";
int functionIndex = askQuestionForIntRange(question.str(), 1,
static\_cast&lt;&lt;int&gt;&gt;(functions.size()));
Aws::String functionName = functions[functionIndex - 1];
Aws::Lambda::Model::GetFunctionRequest request;
request.SetFunctionName(functionName);
Aws::Lambda::Model::GetFunctionOutcome outcome = client.GetFunction(request);
if (outcome.IsSuccess()) {
std::cout &lt;&lt;&lt;&lt; "Function retrieve.\\n" &lt;&lt;&lt;&lt;
outcome.GetResult().GetConfiguration().Jsonize().View().WriteReadable()
&lt;&lt;&lt;&lt; std::endl;
}
else {
std::cerr &lt;&lt;&lt;&lt; "Error with Lambda::GetFunction. "
&lt;&lt;&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt;&lt;&lt; std::endl;
}
}
std::cout &lt;&lt;&lt;&lt; "The resources will be deleted. Press return to continue, ";
std::getline(std::cin, answer);
// 9. Delete the Lambda function.
bool result = deleteLambdaFunction(client);
// 10. Delete the IAM role.
return result &amp;&amp;&amp;&amp; deleteIamRole(clientConfig);
}
//! Routine which invokes a Lambda function and returns the result.
/\*!
\\param jsonPayload: Payload for invoke function.
\\param logType: Log type setting for invoke function.
\\param invokeResult: InvokeResult object to receive the result.
\\param client: Lambda client.
\\return bool: Successful completion.
\*/
bool
AwsDoc::Lambda::invokeLambdaFunction(const Aws::Utils::Json::JsonValue &amp;&amp;jsonPayload,
Aws::Lambda::Model::LogType logType,
Aws::Lambda::Model::InvokeResult &amp;&amp;invokeResult,
const Aws::Lambda::LambdaClient &amp;&amp;client) {
int seconds = 0;
bool result = false;
/\*
\* In this example, the Invoke function can be called before recently created resources are
\* available. The Invoke function is called repeatedly until the resources are
\* available.
\*/
do {
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
// ACCESS\_DENIED: because the role is not available yet.
// RESOURCE\_CONFLICT: because the Lambda function is being created or updated.
else if ((outcome.GetError().GetErrorType() ==
Aws::Lambda::LambdaErrors::ACCESS\_DENIED) ||
(outcome.GetError().GetErrorType() ==
Aws::Lambda::LambdaErrors::RESOURCE\_CONFLICT)) {
if ((seconds % 5) == 0) { // Log status every 10 seconds.
std::cout &lt;&lt;&lt;&lt; "Waiting for the invoke api to be available, status " &lt;&lt;&lt;&lt;
((outcome.GetError().GetErrorType() ==
Aws::Lambda::LambdaErrors::ACCESS\_DENIED ?
"ACCESS\_DENIED" : "RESOURCE\_CONFLICT")) &lt;&lt;&lt;&lt; ". " &lt;&lt;&lt;&lt; seconds
&lt;&lt;&lt;&lt; " seconds elapsed." &lt;&lt;&lt;&lt; std::endl;
}
}
else {
std::cerr &lt;&lt;&lt;&lt; "Error with Lambda::InvokeRequest. "
&lt;&lt;&lt;&lt; outcome.GetError().GetMessage()
&lt;&lt;&lt;&lt; std::endl;
break;
}
++seconds;
std::this\_thread::sleep\_for(std::chrono::seconds(1));
} while (seconds &lt;&lt; 60);
return result;
}
`
```
* For API details, see the following topics in *AWS SDK for C++ API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/CreateFunction)
* [DeleteFunction](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/DeleteFunction)
* [GetFunction](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/GetFunction)
* [Invoke](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/Invoke)
* [ListFunctions](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/ListFunctions)
* [UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForCpp/lambda-2015-03-31/UpdateFunctionConfiguration)
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/gov2/lambda#code-examples).
Create an interactive scenario that shows you how to get started with Lambda functions.
```
`
import (
"archive/zip"
"bytes"
"context"
"encoding/base64"
"encoding/json"
"errors"
"fmt"
"log"
"os"
"strings"
"time"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/iam"
iamtypes "github.com/aws/aws-sdk-go-v2/service/iam/types"
"github.com/aws/aws-sdk-go-v2/service/lambda"
"github.com/awsdocs/aws-doc-sdk-examples/gov2/demotools"
"github.com/awsdocs/aws-doc-sdk-examples/gov2/lambda/actions"
)
// GetStartedFunctionsScenario shows you how to use AWS Lambda to perform the following
// actions:
//
// 1. Create an AWS Identity and Access Management (IAM) role and Lambda function, then upload handler code.
// 2. Invoke the function with a single parameter and get results.
// 3. Update the function code and configure with an environment variable.
// 4. Invoke the function with new parameters and get results. Display the returned execution log.
// 5. List the functions for your account, then clean up resources.
type GetStartedFunctionsScenario struct {
sdkConfig aws.Config
functionWrapper actions.FunctionWrapper
questioner demotools.IQuestioner
helper IScenarioHelper
isTestRun bool
}
// NewGetStartedFunctionsScenario constructs a GetStartedFunctionsScenario instance from a configuration.
// It uses the specified config to get a Lambda client and create wrappers for the actions
// used in the scenario.
func NewGetStartedFunctionsScenario(sdkConfig aws.Config, questioner demotools.IQuestioner,
helper IScenarioHelper) GetStartedFunctionsScenario {
lambdaClient := lambda.NewFromConfig(sdkConfig)
return GetStartedFunctionsScenario{
sdkConfig: sdkConfig,
functionWrapper: actions.FunctionWrapper{LambdaClient: lambdaClient},
questioner: questioner,
helper: helper,
}
}
// Run runs the interactive scenario.
func (scenario GetStartedFunctionsScenario) Run(ctx context.Context) {
defer func() {
if r := recover(); r != nil {
log.Printf("Something went wrong with the demo.\\n")
}
}()
log.Println(strings.Repeat("-", 88))
log.Println("Welcome to the AWS Lambda get started with functions demo.")
log.Println(strings.Repeat("-", 88))
role := scenario.GetOrCreateRole(ctx)
funcName := scenario.CreateFunction(ctx, role)
scenario.InvokeIncrement(ctx, funcName)
scenario.UpdateFunction(ctx, funcName)
scenario.InvokeCalculator(ctx, funcName)
scenario.ListFunctions(ctx)
scenario.Cleanup(ctx, role, funcName)
log.Println(strings.Repeat("-", 88))
log.Println("Thanks for watching!")
log.Println(strings.Repeat("-", 88))
}
// GetOrCreateRole checks whether the specified role exists and returns it if it does.
// Otherwise, a role is created that specifies Lambda as a trusted principal.
// The AWSLambdaBasicExecutionRole managed policy is attached to the role and the role
// is returned.
func (scenario GetStartedFunctionsScenario) GetOrCreateRole(ctx context.Context) \*iamtypes.Role {
var role \*iamtypes.Role
iamClient := iam.NewFromConfig(scenario.sdkConfig)
log.Println("First, we need an IAM role that Lambda can assume.")
roleName := scenario.questioner.Ask("Enter a name for the role:", demotools.NotEmpty{})
getOutput, err := iamClient.GetRole(ctx, &amp;iam.GetRoleInput{
RoleName: aws.String(roleName)})
if err != nil {
var noSuch \*iamtypes.NoSuchEntityException
if errors.As(err, &amp;&amp;noSuch) {
log.Printf("Role %v doesn't exist. Creating it....\\n", roleName)
} else {
log.Panicf("Couldn't check whether role %v exists. Here's why: %v\\n",
roleName, err)
}
} else {
role = getOutput.Role
log.Printf("Found role %v.\\n", \*role.RoleName)
}
if role == nil {
trustPolicy := PolicyDocument{
Version: "2012-10-17",
Statement: []PolicyStatement{{
Effect: "Allow",
Principal: map[string]string{"Service": "lambda.amazonaws.com"},
Action: []string{"sts:AssumeRole"},
}},
}
policyArn := "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
createOutput, err := iamClient.CreateRole(ctx, &amp;iam.CreateRoleInput{
AssumeRolePolicyDocument: aws.String(trustPolicy.String()),
RoleName: aws.String(roleName),
})
if err != nil {
log.Panicf("Couldn't create role %v. Here's why: %v\\n", roleName, err)
}
role = createOutput.Role
\_, err = iamClient.AttachRolePolicy(ctx, &amp;&amp;iam.AttachRolePolicyInput{
PolicyArn: aws.String(policyArn),
RoleName: aws.String(roleName),
})
if err != nil {
log.Panicf("Couldn't attach a policy to role %v. Here's why: %v\\n", roleName, err)
}
log.Printf("Created role %v.\\n", \*role.RoleName)
log.Println("Let's give AWS a few seconds to propagate resources...")
scenario.helper.Pause(10)
}
log.Println(strings.Repeat("-", 88))
return role
}
// CreateFunction creates a Lambda function and uploads a handler written in Python.
// The code for the Python handler is packaged as a []byte in .zip format.
func (scenario GetStartedFunctionsScenario) CreateFunction(ctx context.Context, role \*iamtypes.Role) string {
log.Println("Let's create a function that increments a number.\\n" +
"The function uses the 'lambda\_handler\_basic.py' script found in the \\n" +
"'handlers' directory of this project.")
funcName := scenario.questioner.Ask("Enter a name for the Lambda function:", demotools.NotEmpty{})
zipPackage := scenario.helper.CreateDeploymentPackage("lambda\_handler\_basic.py", fmt.Sprintf("%v.py", funcName))
log.Printf("Creating function %v and waiting for it to be ready.", funcName)
funcState := scenario.functionWrapper.CreateFunction(ctx, funcName, fmt.Sprintf("%v.lambda\_handler", funcName),
role.Arn, zipPackage)
log.Printf("Your function is %v.", funcState)
log.Println(strings.Repeat("-", 88))
return funcName
}
// InvokeIncrement invokes a Lambda function that increments a number. The function
// parameters are contained in a Go struct that is used to serialize the parameters to
// a JSON payload that is passed to the function.
// The result payload is deserialized into a Go struct that contains an int value.
func (scenario GetStartedFunctionsScenario) InvokeIncrement(ctx context.Context, funcName string) {
parameters := actions.IncrementParameters{Action: "increment"}
log.Println("Let's invoke our function. This function increments a number.")
parameters.Number = scenario.questioner.AskInt("Enter a number to increment:", demotools.NotEmpty{})
log.Printf("Invoking %v with %v...\\n", funcName, parameters.Number)
invokeOutput := scenario.functionWrapper.Invoke(ctx, funcName, parameters, false)
var payload actions.LambdaResultInt
err := json.Unmarshal(invokeOutput.Payload, &amp;&amp;payload)
if err != nil {
log.Panicf("Couldn't unmarshal payload from invoking %v. Here's why: %v\\n",
funcName, err)
}
log.Printf("Invoking %v with %v returned %v.\\n", funcName, parameters.Number, payload)
log.Println(strings.Repeat("-", 88))
}
// UpdateFunction updates the code for a Lambda function by uploading a simple arithmetic
// calculator written in Python. The code for the Python handler is packaged as a
// []byte in .zip format.
// After the code is updated, the configuration is also updated with a new log
// level that instructs the handler to log additional information.
func (scenario GetStartedFunctionsScenario) UpdateFunction(ctx context.Context, funcName string) {
log.Println("Let's update the function to an arithmetic calculator.\\n" +
"The function uses the 'lambda\_handler\_calculator.py' script found in the \\n" +
"'handlers' directory of this project.")
scenario.questioner.Ask("Press Enter when you're ready.")
log.Println("Creating deployment package...")
zipPackage := scenario.helper.CreateDeploymentPackage("lambda\_handler\_calculator.py",
fmt.Sprintf("%v.py", funcName))
log.Println("...and updating the Lambda function and waiting for it to be ready.")
funcState := scenario.functionWrapper.UpdateFunctionCode(ctx, funcName, zipPackage)
log.Printf("Updated function %v. Its current state is %v.", funcName, funcState)
log.Println("This function uses an environment variable to control logging level.")
log.Println("Let's set it to DEBUG to get the most logging.")
scenario.functionWrapper.UpdateFunctionConfiguration(ctx, funcName,
map[string]string{"LOG\_LEVEL": "DEBUG"})
log.Println(strings.Repeat("-", 88))
}
// InvokeCalculator invokes the Lambda calculator function. The parameters are stored in a
// Go struct that is used to serialize the parameters to a JSON payload. That payload is then passed
// to the function.
// The result payload is deserialized to a Go struct that stores the result as either an
// int or float32, depending on the kind of operation that was specified.
func (scenario GetStartedFunctionsScenario) InvokeCalculator(ctx context.Context, funcName string) {
wantInvoke := true
choices := []string{"plus", "minus", "times", "divided-by"}
for wantInvoke {
choice := scenario.questioner.AskChoice("Select an arithmetic operation:\\n", choices)
x := scenario.questioner.AskInt("Enter a value for x:", demotools.NotEmpty{})
y := scenario.questioner.AskInt("Enter a value for y:", demotools.NotEmpty{})
log.Printf("Invoking %v %v %v...", x, choices[choice], y)
calcParameters := actions.CalculatorParameters{
Action: choices[choice],
X: x,
Y: y,
}
invokeOutput := scenario.functionWrapper.Invoke(ctx, funcName, calcParameters, true)
var payload any
if choice == 3 { // divide-by results in a float.
payload = actions.LambdaResultFloat{}
} else {
payload = actions.LambdaResultInt{}
}
err := json.Unmarshal(invokeOutput.Payload, &amp;payload)
if err != nil {
log.Panicf("Couldn't unmarshal payload from invoking %v. Here's why: %v\\n",
funcName, err)
}
log.Printf("Invoking %v with %v %v %v returned %v.\\n", funcName,
calcParameters.X, calcParameters.Action, calcParameters.Y, payload)
scenario.questioner.Ask("Press Enter to see the logs from the call.")
logRes, err := base64.StdEncoding.DecodeString(\*invokeOutput.LogResult)
if err != nil {
log.Panicf("Couldn't decode log result. Here's why: %v\\n", err)
}
log.Println(string(logRes))
wantInvoke = scenario.questioner.AskBool("Do you want to calculate again? (y/n)", "y")
}
log.Println(strings.Repeat("-", 88))
}
// ListFunctions lists up to the specified number of functions for your account.
func (scenario GetStartedFunctionsScenario) ListFunctions(ctx context.Context) {
count := scenario.questioner.AskInt(
"Let's list functions for your account. How many do you want to see?", demotools.NotEmpty{})
functions := scenario.functionWrapper.ListFunctions(ctx, count)
log.Printf("Found %v functions:", len(functions))
for \_, function := range functions {
log.Printf("\\t%v", \*function.FunctionName)
}
log.Println(strings.Repeat("-", 88))
}
// Cleanup removes the IAM and Lambda resources created by the example.
func (scenario GetStartedFunctionsScenario) Cleanup(ctx context.Context, role \*iamtypes.Role, funcName string) {
if scenario.questioner.AskBool("Do you want to clean up resources created for this example? (y/n)",
"y") {
iamClient := iam.NewFromConfig(scenario.sdkConfig)
policiesOutput, err := iamClient.ListAttachedRolePolicies(ctx,
&amp;iam.ListAttachedRolePoliciesInput{RoleName: role.RoleName})
if err != nil {
log.Panicf("Couldn't get policies attached to role %v. Here's why: %v\\n",
\*role.RoleName, err)
}
for \_, policy := range policiesOutput.AttachedPolicies {
\_, err = iamClient.DetachRolePolicy(ctx, &amp;&amp;iam.DetachRolePolicyInput{
PolicyArn: policy.PolicyArn, RoleName: role.RoleName,
})
if err != nil {
log.Panicf("Couldn't detach policy %v from role %v. Here's why: %v\\n",
\*policy.PolicyArn, \*role.RoleName, err)
}
}
\_, err = iamClient.DeleteRole(ctx, &amp;&amp;iam.DeleteRoleInput{RoleName: role.RoleName})
if err != nil {
log.Panicf("Couldn't delete role %v. Here's why: %v\\n", \*role.RoleName, err)
}
log.Printf("Deleted role %v.\\n", \*role.RoleName)
scenario.functionWrapper.DeleteFunction(ctx, funcName)
log.Printf("Deleted function %v.\\n", funcName)
} else {
log.Println("Okay. Don't forget to delete the resources when you're done with them.")
}
}
// IScenarioHelper abstracts I/O and wait functions from a scenario so that they
// can be mocked for unit testing.
type IScenarioHelper interface {
Pause(secs int)
CreateDeploymentPackage(sourceFile string, destinationFile string) \*bytes.Buffer
}
// ScenarioHelper lets the caller specify the path to Lambda handler functions.
type ScenarioHelper struct {
HandlerPath string
}
// Pause waits for the specified number of seconds.
func (helper \*ScenarioHelper) Pause(secs int) {
time.Sleep(time.Duration(secs) \* time.Second)
}
// CreateDeploymentPackage creates an AWS Lambda deployment package from a source file. The
// deployment package is stored in .zip format in a bytes.Buffer. The buffer can be
// used to pass a []byte to Lambda when creating the function.
// The specified destinationFile is the name to give the file when it's deployed to Lambda.
func (helper \*ScenarioHelper) CreateDeploymentPackage(sourceFile string, destinationFile string) \*bytes.Buffer {
var err error
buffer := &amp;bytes.Buffer{}
writer := zip.NewWriter(buffer)
zFile, err := writer.Create(destinationFile)
if err != nil {
log.Panicf("Couldn't create destination archive %v. Here's why: %v\\n", destinationFile, err)
}
sourceBody, err := os.ReadFile(fmt.Sprintf("%v/%v", helper.HandlerPath, sourceFile))
if err != nil {
log.Panicf("Couldn't read handler source file %v. Here's why: %v\\n",
sourceFile, err)
} else {
\_, err = zFile.Write(sourceBody)
if err != nil {
log.Panicf("Couldn't write handler %v to zip archive. Here's why: %v\\n",
sourceFile, err)
}
}
err = writer.Close()
if err != nil {
log.Panicf("Couldn't close zip writer. Here's why: %v\\n", err)
}
return buffer
}
`
```
Create a struct that wraps individual Lambda actions.
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
// DeleteFunction deletes the Lambda function specified by functionName.
func (wrapper FunctionWrapper) DeleteFunction(ctx context.Context, functionName string) {
\_, err := wrapper.LambdaClient.DeleteFunction(ctx, &amp;&amp;lambda.DeleteFunctionInput{
FunctionName: aws.String(functionName),
})
if err != nil {
log.Panicf("Couldn't delete function %v. Here's why: %v\\n", functionName, err)
}
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
// IncrementParameters is used to serialize parameters to the increment Lambda handler.
type IncrementParameters struct {
Action string `json:"action"`
Number int `json:"number"`
}
// CalculatorParameters is used to serialize parameters to the calculator Lambda handler.
type CalculatorParameters struct {
Action string `json:"action"`
X int `json:"x"`
Y int `json:"y"`
}
// LambdaResultInt is used to deserialize an int result from a Lambda handler.
type LambdaResultInt struct {
Result int `json:"result"`
}
// LambdaResultFloat is used to deserialize a float32 result from a Lambda handler.
type LambdaResultFloat struct {
Result float32 `json:"result"`
}
`
```
Define a Lambda handler that increments a number.
```
`import logging
logger = logging.getLogger()
logger.setLevel(logging.INFO)
def lambda\_handler(event, context):
"""
Accepts an action and a single number, performs the specified action on the number,
and returns the result. The only allowable action is 'increment'.
:param event: The event dict that contains the parameters sent when the function
is invoked.
:param context: The context in which the function is called.
:return: The result of the action.
"""
result = None
action = event.get("action")
if action == "increment":
result = event.get("number", 0) + 1
logger.info("Calculated result of %s", result)
else:
logger.error("%s is not a valid action.", action)
response = {"result": result}
return response
`
```
Define a second Lambda handler that performs arithmetic operations.
```
`import logging
import os
logger = logging.getLogger()
# Define a list of Python lambda functions that are called by this AWS Lambda function.
ACTIONS = {
"plus": lambda x, y: x + y,
"minus": lambda x, y: x - y,
"times": lambda x, y: x \* y,
"divided-by": lambda x, y: x / y,
}
def lambda\_handler(event, context):
"""
Accepts an action and two numbers, performs the specified action on the numbers,
and returns the result.
:param event: The event dict that contains the parameters sent when the function
is invoked.
:param context: The context in which the function is called.
:return: The result of the specified action.
"""
# Set the log level based on a variable configured in the Lambda environment.
logger.setLevel(os.environ.get("LOG\_LEVEL", logging.INFO))
logger.debug("Event: %s", event)
action = event.get("action")
func = ACTIONS.get(action)
x = event.get("x")
y = event.get("y")
result = None
try:
if func is not None and x is not None and y is not None:
result = func(x, y)
logger.info("%s %s %s is %s", x, action, y, result)
else:
logger.error("I can't calculate %s %s %s.", x, action, y)
except ZeroDivisionError:
logger.warning("I can't divide %s by 0!", x)
response = {"result": result}
return response
`
```
* For API details, see the following topics in *AWS SDK for Go API Reference*.
* [CreateFunction](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.CreateFunction)
* [DeleteFunction](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.DeleteFunction)
* [GetFunction](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.GetFunction)
* [Invoke](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.Invoke)
* [ListFunctions](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.ListFunctions)
* [UpdateFunctionCode](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/lambda#Client.UpdateFunctionConfiguration)
Java
**SDK for Java 2.x**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/example_code/lambda#code-examples).
```
`/\*
\* Lambda function names appear as:
\*
\* arn:aws:lambda:us-west-2:335556666777:function:HelloFunction
\*
\* To find this value, look at the function in the AWS Management Console.
\*
\* Before running this Java code example, set up your development environment, including your credentials.
\*
\* For more information, see this documentation topic:
\*
\* https://docs.aws.amazon.com/sdk-for-java/latest/developer-guide/get-started.html
\*
\* This example performs the following tasks:
\*
\* 1. Creates an AWS Lambda function.
\* 2. Gets a specific AWS Lambda function.
\* 3. Lists all Lambda functions.
\* 4. Invokes a Lambda function.
\* 5. Updates the Lambda function code and invokes it again.
\* 6. Updates a Lambda function's configuration value.
\* 7. Deletes a Lambda function.
\*/
public class LambdaScenario {
public static final String DASHES = new String(new char[80]).replace("\\0", "-");
public static void main(String[] args) throws InterruptedException {
final String usage = """
Usage:
&lt;&lt;functionName&gt;&gt; &lt;&lt;role&gt;&gt; &lt;&lt;handler&gt;&gt; &lt;&lt;bucketName&gt;&gt; &lt;&lt;key&gt;&gt;\\s
Where:
functionName - The name of the Lambda function.\\s
role - The AWS Identity and Access Management (IAM) service role that has Lambda permissions.\\s
handler - The fully qualified method name (for example, example.Handler::handleRequest).\\s
bucketName - The Amazon Simple Storage Service (Amazon S3) bucket name that contains the .zip or .jar used to update the Lambda function's code.\\s
key - The Amazon S3 key name that represents the .zip or .jar (for example, LambdaHello-1.0-SNAPSHOT.jar).
""";
if (args.length != 5) {
System.out.println(usage);
return;
}
String functionName = args[0];
String role = args[1];
String handler = args[2];
String bucketName = args[3];
String key = args[4];
LambdaClient awsLambda = LambdaClient.builder()
.build();
System.out.println(DASHES);
System.out.println("Welcome to the AWS Lambda Basics scenario.");
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("1. Create an AWS Lambda function.");
String funArn = createLambdaFunction(awsLambda, functionName, key, bucketName, role, handler);
System.out.println("The AWS Lambda ARN is " + funArn);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("2. Get the " + functionName + " AWS Lambda function.");
getFunction(awsLambda, functionName);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("3. List all AWS Lambda functions.");
listFunctions(awsLambda);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("4. Invoke the Lambda function.");
System.out.println("\*\*\* Sleep for 1 min to get Lambda function ready.");
Thread.sleep(60000);
invokeFunction(awsLambda, functionName);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("5. Update the Lambda function code and invoke it again.");
updateFunctionCode(awsLambda, functionName, bucketName, key);
System.out.println("\*\*\* Sleep for 1 min to get Lambda function ready.");
Thread.sleep(60000);
invokeFunction(awsLambda, functionName);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("6. Update a Lambda function's configuration value.");
updateFunctionConfiguration(awsLambda, functionName, handler);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("7. Delete the AWS Lambda function.");
LambdaScenario.deleteLambdaFunction(awsLambda, functionName);
System.out.println(DASHES);
System.out.println(DASHES);
System.out.println("The AWS Lambda scenario completed successfully");
System.out.println(DASHES);
awsLambda.close();
}
/\*\*
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
/\*\*
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
/\*\*
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
/\*\*
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
/\*\*
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
/\*\*
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
/\*\*
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
}
`
```
* For API details, see the following topics in *AWS SDK for Java 2.x API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/CreateFunction)
* [DeleteFunction](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/DeleteFunction)
* [GetFunction](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/GetFunction)
* [Invoke](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/Invoke)
* [ListFunctions](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/ListFunctions)
* [UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForJavaV2/lambda-2015-03-31/UpdateFunctionConfiguration)
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/lambda/scenarios/basic#code-examples).
Create an AWS Identity and Access Management (IAM) role that grants Lambda permission to write to logs.
```
` logger.log(`Creating role (${NAME\_ROLE\_LAMBDA})...`);
const response = await createRole(NAME\_ROLE\_LAMBDA);
import { AttachRolePolicyCommand, IAMClient } from "@aws-sdk/client-iam";
const client = new IAMClient({});
/\*\*
\*
\* @param {string} policyArn
\* @param {string} roleName
\*/
export const attachRolePolicy = (policyArn, roleName) =&gt;&gt; {
const command = new AttachRolePolicyCommand({
PolicyArn: policyArn,
RoleName: roleName,
});
return client.send(command);
};
`
```
Create a Lambda function and upload handler code.
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
Invoke the function with a single parameter and get results.
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
Update the function code and configure its Lambda environment with an environment variable.
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
const updateFunctionConfiguration = (funcName) =&gt; {
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
List the functions for your account.
```
`const listFunctions = () =&gt; {
const client = new LambdaClient({});
const command = new ListFunctionsCommand({});
return client.send(command);
};
`
```
Delete the IAM role and the Lambda function.
```
`import { DeleteRoleCommand, IAMClient } from "@aws-sdk/client-iam";
const client = new IAMClient({});
/\*\*
\*
\* @param {string} roleName
\*/
export const deleteRole = (roleName) =&gt;&gt; {
const command = new DeleteRoleCommand({ RoleName: roleName });
return client.send(command);
};
/\*\*
\* @param {string} funcName
\*/
const deleteFunction = (funcName) =&gt;&gt; {
const client = new LambdaClient({});
const command = new DeleteFunctionCommand({ FunctionName: funcName });
return client.send(command);
};
`
```
* For API details, see the following topics in *AWS SDK for JavaScript API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/CreateFunctionCommand)
* [DeleteFunction](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/DeleteFunctionCommand)
* [GetFunction](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/GetFunctionCommand)
* [Invoke](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/InvokeCommand)
* [ListFunctions](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/ListFunctionsCommand)
* [UpdateFunctionCode](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/UpdateFunctionCodeCommand)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/lambda/command/UpdateFunctionConfigurationCommand)
Kotlin
**SDK for Kotlin**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/kotlin/services/lambda#code-examples).
```
`suspend fun main(args: Array&lt;String&gt;) {
val usage = """
Usage:
&lt;functionName&gt; &lt;role&gt; &lt;handler&gt; &lt;bucketName&gt; &lt;updatedBucketName&gt; &lt;key&gt;
Where:
functionName - The name of the AWS Lambda function.
role - The AWS Identity and Access Management (IAM) service role that has AWS Lambda permissions.
handler - The fully qualified method name (for example, example.Handler::handleRequest).
bucketName - The Amazon Simple Storage Service (Amazon S3) bucket name that contains the ZIP or JAR used for the Lambda function's code.
updatedBucketName - The Amazon S3 bucket name that contains the .zip or .jar used to update the Lambda function's code.
key - The Amazon S3 key name that represents the .zip or .jar file (for example, LambdaHello-1.0-SNAPSHOT.jar).
"""
if (args.size != 6) {
println(usage)
exitProcess(1)
}
val functionName = args[0]
val role = args[1]
val handler = args[2]
val bucketName = args[3]
val updatedBucketName = args[4]
val key = args[5]
println("Creating a Lambda function named $functionName.")
val funArn = createScFunction(functionName, bucketName, key, handler, role)
println("The AWS Lambda ARN is $funArn")
// Get a specific Lambda function.
println("Getting the $functionName AWS Lambda function.")
getFunction(functionName)
// List the Lambda functions.
println("Listing all AWS Lambda functions.")
listFunctionsSc()
// Invoke the Lambda function.
println("\*\*\* Invoke the Lambda function.")
invokeFunctionSc(functionName)
// Update the AWS Lambda function code.
println("\*\*\* Update the Lambda function code.")
updateFunctionCode(functionName, updatedBucketName, key)
// println("\*\*\* Invoke the function again after updating the code.")
invokeFunctionSc(functionName)
// Update the AWS Lambda function configuration.
println("Update the run time of the function.")
updateFunctionConfiguration(functionName, handler)
// Delete the AWS Lambda function.
println("Delete the AWS Lambda function.")
delFunction(functionName)
}
suspend fun createScFunction(
myFunctionName: String,
s3BucketName: String,
myS3Key: String,
myHandler: String,
myRole: String,
): String {
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
// Create a Lambda function using a waiter
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
val functionResponse = awsLambda.createFunction(request)
awsLambda.waitUntilFunctionActive {
functionName = myFunctionName
}
return functionResponse.functionArn.toString()
}
}
suspend fun getFunction(functionNameVal: String) {
val functionRequest =
GetFunctionRequest {
functionName = functionNameVal
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
val response = awsLambda.getFunction(functionRequest)
println("The runtime of this Lambda function is ${response.configuration?.runtime}")
}
}
suspend fun listFunctionsSc() {
val request =
ListFunctionsRequest {
maxItems = 10
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
val response = awsLambda.listFunctions(request)
response.functions?.forEach { function -&gt;
println("The function name is ${function.functionName}")
}
}
}
suspend fun invokeFunctionSc(functionNameVal: String) {
val json = """{"inputValue":"1000"}"""
val byteArray = json.trimIndent().encodeToByteArray()
val request =
InvokeRequest {
functionName = functionNameVal
payload = byteArray
logType = LogType.Tail
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
val res = awsLambda.invoke(request)
println("The function payload is ${res.payload?.toString(Charsets.UTF\_8)}")
}
}
suspend fun updateFunctionCode(
functionNameVal: String?,
bucketName: String?,
key: String?,
) {
val functionCodeRequest =
UpdateFunctionCodeRequest {
functionName = functionNameVal
publish = true
s3Bucket = bucketName
s3Key = key
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
val response = awsLambda.updateFunctionCode(functionCodeRequest)
awsLambda.waitUntilFunctionUpdated {
functionName = functionNameVal
}
println("The last modified value is " + response.lastModified)
}
}
suspend fun updateFunctionConfiguration(
functionNameVal: String?,
handlerVal: String?,
) {
val configurationRequest =
UpdateFunctionConfigurationRequest {
functionName = functionNameVal
handler = handlerVal
runtime = Runtime.Java17
}
LambdaClient { region = "us-east-1" }.use { awsLambda -&gt;
awsLambda.updateFunctionConfiguration(configurationRequest)
}
}
suspend fun delFunction(myFunctionName: String) {
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
* For API details, see the following topics in *AWS SDK for Kotlin API reference*.
* [CreateFunction](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
* [DeleteFunction](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
* [GetFunction](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
* [Invoke](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
* [ListFunctions](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
* [UpdateFunctionCode](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
* [UpdateFunctionConfiguration](https://sdk.amazonaws.com/kotlin/api/latest/index.html)
PHP
**SDK for PHP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/php/example_code/lambda#code-examples).
```
`namespace Lambda;
use Aws\\S3\\S3Client;
use GuzzleHttp\\Psr7\\Stream;
use Iam\\IAMService;
class GettingStartedWithLambda
{
public function run()
{
echo("\\n");
echo("--------------------------------------\\n");
print("Welcome to the AWS Lambda getting started demo using PHP!\\n");
echo("--------------------------------------\\n");
$clientArgs = [
'region' =&gt;&gt; 'us-west-2',
'version' =&gt;&gt; 'latest',
'profile' =&gt;&gt; 'default',
];
$uniqid = uniqid();
$iamService = new IAMService();
$s3client = new S3Client($clientArgs);
$lambdaService = new LambdaService();
echo "First, let's create a role to run our Lambda code.\\n";
$roleName = "test-lambda-role-$uniqid";
$rolePolicyDocument = "{
\\"Version\\": \\"2012-10-17\\",
\\"Statement\\": [
{
\\"Effect\\": \\"Allow\\",
\\"Principal\\": {
\\"Service\\": \\"lambda.amazonaws.com\\"
},
\\"Action\\": \\"sts:AssumeRole\\"
}
]
}";
$role = $iamService-&gt;&gt;createRole($roleName, $rolePolicyDocument);
echo "Created role {$role['RoleName']}.\\n";
$iamService-&gt;&gt;attachRolePolicy(
$role['RoleName'],
"arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
);
echo "Attached the AWSLambdaBasicExecutionRole to {$role['RoleName']}.\\n";
echo "\\nNow let's create an S3 bucket and upload our Lambda code there.\\n";
$bucketName = "amzn-s3-demo-bucket-$uniqid";
$s3client-&gt;&gt;createBucket([
'Bucket' =&gt;&gt; $bucketName,
]);
echo "Created bucket $bucketName.\\n";
$functionName = "doc\_example\_lambda\_$uniqid";
$codeBasic = \_\_DIR\_\_ . "/lambda\_handler\_basic.zip";
$handler = "lambda\_handler\_basic";
$file = file\_get\_contents($codeBasic);
$s3client-&gt;&gt;putObject([
'Bucket' =&gt;&gt; $bucketName,
'Key' =&gt;&gt; $functionName,
'Body' =&gt;&gt; $file,
]);
echo "Uploaded the Lambda code.\\n";
$createLambdaFunction = $lambdaService-&gt;&gt;createFunction($functionName, $role, $bucketName, $handler);
// Wait until the function has finished being created.
do {
$getLambdaFunction = $lambdaService-&gt;getFunction($createLambdaFunction['FunctionName']);
} while ($getLambdaFunction['Configuration']['State'] == "Pending");
echo "Created Lambda function {$getLambdaFunction['Configuration']['FunctionName']}.\\n";
sleep(1);
echo "\\nOk, let's invoke that Lambda code.\\n";
$basicParams = [
'action' =&gt;&gt; 'increment',
'number' =&gt;&gt; 3,
];
/\*\* @var Stream $invokeFunction \*/
$invokeFunction = $lambdaService-&gt;&gt;invoke($functionName, $basicParams)['Payload'];
$result = json\_decode($invokeFunction-&gt;&gt;getContents())-&gt;&gt;result;
echo "After invoking the Lambda code with the input of {$basicParams['number']} we received $result.\\n";
echo "\\nSince that's working, let's update the Lambda code.\\n";
$codeCalculator = "lambda\_handler\_calculator.zip";
$handlerCalculator = "lambda\_handler\_calculator";
echo "First, put the new code into the S3 bucket.\\n";
$file = file\_get\_contents($codeCalculator);
$s3client-&gt;&gt;putObject([
'Bucket' =&gt;&gt; $bucketName,
'Key' =&gt;&gt; $functionName,
'Body' =&gt;&gt; $file,
]);
echo "New code uploaded.\\n";
$lambdaService-&gt;&gt;updateFunctionCode($functionName, $bucketName, $functionName);
// Wait for the Lambda code to finish updating.
do {
$getLambdaFunction = $lambdaService-&gt;&gt;getFunction($createLambdaFunction['FunctionName']);
} while ($getLambdaFunction['Configuration']['LastUpdateStatus'] !== "Successful");
echo "New Lambda code uploaded.\\n";
$environment = [
'Variable' =&gt;&gt; ['Variables' =&gt;&gt; ['LOG\_LEVEL' =&gt;&gt; 'DEBUG']],
];
$lambdaService-&gt;&gt;updateFunctionConfiguration($functionName, $handlerCalculator, $environment);
do {
$getLambdaFunction = $lambdaService-&gt;&gt;getFunction($createLambdaFunction['FunctionName']);
} while ($getLambdaFunction['Configuration']['LastUpdateStatus'] !== "Successful");
echo "Lambda code updated with new handler and a LOG\_LEVEL of DEBUG for more information.\\n";
echo "Invoke the new code with some new data.\\n";
$calculatorParams = [
'action' =&gt;&gt; 'plus',
'x' =&gt;&gt; 5,
'y' =&gt;&gt; 4,
];
$invokeFunction = $lambdaService-&gt;&gt;invoke($functionName, $calculatorParams, "Tail");
$result = json\_decode($invokeFunction['Payload']-&gt;&gt;getContents())-&gt;&gt;result;
echo "Indeed, {$calculatorParams['x']} + {$calculatorParams['y']} does equal $result.\\n";
echo "Here's the extra debug info: ";
echo base64\_decode($invokeFunction['LogResult']) . "\\n";
echo "\\nBut what happens if you try to divide by zero?\\n";
$divZeroParams = [
'action' =&gt;&gt; 'divide',
'x' =&gt;&gt; 5,
'y' =&gt;&gt; 0,
];
$invokeFunction = $lambdaService-&gt;&gt;invoke($functionName, $divZeroParams, "Tail");
$result = json\_decode($invokeFunction['Payload']-&gt;&gt;getContents())-&gt;&gt;result;
echo "You get a |$result| result.\\n";
echo "And an error message: ";
echo base64\_decode($invokeFunction['LogResult']) . "\\n";
echo "\\nHere's all the Lambda functions you have in this Region:\\n";
$listLambdaFunctions = $lambdaService-&gt;&gt;listFunctions(5);
$allLambdaFunctions = $listLambdaFunctions['Functions'];
$next = $listLambdaFunctions-&gt;&gt;get('NextMarker');
while ($next != false) {
$listLambdaFunctions = $lambdaService-&gt;&gt;listFunctions(5, $next);
$next = $listLambdaFunctions-&gt;&gt;get('NextMarker');
$allLambdaFunctions = array\_merge($allLambdaFunctions, $listLambdaFunctions['Functions']);
}
foreach ($allLambdaFunctions as $function) {
echo "{$function['FunctionName']}\\n";
}
echo "\\n\\nAnd don't forget to clean up your data!\\n";
$lambdaService-&gt;&gt;deleteFunction($functionName);
echo "Deleted Lambda function.\\n";
$iamService-&gt;&gt;deleteRole($role['RoleName']);
echo "Deleted Role.\\n";
$deleteObjects = $s3client-&gt;&gt;listObjectsV2([
'Bucket' =&gt;&gt; $bucketName,
]);
$deleteObjects = $s3client-&gt;&gt;deleteObjects([
'Bucket' =&gt;&gt; $bucketName,
'Delete' =&gt;&gt; [
'Objects' =&gt;&gt; $deleteObjects['Contents'],
]
]);
echo "Deleted all objects from the S3 bucket.\\n";
$s3client-&gt;&gt;deleteBucket(['Bucket' =&gt;&gt; $bucketName]);
echo "Deleted the bucket.\\n";
}
}
`
```
* For API details, see the following topics in *AWS SDK for PHP API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/CreateFunction)
* [DeleteFunction](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/DeleteFunction)
* [GetFunction](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/GetFunction)
* [Invoke](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/Invoke)
* [ListFunctions](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/ListFunctions)
* [UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForPHPV3/lambda-2015-03-31/UpdateFunctionConfiguration)
Python
**SDK for Python (Boto3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/python/example_code/lambda#code-examples).
Define a Lambda handler that increments a number.
```
`import logging
logger = logging.getLogger()
logger.setLevel(logging.INFO)
def lambda\_handler(event, context):
"""
Accepts an action and a single number, performs the specified action on the number,
and returns the result. The only allowable action is 'increment'.
:param event: The event dict that contains the parameters sent when the function
is invoked.
:param context: The context in which the function is called.
:return: The result of the action.
"""
result = None
action = event.get("action")
if action == "increment":
result = event.get("number", 0) + 1
logger.info("Calculated result of %s", result)
else:
logger.error("%s is not a valid action.", action)
response = {"result": result}
return response
`
```
Define a second Lambda handler that performs arithmetic operations.
```
`import logging
import os
logger = logging.getLogger()
# Define a list of Python lambda functions that are called by this AWS Lambda function.
ACTIONS = {
"plus": lambda x, y: x + y,
"minus": lambda x, y: x - y,
"times": lambda x, y: x \* y,
"divided-by": lambda x, y: x / y,
}
def lambda\_handler(event, context):
"""
Accepts an action and two numbers, performs the specified action on the numbers,
and returns the result.
:param event: The event dict that contains the parameters sent when the function
is invoked.
:param context: The context in which the function is called.
:return: The result of the specified action.
"""
# Set the log level based on a variable configured in the Lambda environment.
logger.setLevel(os.environ.get("LOG\_LEVEL", logging.INFO))
logger.debug("Event: %s", event)
action = event.get("action")
func = ACTIONS.get(action)
x = event.get("x")
y = event.get("y")
result = None
try:
if func is not None and x is not None and y is not None:
result = func(x, y)
logger.info("%s %s %s is %s", x, action, y, result)
else:
logger.error("I can't calculate %s %s %s.", x, action, y)
except ZeroDivisionError:
logger.warning("I can't divide %s by 0!", x)
response = {"result": result}
return response
`
```
Create functions that wrap Lambda actions.
```
`class LambdaWrapper:
def \_\_init\_\_(self, lambda\_client, iam\_resource):
self.lambda\_client = lambda\_client
self.iam\_resource = iam\_resource
@staticmethod
def create\_deployment\_package(source\_file, destination\_file):
"""
Creates a Lambda deployment package in .zip format in an in-memory buffer. This
buffer can be passed directly to Lambda when creating the function.
:param source\_file: The name of the file that contains the Lambda handler
function.
:param destination\_file: The name to give the file when it's deployed to Lambda.
:return: The deployment package.
"""
buffer = io.BytesIO()
with zipfile.ZipFile(buffer, "w") as zipped:
zipped.write(source\_file, destination\_file)
buffer.seek(0)
return buffer.read()
def get\_iam\_role(self, iam\_role\_name):
"""
Get an AWS Identity and Access Management (IAM) role.
:param iam\_role\_name: The name of the role to retrieve.
:return: The IAM role.
"""
role = None
try:
temp\_role = self.iam\_resource.Role(iam\_role\_name)
temp\_role.load()
role = temp\_role
logger.info("Got IAM role %s", role.name)
except ClientError as err:
if err.response["Error"]["Code"] == "NoSuchEntity":
logger.info("IAM role %s does not exist.", iam\_role\_name)
else:
logger.error(
"Couldn't get IAM role %s. Here's why: %s: %s",
iam\_role\_name,
err.response["Error"]["Code"],
err.response["Error"]["Message"],
)
raise
return role
def create\_iam\_role\_for\_lambda(self, iam\_role\_name):
"""
Creates an IAM role that grants the Lambda function basic permissions. If a
role with the specified name already exists, it is used for the demo.
:param iam\_role\_name: The name of the role to create.
:return: The role and a value that indicates whether the role is newly created.
"""
role = self.get\_iam\_role(iam\_role\_name)
if role is not None:
return role, False
lambda\_assume\_role\_policy = {
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {"Service": "lambda.amazonaws.com"},
"Action": "sts:AssumeRole",
}
],
}
policy\_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
try:
role = self.iam\_resource.create\_role(
RoleName=iam\_role\_name,
AssumeRolePolicyDocument=json.dumps(lambda\_assume\_role\_policy),
)
logger.info("Created role %s.", role.name)
role.attach\_policy(PolicyArn=policy\_arn)
logger.info("Attached basic execution policy to role %s.", role.name)
except ClientError as error:
if error.response["Error"]["Code"] == "EntityAlreadyExists":
role = self.iam\_resource.Role(iam\_role\_name)
logger.warning("The role %s already exists. Using it.", iam\_role\_name)
else:
logger.exception(
"Couldn't create role %s or attach policy %s.",
iam\_role\_name,
policy\_arn,
)
raise
return role, True
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
Create a function that runs the scenario.
```
`class UpdateFunctionWaiter(CustomWaiter):
"""A custom waiter that waits until a function is successfully updated."""
def \_\_init\_\_(self, client):
super().\_\_init\_\_(
"UpdateSuccess",
"GetFunction",
"Configuration.LastUpdateStatus",
{"Successful": WaitState.SUCCESS, "Failed": WaitState.FAILURE},
client,
)
def wait(self, function\_name):
self.\_wait(FunctionName=function\_name)
def run\_scenario(lambda\_client, iam\_resource, basic\_file, calculator\_file, lambda\_name):
"""
Runs the scenario.
:param lambda\_client: A Boto3 Lambda client.
:param iam\_resource: A Boto3 IAM resource.
:param basic\_file: The name of the file that contains the basic Lambda handler.
:param calculator\_file: The name of the file that contains the calculator Lambda handler.
:param lambda\_name: The name to give resources created for the scenario, such as the
IAM role and the Lambda function.
"""
logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
print("-" \* 88)
print("Welcome to the AWS Lambda getting started with functions demo.")
print("-" \* 88)
wrapper = LambdaWrapper(lambda\_client, iam\_resource)
print("Checking for IAM role for Lambda...")
iam\_role, should\_wait = wrapper.create\_iam\_role\_for\_lambda(lambda\_name)
if should\_wait:
logger.info("Giving AWS time to create resources...")
wait(10)
print(f"Looking for function {lambda\_name}...")
function = wrapper.get\_function(lambda\_name)
if function is None:
print("Zipping the Python script into a deployment package...")
deployment\_package = wrapper.create\_deployment\_package(
basic\_file, f"{lambda\_name}.py"
)
print(f"...and creating the {lambda\_name} Lambda function.")
wrapper.create\_function(
lambda\_name, f"{lambda\_name}.lambda\_handler", iam\_role, deployment\_package
)
else:
print(f"Function {lambda\_name} already exists.")
print("-" \* 88)
print(f"Let's invoke {lambda\_name}. This function increments a number.")
action\_params = {
"action": "increment",
"number": q.ask("Give me a number to increment: ", q.is\_int),
}
print(f"Invoking {lambda\_name}...")
response = wrapper.invoke\_function(lambda\_name, action\_params)
print(
f"Incrementing {action\_params['number']} resulted in "
f"{json.load(response['Payload'])}"
)
print("-" \* 88)
print(f"Let's update the function to an arithmetic calculator.")
q.ask("Press Enter when you're ready.")
print("Creating a new deployment package...")
deployment\_package = wrapper.create\_deployment\_package(
calculator\_file, f"{lambda\_name}.py"
)
print(f"...and updating the {lambda\_name} Lambda function.")
update\_waiter = UpdateFunctionWaiter(lambda\_client)
wrapper.update\_function\_code(lambda\_name, deployment\_package)
update\_waiter.wait(lambda\_name)
print(f"This function uses an environment variable to control logging level.")
print(f"Let's set it to DEBUG to get the most logging.")
wrapper.update\_function\_configuration(
lambda\_name, {"LOG\_LEVEL": logging.getLevelName(logging.DEBUG)}
)
actions = ["plus", "minus", "times", "divided-by"]
want\_invoke = True
while want\_invoke:
print(f"Let's invoke {lambda\_name}. You can invoke these actions:")
for index, action in enumerate(actions):
print(f"{index + 1}: {action}")
action\_params = {}
action\_index = q.ask(
"Enter the number of the action you want to take: ",
q.is\_int,
q.in\_range(1, len(actions)),
)
action\_params["action"] = actions[action\_index - 1]
print(f"You've chosen to invoke 'x {action\_params['action']} y'.")
action\_params["x"] = q.ask("Enter a value for x: ", q.is\_int)
action\_params["y"] = q.ask("Enter a value for y: ", q.is\_int)
print(f"Invoking {lambda\_name}...")
response = wrapper.invoke\_function(lambda\_name, action\_params, True)
print(
f"Calculating {action\_params['x']} {action\_params['action']} {action\_params['y']} "
f"resulted in {json.load(response['Payload'])}"
)
q.ask("Press Enter to see the logs from the call.")
print(base64.b64decode(response["LogResult"]).decode())
want\_invoke = q.ask("That was fun. Shall we do it again? (y/n) ", q.is\_yesno)
print("-" \* 88)
if q.ask(
"Do you want to list all of the functions in your account? (y/n) ", q.is\_yesno
):
wrapper.list\_functions()
print("-" \* 88)
if q.ask("Ready to delete the function and role? (y/n) ", q.is\_yesno):
for policy in iam\_role.attached\_policies.all():
policy.detach\_role(RoleName=iam\_role.name)
iam\_role.delete()
print(f"Deleted role {lambda\_name}.")
wrapper.delete\_function(lambda\_name)
print(f"Deleted function {lambda\_name}.")
print("\\nThanks for watching!")
print("-" \* 88)
if \_\_name\_\_ == "\_\_main\_\_":
try:
run\_scenario(
boto3.client("lambda"),
boto3.resource("iam"),
"lambda\_handler\_basic.py",
"lambda\_handler\_calculator.py",
"doc\_example\_lambda\_calculator",
)
except Exception:
logging.exception("Something went wrong with the demo!")
`
```
* For API details, see the following topics in *AWS SDK for Python (Boto3) API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/CreateFunction)
* [DeleteFunction](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/DeleteFunction)
* [GetFunction](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/GetFunction)
* [Invoke](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/Invoke)
* [ListFunctions](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/ListFunctions)
* [UpdateFunctionCode](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/boto3/lambda-2015-03-31/UpdateFunctionConfiguration)
Ruby
**SDK for Ruby**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/ruby/example_code/lambda#code-examples).
Set up pre-requisite IAM permissions for a Lambda function capable of writing logs.
```
` # Get an AWS Identity and Access Management (IAM) role.
# @param iam\_role\_name: The name of the role to retrieve.
# @param action: Whether to create or destroy the IAM apparatus.
# @return: The IAM role.
def manage\_iam(iam\_role\_name, action)
case action
when 'create'
create\_iam\_role(iam\_role\_name)
when 'destroy'
destroy\_iam\_role(iam\_role\_name)
else
raise "Incorrect action provided. Must provide 'create' or 'destroy'"
end
end
private
def create\_iam\_role(iam\_role\_name)
role\_policy = {
'Version': '2012-10-17',
'Statement': [
{
'Effect': 'Allow',
'Principal': { 'Service': 'lambda.amazonaws.com' },
'Action': 'sts:AssumeRole'
}
]
}
role = @iam\_client.create\_role(
role\_name: iam\_role\_name,
assume\_role\_policy\_document: role\_policy.to\_json
)
@iam\_client.attach\_role\_policy(
{
policy\_arn: 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole',
role\_name: iam\_role\_name
}
)
wait\_for\_role\_to\_exist(iam\_role\_name)
@logger.debug("Successfully created IAM role: #{role['role']['arn']}")
sleep(10)
[role, role\_policy.to\_json]
end
def destroy\_iam\_role(iam\_role\_name)
@iam\_client.detach\_role\_policy(
{
policy\_arn: 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole',
role\_name: iam\_role\_name
}
)
@iam\_client.delete\_role(role\_name: iam\_role\_name)
@logger.debug("Detached policy &amp;&amp; deleted IAM role: #{iam\_role\_name}")
end
def wait\_for\_role\_to\_exist(iam\_role\_name)
@iam\_client.wait\_until(:role\_exists, { role\_name: iam\_role\_name }) do |w|
w.max\_attempts = 5
w.delay = 5
end
end
`
```
Define a Lambda handler that increments a number provided as an invocation parameter.
```
`require 'logger'
# A function that increments a whole number by one (1) and logs the result.
# Requires a manually-provided runtime parameter, 'number', which must be Int
# @param event [Hash] Parameters sent when the function is invoked
# @param context [Hash] Methods and properties that provide information
# @return incremented\_number [String] The incremented number.
def lambda\_handler(event:, context:)
logger = Logger.new($stdout)
log\_level = ENV['LOG\_LEVEL']
logger.level = case log\_level
when 'debug'
Logger::DEBUG
when 'info'
Logger::INFO
else
Logger::ERROR
end
logger.debug('This is a debug log message.')
logger.info('This is an info log message. Code executed successfully!')
number = event['number'].to\_i
incremented\_number = number + 1
logger.info("You provided #{number.round} and it was incremented to #{incremented\_number.round}")
incremented\_number.round.to\_s
end
`
```
Zip your Lambda function into a deployment package.
```
` # Creates a Lambda deployment package in .zip format.
# @param source\_file: The name of the object, without suffix, for the Lambda file and zip.
# @return: The deployment package.
def create\_deployment\_package(source\_file)
Dir.chdir(File.dirname(\_\_FILE\_\_))
if File.exist?('lambda\_function.zip')
File.delete('lambda\_function.zip')
@logger.debug('Deleting old zip: lambda\_function.zip')
end
Zip::File.open('lambda\_function.zip', create: true) do |zipfile|
zipfile.add('lambda\_function.rb', "#{source\_file}.rb")
end
@logger.debug("Zipping #{source\_file}.rb into: lambda\_function.zip.")
File.read('lambda\_function.zip').to\_s
rescue StandardError =&gt;&gt; e
@logger.error("There was an error creating deployment package:\\n #{e.message}")
end
`
```
Create a new Lambda function.
```
` # Deploys a Lambda function.
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
Invoke your Lambda function with optional runtime parameters.
```
` # Invokes a Lambda function.
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
Update your Lambda function's configuration to inject a new environment variable.
```
` # Updates the environment variables for a Lambda function.
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
Update your Lambda function's code with a different deployment package containing different code.
```
` # Updates the code for a Lambda function by submitting a .zip archive that contains
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
List all existing Lambda functions using the built-in paginator.
```
` # Lists the Lambda functions for the current account.
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
Delete a specific Lambda function.
```
` # Deletes a Lambda function.
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
* For API details, see the following topics in *AWS SDK for Ruby API Reference*.
* [CreateFunction](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/CreateFunction)
* [DeleteFunction](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/DeleteFunction)
* [GetFunction](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/GetFunction)
* [Invoke](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/Invoke)
* [ListFunctions](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/ListFunctions)
* [UpdateFunctionCode](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/UpdateFunctionCode)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/goto/SdkForRubyV3/lambda-2015-03-31/UpdateFunctionConfiguration)
Rust
**SDK for Rust**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1/examples/lambda#code-examples).
The Cargo.toml with dependencies used in this scenario.
```
`[package]
name = "lambda-code-examples"
version = "0.1.0"
edition = "2021"
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
aws-config = { version = "1.0.1", features = ["behavior-version-latest"] }
aws-sdk-ec2 = { version = "1.3.0" }
aws-sdk-iam = { version = "1.3.0" }
aws-sdk-lambda = { version = "1.3.0" }
aws-sdk-s3 = { version = "1.4.0" }
aws-smithy-types = { version = "1.0.1" }
aws-types = { version = "1.0.1" }
clap = { version = "4.4", features = ["derive"] }
tokio = { version = "1.20.1", features = ["full"] }
tracing-subscriber = { version = "0.3.15", features = ["env-filter"] }
tracing = "0.1.37"
serde\_json = "1.0.94"
anyhow = "1.0.71"
uuid = { version = "1.3.3", features = ["v4"] }
lambda\_runtime = "0.8.0"
serde = "1.0.164"
`
```
A collection of utilities that streamline calling Lambda for this scenario. This file is src/ations.rs in the crate.
```
`
use anyhow::anyhow;
use aws\_sdk\_iam::operation::{create\_role::CreateRoleError, delete\_role::DeleteRoleOutput};
use aws\_sdk\_lambda::{
operation::{
delete\_function::DeleteFunctionOutput, get\_function::GetFunctionOutput,
invoke::InvokeOutput, list\_functions::ListFunctionsOutput,
update\_function\_code::UpdateFunctionCodeOutput,
update\_function\_configuration::UpdateFunctionConfigurationOutput,
},
primitives::ByteStream,
types::{Environment, FunctionCode, LastUpdateStatus, State},
};
use aws\_sdk\_s3::{
error::ErrorMetadata,
operation::{delete\_bucket::DeleteBucketOutput, delete\_object::DeleteObjectOutput},
types::CreateBucketConfiguration,
};
use aws\_smithy\_types::Blob;
use serde::{ser::SerializeMap, Serialize};
use std::{fmt::Display, path::PathBuf, str::FromStr, time::Duration};
use tracing::{debug, info, warn};
/\* Operation describes \*/
#[serde(rename = "divided-by")]
DividedBy,
}
impl FromStr for Operation {
type Err = anyhow::Error;
fn from\_str(s: &amp;&amp;str) -&gt;&gt; Result&lt;&lt;Self, Self::Err&gt;&gt; {
match s {
"plus" =&gt;&gt; Ok(Operation::Plus),
"minus" =&gt;&gt; Ok(Operation::Minus),
"times" =&gt;&gt; Ok(Operation::Times),
"divided-by" =&gt;&gt; Ok(Operation::DividedBy),
\_ =&gt;&gt; Err(anyhow!("Unknown operation {s}")),
}
}
}
impl Display for Operation {
fn fmt(&amp;&amp;self, f: &amp;&amp;mut std::fmt::Formatter&lt;&lt;'\_&gt;&gt;) -&gt;&gt; std::fmt::Result {
match self {
Operation::Plus =&gt;&gt; write!(f, "plus"),
Operation::Minus =&gt;&gt; write!(f, "minus"),
Operation::Times =&gt;&gt; write!(f, "times"),
Operation::DividedBy =&gt;&gt; write!(f, "divided-by"),
}
}
}
/\*\*
\* InvokeArgs will be serialized as JSON and sent to the AWS Lambda handler.
\*/
#[derive(Debug)]
pub enum InvokeArgs {
Increment(i32),
Arithmetic(Operation, i32, i32),
}
impl Serialize for InvokeArgs {
fn serialize&lt;S&gt;(&amp;self, serializer: S) -&gt; Result&lt;S::Ok, S::Error&gt;
where
S: serde::Serializer,
{
match self {
InvokeArgs::Increment(i) =&gt;&gt; serializer.serialize\_i32(\*i),
InvokeArgs::Arithmetic(o, i, j) =&gt;&gt; {
let mut map: S::SerializeMap = serializer.serialize\_map(Some(3))?;
map.serialize\_key(&amp;&amp;"op".to\_string())?;
map.serialize\_value(&amp;&amp;o.to\_string())?;
map.serialize\_key(&amp;&amp;"i".to\_string())?;
map.serialize\_value(&amp;&amp;i)?;
map.serialize\_key(&amp;&amp;"j".to\_string())?;
map.serialize\_value(&amp;&amp;j)?;
map.end()
}
}
}
}
/\*\* A policy document allowing Lambda to execute this function on the account's behalf. \*/
const ROLE\_POLICY\_DOCUMENT: &amp;&amp;str = r#"{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": { "Service": "lambda.amazonaws.com" },
"Action": "sts:AssumeRole"
}
]
}"#;
/\*\*
\* A LambdaManager gathers all the resources necessary to run the Lambda example scenario.
\* This includes instantiated aws\_sdk clients and details of resource names.
\*/
pub struct LambdaManager {
iam\_client: aws\_sdk\_iam::Client,
lambda\_client: aws\_sdk\_lambda::Client,
s3\_client: aws\_sdk\_s3::Client,
lambda\_name: String,
role\_name: String,
bucket: String,
own\_bucket: bool,
}
// These unit type structs provide nominal typing on top of String parameters for LambdaManager::new
pub struct LambdaName(pub String);
pub struct RoleName(pub String);
pub struct Bucket(pub String);
pub struct OwnBucket(pub bool);
impl LambdaManager {
pub fn new(
iam\_client: aws\_sdk\_iam::Client,
lambda\_client: aws\_sdk\_lambda::Client,
s3\_client: aws\_sdk\_s3::Client,
lambda\_name: LambdaName,
role\_name: RoleName,
bucket: Bucket,
own\_bucket: OwnBucket,
) -&gt;&gt; Self {
Self {
iam\_client,
lambda\_client,
s3\_client,
lambda\_name: lambda\_name.0,
role\_name: role\_name.0,
bucket: bucket.0,
own\_bucket: own\_bucket.0,
}
}
/\*\*
\* Load the AWS configuration from the environment.
\* Look up lambda\_name and bucket if none are given, or generate a random name if not present in the environment.
\* If the bucket name is provided, the caller needs to have created the bucket.
\* If the bucket name is generated, it will be created.
\*/
pub async fn load\_from\_env(lambda\_name: Option&lt;&lt;String&gt;&gt;, bucket: Option&lt;&lt;String&gt;&gt;) -&gt;&gt; Self {
let sdk\_config = aws\_config::load\_from\_env().await;
let lambda\_name = LambdaName(lambda\_name.unwrap\_or\_else(|| {
std::env::var("LAMBDA\_NAME").unwrap\_or\_else(|\_| "rust\_lambda\_example".to\_string())
}));
let role\_name = RoleName(format!("{}\_role", lambda\_name.0));
let (bucket, own\_bucket) =
match bucket {
Some(bucket) =&gt;&gt; (Bucket(bucket), false),
None =&gt;&gt; (
Bucket(std::env::var("LAMBDA\_BUCKET").unwrap\_or\_else(|\_| {
format!("rust-lambda-example-{}", uuid::Uuid::new\_v4())
})),
true,
),
};
let s3\_client = aws\_sdk\_s3::Client::new(&amp;&amp;sdk\_config);
if own\_bucket {
info!("Creating bucket for demo: {}", bucket.0);
s3\_client
.create\_bucket()
.bucket(bucket.0.clone())
.create\_bucket\_configuration(
CreateBucketConfiguration::builder()
.location\_constraint(aws\_sdk\_s3::types::BucketLocationConstraint::from(
sdk\_config.region().unwrap().as\_ref(),
))
.build(),
)
.send()
.await
.unwrap();
}
Self::new(
aws\_sdk\_iam::Client::new(&amp;&amp;sdk\_config),
aws\_sdk\_lambda::Client::new(&amp;&amp;sdk\_config),
s3\_client,
lambda\_name,
role\_name,
bucket,
OwnBucket(own\_bucket),
)
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
/\*\*
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
\* Create an IAM execution role for the managed Lambda function.
\* If the role already exists, use that instead.
\*/
async fn create\_role(&amp;&amp;self) -&gt;&gt; Result&lt;&lt;aws\_sdk\_iam::types::Role, CreateRoleError&gt;&gt; {
info!("Creating execution role for function");
let get\_role = self
.iam\_client
.get\_role()
.role\_name(self.role\_name.clone())
.send()
.await;
if let Ok(get\_role) = get\_role {
if let Some(role) = get\_role.role {
return Ok(role);
}
}
let create\_role = self
.iam\_client
.create\_role()
.role\_name(self.role\_name.clone())
.assume\_role\_policy\_document(ROLE\_POLICY\_DOCUMENT)
.send()
.await;
match create\_role {
Ok(create\_role) =&gt;&gt; match create\_role.role {
Some(role) =&gt;&gt; Ok(role),
None =&gt;&gt; Err(CreateRoleError::generic(
ErrorMetadata::builder()
.message("CreateRole returned empty success")
.build(),
)),
},
Err(err) =&gt;&gt; Err(err.into\_service\_error()),
}
}
/\*\*
\* Poll `is\_function\_ready` with a 1-second delay. It returns when the function is ready or when there's an error checking the function's state.
\*/
pub async fn wait\_for\_function\_ready(&amp;&amp;self) -&gt;&gt; Result&lt;&lt;(), anyhow::Error&gt;&gt; {
info!("Waiting for function");
while !self.is\_function\_ready(None).await? {
info!("Function is not ready, sleeping 1s");
tokio::time::sleep(Duration::from\_secs(1)).await;
}
Ok(())
}
/\*\*
\* Check if a Lambda function is ready to be invoked.
\* A Lambda function is ready for this scenario when its state is active and its LastUpdateStatus is Successful.
\* Additionally, if a sha256 is provided, the function must have that as its current code hash.
\* Any missing properties or failed requests will be reported as an Err.
\*/
async fn is\_function\_ready(
&amp;&amp;self,
expected\_code\_sha256: Option&lt;&lt;&amp;&amp;str&gt;&gt;,
) -&gt;&gt; Result&lt;&lt;bool, anyhow::Error&gt;&gt; {
match self.get\_function().await {
Ok(func) =&gt; {
if let Some(config) = func.configuration() {
if let Some(state) = config.state() {
info!(?state, "Checking if function is active");
if !matches!(state, State::Active) {
return Ok(false);
}
}
match config.last\_update\_status() {
Some(last\_update\_status) =&gt;&gt; {
info!(?last\_update\_status, "Checking if function is ready");
match last\_update\_status {
LastUpdateStatus::Successful =&gt; {
// continue
}
LastUpdateStatus::Failed | LastUpdateStatus::InProgress =&gt; {
return Ok(false);
}
unknown =&gt; {
warn!(
status\_variant = unknown.as\_str(),
"LastUpdateStatus unknown"
);
return Err(anyhow!(
"Unknown LastUpdateStatus, fn config is {config:?}"
));
}
}
}
None =&gt; {
warn!("Missing last update status");
return Ok(false);
}
};
if expected\_code\_sha256.is\_none() {
return Ok(true);
}
if let Some(code\_sha256) = config.code\_sha256() {
return Ok(code\_sha256 == expected\_code\_sha256.unwrap\_or\_default());
}
}
}
Err(e) =&gt;&gt; {
warn!(?e, "Could not get function while waiting");
}
}
Ok(false)
}
/\*\* Get the Lambda function with this Manager's name. \*/
pub async fn get\_function(&amp;&amp;self) -&gt;&gt; Result&lt;&lt;GetFunctionOutput, anyhow::Error&gt;&gt; {
info!("Getting lambda function");
self.lambda\_client
.get\_function()
.function\_name(self.lambda\_name.clone())
.send()
.await
.map\_err(anyhow::Error::from)
}
/\*\* List all Lambda functions in the current Region. \*/
pub async fn list\_functions(&amp;&amp;self) -&gt;&gt; Result&lt;&lt;ListFunctionsOutput, anyhow::Error&gt;&gt; {
info!("Listing lambda functions");
self.lambda\_client
.list\_functions()
.send()
.await
.map\_err(anyhow::Error::from)
}
/\*\* Invoke the lambda function using calculator InvokeArgs. \*/
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
/\*\* Given a Path to a zip file, update the function's code and wait for the update to finish. \*/
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
/\*\* Update the environment for a function. \*/
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
/\*\* Delete a function and its role, and if possible or necessary, its associated code object and bucket. \*/
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
pub async fn cleanup(
&amp;&amp;self,
location: Option&lt;&lt;String&gt;&gt;,
) -&gt;&gt; (
(
Result&lt;&lt;DeleteFunctionOutput, anyhow::Error&gt;&gt;,
Result&lt;&lt;DeleteRoleOutput, anyhow::Error&gt;&gt;,
Option&lt;&lt;Result&lt;&lt;DeleteObjectOutput, anyhow::Error&gt;&gt;&gt;&gt;,
),
Option&lt;&lt;Result&lt;&lt;DeleteBucketOutput, anyhow::Error&gt;&gt;&gt;&gt;,
) {
let delete\_function = self.delete\_function(location).await;
let delete\_bucket = if self.own\_bucket {
info!("Deleting bucket {}", self.bucket);
if delete\_function.2.is\_none() || delete\_function.2.as\_ref().unwrap().is\_ok() {
Some(
self.s3\_client
.delete\_bucket()
.bucket(self.bucket.clone())
.send()
.await
.map\_err(anyhow::Error::from),
)
} else {
None
}
} else {
info!("No bucket to clean up");
None
};
(delete\_function, delete\_bucket)
}
}
/\*\*
\* Testing occurs primarily as an integration test running the `scenario` bin successfully.
\* Each action relies deeply on the internal workings and state of Amazon Simple Storage Service (Amazon S3), Lambda, and IAM working together.
\* It is therefore infeasible to mock the clients to test the individual actions.
\*/
#[cfg(test)]
mod test {
use super::{InvokeArgs, Operation};
use serde\_json::json;
/\*\* Make sure that the JSON output of serializing InvokeArgs is what's expected by the calculator. \*/
#[test]
fn test\_serialize() {
assert\_eq!(json!(InvokeArgs::Increment(5)), 5);
assert\_eq!(
json!(InvokeArgs::Arithmetic(Operation::Plus, 5, 7)).to\_string(),
r#"{"op":"plus","i":5,"j":7}"#.to\_string(),
);
}
}
`
```
A binary to run the scenario from front to end, using command line flags to control some behavior. This file is src/bin/scenario.rs in the crate.
```
`
/\*
## Service actions
Service actions wrap the SDK call, taking a client and any specific parameters necessary for the call.
\* CreateFunction
\* GetFunction
\* ListFunctions
\* Invoke
\* UpdateFunctionCode
\* UpdateFunctionConfiguration
\* DeleteFunction
## Scenario
A scenario runs at a command prompt and prints output to the user on the result of each service action. A scenario can run in one of two ways: straight through, printing out progress as it goes, or as an interactive question/answer script.
## Getting started with functions
Use an SDK to manage AWS Lambda functions: create a function, invoke it, update its code, invoke it again, view its output and logs, and delete it.
This scenario uses two Lambda handlers:
\_Note: Handlers don't use AWS SDK API calls.\_
The increment handler is straightforward:
1. It accepts a number, increments it, and returns the new value.
2. It performs simple logging of the result.
The arithmetic handler is more complex:
1. It accepts a set of actions ['plus', 'minus', 'times', 'divided-by'] and two numbers, and returns the result of the calculation.
2. It uses an environment variable to control log level (such as DEBUG, INFO, WARNING, ERROR).
It logs a few things at different levels, such as:
\* DEBUG: Full event data.
\* INFO: The calculation result.
\* WARN\~ING\~: When a divide by zero error occurs.
\* This will be the typical `RUST\_LOG` variable.
The steps of the scenario are:
1. Create an AWS Identity and Access Management (IAM) role that meets the following requirements:
\* Has an assume\_role policy that grants 'lambda.amazonaws.com' the 'sts:AssumeRole' action.
\* Attaches the 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole' managed role.
\* \_You must wait for \~10 seconds after the role is created before you can use it!\_
2. Create a function (CreateFunction) for the increment handler by packaging it as a zip and doing one of the following:
\* Adding it with CreateFunction Code.ZipFile.
\* --or--
\* Uploading it to Amazon Simple Storage Service (Amazon S3) and adding it with CreateFunction Code.S3Bucket/S3Key.
\* \_Note: Zipping the file does not have to be done in code.\_
\* If you have a waiter, use it to wait until the function is active. Otherwise, call GetFunction until State is Active.
3. Invoke the function with a number and print the result.
4. Update the function (UpdateFunctionCode) to the arithmetic handler by packaging it as a zip and doing one of the following:
\* Adding it with UpdateFunctionCode ZipFile.
\* --or--
\* Uploading it to Amazon S3 and adding it with UpdateFunctionCode S3Bucket/S3Key.
5. Call GetFunction until Configuration.LastUpdateStatus is 'Successful' (or 'Failed').
6. Update the environment variable by calling UpdateFunctionConfiguration and pass it a log level, such as:
\* Environment={'Variables': {'RUST\_LOG': 'TRACE'}}
7. Invoke the function with an action from the list and a couple of values. Include LogType='Tail' to get logs in the result. Print the result of the calculation and the log.
8. [Optional] Invoke the function to provoke a divide-by-zero error and show the log result.
9. List all functions for the account, using pagination (ListFunctions).
10. Delete the function (DeleteFunction).
11. Delete the role.
Each step should use the function created in Service Actions to abstract calling the SDK.
\*/
use aws\_sdk\_lambda::{operation::invoke::InvokeOutput, types::Environment};
use clap::Parser;
use std::{collections::HashMap, path::PathBuf};
use tracing::{debug, info, warn};
use tracing\_subscriber::EnvFilter;
use lambda\_code\_examples::actions::{
InvokeArgs::{Arithmetic, Increment},
LambdaManager, Operation,
};
#[derive(Debug, Parser)]
pub struct Opt {
/// The AWS Region.
#[structopt(short, long)]
pub region: Option&lt;&lt;String&gt;&gt;,
// The bucket to use for the FunctionCode.
#[structopt(short, long)]
pub bucket: Option&lt;&lt;String&gt;&gt;,
// The name of the Lambda function.
#[structopt(short, long)]
pub lambda\_name: Option&lt;&lt;String&gt;&gt;,
// The number to increment.
#[structopt(short, long, default\_value = "12")]
pub inc: i32,
// The left operand.
#[structopt(long, default\_value = "19")]
pub num\_a: i32,
// The right operand.
#[structopt(long, default\_value = "23")]
pub num\_b: i32,
// The arithmetic operation.
#[structopt(long)]
pub no\_cleanup: Option&lt;&lt;bool&gt;&gt;,
}
fn code\_path(lambda: &amp;&amp;str) -&gt;&gt; PathBuf {
PathBuf::from(format!("../target/lambda/{lambda}/bootstrap.zip"))
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
async fn main\_block(
opt: &amp;&amp;Opt,
manager: &amp;&amp;LambdaManager,
code\_location: String,
) -&gt;&gt; Result&lt;&lt;(), anyhow::Error&gt;&gt; {
let invoke = manager.invoke(Increment(opt.inc)).await?;
log\_invoke\_output(&amp;&amp;invoke, "Invoked function configured as increment");
let update\_code = manager
.update\_function\_code(code\_path("arithmetic"), code\_location.clone())
.await?;
let code\_sha256 = update\_code.code\_sha256().unwrap\_or("Unknown SHA");
info!(?code\_sha256, "Updated function code with arithmetic.zip");
let arithmetic\_args = Arithmetic(opt.operation, opt.num\_a, opt.num\_b);
let invoke = manager.invoke(arithmetic\_args).await?;
log\_invoke\_output(&amp;&amp;invoke, "Invoked function configured as arithmetic");
let update = manager
.update\_function\_configuration(
Environment::builder()
.set\_variables(Some(HashMap::from([(
"RUST\_LOG".to\_string(),
"trace".to\_string(),
)])))
.build(),
)
.await?;
let updated\_environment = update.environment();
info!(?updated\_environment, "Updated function configuration");
let invoke = manager
.invoke(Arithmetic(opt.operation, opt.num\_a, opt.num\_b))
.await?;
log\_invoke\_output(
&amp;&amp;invoke,
"Invoked function configured as arithmetic with increased logging",
);
let invoke = manager
.invoke(Arithmetic(Operation::DividedBy, opt.num\_a, 0))
.await?;
log\_invoke\_output(
&amp;&amp;invoke,
"Invoked function configured as arithmetic with divide by zero",
);
Ok::&lt;&lt;(), anyhow::Error&gt;&gt;(())
}
#[tokio::main]
async fn main() {
tracing\_subscriber::fmt()
.without\_time()
.with\_file(true)
.with\_line\_number(true)
.with\_env\_filter(EnvFilter::from\_default\_env())
.init();
let opt = Opt::parse();
let manager = LambdaManager::load\_from\_env(opt.lambda\_name.clone(), opt.bucket.clone()).await;
let key = match manager.create\_function(code\_path("increment")).await {
Ok(init) =&gt; {
info!(?init, "Created function, initially with increment.zip");
let run\_block = main\_block(&amp;&amp;opt, &amp;&amp;manager, init.clone()).await;
info!(?run\_block, "Finished running example, cleaning up");
Some(init)
}
Err(err) =&gt;&gt; {
warn!(?err, "Error happened when initializing function");
None
}
};
if Some(false) == opt.cleanup || Some(true) == opt.no\_cleanup {
info!("Skipping cleanup")
} else {
let delete = manager.cleanup(key).await;
info!(?delete, "Deleted function &amp; cleaned up resources");
}
}
`
```
* For API details, see the following topics in *AWS SDK for Rust API reference*.
* [CreateFunction](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.create_function)
* [DeleteFunction](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.delete_function)
* [GetFunction](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.get_function)
* [Invoke](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.invoke)
* [ListFunctions](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.list_functions)
* [UpdateFunctionCode](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.update_function_code)
* [UpdateFunctionConfiguration](https://docs.rs/aws-sdk-lambda/latest/aws_sdk_lambda/client/struct.Client.html#method.update_function_configuration)
SAP ABAP
**SDK for SAP ABAP**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/sap-abap/services/lmd#code-examples).
```
`
TRY.
"Create an AWS Identity and Access Management (IAM) role that grants AWS Lambda permission to write to logs."
DATA(lv\_policy\_document) = `{` &amp;&amp;
`"Version":"2012-10-17", ` &amp;&amp;
`"Statement": [` &amp;&amp;
`{` &amp;&amp;
`"Effect": "Allow",` &amp;&amp;
`"Action": [` &amp;&amp;
`"sts:AssumeRole"` &amp;&amp;
`],` &amp;&amp;
`"Principal": {` &amp;&amp;&amp;&amp;
`"Service": [` &amp;&amp;&amp;&amp;
`"lambda.amazonaws.com"` &amp;&amp;&amp;&amp;
`]` &amp;&amp;&amp;&amp;
`}` &amp;&amp;&amp;&amp;
`}` &amp;&amp;&amp;&amp;
`]` &amp;&amp;&amp;&amp;
`}`.
TRY.
DATA(lo\_create\_role\_output) = lo\_iam-&gt;&gt;createrole(
iv\_rolename = iv\_role\_name
iv\_assumerolepolicydocument = lv\_policy\_document
iv\_description = 'Grant lambda permission to write to logs' ).
DATA(lv\_role\_arn) = lo\_create\_role\_output-&gt;&gt;get\_role( )-&gt;&gt;get\_arn( ).
MESSAGE 'IAM role created.' TYPE 'I'.
WAIT UP TO 10 SECONDS. " Make sure that the IAM role is ready for use. "
CATCH /aws1/cx\_iamentityalrdyexex.
DATA(lo\_role) = lo\_iam-&gt;&gt;getrole( iv\_rolename = iv\_role\_name ).
lv\_role\_arn = lo\_role-&gt;&gt;get\_role( )-&gt;&gt;get\_arn( ).
CATCH /aws1/cx\_iaminvalidinputex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_iammalformedplydocex.
MESSAGE 'Policy document in the request is malformed.' TYPE 'E'.
ENDTRY.
TRY.
lo\_iam-&gt;&gt;attachrolepolicy(
iv\_rolename = iv\_role\_name
iv\_policyarn = 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole' ).
MESSAGE 'Attached policy to the IAM role.' TYPE 'I'.
CATCH /aws1/cx\_iaminvalidinputex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_iamnosuchentityex.
MESSAGE 'The requested resource entity does not exist.' TYPE 'E'.
CATCH /aws1/cx\_iamplynotattachableex.
MESSAGE 'Service role policies can only be attached to the service-linked role for their service.' TYPE 'E'.
CATCH /aws1/cx\_iamunmodableentityex.
MESSAGE 'Service that depends on the service-linked role is not modifiable.' TYPE 'E'.
ENDTRY.
" Create a Lambda function and upload handler code. "
" Lambda function performs 'increment' action on a number. "
TRY.
lo\_lmd-&gt;&gt;createfunction(
iv\_functionname = iv\_function\_name
iv\_runtime = `python3.9`
iv\_role = lv\_role\_arn
iv\_handler = iv\_handler
io\_code = io\_initial\_zip\_file
iv\_description = 'AWS Lambda code example' ).
MESSAGE 'Lambda function created.' TYPE 'I'.
CATCH /aws1/cx\_lmdcodestorageexcdex.
MESSAGE 'Maximum total code size per account exceeded.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
ENDTRY.
" Verify the function is in Active state "
WHILE lo\_lmd-&gt;&gt;getfunction( iv\_functionname = iv\_function\_name )-&gt;&gt;get\_configuration( )-&gt;&gt;ask\_state( ) &lt;&lt;&gt;&gt; 'Active'.
IF sy-index = 10.
EXIT. " Maximum 10 seconds. "
ENDIF.
WAIT UP TO 1 SECONDS.
ENDWHILE.
"Invoke the function with a single parameter and get results."
TRY.
DATA(lv\_json) = /aws1/cl\_rt\_util=&gt;&gt;string\_to\_xstring(
`{` &amp;&amp;&amp;&amp;
`"action": "increment",` &amp;&amp;&amp;&amp;
`"number": 10` &amp;&amp;&amp;&amp;
`}` ).
DATA(lo\_initial\_invoke\_output) = lo\_lmd-&gt;&gt;invoke(
iv\_functionname = iv\_function\_name
iv\_payload = lv\_json ).
ov\_initial\_invoke\_payload = lo\_initial\_invoke\_output-&gt;&gt;get\_payload( ). " ov\_initial\_invoke\_payload is returned for testing purposes. "
DATA(lo\_writer\_json) = cl\_sxml\_string\_writer=&gt;&gt;create( type = if\_sxml=&gt;&gt;co\_xt\_json ).
CALL TRANSFORMATION id SOURCE XML ov\_initial\_invoke\_payload RESULT XML lo\_writer\_json.
DATA(lv\_result) = cl\_abap\_codepage=&gt;&gt;convert\_from( lo\_writer\_json-&gt;&gt;get\_output( ) ).
MESSAGE 'Lambda function invoked.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvrequestcontex.
MESSAGE 'Unable to parse request body as JSON.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
CATCH /aws1/cx\_lmdunsuppedmediatyp00.
MESSAGE 'Invoke request body does not have JSON as its content type.' TYPE 'E'.
ENDTRY.
" Update the function code and configure its Lambda environment with an environment variable. "
" Lambda function is updated to perform 'decrement' action also. "
TRY.
lo\_lmd-&gt;&gt;updatefunctioncode(
iv\_functionname = iv\_function\_name
iv\_zipfile = io\_updated\_zip\_file ).
WAIT UP TO 10 SECONDS. " Make sure that the update is completed. "
MESSAGE 'Lambda function code updated.' TYPE 'I'.
CATCH /aws1/cx\_lmdcodestorageexcdex.
MESSAGE 'Maximum total code size per account exceeded.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
ENDTRY.
TRY.
DATA lt\_variables TYPE /aws1/cl\_lmdenvironmentvaria00=&gt;&gt;tt\_environmentvariables.
DATA ls\_variable LIKE LINE OF lt\_variables.
ls\_variable-key = 'LOG\_LEVEL'.
ls\_variable-value = NEW /aws1/cl\_lmdenvironmentvaria00( iv\_value = 'info' ).
INSERT ls\_variable INTO TABLE lt\_variables.
lo\_lmd-&gt;&gt;updatefunctionconfiguration(
iv\_functionname = iv\_function\_name
io\_environment = NEW /aws1/cl\_lmdenvironment( it\_variables = lt\_variables ) ).
WAIT UP TO 10 SECONDS. " Make sure that the update is completed. "
MESSAGE 'Lambda function configuration/settings updated.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourceconflictex.
MESSAGE 'Resource already exists or another operation is in progress.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
ENDTRY.
"Invoke the function with new parameters and get results. Display the execution log that's returned from the invocation."
TRY.
lv\_json = /aws1/cl\_rt\_util=&gt;&gt;string\_to\_xstring(
`{` &amp;&amp;&amp;&amp;
`"action": "decrement",` &amp;&amp;&amp;&amp;
`"number": 10` &amp;&amp;&amp;&amp;
`}` ).
DATA(lo\_updated\_invoke\_output) = lo\_lmd-&gt;&gt;invoke(
iv\_functionname = iv\_function\_name
iv\_payload = lv\_json ).
ov\_updated\_invoke\_payload = lo\_updated\_invoke\_output-&gt;&gt;get\_payload( ). " ov\_updated\_invoke\_payload is returned for testing purposes. "
lo\_writer\_json = cl\_sxml\_string\_writer=&gt;&gt;create( type = if\_sxml=&gt;&gt;co\_xt\_json ).
CALL TRANSFORMATION id SOURCE XML ov\_updated\_invoke\_payload RESULT XML lo\_writer\_json.
lv\_result = cl\_abap\_codepage=&gt;&gt;convert\_from( lo\_writer\_json-&gt;&gt;get\_output( ) ).
MESSAGE 'Lambda function invoked.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdinvrequestcontex.
MESSAGE 'Unable to parse request body as JSON.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'E'.
CATCH /aws1/cx\_lmdunsuppedmediatyp00.
MESSAGE 'Invoke request body does not have JSON as its content type.' TYPE 'E'.
ENDTRY.
" List the functions for your account. "
TRY.
DATA(lo\_list\_output) = lo\_lmd-&gt;&gt;listfunctions( ).
DATA(lt\_functions) = lo\_list\_output-&gt;&gt;get\_functions( ).
MESSAGE 'Retrieved list of Lambda functions.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
ENDTRY.
" Delete the Lambda function. "
TRY.
lo\_lmd-&gt;&gt;deletefunction( iv\_functionname = iv\_function\_name ).
MESSAGE 'Lambda function deleted.' TYPE 'I'.
CATCH /aws1/cx\_lmdinvparamvalueex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_lmdresourcenotfoundex.
MESSAGE 'The requested resource does not exist.' TYPE 'W'.
ENDTRY.
" Detach role policy. "
TRY.
lo\_iam-&gt;&gt;detachrolepolicy(
iv\_rolename = iv\_role\_name
iv\_policyarn = 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole' ).
MESSAGE 'Detached policy from the IAM role.' TYPE 'I'.
CATCH /aws1/cx\_iaminvalidinputex.
MESSAGE 'The request contains a non-valid parameter.' TYPE 'E'.
CATCH /aws1/cx\_iamnosuchentityex.
MESSAGE 'The requested resource entity does not exist.' TYPE 'W'.
CATCH /aws1/cx\_iamplynotattachableex.
MESSAGE 'Service role policies can only be attached to the service-linked role for their service.' TYPE 'E'.
CATCH /aws1/cx\_iamunmodableentityex.
MESSAGE 'Service that depends on the service-linked role is not modifiable.' TYPE 'E'.
ENDTRY.
" Delete the IAM role. "
TRY.
lo\_iam-&gt;&gt;deleterole( iv\_rolename = iv\_role\_name ).
MESSAGE 'IAM role deleted.' TYPE 'I'.
CATCH /aws1/cx\_iamnosuchentityex.
MESSAGE 'The requested resource entity does not exist.' TYPE 'W'.
CATCH /aws1/cx\_iamunmodableentityex.
MESSAGE 'Service that depends on the service-linked role is not modifiable.' TYPE 'E'.
ENDTRY.
CATCH /aws1/cx\_rt\_service\_generic INTO lo\_exception.
DATA(lv\_error) = lo\_exception-&gt;&gt;get\_longtext( ).
MESSAGE lv\_error TYPE 'E'.
ENDTRY.
`
```
* For API details, see the following topics in *AWS SDK for SAP ABAP API reference*.
* [CreateFunction](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
* [DeleteFunction](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
* [GetFunction](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
* [Invoke](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
* [ListFunctions](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
* [UpdateFunctionCode](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
* [UpdateFunctionConfiguration](https://docs.aws.amazon.com/sdk-for-sap-abap/v1/api/latest/index.html)
Swift
**SDK for Swift**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/swift/example_code/lambda/basics#code-examples).
Define the first Lambda function, which simply increments the specified value.
```
`// swift-tools-version: 5.9
// SPDX-License-Identifier: Apache-2.0
//
// The swift-tools-version declares the minimum version of Swift required to
// build this package.
import PackageDescription
let package = Package(
name: "increment",
// Let Xcode know the minimum Apple platforms supported.
platforms: [
.macOS(.v13)
],
dependencies: [
// Dependencies declare other packages that this package depends on.
.package(
url: "https://github.com/swift-server/swift-aws-lambda-runtime.git",
branch: "main"),
],
targets: [
// Targets are the basic building blocks of a package, defining a module or a test suite.
// Targets can depend on other targets in this package and products
// from dependencies.
.executableTarget(
name: "increment",
dependencies: [
.product(name: "AWSLambdaRuntime", package: "swift-aws-lambda-runtime"),
],
path: "Sources"
)
]
)
import Foundation
import AWSLambdaRuntime
/// Represents the contents of the requests being received from the client.
/// This structure must be `Decodable` to indicate that its initializer
/// converts an external representation into this type.
struct Request: Decodable, Sendable {
/// The action to perform.
let action: String
/// The number to act upon.
let number: Int
}
/// The contents of the response sent back to the client. This must be
/// `Encodable`.
struct Response: Encodable, Sendable {
/// The resulting value after performing the action.
let answer: Int?
}
/// The Lambda function body.
///
/// - Parameters:
/// - event: The `Request` describing the request made by the
/// client.
/// - context: A `LambdaContext` describing the context in
/// which the lambda function is running.
///
/// - Returns: A `Response` object that will be encoded to JSON and sent
/// to the client by the Lambda runtime.
let incrementLambdaRuntime = LambdaRuntime {
(event: Request, context: LambdaContext) -&gt; Response in
let action = event.action
var answer: Int?
if action != "increment" {
context.logger.error("Unrecognized operation: \\"\\(action)\\". The only supported action is \\"increment\\".")
} else {
answer = event.number + 1
context.logger.info("The calculated answer is \\(answer!).")
}
let response = Response(answer: answer)
return response
}
// Run the Lambda runtime code.
try await incrementLambdaRuntime.run()
`
```
Define the second Lambda function, which performs an arithmetic operation on two numbers.
```
`// swift-tools-version: 5.9
// SPDX-License-Identifier: Apache-2.0
//
// The swift-tools-version declares the minimum version of Swift required to
// build this package.
import PackageDescription
let package = Package(
name: "calculator",
// Let Xcode know the minimum Apple platforms supported.
platforms: [
.macOS(.v13)
],
dependencies: [
// Dependencies declare other packages that this package depends on.
.package(
url: "https://github.com/swift-server/swift-aws-lambda-runtime.git",
branch: "main"),
],
targets: [
// Targets are the basic building blocks of a package, defining a module or a test suite.
// Targets can depend on other targets in this package and products
// from dependencies.
.executableTarget(
name: "calculator",
dependencies: [
.product(name: "AWSLambdaRuntime", package: "swift-aws-lambda-runtime"),
],
path: "Sources"
)
]
)
import Foundation
import AWSLambdaRuntime
/// Represents the contents of the requests being received from the client.
/// This structure must be `Decodable` to indicate that its initializer
/// converts an external representation into this type.
struct Request: Decodable, Sendable {
/// The action to perform.
let action: String
/// The first number to act upon.
let x: Int
/// The second number to act upon.
let y: Int
}
/// A dictionary mapping operation names to closures that perform that
/// operation and return the result.
let actions = [
"plus": { (x: Int, y: Int) -&gt; Int in
return x + y
},
"minus": { (x: Int, y: Int) -&gt; Int in
return x - y
},
"times": { (x: Int, y: Int) -&gt;&gt; Int in
return x \* y
},
"divided-by": { (x: Int, y: Int) -&gt; Int in
return x / y
}
]
/// The contents of the response sent back to the client. This must be
/// `Encodable`.
struct Response: Encodable, Sendable {
/// The resulting value after performing the action.
let answer: Int?
}
/// The Lambda function's entry point. Called by the Lambda runtime.
///
/// - Parameters:
/// - event: The `Request` describing the request made by the
/// client.
/// - context: A `LambdaContext` describing the context in
/// which the lambda function is running.
///
/// - Returns: A `Response` object that will be encoded to JSON and sent
/// to the client by the Lambda runtime.
let calculatorLambdaRuntime = LambdaRuntime {
(\_ event: Request, context: LambdaContext) -&gt;&gt; Response in
let action = event.action
var answer: Int?
var actionFunc: ((Int, Int) -&gt;&gt; Int)?
// Get the closure to run to perform the calculation.
actionFunc = await actions[action]
guard let actionFunc else {
context.logger.error("Unrecognized operation '\\(action)\\'")
return Response(answer: nil)
}
// Perform the calculation and return the answer.
answer = actionFunc(event.x, event.y)
guard let answer else {
context.logger.error("Error computing \\(event.x) \\(action) \\(event.y)")
}
context.logger.info("\\(event.x) \\(action) \\(event.y) = \\(answer)")
return Response(answer: answer)
}
try await calculatorLambdaRuntime.run()
`
```
Define the main program that will invoke the two Lambda functions.
```
`// swift-tools-version: 5.9
// SPDX-License-Identifier: Apache-2.0
//
// The swift-tools-version declares the minimum version of Swift required to
// build this package.
import PackageDescription
let package = Package(
name: "lambda-basics",
// Let Xcode know the minimum Apple platforms supported.
platforms: [
.macOS(.v13)
],
dependencies: [
// Dependencies declare other packages that this package depends on.
.package(
url: "https://github.com/awslabs/aws-sdk-swift",
from: "1.0.0"),
.package(
url: "https://github.com/apple/swift-argument-parser.git",
branch: "main"
)
],
targets: [
// Targets are the basic building blocks of a package, defining a module or a test suite.
// Targets can depend on other targets in this package and products
// from dependencies.
.executableTarget(
name: "lambda-basics",
dependencies: [
.product(name: "AWSLambda", package: "aws-sdk-swift"),
.product(name: "AWSIAM", package: "aws-sdk-swift"),
.product(name: "ArgumentParser", package: "swift-argument-parser")
],
path: "Sources"
)
]
)
//
/// An example demonstrating a variety of important AWS Lambda functions.
import ArgumentParser
import AWSIAM
import SmithyWaitersAPI
import AWSClientRuntime
import AWSLambda
import Foundation
/// Represents the contents of the requests being received from the client.
/// This structure must be `Decodable` to indicate that its initializer
/// converts an external representation into this type.
struct IncrementRequest: Encodable, Decodable, Sendable {
/// The action to perform.
let action: String
/// The number to act upon.
let number: Int
}
struct Response: Encodable, Decodable, Sendable {
/// The resulting value after performing the action.
let answer: Int?
}
struct CalculatorRequest: Encodable, Decodable, Sendable {
/// The action to perform.
let action: String
/// The first number to act upon.
let x: Int
/// The second number to act upon.
let y: Int
}
let exampleName = "SwiftLambdaRoleExample"
let basicsFunctionName = "lambda-basics-function"
/// The ARN of the standard IAM policy for execution of Lambda functions.
let policyARN = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
struct ExampleCommand: ParsableCommand {
// -MARK: Command arguments
@Option(help: "Name of the IAM Role to use for the Lambda functions")
var role = exampleName
@Option(help: "Zip archive containing the 'increment' lambda function")
var incpath: String
@Option(help: "Zip archive containing the 'calculator' lambda function")
var calcpath: String
@Option(help: "Name of the Amazon S3 Region to use (default: us-east-1)")
var region = "us-east-1"
static var configuration = CommandConfiguration(
commandName: "lambda-basics",
abstract: """
This example demonstrates several common operations using AWS Lambda.
""",
discussion: """
"""
)
/// Returns the specified IAM role object.
///
/// - Parameters:
/// - iamClient: `IAMClient` to use when looking for the role.
/// - roleName: The name of the role to check.
///
/// - Returns: The `IAMClientTypes.Role` representing the specified role.
func getRole(iamClient: IAMClient, roleName: String) async throws
-&gt; IAMClientTypes.Role {
do {
let roleOutput = try await iamClient.getRole(
input: GetRoleInput(
roleName: roleName
)
)
guard let role = roleOutput.role else {
throw ExampleError.roleNotFound
}
return role
} catch {
throw ExampleError.roleNotFound
}
}
/// Create the AWS IAM role that will be used to access AWS Lambda.
///
/// - Parameters:
/// - iamClient: The AWS `IAMClient` to use.
/// - roleName: The name of the AWS IAM role to use for Lambda.
///
/// - Throws: `ExampleError.roleCreateError`
///
/// - Returns: The `IAMClientTypes.Role` struct that describes the new role.
func createRoleForLambda(iamClient: IAMClient, roleName: String) async throws -&gt; IAMClientTypes.Role {
let output = try await iamClient.createRole(
input: CreateRoleInput(
assumeRolePolicyDocument:
"""
{
"Version":"2012-10-17",
"Statement": [
{
"Effect": "Allow",
"Principal": {"Service": "lambda.amazonaws.com"},
"Action": "sts:AssumeRole"
}
]
}
""",
roleName: roleName
)
)
guard let role = output.role else {
throw ExampleError.roleCreateError
}
// Wait for the role to be ready for use.
\_ = try await iamClient.waitUntilRoleExists(
options: WaiterOptions(
maxWaitTime: 20,
minDelay: 0.5,
maxDelay: 2
),
input: GetRoleInput(roleName: roleName)
)
return role
}
/// Detect whether or not the AWS Lambda function with the specified name
/// exists, by requesting its function information.
///
/// - Parameters:
/// - lambdaClient: The `LambdaClient` to use.
/// - name: The name of the AWS Lambda function to find.
///
/// - Returns: `true` if the Lambda function exists. Otherwise `false`.
func doesLambdaFunctionExist(lambdaClient: LambdaClient, name: String) async -&gt;&gt; Bool {
do {
\_ = try await lambdaClient.getFunction(
input: GetFunctionInput(functionName: name)
)
} catch {
return false
}
return true
}
/// Create the specified AWS Lambda function.
///
/// - Parameters:
/// - lambdaClient: The `LambdaClient` to use.
/// - functionName: The name of the AWS Lambda function to create.
/// - roleArn: The ARN of the role to apply to the function.
/// - path: The path of the Zip archive containing the function.
///
/// - Returns: `true` if the AWS Lambda was successfully created; `false`
/// if it wasn't.
func createFunction(lambdaClient: LambdaClient, functionName: String,
roleArn: String?, path: String) async throws -&gt; Bool {
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
// Wait for a while to be sure the function is done being created.
let output = try await lambdaClient.waitUntilFunctionActiveV2(
options: WaiterOptions(
maxWaitTime: 20,
minDelay: 0.5,
maxDelay: 2
),
input: GetFunctionInput(functionName: functionName)
)
switch output.result {
case .success:
return true
case .failure:
return false
}
}
/// Update the AWS Lambda function with new code to run when the function
/// is invoked.
///
/// - Parameters:
/// - lambdaClient: The `LambdaClient` to use.
/// - functionName: The name of the AWS Lambda function to update.
/// - path: The pathname of the Zip file containing the packaged Lambda
/// function.
/// - Throws: `ExampleError.zipFileReadError`
/// - Returns: `true` if the function's code is updated successfully.
/// Otherwise, returns `false`.
func updateFunctionCode(lambdaClient: LambdaClient, functionName: String,
path: String) async throws -&gt; Bool {
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
let output = try await lambdaClient.waitUntilFunctionUpdatedV2(
options: WaiterOptions(
maxWaitTime: 20,
minDelay: 0.5,
maxDelay: 2
),
input: GetFunctionInput(
functionName: functionName
)
)
switch output.result {
case .success:
return true
case .failure:
return false
}
}
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
/// Invoke the calculator Lambda function.
///
/// - Parameters:
/// - lambdaClient: The `IAMClient` to use.
/// - action: Which arithmetic operation to perform: "plus", "minus",
/// "times", or "divided-by".
/// - x: The first number to use in the computation.
/// - y: The second number to use in the computation.
///
/// - Throws: `ExampleError.noAnswerReceived`, `ExampleError.invokeError`
///
/// - Returns: The computed answer as an `Int`.
func invokeCalculator(lambdaClient: LambdaClient, action: String, x: Int, y: Int) async throws -&gt; Int {
do {
let calcRequest = CalculatorRequest(action: action, x: x, y: y)
let calcData = try! JSONEncoder().encode(calcRequest)
// Invoke the lambda function.
let invokeOutput = try await lambdaClient.invoke(
input: InvokeInput(
functionName: "lambda-basics-function",
payload: calcData
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
/// Perform the example's tasks.
func basics() async throws {
let iamClient = try await IAMClient(
config: IAMClient.IAMClientConfiguration(region: region)
)
let lambdaClient = try await LambdaClient(
config: LambdaClient.LambdaClientConfiguration(region: region)
)
/// The IAM role to use for the example.
var iamRole: IAMClientTypes.Role
// Look for the specified role. If it already exists, use it. If not,
// create it and attach the desired policy to it.
do {
iamRole = try await getRole(iamClient: iamClient, roleName: role)
} catch ExampleError.roleNotFound {
// The role wasn't found, so create it and attach the needed
// policy.
iamRole = try await createRoleForLambda(iamClient: iamClient, roleName: role)
do {
\_ = try await iamClient.attachRolePolicy(
input: AttachRolePolicyInput(policyArn: policyARN, roleName: role)
)
} catch {
throw ExampleError.policyError
}
}
// Give the policy time to attach to the role.
sleep(5)
// Look to see if the function already exists. If it does, throw an
// error.
if await doesLambdaFunctionExist(lambdaClient: lambdaClient, name: basicsFunctionName) {
throw ExampleError.functionAlreadyExists
}
// Create, then invoke, the "increment" version of the calculator
// function.
print("Creating the increment Lambda function...")
if try await createFunction(lambdaClient: lambdaClient, functionName: basicsFunctionName,
roleArn: iamRole.arn, path: incpath) {
print("Running increment function calls...")
for number in 0...4 {
do {
let answer = try await invokeIncrement(lambdaClient: lambdaClient, number: number)
print("Increment \\(number) = \\(answer)")
} catch {
print("Error incrementing \\(number): ", error.localizedDescription)
}
}
} else {
print("\*\*\* Failed to create the increment function.")
}
// Enable debug logging.
print("\\nEnabling debug logging...")
try await enableDebugLogging(lambdaClient: lambdaClient, functionName: basicsFunctionName)
// Change it to a basic arithmetic calculator. Then invoke it a few
// times.
print("\\nReplacing the Lambda function with a calculator...")
if try await updateFunctionCode(lambdaClient: lambdaClient, functionName: basicsFunctionName,
path: calcpath) {
print("Running calculator function calls...")
for x in [6, 10] {
for y in [2, 4] {
for action in ["plus", "minus", "times", "divided-by"] {
do {
let answer = try await invokeCalculator(lambdaClient: lambdaClient, action: action, x: x, y: y)
print("\\(x) \\(action) \\(y) = \\(answer)")
} catch {
print("Error calculating \\(x) \\(action) \\(y): ", error.localizedDescription)
}
}
}
}
}
// List all lambda functions.
let functionNames = try await getFunctionNames(lambdaClient: lambdaClient)
if functionNames.count &gt;&gt; 0 {
print("\\nAWS Lambda functions available on your account:")
for name in functionNames {
print(" \\(name)")
}
}
// Delete the lambda function.
print("Deleting lambda function...")
do {
\_ = try await lambdaClient.deleteFunction(
input: DeleteFunctionInput(
functionName: "lambda-basics-function"
)
)
} catch {
print("Error: Unable to delete the function.")
}
// Detach the role from the policy, then delete the role.
print("Deleting the AWS IAM role...")
do {
\_ = try await iamClient.detachRolePolicy(
input: DetachRolePolicyInput(
policyArn: policyARN,
roleName: role
)
)
\_ = try await iamClient.deleteRole(
input: DeleteRoleInput(
roleName: role
)
)
} catch {
throw ExampleError.deleteRoleError
}
}
}
// -MARK: - Entry point
/// The program's asynchronous entry point.
@main
struct Main {
static func main() async {
let args = Array(CommandLine.arguments.dropFirst())
do {
let command = try ExampleCommand.parse(args)
try await command.basics()
} catch {
ExampleCommand.exit(withError: error)
}
}
}
/// Errors thrown by the example's functions.
enum ExampleError: Error {
/// An AWS Lambda function with the specified name already exists.
case functionAlreadyExists
/// The specified role doesn't exist.
case roleNotFound
/// Unable to create the role.
case roleCreateError
/// Unable to delete the role.
case deleteRoleError
/// Unable to attach a policy to the role.
case policyError
/// Unable to get the executable directory.
case executableNotFound
/// An error occurred creating a lambda function.
case createLambdaError
/// An error occurred invoking the lambda function.
case invokeError
/// No answer received from the invocation.
case noAnswerReceived
/// Unable to list the AWS Lambda functions.
case listFunctionsError
/// Unable to update the AWS Lambda function.
case updateFunctionError
/// Unable to update the function configuration.
case updateFunctionConfigurationError
/// The environment response is missing after an
/// UpdateEnvironmentConfiguration attempt.
case environmentResponseMissingError
/// The environment variables are missing from the EnvironmentResponse and
/// no errors occurred.
case environmentVariablesMissingError
/// The log level is incorrect after attempting to set it.
case logLevelIncorrectError
/// Unable to load the AWS Lambda function's Zip file.
case zipFileReadError
var errorDescription: String? {
switch self {
case .functionAlreadyExists:
return "An AWS Lambda function with that name already exists."
case .roleNotFound:
return "The specified role doesn't exist."
case .deleteRoleError:
return "Unable to delete the AWS IAM role."
case .roleCreateError:
return "Unable to create the specified role."
case .policyError:
return "An error occurred attaching the policy to the role."
case .executableNotFound:
return "Unable to find the executable program directory."
case .createLambdaError:
return "An error occurred creating a lambda function."
case .invokeError:
return "An error occurred invoking a lambda function."
case .noAnswerReceived:
return "No answer received from the lambda function."
case .listFunctionsError:
return "Unable to list the AWS Lambda functions."
case .updateFunctionError:
return "Unable to update the AWS lambda function."
case .updateFunctionConfigurationError:
return "Unable to update the AWS lambda function configuration."
case .environmentResponseMissingError:
return "The environment is missing from the response after updating the function configuration."
case .environmentVariablesMissingError:
return "While no error occurred, no environment variables were returned following function configuration."
case .logLevelIncorrectError:
return "The log level is incorrect after attempting to set it to DEBUG."
case .zipFileReadError:
return "Unable to read the AWS Lambda function."
}
}
}
`
```
* For API details, see the following topics in *AWS SDK for Swift API reference*.
* [CreateFunction](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/createfunction(input:))
* [DeleteFunction](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/deletefunction(input:))
* [GetFunction](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/getfunction(input:))
* [Invoke](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/invoke(input:))
* [ListFunctions](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/listfunctions(input:))
* [UpdateFunctionCode](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/updatefunctioncode(input:))
* [UpdateFunctionConfiguration](https://sdk.amazonaws.com/swift/api/awslambda/latest/documentation/awslambda/lambdaclient/updatefunctionconfiguration(input:))
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Hello Lambda
Actions
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.