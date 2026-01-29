---
url: https://docs.aws.amazon.com/step-functions/latest/dg/synchronous-execution.html
title: Invoke Synchronous Express Workflows through API Gateway
word_count: 470
filtered: true
elements_removed: 0
density_score: 0.89
---

Invoke Synchronous Express Workflows through API Gateway - AWS Step Functions
Invoke Synchronous Express Workflows through API Gateway - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#synchronous-execution)
[Step 1: Create the state machine](#synchronous-execution-create)[Step 2: Run the demo state machine](#synchronous-execution-start-execution)
# Invoke Synchronous Express Workflows through API Gateway
This sample project demonstrates how to invoke Synchronous Express Workflows through Amazon API Gateway
to manage an employee database.
In this project, Step Functions uses API Gateway endpoints to start Step Functions Synchronous Express Workflows.
These then use DynamoDB to search for, add, and remove employees in an employee
database.
For more information about Step Functions Synchronous Express Workflows, see [Synchronous and Asynchronous Express
Workflows in Step Functions](./choosing-workflow-type.html#concepts-express-synchronous).
###### Note
This sample project may incur charges.
For new AWS users, a free usage tier is available. On this tier, services are free below
a certain level of usage. For more information about AWS costs and the Free Tier, see [Step Functions Pricing](https://aws.amazon.com/step-functions/pricing/).
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
Send a custom event to EventBridge
ETL job in Amazon Redshift
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.