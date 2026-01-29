---
url: https://docs.aws.amazon.com/lambda/latest/dg/services-alb.html
title: Process Application Load Balancer requests with Lambda
word_count: 582
filtered: true
elements_removed: 0
density_score: 0.87
---

Process Application Load Balancer requests with Lambda - AWS Lambda
Process Application Load Balancer requests with Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#services-alb)
[Event Handler from Powertools for AWS Lambda](#services-alb-powertools)
# Process Application Load Balancer requests with Lambda
You can use a Lambda function to process requests from an Application Load Balancer. Elastic Load Balancing supports Lambda functions as a target for
an Application Load Balancer. Use load balancer rules to route HTTP requests to a function, based on path or header values. Process the
request and return an HTTP response from your Lambda function.
Elastic Load Balancing invokes your Lambda function synchronously with an event that contains the request body and
metadata.
###### Example Application Load Balancer request event
```
`{
"requestContext": {
"elb": {
"targetGroupArn": "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/lambda-279XGJDqGZ5rsrHC2Fjr/49e9d65c45c6791a"
}
},
"httpMethod": "GET",
"path": "/lambda",
"queryStringParameters": {
"query": "1234ABCD"
},
"headers": {
"accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,\*/\*;q=0.8",
"accept-encoding": "gzip",
"accept-language": "en-US,en;q=0.9",
"connection": "keep-alive",
"host": "lambda-alb-123578498.us-east-1.elb.amazonaws.com",
"upgrade-insecure-requests": "1",
"user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/71.0.3578.98 Safari/537.36",
"x-amzn-trace-id": "Root=1-5c536348-3d683b8b04734faae651f476",
"x-forwarded-for": "72.12.164.125",
"x-forwarded-port": "80",
"x-forwarded-proto": "http",
"x-imforwards": "20"
},
"body": "",
"isBase64Encoded": False
}`
```
Your function processes the event and returns a response document to the load balancer in JSON. Elastic Load Balancing converts
the document to an HTTP success or error response and returns it to the user.
###### Example response document format
```
`{
"statusCode": 200,
"statusDescription": "200 OK",
"isBase64Encoded": False,
"headers": {
"Content-Type": "text/html"
},
"body": "&lt;h1&gt;Hello from Lambda!&lt;/h1&gt;"
}`
```
To configure an Application Load Balancer as a function trigger, grant Elastic Load Balancing permission to run the function, create a target
group that routes requests to the function, and add a rule to the load balancer that sends requests to the target
group.
Use the `add-permission` command to add a permission statement to your function's resource-based
policy.
```
``aws lambda add-permission --function-name `alb-function` \\
--statement-id load-balancer --action "lambda:InvokeFunction" \\
--principal elasticloadbalancing.amazonaws.com``
```
You should see the following output:
```
`{
"Statement": "{\\"Sid\\":\\"load-balancer\\",\\"Effect\\":\\"Allow\\",\\"Principal\\":{\\"Service\\":\\"elasticloadbalancing.amazonaws.com\\"},\\"Action\\":\\"lambda:InvokeFunction\\",\\"Resource\\":\\"arn:aws:lambda:us-west-2:123456789012:function:alb-function\\"}"
}`
```
For instructions on configuring the Application Load Balancer listener and target group, see [Lambda functions as a target](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/lambda-functions.html) in the
*User Guide for Application Load Balancers*.
## Event Handler from Powertools for AWS Lambda
The event handler from the Powertools for AWS Lambda toolkit provides routing, middleware, CORS configuration, OpenAPI spec generation, request validation, error handling, and other useful features when writing Lambda functions invoked by an Application Load Balancer.
The Event Handler utility is available for Python. For more information, see [Event Handler REST API](https://docs.powertools.aws.dev/lambda/python/latest/core/event_handler/api_gateway/) in the *Powertools for AWS Lambda (Python) documentation*.
### Python
```
`import requests
from requests import Response
from aws\_lambda\_powertools import Logger, Tracer
from aws\_lambda\_powertools.event\_handler import ALBResolver
from aws\_lambda\_powertools.logging import correlation\_paths
from aws\_lambda\_powertools.utilities.typing import LambdaContext
tracer = Tracer()
logger = Logger()
app = ALBResolver()
@app.get("/todos")
@tracer.capture\_method
def get\_todos():
todos: Response = requests.get("https://jsonplaceholder.typicode.com/todos")
todos.raise\_for\_status()
# for brevity, we'll limit to the first 10 only
return {"todos": todos.json()[:10]}
# You can continue to use other utilities just as before
@logger.inject\_lambda\_context(correlation\_id\_path=correlation\_paths.APPLICATION\_LOAD\_BALANCER)
@tracer.capture\_lambda\_handler
def lambda\_handler(event: dict, context: LambdaContext) -&gt;&gt; dict:
return app.resolve(event, context)`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
EC2
Invoke using an EventBridge Scheduler
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.