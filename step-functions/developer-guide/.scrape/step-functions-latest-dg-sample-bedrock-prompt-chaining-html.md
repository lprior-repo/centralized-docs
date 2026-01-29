---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-bedrock-prompt-chaining.html
title: Perform AI prompt-chaining with Amazon Bedrock
word_count: 536
filtered: true
elements_removed: 0
density_score: 0.89
---

Perform AI prompt-chaining with Amazon Bedrock - AWS Step Functions
Perform AI prompt-chaining with Amazon Bedrock - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-bedrock-prompt-chaining)
[Prerequisites](#sample-bedrock-prerequisites)[Step 1: Create the state machine](#sample-bedrock-create)[Step 2: Run the demo state machine](#sample-bedrock-run)
# Perform AI prompt-chaining with Amazon Bedrock
This sample project demonstrates how you can integrate with Amazon Bedrock to perform AI prompt-chaining and build high-quality chatbots using Amazon Bedrock. The project chains together some prompts and resolves them in the sequence in which they're provided. Chaining of these prompts augments the ability of the language model being used to deliver a highly-curated response.
This sample project creates the state machine, the supporting AWS resources, and configures the related IAM permissions. Explore this sample project to learn about using Amazon Bedrock optimized service integration with Step Functions state machines, or use it as a starting point for your own projects.
## Prerequisites
This sample project uses the Cohere Command large language model (LLM). To successfully run this sample project, you must add access to this LLM from the Amazon Bedrock console. To add the model access, do the following:
1. Open the [Amazon Bedrock console](https://console.aws.amazon.com/bedrock).
2. On the navigation pane, choose **Model access**.
3. Choose **Manage model access**.
4. Select the check box next to **Cohere**.
5. Choose **Request access**. The **Access status** for **Cohere** model shows as **Access granted**.
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
Tune a machine learning model
Process high-volume messages from SQS
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.