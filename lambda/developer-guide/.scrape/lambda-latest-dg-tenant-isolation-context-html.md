---
url: https://docs.aws.amazon.com/lambda/latest/dg/tenant-isolation-context.html
title: Accessing tenant identifier in Lambda function code
word_count: 551
filtered: true
elements_removed: 0
density_score: 0.78
---

Accessing tenant identifier in Lambda function code - AWS Lambda
Accessing tenant identifier in Lambda function code - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#tenant-isolation-context)
[Access](#tenant-isolation-context-access)[Usage patterns](#tenant-isolation-context-patterns)[Monitoring](#tenant-isolation-context-monitoring)
# Accessing tenant identifier in Lambda function code
When your Lambda function has tenant isolation enabled, the tenant identifier used to invoke your function
is made available within the context object passed to your function handler. You can use this identifier
to implement tenant-specific logic, monitoring, and debugging capabilities.
###### Topics
* [Accessing the tenant identifier](#tenant-isolation-context-access)
* [Common usage patterns](#tenant-isolation-context-patterns)
* [Monitoring and debugging](#tenant-isolation-context-monitoring)
## Accessing the tenant identifier
The tenant identifier is available through the `tenantId` property of the context object.
Note that this property is available during the [invocation phase](./lambda-runtime-environment.html#runtimes-lifecycle-invoke),
not during the [initialization phase](./lambda-runtime-environment.html#runtimes-lifecycle-ib).
Python
```
``def lambda\_handler(event, context):
tenant\_id = context.tenant\_id
print(f"Processing request for tenant: {tenant\_id}")
# Implement tenant-specific logic
if tenant\_id == "blue":
return process\_blue\_tenant(event)
elif tenant\_id == "green":
return process\_green\_tenant(event)
else:
return process\_default\_tenant(event)``
```
Node.js
```
``exports.handler = async (event, context) =&gt; {
const tenantId = context.tenantId;
console.log(`Processing request for tenant: ${tenantId}`);
// Implement tenant-specific logic
switch (tenantId) {
case 'blue':
return processBlueTenant(event);
case 'green':
return processGreenTenant(event);
default:
return processDefaultTenant(event);
}
};``
```
Java
```
``public class TenantHandler implements RequestHandler&lt;Map&lt;String, Object&gt;, String&gt; {
@Override
public String handleRequest(Map&lt;String, Object&gt; event, Context context) {
String tenantId = context.getTenantId();
System.out.println("Processing request for tenant: " + tenantId);
// Implement tenant-specific logic
switch (tenantId) {
case "blue":
return processBlueTenant(event);
case "green":
return processGreenTenant(event);
default:
return processDefaultTenant(event);
}
}
}``
```
## Common usage patterns
Here are common ways to use the tenant identifier in your function code:
**Tenant-specific configuration**
Use the tenant ID to load tenant-specific configuration or settings:
```
``def lambda\_handler(event, context):
tenant\_id = context.tenant\_id
# Load tenant-specific configuration
config = load\_tenant\_config(tenant\_id)
database\_url = config['database\_url']
api\_key = config['api\_key']
# Process with tenant-specific settings
return process\_request(event, database\_url, api\_key)``
```
**Tenant-specific data access**
Use the tenant ID to ensure data isolation and access control:
```
``import boto3
def lambda\_handler(event, context):
tenant\_id = context.tenant\_id
# Ensure data access is scoped to the tenant
dynamodb = boto3.resource('dynamodb')
table = dynamodb.Table('user\_data')
user\_id = event.get('userId')
response = table.get\_item(
Key={
'tenant\_id': tenant\_id,
'user\_id': user\_id
}
)
return process\_results(response.get('Item'), tenant\_id)``
```
## Monitoring and debugging
The tenant identifier is automatically included in Lambda logs when you have
[JSON logging enabled](./monitoring-cloudwatchlogs-logformat.html),
making it easier to monitor and debug tenant-specific issues. You can also use the tenant ID
for custom metrics and tracing.
###### Example Custom metrics with tenant ID
The following example demonstrates how to use the tenant ID to create tenant-specific CloudWatch metrics for monitoring usage patterns and performance by tenant:
```
``import boto3
def lambda\_handler(event, context):
tenant\_id = context.tenant\_id
cloudwatch = boto3.client('cloudwatch')
# Record tenant-specific metrics
cloudwatch.put\_metric\_data(
Namespace='MyApp/TenantMetrics',
MetricData=[
{
'MetricName': 'RequestCount',
'Dimensions': [
{
'Name': 'TenantId',
'Value': tenant\_id
}
],
'Value': 1,
'Unit': 'Count'
}
]
)
return process\_request(event, tenant\_id)``
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Invoking functions with tenant isolation
Monitoring
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.