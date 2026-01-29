---
url: https://docs.aws.amazon.com/step-functions/latest/dg/starter-templates.html
title: Deploy a state machine using a starter template for Step Functions
word_count: 594
filtered: true
elements_removed: 0
density_score: 0.93
---

Deploy a state machine using a starter template for Step Functions - AWS Step Functions
Deploy a state machine using a starter template for Step Functions - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#starter-templates)
# Deploy a state machine using a starter template for Step Functions
To deploy state machines for a variety of example use cases and patterns, you can choose one of the following starter templates in the [AWS Step Functions
console](https://console.aws.amazon.com/states/home?region=us-east-1#/). These starter templates are ready-to-run sample projects that automatically create the workflow prototype and definition, and all related AWS resources for the project.
You can use these sample projects to deploy and run them as is, or use the workflow prototypes to build on them. If you build upon these projects, Step Functions creates the workflow prototype, but doesn't deploy the resources listed in the workflow definition.
When you deploy the sample projects, they provision a fully functional state machine, and create the
related resources for the state machine to run. When you create a sample project, Step Functions uses CloudFormation to create
the related resources referenced by the state machine.
###### List of starter templates
* [Manage a container task with Amazon ECS and Amazon SNS](./sample-project-container-task-notification.html)
* [Transfer data records with Lambda, DynamoDB,
and Amazon SQS](./sample-project-transfer-data-sqs.html)
* [Poll for job status with Lambda and AWS Batch](./sample-project-job-poller.html)
* [Create a task timer with Lambda and Amazon SNS](./task-timer-sample.html)
* [Create a callback pattern example with Amazon SQS, Amazon SNS, and Lambda](./callback-task-sample-sqs.html)
* [Manage an Amazon EMR job](./sample-emr-job.html)
* [Run an EMR Serverless job](./sample-emr-serverless-job.html)
* [Start a workflow within a workflow with Step Functions and Lambda](./sample-start-workflow.html)
* [Process data from a queue with a Map state in Step Functions](./sample-map-state.html)
* [Process a CSV file from Amazon S3 using a Distributed Map](./sample-dist-map-csv-process.html)
* [Process data in an Amazon S3 bucket with Distributed Map](./sample-dist-map-s3data-process.html)
* [Train a machine learning model using Amazon SageMaker AI](./sample-train-model.html)
* [Tune the hyperparameters of a machine learning model in SageMaker AI](./sample-hyper-tuning.html)
* [Perform AI prompt-chaining with Amazon Bedrock](./sample-bedrock-prompt-chaining.html)
* [Process high-volume messages from Amazon SQS
with Step Functions Express workflows](./sample-project-express-high-volume-sqs.html)
* [Perform selective checkpointing using Standard and Express workflows](./sample-project-express-selective-checkpointing.html)
* [Build an AWS CodeBuild project using Step Functions](./sample-project-codebuild.html)
* [Preprocess data and train a machine learning model with Amazon SageMaker AI](./sample-preprocess-feature-transform.html)
* [Orchestrate AWS Lambda functions with Step Functions](./sample-lambda-orchestration.html)
* [Start an Athena query and send a results notification](./sample-athena-query.html)
* [Execute queries in sequence and parallel using Athena](./run-multiple-queries.html)
* [Query large datasets using an AWS Glue crawler](./sample-query-large-datasets.html)
* [Keep data in a target table updated with AWS Glue and Athena](./sample-keep-data-updated.html)
* [Create and manage an Amazon EKS cluster with a node group](./sample-eks-cluster.html)
* [Interact with an API managed by API Gateway](./sample-apigateway-workflow.html)
* [Call a microservice running on Fargate using API Gateway integration](./sample-apigateway-ecs-workflow.html)
* [Send a custom event to an EventBridge event bus](./sample-eventbridge-custom-event.html)
* [Invoke Synchronous Express Workflows through API Gateway](./synchronous-execution.html)
* [Run an ETL/ELT workflow using Step Functions and the Amazon Redshift API](./sample-etl-orchestration.html)
* [Manage a batch job with AWS Batch and Amazon SNS](./batch-job-notification.html)
* [Fan out batch jobs with Map state](./sample-batch-fan-out.html)
* [Run an AWS Batch job with Lambda](./sample-batch-lambda.html)
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Access cross-account resources
Manage a container task
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.