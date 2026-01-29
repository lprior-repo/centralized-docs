---
url: https://docs.aws.amazon.com/step-functions/latest/dg/callback-task-sample-sqs.html
title: Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda
word_count: 439
filtered: true
elements_removed: 0
density_score: 0.89
---

Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda - AWS Step Functions
Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#callback-task-sample-sqs)
[Step 1: Create the state machine](#callback-pattern-create-resources)[Step 2: Run the demo state machine](#callback-pattern-run-state-machine)
# Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda
This sample project demonstrates how to have AWS Step Functions pause during a task, and wait for an
external process to return a task token that was generated when the task started.
To learn how to implement the callback pattern in Step Functions, see [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token).
For more information about how AWS Step Functions can control other AWS services, see [Integrating services with Step Functions](./integrate-services.html).
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
Task timer
Manage an Amazon EMR job
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.