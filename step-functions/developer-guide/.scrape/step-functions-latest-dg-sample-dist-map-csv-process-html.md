---
url: https://docs.aws.amazon.com/step-functions/latest/dg/sample-dist-map-csv-process.html
title: Process a CSV file from Amazon S3 using a Distributed Map
word_count: 585
filtered: true
elements_removed: 0
density_score: 0.87
---

Process a CSV file from Amazon S3 using a Distributed Map - AWS Step Functions
Process a CSV file from Amazon S3 using a Distributed Map - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#sample-dist-map-csv-process)
[Step 1: Create the state machine](#sample-dist-map-csv-create)[Step 2: Run the demo state machine](#sample-dist-map-csv-run)
# Process a CSV file from Amazon S3 using a Distributed Map
This sample project demonstrates how you can use the [Distributed Map state](./state-map-distributed.html) to iterate over 10,000 rows of a CSV file
that is generated using a Lambda function. The CSV file contains shipping information of customer orders and is stored in an Amazon S3 bucket. The
Distributed Map iterates over a batch of 10 rows in the CSV file for data analysis.
The Distributed Map contains a Lambda function to detect any delayed orders. The Distributed Map also contains an [Inline Map](./state-map-inline.html) to process the delayed orders in a batch and returns these delayed orders in an array. For each delayed order, the Inline Map sends
a message to an Amazon SQS queue. Finally, this sample project stores the [Map Run](./concepts-examine-map-run.html) results to
another Amazon S3 bucket in your AWS account.
With Distributed Map, you can run up to 10,000 parallel child workflow executions at a time. In this sample project, the maximum concurrency of Distributed Map is
set at 1000 that limits it to 1000 parallel child workflow executions.
This sample project creates the state machine, the supporting AWS resources, and configures the related IAM permissions. Explore this sample project
to learn about using the Distributed Map for orchestrating large-scale, parallel workloads, or use it as a starting point for your own projects.
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
Process data with a Map
Distributed Map to process files in S3
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.