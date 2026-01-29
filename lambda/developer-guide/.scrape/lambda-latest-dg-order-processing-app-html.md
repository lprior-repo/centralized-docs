---
url: https://docs.aws.amazon.com/lambda/latest/dg/order-processing-app.html
title: Creating an Order Processing System with Lambda Durable Functions
word_count: 462
filtered: true
elements_removed: 0
density_score: 0.86
---

Creating an Order Processing System with Lambda Durable Functions - AWS Lambda
Creating an Order Processing System with Lambda Durable Functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#order-processing-app)
[Prerequisites](#order-processing-prerequisites)[Create the Source Code Files](#order-processing-source)[Deploy the App](#order-processing-deploy)[Test the App](#order-processing-test)[Next Steps](#order-processing-next-steps)
# Creating an Order Processing System with Lambda Durable Functions
###### Note
NEED: Add architecture diagram showing API Gateway, Durable Function workflow, and supporting services (DynamoDB, EventBridge)
## Prerequisites
* AWS CLI installed and configured
* NEED: Specific Durable Functions requirements
## Create the Source Code Files
Create the following files in your project directory:
* `lambda\_function.py` - the function code
* `requirements.txt` - dependencies manifest
### Function Code
```
`# NEED: Verify correct imports
import boto3
import json
def lambda\_handler(event, context):
# Validate and store order
order = await durable.step('validate', async () =&gt;&gt; {
return validate\_order(event['order'])
})
# NEED: Verify wait syntax
await durable.wait(/\* wait configuration \*/)
### Requirements File
```
`# NEED: List of required packages`
```
### Create a DynamoDB Table for Orders
1. Open the DynamoDB console at [https://console.aws.amazon.com/dynamodb/](https://console.aws.amazon.com/dynamodb/)
2. Choose **Create table**
3. For **Table name**, enter `Orders`
4. For **Partition key**, enter `orderId`
5. Leave other settings as default
6. Choose **Create table**
### Create the Lambda Function
1. Open the Lambda console at [https://console.aws.amazon.com/lambda/](https://console.aws.amazon.com/lambda/)
2. Choose **Create function**
3. Select **Author from scratch**
4. For **Function name**, enter `ProcessOrder`
5. For **Runtime**, choose your preferred runtime
6. NEED: Add Durable Functions-specific configuration
7. Choose **Create function**
### Create the API Gateway Endpoint
1. Open the API Gateway console at [https://console.aws.amazon.com/apigateway/](https://console.aws.amazon.com/apigateway/)
2. Choose **Create API**
3. Select **HTTP API**
4. Choose **Build**
5. Add an integration with your Lambda function
6. Configure routes for order processing
7. Deploy the API
## Test the App
Submit a test order:
```
`{
"orderId": "12345",
"items": [
{
"productId": "ABC123",
"quantity": 1
}
]
}`
```
NEED: Add specific monitoring instructions for Durable Functions
### Add Business Logic
Implement inventory management:
```
`async def check\_inventory(order):
# Add inventory check logic
pass`
```
Add price calculations:
```
`async def calculate\_total(order):
### Improve Error Handling
Add compensation logic:
```
`async def reverse\_payment(order):
# Add payment reversal logic
pass`
```
Handle order cancellations:
```
`async def cancel\_order(order):
### Enhance Monitoring
* Create CloudWatch dashboards
* Set up metrics for order processing times
* Configure alerts for delayed orders
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Scheduled-maintenance app
Development tools
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.