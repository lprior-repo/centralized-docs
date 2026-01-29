---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-lambda-orchestration.html
title: Orchestrate AWS Lambda functions with Step Functions
word_count: 531
filtered: true
elements_removed: 0
density_score: 0.88
---

Orchestrate AWS Lambda functions with Step Functions - AWS Step Functions
Orchestrate AWS Lambda functions with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-lambda-orchestration)
[Step 1: Create the state machine](#sample-lambda-orchestration-create)[Step 2: Run the demo state machine](#sample-lambda-orchestration-start-execution)
# Orchestrate AWS Lambda functions with Step Functions
The **Orchestrate Lambda functions** template uses several Lambda functions in a sample stock trading workflow. One function checks a stock price, then a human is prompted to choose to buy or sell the stock. A choice state selects the next function based on the `recommended\_type` variable to complete the purchase or sale. After either function finishes, the result of the trade is then published before reaching the end of the workflow.
To implement the human approval step, the workflow execution pauses until a unique TaskToken is returned. In this project, the workflow passes a
message with the task token to an Amazon SQS queue. The message triggers another Lambda function that's
configured to handle a callback based on the payload of the message. The workflow pauses until it receives
the task token back from a [`SendTaskSuccess`](https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskSuccess.html) API call. For more information about task
tokens, see [Wait for a Callback with Task Token](./connect-to-resource.html#connect-wait-token).
![Illustrative view of the state machine](https://docs.aws.amazon.com/images/step-functions/latest/dg/images/sample-lambda-orchestration.png)
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
For more information about Step Functions service integrations, see [Integrating services with Step Functions](./integrate-services.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Preprocess data and train a machine learning model
Start an Athena query
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.