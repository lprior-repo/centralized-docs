---
url: https://docs.aws.amazon.com/step-functions/latest/dg/task-timer-sample.html
title: Create a task timer with Lambda and Amazon SNS
word_count: 540
filtered: true
elements_removed: 0
density_score: 0.88
---

Create a task timer with Lambda and Amazon SNS - AWS Step Functions
Create a task timer with Lambda and Amazon SNS - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#task-timer-sample)
[Step 1: Create the state machine](#task-timer-create-resources)[Step 2: Run the demo state machine](#task-timer-run-state-machine)
# Create a task timer with Lambda and Amazon SNS
This sample project creates a task timer. It implements an AWS Step Functions state machine that
implements a `Wait` state, and uses an AWS Lambda function that sends an Amazon Simple Notification Service
(Amazon SNS) notification. A [Wait workflow state](./state-wait.html) state is a state type that waits for a trigger to
perform a single unit of work.
###### Note
This sample project implements an AWS Lambda function to send an Amazon Simple Notification Service (Amazon SNS)
notification. You can also send an Amazon SNS notification directly from the Amazon States Language. See [Integrating services with Step Functions](./integrate-services.html).
This sample project creates the state machine, a Lambda function, and an Amazon SNS topic, and
configures the related AWS Identity and Access Management (IAM) permissions. For more information about the resources
that are created with the **Task Timer** sample project, see the
following:
For more information about how AWS Step Functions can control other AWS services, see
[Integrating services with Step Functions](./integrate-services.html).
* [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/)
* [Amazon Simple Notification Service Developer Guide](https://docs.aws.amazon.com/sns/latest/dg/)
* [AWS Lambda Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/)
* [IAM Getting Started Guide](https://docs.aws.amazon.com/IAM/latest/GettingStartedGuide/)
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
Job poller
Callback pattern example
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.