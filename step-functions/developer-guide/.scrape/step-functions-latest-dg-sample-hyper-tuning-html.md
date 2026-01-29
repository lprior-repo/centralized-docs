---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-hyper-tuning.html
title: Tune the hyperparameters of a machine learning model in SageMaker AI
word_count: 532
filtered: true
elements_removed: 0
density_score: 0.87
---

Tune the hyperparameters of a machine learning model in SageMaker AI - AWS Step Functions
Tune the hyperparameters of a machine learning model in SageMaker AI - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-hyper-tuning)
[Step 1: Create the state machine](#sample-hyper-tuning-create)[Step 2: Run the demo state machine](#sample-hyper-tuning-start-execution)
# Tune the hyperparameters of a machine learning model in SageMaker AI
This sample project demonstrates using SageMaker AI to tune the hyperparameters of a machine
learning model, and to batch transform a test dataset.
In this project, Step Functions uses a Lambda function to seed an Amazon S3 bucket with a test dataset.
It then creates a hyperparameter tuning job using the [SageMaker AI
service integration](./connect-sagemaker.html). It then uses a Lambda function to extract the data path,
saves the tuning model, extracts the model name, and then runs a batch transform job to
perform inference in SageMaker AI.
For more information about SageMaker AI and Step Functions service integrations, see the following:
* [Integrating services with Step Functions](./integrate-services.html)
* [Create and manage Amazon SageMaker AI jobs with Step Functions](./connect-sagemaker.html)
###### Note
This sample project may incur charges.
For new AWS users, a free usage tier is available. On this tier, services are free
below a certain level of usage. For more information about AWS costs and the Free
Tier, see [SageMaker AI
Pricing](https://aws.amazon.com/sagemaker/pricing/).
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
Train a machine learning model
Perform AI prompt-chaining with Amazon Bedrock
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.