---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-batch-fan-out.html
title: Fan out batch jobs with Map state
word_count: 406
filtered: true
elements_removed: 0
density_score: 0.89
---

Fan out batch jobs with Map state - AWS Step Functions
Fan out batch jobs with Map state - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-batch-fan-out)
[Step 1: Create the state machine](#sample-batch-fan-out-create)[Step 2: Run the demo state machine](#sample-batch-fan-out-start-execution)
# Fan out batch jobs with Map state
This sample project demonstrates how to use Step Functions’s [Map workflow state](./state-map.html) state to fan out AWS Batch jobs.
In this project, Step Functions uses a state machine to invoke a Lambda function to do simple pre-processing, then invokes
multiple AWS Batch jobs in parallel using the [Map workflow state](./state-map.html) state.
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
Manage a batch job
Batch job with Lambda
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.