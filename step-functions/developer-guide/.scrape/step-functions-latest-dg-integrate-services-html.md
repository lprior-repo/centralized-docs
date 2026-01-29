---
url: https://docs.aws.amazon.com/step-functions/latest/dg/integrate-services.html
title: Integrating services with Step Functions
word_count: 541
filtered: true
elements_removed: 0
density_score: 0.88
---

Integrating services with Step Functions - AWS Step Functions
Integrating services with Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#integrate-services)
[AWS SDK integrations](#connect-to-services-awssdk)[Optimized integrations](#connect-to-services-optimized)[Call HTTPS APIs](#connect-to-services-https)
# Integrating services with Step Functions
Learn how to integrate AWS services and call HTTPS APIs with Step Functions. With service integrations, your workflows can coordinate resources and orchestrate your business processes.
Depending on workflow type and availability, your workflows call services using one of three service integration patterns:
* Request a Response (default) - wait for HTTP response, then go to the next state
* Run a Job (`.sync`) - wait for the job to complete
* Wait for Callback (`.waitForTaskToken`) - pause a workflow until a task token is returned
To learn more, see [Service integration patterns](./connect-to-resource.html). And to learn more about controlling the flow of data to your integrated services, see [Passing parameters to a service API in Step Functions](./connect-parameters.html).
## AWS SDK integrations
AWS SDK integrations work exactly like an API call using the AWS SDK.
Using [AWS SDK integrations](./supported-services-awssdk.html), your state machines can call over nine thousand API actions for over two hundred AWS services.
###### Example integrations you might use:
* Invoke a AWS Lambda function.
* Run an AWS Batch job and take different actions based on the results.
* Retrieve or updated items in Amazon DynamoDB.
* Run an Amazon Elastic Container Service (Amazon ECS) task and wait for it to complete.
* Publish to a topic in Amazon Simple Notification Service (Amazon SNS).
* Send a message in Amazon Simple Queue Service (Amazon SQS).
* Manage a job for AWS Glue or Amazon SageMaker AI.
* Build workflows for executing Amazon EMR jobs.
* Launch another AWS Step Functions workflow execution.
## Optimized integrations
In addition to standard integrations, Step Functions provides optimized integrations which provide enhanced functionality. Optimized integrations have been customized by Step Functions to provide an improved developer experience when integrating the service in a workflow context.
For example, the optimized [Lambda
Invoke](./connect-lambda.html) automatically converts API output from escaped JSON to a JSON
object which you can more easily use. Another example is how [AWS BatchSubmitJob](./connect-batch.html) can pause execution until the batch job completes, which is a common scenario.
When possible, we **recommend** using the optimized integrations.
For the full list of optimized integrations, see the dedicated chapter for [Integrating optimized services with Step Functions](./integrate-optimized.html)
## Call HTTPS APIs
An HTTP Task is a type of [Task workflow state](./state-task.html) state that you can use to call HTTPS APIs in your workflows. The API can be
public, such as third-party SaaS applications like Stripe or Salesforce. You can also call
private API, such as HTTPS-based applications in an Amazon Virtual Private Cloud.
For more information, see [Call HTTPS APIs in Step Functions
workflows](./call-https-apis.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Parsing input CSV files
AWS SDK integrations
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.