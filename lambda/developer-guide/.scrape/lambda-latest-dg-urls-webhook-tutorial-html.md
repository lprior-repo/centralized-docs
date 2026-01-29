---
url: https://docs.aws.amazon.com/lambda/latest/dg/urls-webhook-tutorial.html
title: Tutorial: Creating a webhook endpoint using a Lambda function URL
word_count: 2471
filtered: true
elements_removed: 0
density_score: 0.83
---

Tutorial: Creating a webhook endpoint using a Lambda function URL - AWS Lambda
Tutorial: Creating a webhook endpoint using a Lambda function URL - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#urls-webhook-tutorial)
[Prerequisites](#urls-webhook-tutorial-prereqs)[Create the Lambda function](#urls-webhook-tutorial-function)[Create the secret key](#urls-webhook-tutorial-key)[Create the function URL endpoint](#urls-webhook-tutorial-furl)[Test the function in the console](#urls-webhook-tutorial-test-console)[Test the function using an HTTP request](#urls-webhook-tutorial-test-curl)[Clean up your resources](#urls-webhook-tutorial-cleanup)
# Tutorial: Creating a webhook endpoint using a Lambda function URL
In this tutorial, you create a Lambda function URL to implement a webhook endpoint. A webhook is a lightweight, event-driven communication that automatically sends
data between applications using HTTP.
You can use a webhook to receive immediate updates about events happening in another system, such as when a new customer signs up on a website, a payment is processed,
or a file is uploaded.
With Lambda, webhooks can be implemented using either Lambda function URLs or API Gateway. Function URLs are a good choice for simple webhooks that don't require features like advanced authorization
or request validation.
###### Tip
If you're not sure which solution is best for your particular use case, see [Select a method to invoke your Lambda function using an HTTP request](./furls-http-invoke-decision.html).
## Prerequisites
To complete this tutorial, you must have either Python (version 3.8 or later) or Node.js (version 18 or later) installed on your local machine.
To test the endpoint using an HTTP request, the tutorial uses [curl](https://curl.se/), a command line tool you can use to transfer data using various network protocols. Refer to
the [curl documentation](https://curl.se/docs/install.html) to learn how to install the tool if you don't already have it.
## Create the Lambda function
First create the Lambda function that runs when an HTTP request is sent to your webhook endpoint. In this example, the sending application sends an update whenever a payment is submitted and
indicates in the body of the HTTP request whether the payment was successful. The Lambda function parses the request and takes action according to the status of the payment. In this example, the code just prints the
order ID for the payment, but in a real application, you might add the order to a database or send a notification.
The function also implements the most common authentication method used for webhooks, hash-based message authentication (HMAC). With this method, both the sending and receiving applications share a secret key.
The sending application uses a hashing algorithm to generate a unique signature using this key together with the message content, and includes the signature in the webhook request as an HTTP header. The receiving application then
repeats this step, generating the signature using the secret key, and compares the resulting value with the signature sent in the request header. If the result matches, the request is considered legitimate.
Create the function using the Lambda console with either the Python or Node.js runtime.
Python
###### Create the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Create a basic 'Hello world' function by doing the following:
1. Choose **Create function**.
2. Select **Author from scratch**.
3. For **Function name**, enter `myLambdaWebhook`.
4. For **Runtime**, select **python3.14**.
5. Choose **Create function**.
6. In the **Code source** pane, replace the existing code by copying and pasting the following:
```
`import json
import hmac
import hashlib
import os
def lambda\_handler(event, context):
# Get the webhook secret from environment variables
webhook\_secret = os.environ['WEBHOOK\_SECRET']
# Verify the webhook signature
if not verify\_signature(event, webhook\_secret):
return {
'statusCode': 401,
'body': json.dumps({'error': 'Invalid signature'})
}
try:
# Handle different event types
event\_type = payload.get('type')
if event\_type == 'payment.success':
# Handle successful payment
order\_id = payload.get('orderId')
print(f"Processing successful payment for order {order\_id}")
# For example, update database, send notifications, etc.
elif event\_type == 'payment.failed':
# Handle failed payment
order\_id = payload.get('orderId')
print(f"Processing failed payment for order {order\_id}")
# Add your business logic here
else:
print(f"Received unhandled event type: {event\_type}")
# Return success response
return {
'statusCode': 200,
'body': json.dumps({'received': True})
}
except json.JSONDecodeError:
return {
'statusCode': 400,
'body': json.dumps({'error': 'Invalid JSON payload'})
}
except Exception as e:
print(f"Error processing webhook: {e}")
return {
'statusCode': 500,
'body': json.dumps({'error': 'Internal server error'})
}
def verify\_signature(event, webhook\_secret):
"""
Verify the webhook signature using HMAC
"""
try:
# Get the raw body (return an empty string if the body key doesn't exist)
body = event.get('body', '')
# Create HMAC using the secret key
expected\_signature = hmac.new(
webhook\_secret.encode('utf-8'),
body.encode('utf-8'),
hashlib.sha256
).hexdigest()
# Compare the expected signature with the received signature to authenticate the message
is\_valid = hmac.compare\_digest(signature, expected\_signature)
if not is\_valid:
print(f"Error: Invalid signature. Received: {signature}, Expected: {expected\_signature}")
return False
return True
except Exception as e:
print(f"Error verifying signature: {e}")
return False`
```
7. In the **DEPLOY** section, choose **Deploy** to update your function's code.
Node.js
###### Create the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Create a basic 'Hello world' function by doing the following:
1. Choose **Create function**.
2. Select **Author from scratch**.
3. For **Function name**, enter `myLambdaWebhook`.
4. For **Runtime**, select **nodejs24.x**.
5. Choose **Create function**.
6. In the **Code source** pane, replace the existing code by copying and pasting the following:
```
`import crypto from 'crypto';
export const handler = async (event, context) =&gt; {
// Get the webhook secret from environment variables
const webhookSecret = process.env.WEBHOOK\_SECRET;
// Verify the webhook signature
if (!verifySignature(event, webhookSecret)) {
return {
statusCode: 401,
body: JSON.stringify({ error: 'Invalid signature' })
};
}
try {
// Parse the webhook payload
const payload = JSON.parse(event.body);
// Handle different event types
const eventType = payload.type;
switch (eventType) {
case 'payment.success': {
// Handle successful payment
const orderId = payload.orderId;
console.log(`Processing successful payment for order ${orderId}`);
// Add your business logic here
// For example, update database, send notifications, etc.
break;
}
case 'payment.failed': {
// Handle failed payment
const orderId = payload.orderId;
console.log(`Processing failed payment for order ${orderId}`);
// Add your business logic here
break;
}
default:
console.log(`Received unhandled event type: ${eventType}`);
}
// Return success response
return {
statusCode: 200,
body: JSON.stringify({ received: true })
};
} catch (error) {
if (error instanceof SyntaxError) {
// Handle JSON parsing errors
return {
statusCode: 400,
body: JSON.stringify({ error: 'Invalid JSON payload' })
};
}
// Handle all other errors
console.error('Error processing webhook:', error);
return {
statusCode: 500,
body: JSON.stringify({ error: 'Internal server error' })
};
}
};
// Verify the webhook signature using HMAC
const verifySignature = (event, webhookSecret) =&gt; {
try {
// Get the signature from headers
const signature = event.headers['x-webhook-signature'];
if (!signature) {
console.log('No signature found in headers:', event.headers);
return false;
}
// Get the raw body (return an empty string if the body key doesn't exist)
const body = event.body || '';
// Create HMAC using the secret key
const hmac = crypto.createHmac('sha256', webhookSecret);
const expectedSignature = hmac.update(body).digest('hex');
// Compare expected and received signatures
const isValid = signature === expectedSignature;
if (!isValid) {
console.log(`Invalid signature. Received: ${signature}, Expected: ${expectedSignature}`);
return false;
}
return true;
} catch (error) {
console.error('Error during signature verification:', error);
return false;
}
};`
```
7. In the **DEPLOY** section, choose **Deploy** to update your function's code.
## Create the secret key
For the Lambda function to authenticate the webhook request, it uses a secret key which it shares with the calling application. In this example, the key is stored in an
environment variable. In a production application, don't include sensitive information like passwords in your function code. Instead, [create an AWS Secrets Manager secret](https://docs.aws.amazon.com/secretsmanager/latest/userguide/create_secret.html) and then [use the AWS Parameters and Secrets Lambda extension](./with-secrets-manager.html) to retrieve your credentials in your Lambda function.
###### Create and store the webhook secret key
1. Generate a long, random string using a cryptographically secure random number generator. You can use the following code snippets in Python or Node.js to generate and print a 32-character
secret, or use your own preferred method.
Python
###### Example code to generate a secret
```
`import secrets
webhook\_secret = secrets.token\_urlsafe(32)
print(webhook\_secret)`
```
Node.js
###### Example code to generate a secret (ES module format)
```
`import crypto from 'crypto';
let webhookSecret = crypto.randomBytes(32).toString('base64');
console.log(webhookSecret)`
```
2. Store your generated string as an environment variable for your function by doing the following:
1. In the **Configuration** tab for your function, select **Environment variables**.
2. Choose **Edit**.
3. Choose **Add environment variable**.
4. For **Key**, enter `WEBHOOK\_SECRET`, then for **Value**, enter the secret you generated in the previous step.
5. Choose **Save**.
You'll need to use this secret again later in the tutorial to test your function, so make a note of it now.
## Create the function URL endpoint
Create an endpoint for your webhook using a Lambda function URL. Beacuse you use the auth type of `NONE` to create an endpoint with public access,
anyone with the URL can invoke your function. To learn more about controlling access to function URLs, see [Control access to Lambda function URLs](./urls-auth.html).
If you need more advanced authentication options for your webhook, consider using API Gateway.
###### Create the function URL endpoint
1. In the **Configuration** tab for your function, select **Function URL**.
2. Choose **Create function URL**.
3. For **Auth type**, select **NONE**.
4. Choose **Save**.
The endpoint for the function URL you just created is displayed in the **Function URL** pane. Copy the endpoint to use later in the tutorial.
## Test the function in the console
Before using an HTTP request to invoke your function using the URL endpoint, test it in the console to confirm your code is working as expected.
To verify the function in the console, you first calculate a webhook signature using the secret you generated earlier in the tutorial with the following test JSON payload:
```
`{
"type": "payment.success",
"orderId": "1234",
"amount": "99.99"
}`
```
Use either of the following Python or Node.js code examples to calculate the webhook signature using your own secret.
Python
###### Calculate the webhook signature
1. Save the following code as a file named `calculate\_signature.py`. Replace the webhook secret in the code with your own value.
```
`import secrets
import hmac
import json
import hashlib
webhook\_secret = "`arlbSDCP86n\_1H90s0fL\_Qb2NAHBIBQOyGI0X4Zay4M`"
body = json.dumps({"type": "payment.success", "orderId": "1234", "amount": "99.99"})
signature = hmac.new(
webhook\_secret.encode('utf-8'),
body.encode('utf-8'),
hashlib.sha256
).hexdigest()
print(signature)`
```
2. Calculate the signature by running the following command from the same directory you saved the code in. Copy the signature the code outputs.
```
``python calculate\_signature.py``
```
Node.js
###### Calculate the webhook signature
1. Save the following code as a file named `calculate\_signature.mjs`. Replace the webhook secret in the code with your own value.
```
`import crypto from 'crypto';
const webhookSecret = "`arlbSDCP86n\_1H90s0fL\_Qb2NAHBIBQOyGI0X4Zay4M`"
const body = "{\\"type\\": \\"payment.success\\", \\"orderId\\": \\"1234\\", \\"amount\\": \\"99.99\\"}";
let hmac = crypto.createHmac('sha256', webhookSecret);
let signature = hmac.update(body).digest('hex');
console.log(signature);`
```
2. Calculate the signature by running the following command from the same directory you saved the code in. Copy the signature the code outputs.
```
``node calculate\_signature.mjs``
```
You can now test your function code using a test HTTP request in the console.
###### Test the function in the console
1. Select the **Code** tab for your function.
2. In the **TEST EVENTS** section, choose **Create new test event**
3. For **Event Name**, enter `myEvent`.
4. Replace the existing JSON by copying and pasting the following into the **Event JSON** pane. Replace the webhook signature with the
value you calculated in the previous step.
```
`{
"headers": {
"Content-Type": "application/json",
"x-webhook-signature": "`2d672e7a0423fab740fbc040e801d1241f2df32d2ffd8989617a599486553e2a`"
},
"body": "{\\"type\\": \\"payment.success\\", \\"orderId\\": \\"1234\\", \\"amount\\": \\"99.99\\"}"
}`
```
5. Choose **Save**.
6. Choose **Invoke**.
You should see output similar to the following:
Python
```
`Status: Succeeded
Test Event Name: myEvent
Response:
{
"statusCode": 200,
"body": "{\\"received\\": true}"
}
Function Logs:
START RequestId: 50cc0788-d70e-453a-9a22-ceaa210e8ac6 Version: $LATEST
Processing successful payment for order 1234
END RequestId: 50cc0788-d70e-453a-9a22-ceaa210e8ac6
REPORT RequestId: 50cc0788-d70e-453a-9a22-ceaa210e8ac6 Duration: 1.55 ms Billed Duration: 2 ms Memory Size: 128 MB Max Memory Used: 36 MB Init Duration: 136.32 ms`
```
Node.js
```
`Status: Succeeded
Test Event Name: myEvent
Response:
{
"statusCode": 200,
"body": "{\\"received\\":true}"
}
Function Logs:
START RequestId: e54fe6c7-1df9-4f05-a4c4-0f71cacd64f4 Version: $LATEST
2025-01-10T18:05:42.062Z e54fe6c7-1df9-4f05-a4c4-0f71cacd64f4 INFO Processing successful payment for order 1234
END RequestId: e54fe6c7-1df9-4f05-a4c4-0f71cacd64f4
REPORT RequestId: e54fe6c7-1df9-4f05-a4c4-0f71cacd64f4 Duration: 60.10 ms Billed Duration: 61 ms Memory Size: 128 MB Max Memory Used: 72 MB Init Duration: 174.46 ms
Request ID: e54fe6c7-1df9-4f05-a4c4-0f71cacd64f4`
```
## Test the function using an HTTP request
Use the curl command line tool to test your webhook endpoint.
###### Test the function using HTTP requests
1. In a terminal or shell program, run the following curl command. Replace the URL with the value for your own function URL endpoint and replace the webhook signature with the
signature you calculated using your own secret key.
```
``curl -X POST `https://ryqgmbx5xjzxahif6frvzikpre0bpvpf.lambda-url.us-west-2.on.aws/` \\
-H "Content-Type: application/json" \\
-H "x-webhook-signature: `d5f52b76ffba65ff60ea73da67bdf1fc5825d4db56b5d3ffa0b64b7cb85ef48b`" \\
-d '{"type": "payment.success", "orderId": "1234", "amount": "99.99"}'``
```
You should see the following output:
```
`{"received": true}`
```
2. Inspect the CloudWatch logs for your function to confirm it parsed the payload correctly by doing the following:
1. Open the [Logs group](https://console.aws.amazon.com/cloudwatch/home#logsV2:log-groups) page in the Amazon CloudWatch console.
2. Select your function's log group (`/aws/lambda/myLambdaWebhook`).
3. Select the most recent log stream.
You should see output similar to the following in your function's logs:
Python
```
`Processing successful payment for order 1234`
```
Node.js
```
`2025-01-10T18:05:42.062Z e54fe6c7-1df9-4f05-a4c4-0f71cacd64f4 INFO Processing successful payment for order 1234`
```
4. Confirm that your code detects an invalid signature by running the following curl command. Replace the URL with your own function URL endpoint.
```
``curl -X POST `https://ryqgmbx5xjzxahif6frvzikpre0bpvpf.lambda-url.us-west-2.on.aws/` \\
-H "Content-Type: application/json" \\
-H "x-webhook-signature: abcdefg" \\
-d '{"type": "payment.success", "orderId": "1234", "amount": "99.99"}'``
```
You should see the following output:
```
`{"error": "Invalid signature"}`
```
## Clean up your resources
You can now delete the resources that you created for this tutorial, unless you want to retain them. By deleting AWS resources that you're no longer using, you prevent unnecessary charges to your AWS account.
###### To delete the Lambda function
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Select the function that you created.
3. Choose **Actions**, **Delete**.
4. Type `confirm` in the text input field and choose **Delete**.
When you created the Lambda function in the console, Lambda also created an [execution role](./lambda-intro-execution-role.html) for your function.
###### To delete the execution role
1. Open the [Roles page](https://console.aws.amazon.com/iam/home#/roles) of the IAM console.
2. Select the execution role that Lambda created. The role has the name format `myLambdaWebhook-role-&lt;random string&gt;`.
3. Choose **Delete**.
4. Enter the name of the role in the text input field and choose **Delete**.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Function URLs vs Amazon API Gateway
Function scaling
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.