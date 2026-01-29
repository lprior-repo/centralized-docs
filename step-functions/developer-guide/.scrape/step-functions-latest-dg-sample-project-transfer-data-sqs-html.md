---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-project-transfer-data-sqs.html
title: Transfer data records with Lambda, DynamoDB,
word_count: 449
filtered: true
elements_removed: 0
density_score: 0.88
---

Transfer data records with Lambda, DynamoDB, and Amazon SQS - AWS Step Functions
Transfer data records with Lambda, DynamoDB, and Amazon SQS - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-project-transfer-data-sqs)
[Step 1: Create the state machine](#sample-project-transfer-data-sqs-create)[Step 2: Run the demo state machine](#sample-sqs-start-execution)
# Transfer data records with Lambda, DynamoDB,
and Amazon SQS
This sample project demonstrates how to iteratively read items from an Amazon DynamoDB table and send these items to an Amazon SQS queue using a Step Functions state machine. Deploying this sample project will create a Step Functions state machine, a DynamoDB table, an AWS Lambda function, and an Amazon SQS queue.
In this project, Step Functions uses the Lambda function to populate the DynamoDB table. The state machine also uses a
`for` loop to read each of the entries, and then sends each entry to an Amazon SQS queue.
## Step 1: Create the state machine
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/) and choose **Create state machine**.
2. Choose **Create from template** and find the related starter template. Choose **Next** to continue.
3. Choose how to use the template:
1. **Run a demo** – creates a read-only state machine. After review, you can create the workflow and all related resources.
2. **Build on it** – provides an editable workflow definition that you can review, customize, and deploy with your own resources. (Related resources, such as functions or queues, will **not** be created automatically.)
3. Choose **Use template** to continue with your selection.
###### Note
*Standard charges apply for services deployed to your account.*
## Step 2: Run the demo state machine
If you chose the **Run a demo** option, all related resources will be deployed and ready to run. If you chose the **Build on it** option, you might need to set placeholder values and create additional resources before you can run your custom workflow.
1. Choose **Deploy and run**.
2. Wait for the CloudFormation stack to deploy. This can take up to 10 minutes.
3. After the **Start execution** option appears, review the **Input** and choose **Start execution**.
###### Congratulations!
You should now have a running demo of your state machine. You can choose states in the **Graph view** to review input, output, variables, definition, and events.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Manage a container task
Job poller
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.