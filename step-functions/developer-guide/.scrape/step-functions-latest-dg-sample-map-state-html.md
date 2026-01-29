---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-map-state.html
title: Process data from a queue with a Map state in Step Functions
word_count: 622
filtered: true
elements_removed: 0
density_score: 0.89
---

Process data from a queue with a Map state in Step Functions - AWS Step Functions
Process data from a queue with a Map state in Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-map-state)
[Step 1: Create the state machine](#sample-map-state-create)[Step 2: Subscribe to the Amazon SNS topic](#sample-map-subscribe-topic)[Step 3: Add messages to the Amazon SQS queue](#sample-map-create-queue)[Step 4: Run the state machine](#sample-map-start-execution)
# Process data from a queue with a Map state in Step Functions
In this sample workflow, a [Map workflow state](./state-map.html) state processes data from a queue, sending messages to subscribers and storing them in a database.
Step Functions uses an optimized integration to pull messages from an Amazon SQS queue. When messages are available, a [Choice](./state-choice.html) state passes an array of JSON messages to a [Map](./state-map.html) state for processing. For each message, the state machine writes the message to DynamoDB, removes the message from the queue, and publishes the message to an Amazon SNS topic.
## Step 1: Create the state machine
1. Open the [Step Functions console](https://console.aws.amazon.com/states/home?region=us-east-1#/) and choose **Create state machine**.
2. Choose **Create from template** and find the related starter template. Choose **Next** to continue.
3. Choose how to use the template:
1. **Run a demo** – creates a read-only state machine. After review, you can create the workflow and all related resources.
2. **Build on it** – provides an editable workflow definition that you can review, customize, and deploy with your own resources. (Related resources, such as functions or queues, will **not** be created automatically.)
3. Choose **Use template** to continue with your selection.
###### Note
*Standard charges apply for services deployed to your account.*
###### Tip
Subscribe to the Amazon SNS topic and add items to the Amazon SQS queue **before** you run your state machine.
1. Open the [Amazon SNS console](https://console.aws.amazon.com/sns/home).
2. Choose **Topics** and find the topic that was created by the sample project.
3. Choose **Create subscription**, and for **Protocol**, choose **Email**.
4. Under **Endpoint**, enter your email address to subscribe to the
topic.
5. Choose **Create subscription**.
6. Confirm the subscription in your email to activate the subscription.
## Step 3: Add messages to the Amazon SQS queue
1. Open the [Amazon SQS console](https://console.aws.amazon.com/sqs/home).
2. Choose the queue that was created by the sample project.
3. Choose **Send and receive messages**, enter a message and choose
**Send message**. Repeat this step to add several messages to the queue.
###### Tip
Queues in Amazon SNS are eventually consistent. You may need to wait a few minutes after sending messages to the queue before running your state machine.
If you chose the **Run a demo** option, all related resources will be deployed and ready to run. If you chose the **Build on it** option, you might need to set placeholder values and create additional resources before you can run your custom workflow.
1. Choose **Deploy and run**.
2. Wait for the CloudFormation stack to deploy. This can take up to 10 minutes.
3. After the **Start execution** option appears, review the **Input** and choose **Start execution**.
###### Congratulations!
You should now have a running demo of your state machine. You can choose states in the **Graph view** to review input, output, variables, definition, and events.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Start a workflow within a workflow
Distributed Map to process a CSV file in S3
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.