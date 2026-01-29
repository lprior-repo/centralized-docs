---
url: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-list-deprecated.html
title: Retrieve data about Lambda functions that use a deprecated runtime
word_count: 1324
filtered: true
elements_removed: 0
density_score: 0.85
---

Retrieve data about Lambda functions that use a deprecated runtime - AWS Lambda
Retrieve data about Lambda functions that use a deprecated runtime - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#runtimes-list-deprecated)
[Listing function versions that use a particular runtime](#runtimes-list-deprecated-versions)[Identifying most commonly and most recently invoked functions](#runtimes-list-deprecated-statistics)
# Retrieve data about Lambda functions that use a deprecated runtime
When a Lambda runtime is approaching deprecation, Lambda alerts you through email and provides notifications in the Health Dashboard and Trusted Advisor. These
emails and notifications list the $LATEST versions of functions using the runtime. To list all of your function versions that use a particular
runtime, you can use the AWS Command Line Interface (AWS CLI) or one of the AWS SDKs.
If you have a large number of functions which use a runtime that is due to be deprecated, you can also use the AWS CLI or AWS SDKs to
help you prioritize updates to your most commonly invoked functions.
Refer to the following sections to learn how to use the AWS CLI and AWS SDKs to gather data about functions that use a particular runtime.
## Listing function versions that use a particular runtime
To use the AWS CLI to list all of your function versions that use a particular runtime, run the following command. Replace `RUNTIME\_IDENTIFIER` with the name of the runtime that’s being
deprecated and choose your own AWS Region. To list only $LATEST function versions, omit `--function-version ALL` from the command.
```
``aws lambda list-functions --function-version ALL --region `us-east-1` --output text --query "Functions[?Runtime=='`RUNTIME\_IDENTIFIER`'].FunctionArn" ``
```
###### Tip
The example command lists functions in the `us-east-1` region for a particular AWS account You’ll need to repeat this command for
each region in which your account has functions and for each of your AWS accounts.
You can also list functions that use a particular runtime using one of the AWS SDKs. The following example code uses the V3 AWS SDK for JavaScript and the
AWS SDK for Python (Boto3) to return a list of the function ARNs for functions using a particular runtime. The example code also returns the CloudWatch log group for each
of the listed functions. You can use this log group to find the last invocation date for the function. See the following section [Identifying most commonly and most recently invoked functions](#runtimes-list-deprecated-statistics)
for more information.
Node.js
###### Example JavaScript code to list functions using a particular runtime
```
`import { LambdaClient, ListFunctionsCommand } from "@aws-sdk/client-lambda";
const lambdaClient = new LambdaClient();
const command = new ListFunctionsCommand({
FunctionVersion: "ALL",
MaxItems: 50
});
const response = await lambdaClient.send(command);
for (const f of response.Functions){
if (f.Runtime == '`&lt;&lt;your\_runtime&gt;&gt;`'){ // Use the runtime id, e.g. 'nodejs24.x' or 'python3.14'
console.log(f.FunctionArn);
// get the CloudWatch log group of the function to
// use later for finding the last invocation date
console.log(f.LoggingConfig.LogGroup);
}
}
// If your account has more functions than the specified
// MaxItems, use the returned pagination token in the
// next request with the 'Marker' parameter
if ('NextMarker' in response){
let paginationToken = response.NextMarker;
}`
```
Python
###### Example Python code to list functions using a particular runtime
```
`import boto3
from botocore.exceptions import ClientError
def list\_lambda\_functions(target\_runtime):
lambda\_client = boto3.client('lambda')
response = lambda\_client.list\_functions(
FunctionVersion='ALL',
MaxItems=50
)
if not response['Functions']:
print("No Lambda functions found")
else:
for function in response['Functions']:
if function['PackageType']=='Zip' and function['Runtime'] == target\_runtime:
print(function['FunctionArn'])
# to use later for finding last invocation date
print(function['LoggingConfig']['LogGroup'])
if 'NextMarker' in response:
pagination\_token = response['NextMarker']
if \_\_name\_\_ == "\_\_main\_\_":
# Replace python3.12 with the appropriate runtime ID for your Lambda functions
list\_lambda\_functions('`python3.12`')`
```
To learn more about using an AWS SDK to list your functions using the [ListFunctions](https://docs.aws.amazon.com/lambda/latest/api/API_ListFunctions.html)
action, see the [SDK documentation](https://aws.amazon.com/developer/tools/) for your preferred programming language.
You can also use the AWS Config Advanced queries feature to list all your functions that use an affected runtime. This query only returns function
$LATEST versions, but you can aggregate queries to list function across all regions and multiple AWS accounts with a single command. To learn more,
see [Querying the Current Configuration State of AWS Auto Scaling Resources](https://docs.aws.amazon.com/config/latest/developerguide/querying-AWS-resources.html) in the
*AWS Config Developer Guide*.
## Identifying most commonly and most recently invoked functions
If your AWS account contains functions that use a runtime that's due to be deprecated, you might want to prioritize updating
functions that are frequently invoked or functions that have been invoked recently.
If you have only a few functions, you can use the CloudWatch Logs console to gather this information by looking at your functions' log streams. See
[View log data sent to CloudWatch Logs](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html#ViewingLogData)
for more information.
To see the number of recent function invocations, you can also use the CloudWatch metrics information shown in the Lambda console. To view this information, do the following:
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function you want to see invocation statistics for.
3. Choose the **Monitor** tab.
4. Set the time period you wish to view statistics for using the date range picker. Recent invocations are displayed in the **Invocations** pane.
For accounts with larger numbers of functions, it can be more efficient to gather this data programmatically using the AWS CLI or one of the AWS SDKs using the
[DescribeLogStreams](https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeLogStreams.html) and
[GetMetricStatistics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_GetMetricStatistics.html) API actions.
The following examples provide code snippets using the V3 AWS SDK for JavaScript and the AWS SDK for Python (Boto3) to identify the last invoke date for a particular function and to
determine the number of invocations for a particular function in the last 14 days.
Node.js
###### Example JavaScript code to find last invocation time for a function
```
`import { CloudWatchLogsClient, DescribeLogStreamsCommand } from "@aws-sdk/client-cloudwatch-logs";
const cloudWatchLogsClient = new CloudWatchLogsClient();
const command = new DescribeLogStreamsCommand({
logGroupName: '`&lt;&lt;your\_log\_group\_name&gt;&gt;`',
orderBy: 'LastEventTime',
descending: true,
limit: 1
});
try {
const response = await cloudWatchLogsClient.send(command);
const lastEventTimestamp = response.logStreams.length &gt; 0 ?
response.logStreams[0].lastEventTimestamp : null;
// Convert the UNIX timestamp to a human-readable format for display
const date = new Date(lastEventTimestamp).toLocaleDateString();
const time = new Date(lastEventTimestamp).toLocaleTimeString();
console.log(`${date} ${time}`);
} catch (e){
console.error('Log group not found.')
}`
```
Python
###### Example Python code to find last invocation time for a function
```
`import boto3
from datetime import datetime
cloudwatch\_logs\_client = boto3.client('logs')
response = cloudwatch\_logs\_client.describe\_log\_streams(
logGroupName='`&lt;&lt;your\_log\_group\_name&gt;&gt;`',
orderBy='LastEventTime',
descending=True,
limit=1
)
try:
if len(response['logStreams']) &gt;&gt; 0:
last\_event\_timestamp = response['logStreams'][0]['lastEventTimestamp']
print(datetime.fromtimestamp(last\_event\_timestamp/1000)) # Convert timestamp from ms to seconds
else:
last\_event\_timestamp = None
except:
print('Log group not found')`
```
###### Tip
You can find your function's log group name using the [ListFunctions](https://docs.aws.amazon.com/lambda/latest/api/API_ListFunctions.html) API operation. See the
code in [Listing function versions that use a particular runtime](#runtimes-list-deprecated-versions) for an example of how to do this.
Node.js
###### Example JavaScript code to find number of invocations in last 14 days
```
`import { CloudWatchClient, GetMetricStatisticsCommand } from "@aws-sdk/client-cloudwatch";
const cloudWatchClient = new CloudWatchClient();
const command = new GetMetricStatisticsCommand({
Namespace: 'AWS/Lambda',
MetricName: 'Invocations',
StartTime: new Date(Date.now()-86400\*1000\*14), // 14 days ago
EndTime: new Date(Date.now()),
Period: 86400 \* 14, // 14 days.
Statistics: ['Sum'],
Dimensions: [{
Name: 'FunctionName',
Value: '`&lt;&lt;your\_function\_name&gt;&gt;`'
}]
});
const response = await cloudWatchClient.send(command);
const invokesInLast14Days = response.Datapoints.length &gt; 0 ?
response.Datapoints[0].Sum : 0;
console.log('Number of invocations: ' + invokesInLast14Days);`
```
Python
###### Example Python code to find number of invocations in last 14 days
```
`import boto3
from datetime import datetime, timedelta
cloudwatch\_client = boto3.client('cloudwatch')
response = cloudwatch\_client.get\_metric\_statistics(
Namespace='AWS/Lambda',
MetricName='Invocations',
Dimensions=[
{
'Name': 'FunctionName',
'Value': '`&lt;&lt;your\_function\_name&gt;&gt;`'
},
],
StartTime=datetime.now() - timedelta(days=14),
EndTime=datetime.now(),
Period=86400 \* 14, # 14 days
Statistics=[
'Sum'
]
)
if len(response['Datapoints']) &gt;&gt; 0:
invokes\_in\_last\_14\_days = int(response['Datapoints'][0]['Sum'])
else:
invokes\_in\_last\_14\_days = 0
print(f'Number of invocations: {invokes\_in\_last\_14\_days}')`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Permissions
Runtime modifications
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.