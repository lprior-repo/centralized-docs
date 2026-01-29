---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-project-express-selective-checkpointing.html
title: Perform selective checkpointing using Standard and Express workflows
word_count: 457
filtered: true
elements_removed: 0
density_score: 0.88
---

Perform selective checkpointing using Standard and Express workflows - AWS Step Functions
Perform selective checkpointing using Standard and Express workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-project-express-selective-checkpointing)
[Step 1: Create the State Machine](#sample-project-express-selective)[Step 2: Run the demo state machine](#sample-selective-checkpointing-start-execution)
# Perform selective checkpointing using Standard and Express workflows
This sample project demonstrates how to combine Standard and Express Workflows by running
a mock e-commerce workflow that does selective checkpointing. Deploying this sample project
creates a Standard workflows state machine, a nested Express Workflows state machine, an
AWS Lambda function, an Amazon Simple Queue Service (Amazon SQS) queue, and an Amazon Simple Notification Service (Amazon SNS) topic.
For more information about Express Workflows, nested workflows, and Step Functions service
integrations, see the following:
* [Choosing workflow type in Step Functions](./choosing-workflow-type.html)
* [Start workflow executions from a task state in Step Functions](./concepts-nested-workflows.html)
* [Integrating services with Step Functions](./integrate-services.html)
## Step 1: Create the State Machine
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
Process high-volume messages from SQS
Start a CodeBuild build
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.