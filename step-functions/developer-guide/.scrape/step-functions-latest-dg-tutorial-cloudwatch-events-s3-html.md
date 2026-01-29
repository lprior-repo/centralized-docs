---
url: https://docs.aws.amazon.com/step-functions/latest/dg/tutorial-cloudwatch-events-s3.html
title: Starting a Step Functions workflow in response to events
word_count: 1165
filtered: true
elements_removed: 0
density_score: 0.84
---

Starting a Step Functions workflow in response to events - AWS Step Functions
Starting a Step Functions workflow in response to events - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#tutorial-cloudwatch-events-s3)
[Prerequisite: Create a State Machine](#tutorial-cloudwatch-events-s3-prereqs)[Step 1: Create a Bucket in Amazon S3](#tutorial-cloudwatch-events-s3-bucket)[Step 2: Enable Amazon S3 Event Notification with EventBridge](#tutorial-cloudwatch-events-s3-eb-notification)[Step 3: Create an Amazon EventBridge Rule](#tutorial-cloudwatch-events-s3-cwe)[Step 4: Test the Rule](#tutorial-cloudwatch-events-test-rule)[Example of Execution Input](#tutorial-cloudwatch-events-example)
# Starting a Step Functions workflow in response to events
You can execute an AWS Step Functions state machine in response to an event routed by an Amazon EventBridge rule to Step Functions as a target.
The following tutorial shows you how to configure a state machine as a target of an Amazon EventBridge rule. Whenever files are added to an Amazon Simple Storage Service (Amazon S3) bucket, the EventBridge rule will start the state machine.
A practical example of this approach could be a state machine that runs Amazon Rekognition analysis on image files added to the bucket to categorize and assign keywords.
In this tutorial, you start the execution of a `Helloworld` state machine by uploading a file to an Amazon S3 bucket. Then you review the example input of that
execution to identify the information that is included in input from the Amazon S3 event notification delivered to EventBridge.
## Prerequisite: Create a State Machine
Before you can configure a state machine as an Amazon EventBridge target, you must create the state machine.
* To create a basic state machine, use the [Creating state machine that uses a Lambda function](./tutorial-creating-lambda-state-machine.html) tutorial.
* If you already have a `Helloworld` state machine, proceed to the next step.
## Step 1: Create a Bucket in Amazon S3
Now that you have a `Helloworld` state machine, you need to create an Amazon S3 bucket which stores your files. In
Step 3 of this tutorial, you set up a rule so that when a file is uploaded to this bucket, EventBridge triggers an execution of your state machine.
1. Navigate to the [Amazon S3 console](https://console.aws.amazon.com/s3/), and then choose **Create bucket** to create the bucket in
which you want to store your files and trigger an Amazon S3 event rule.
2. Enter a **Bucket name**, such as ``username`-sfn-tutorial`.
###### Note
Bucket names must be unique across all existing bucket names in all AWS Regions in Amazon S3. Use your own `username` to make
this name unique. You need to create all resources in the same AWS Region.
3. Keep all the default selections on the page, and choose **Create bucket**.
## Step 2: Enable Amazon S3 Event Notification with EventBridge
After you create the Amazon S3 bucket, configure it to send events to EventBridge whenever certain events happen in your S3 bucket, such as file uploads.
1. Navigate to the [Amazon S3 console](https://console.aws.amazon.com/s3/).
2. In the **Buckets** list, choose the name of the bucket that you want to enable events for.
3. Choose **Properties**.
4. Scroll down the page to view the **Event Notifications** section, and then choose **Edit** in the
**Amazon EventBridge** subsection.
5. Under **Send notifications to Amazon EventBridge for all events in this bucket**, choose **On**.
6. Choose **Save changes**.
###### Note
After you enable EventBridge, it takes around five minutes for the changes to take effect.
## Step 3: Create an Amazon EventBridge Rule
After you have a state machine, and have created the Amazon S3 bucket and configured it to send event notifications to EventBridge, create an EventBridge rule.
###### Note
You must configure EventBridge rule in the same AWS Region as the Amazon S3 bucket.
### To create the
rule
1. Navigate to the [Amazon EventBridge console](https://console.aws.amazon.com/events/), choose **Create rule**.
###### Tip
Alternatively, in the navigation pane on the EventBridge console, choose
**Rules** under **Buses**, and then choose **Create rule**.
2. Enter a **Name** for your rule (for example, ``S3Step Functions``) and optionally enter a **Description**
for the rule.
3. For **Event bus** and **Rule type**, keep the default selections.
4. Choose **Next**. This opens the **Build event pattern** page.
5. Scroll down to the **Event pattern** section, and do the following:
1. For **Event source**, keep the default selection of **AWS events or EventBridge partner events**.
2. For **AWS service**, choose **Simple Storage Service (S3)**.
3. For **Event type**, choose **Amazon S3 Event Notification**.
4. Choose **Specific event(s)**, and then choose **Object Created**.
5. Choose **Specific bucket(s) by name** and enter the bucket name you created in
[Step 1](#tutorial-cloudwatch-events-s3-bucket) (``username`-sfn-tutorial`) to store your
files.
6. Choose **Next**. This opens the **Select target(s)** page.
### To create the target
1. In **Target 1**, keep the default selection of **AWS service**.
2. In the **Select a target** dropdown list, select **Step Functions state machine**.
3. In the **State machine** list, select the state machine that you [created earlier](#tutorial-cloudwatch-events-s3-prereqs) (for example, `Helloworld`).
4. Keep all the default selections on the page, and choose **Next**. This opens the **Configure tags** page.
5. Choose **Next** again. This opens the **Review and create** page.
6. Review the details of the rule and choose **Create rule**.
The rule is created and the **Rules** page is displayed, listing all your Amazon EventBridge rules.
## Step 4: Test the Rule
Now that everything is in place, test adding a file to the Amazon S3 bucket, and then look at the input of the resulting state machine execution.
1. Add a file to your Amazon S3 bucket.
Navigate to the [Amazon S3 console](https://console.aws.amazon.com/s3/), choose the bucket you created to store files
(``username`-sfn-tutorial`), and then choose **Upload**.
2. Add a file, for example ``test.png``, and then choose **Upload**.
This launches an execution of your state machine, passing information from
AWS CloudTrail as the input.
3. Check the execution for your state machine.
Navigate to the [Step Functions console and
select the state machine used in your Amazon EventBridge rule
(`Helloworld`)](https://console.aws.amazon.com/states/).
4. Select the most recent execution of that state machine and expand the
**Execution Input** section.
This input includes information such as the bucket name and the object name.
In a real-world use case, a state machine can use this input to perform actions
on that object.
## Example of Execution Input
The following example shows a typical input to the state machine execution.
```
`{
"version": "0",
"id": "6c540ad4-0671-9974-6511-756fbd7771c3",
"detail-type": "Object Created",
"source": "aws.s3",
"account": "123456789012",
"time": "2023-06-23T23:45:48Z",
"region": "us-east-2",
"resources": [
"arn:aws:s3:::``username`-sfn-tutorial`"
],
"detail": {
"version": "0",
"bucket": {
"name": "``username`-sfn-tutorial`"
},
"object": {
"key": "test.png",
"size": 800704,
"etag": "f31d8546bb67845b4d3048cde533b937",
"sequencer": "00621049BA9A8C712B"
},
"request-id": "79104EXAMPLEB723",
"requester": "123456789012",
"source-ip-address": "200.0.100.11",
"reason": "PutObject"
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Process individual items with Lambda
Create an API using API Gateway
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.