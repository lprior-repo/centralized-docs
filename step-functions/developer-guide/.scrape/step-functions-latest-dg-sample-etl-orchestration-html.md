---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-etl-orchestration.html
title: Run an ETL/ELT workflow using Step Functions and the Amazon Redshift API
word_count: 630
filtered: true
elements_removed: 0
density_score: 0.88
---

Run an ETL/ELT workflow using Step Functions and the Amazon Redshift API - AWS Step Functions
Run an ETL/ELT workflow using Step Functions and the Amazon Redshift API - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-etl-orchestration)
[Step 1: Create the state machine](#sample-etl-orchestration-create)[Step 2: Run the demo state machine](#sample-etl-orchestration-start-execution)
# Run an ETL/ELT workflow using Step Functions and the Amazon Redshift API
This sample project demonstrates how to use Step Functions and the Amazon Redshift Data API to run an ETL/ELT workflow that loads data into the Amazon Redshift data warehouse.
In this project, Step Functions uses an AWS Lambda function and the Amazon Redshift Data API to create the required database objects and to generate a set of example data,
then executes two jobs in parallel that perform loading dimension tables, followed by a fact table. Once both dimension load jobs end successfully, Step Functions
executes the load job for the fact table, runs the validation job, then pauses the Amazon Redshift cluster.
###### Note
You can modify the ETL logic to receive data from other sources such as Amazon S3, which can use the [COPY](https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html) command to copy data from Amazon S3 to an Amazon Redshift table.
For more information about Amazon Redshift and Step Functions service integrations, see the following guides:
* [Integrating services with Step Functions](./integrate-services.html)
* [Using the Amazon Redshift Data API](https://docs.aws.amazon.com/redshift/latest/mgmt/data-api.html)
* [Amazon Redshift Data API service](https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/rds-data.html)
* [Creating a Step Functions state machine that uses Lambda](./tutorial-creating-lambda-state-machine.html)
For more information about IAM policies for Lambda and Amazon Redshift, see the following guides:
* [IAM policies for calling AWS Lambda](./connect-lambda.html#lambda-iam)
* [Authorizing access to the Amazon Redshift Data API](https://docs.aws.amazon.com/redshift/latest/mgmt/data-api.html#data-api-access)
###### Note
This sample project may incur charges.
For new AWS users, a free usage tier is available. On this tier, services are free below a certain level of usage. For more information about AWS
costs and the Free Tier, see [AWS Step Functions pricing](https://aws.amazon.com/step-functions/pricing/).
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
Invoke Synchronous Express Workflows through API Gateway
Manage a batch job
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.