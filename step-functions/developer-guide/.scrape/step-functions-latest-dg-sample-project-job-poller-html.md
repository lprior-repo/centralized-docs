---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-project-job-poller.html
title: Poll for job status with Lambda and AWS Batch
word_count: 489
filtered: true
elements_removed: 0
density_score: 0.88
---

Poll for job status with Lambda and AWS Batch - AWS Step Functions
Poll for job status with Lambda and AWS Batch - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-project-job-poller)
[Step 1: Create the state machine](#sample-project-job-poller-create)[Step 2: Run the demo state machine](#job-status-poller-start-execution)
# Poll for job status with Lambda and AWS Batch
This sample project creates an AWS Batch job poller. It implements an AWS Step Functions state
machine that uses AWS Lambda to create a `Wait` state loop that checks on an AWS Batch
job.
This sample project creates and configures all resources so that your Step Functions workflow will
submit an AWS Batch job, and will wait for that job to complete before ending successfully.
###### Note
You can also implement this pattern without using a Lambda function. For information about
controlling AWS Batch directly, see [Integrating services with Step Functions](./integrate-services.html).
This sample project creates the state machine, two Lambda functions, and an AWS Batch queue,
and configures the related IAM permissions.
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
Transfer data records
Task timer
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.