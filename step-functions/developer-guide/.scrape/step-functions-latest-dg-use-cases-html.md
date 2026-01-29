---
url: https://docs.aws.amazon.com/step-functions/latest/dg/use-cases.html
title: Discover use cases for Step Functions workflows
word_count: 1702
filtered: true
elements_removed: 0
density_score: 0.79
---

Discover use cases for Step Functions workflows - AWS Step Functions
Discover use cases for Step Functions workflows - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#use-cases)
[Data processing](#use-cases-data-processing)[Machine learning](#use-cases-machine-learning)[Microservice orchestration](#use-cases-orchestration)[IT and security automation](#use-cases-security-automation)
# Discover use cases for Step Functions workflows
With AWS Step Functions, you can build workflows that manage state over time, make decisions based on incoming data, and handle errors and exceptions.
###### Use case categories
* [Data processing](#use-cases-data-processing)
* [Machine learning](#use-cases-machine-learning)
* [Microservice orchestration](#use-cases-orchestration)
* [IT and security automation](#use-cases-security-automation)
## Data processing
As the volume of data grows from diverse sources, organizations need to process their
data faster so they can quickly make well-informed business decisions. To process data
at scale, organizations need to elastically provision resources to manage the
information they receive from mobile devices, applications, satellites, marketing and
sales, operational data stores, infrastructure, and more.
With horizontal scaling and fault-tolerant workflows, Step Functions can operate millions of
concurrent executions. You can process your data faster using parallel executions with
`[Parallel workflow state](./state-parallel.html)` state. Or, you can use
the dynamic parallelism of the `[Map workflow state](./state-map.html)` state to iterate over large data sets in a data
stores, such as Amazon S3 buckets. Step Functions also provide the capability to retry failed
executions, or choose a specific path to handle errors without managing complex error
handling processes.
Step Functions directly integrates with other data processing services provided by AWS such as
[AWS Batch](./connect-batch.html) for batch processing, [Amazon EMR](./connect-emr.html) for big data processing, [AWS Glue](./connect-glue.html) for data preparation, [Athena](./connect-athena.html) for data
analysis, and [AWS Lambda](./connect-lambda.html) for compute.
Examples of the types of data processing workflows that customers use Step Functions to
accomplish include:
**File, video, and image processing**
* Take a collection of video files and convert them to other sizes or resolutions
that are ideal for the device they will be displayed on, such as mobile phones,
laptops, or a television.
* Take a large collection of photos uploaded by users and convert them into
thumbnails or various resolution images that can then be displayed on users’
websites.
* Take semi-structured data, such as a CSV file, and combine it with unstructured
data, such as an invoice, to produce a business report that is sent to business
stakeholders monthly.
* Take earth observing data collected from satellites, convert it into formats that
align with each other and then add other data sources collected on earth for
additional insight.
* Take the transportation logs from various modes of transportation for products and
look for optimizations using Monte Carlo Simulations and then send reports back to
the organizations and people that are relying on you to ship their goods.
**Coordinate extract, transform and load (ETL)
jobs:**
* Combine sales opportunity records with marketing metric datasets through a series
of data preparation steps using AWS Glue, and produce business intelligence reports that
can be used across the organization.
* Create, start, and terminate an Amazon EMR cluster for big data processing.
**Batch processing and High Performance Computing (HPC)
workloads:**
* Build a genomics secondary analysis pipeline that processes raw whole genome
sequences into variant calls. Align raw files to a reference sequence, and call
variants on a specified list of chromosomes using dynamic parallelism.
* Find efficiencies in the production of your next mobile device or other
electronics by simulating various layouts using different electric and chemical
compounds. Run large batch processing of your workloads through various simulations
to get the optimal design.
## Machine learning
Machine learning provides a way for organizations to quickly analyze collected data to
identify patterns and make decisions with minimal human intervention. Machine learning
starts with an initial set of data, known as *training
data*. Training data increases a machine learning model’s prediction accuracy
and acts as the foundation through which the model learns. After the trained model is
considered accurate enough to meet business needs, you can deploy the model into
production. The [AWS Step Functions Data Science Project
on Github](https://github.com/aws/aws-step-functions-data-science-sdk-python) is an open-source library that provides workflows to
preprocess data, train, and then publish your models using Amazon SageMaker AI and Step Functions.
Preprocessing existing data sets is how an organization often creates training data.
This preprocessing method adds information, such as by labeling objects in an image,
annotating text or processing audio. To preprocess data you can use AWS Glue, or you can
create an SageMaker AI notebook instance that runs in a Jupyter Notebook. After your data is
ready, it can be uploaded to Amazon S3 for access. As machine learning models are trained,
you can make adjustments to each model’s parameters to improve accuracy.
Step Functions provides a way to orchestrate end-to-end machine learning workflows on SageMaker AI. These
workflows can include data preprocessing, post-processing, feature engineering, data
validation, and model evaluation. After the model has been deployed to production, you
can refine and test new approaches to continually improve business outcomes. You can
create production-ready workflows directly in Python, or you can use the Step Functions Data
Science SDK to copy that workflow, experiment with new options, and place the refined
workflow in production.
Some types of machine learning workflows that customers use Step Functions for include:
**Fraud Detection**
* Identify and prevent fraudulent transactions, such as credit fraud, from
occurring.
* Detect and prevent account takeovers using trained machine learning models.
* Identify promotional abuse, including the creation of fake accounts, so you can
quickly take action.
**Personalization and Recommendations**
* Recommend products to targeted customers based upon what is predicted to attract
their interest.
* Predict whether a customer will upgrade their account from a free tier to a paid
subscription.
**Data Enrichment**
* Use data enrichment as part of preprocessing to provide better training data for
more accurate machine learning models.
* Annotate text and audio excerpts to add syntactical information, such as sarcasm
and slang.
* Label additional objects in images to provide critical information for the model
to learn from, such as whether an object is an apple, a basketball, a rock, or an
animal.
## Microservice orchestration
Step Functions gives you options to manage your microservice workflows.
Microservice architecture breaks applications into loosely coupled services. Benefits
include improved scalability, increased resiliency, and faster time to market. Each
microservice is independent, making it easy to scale up a single service or function
without needing to scale the entire application. Individual services are loosely
coupled, so that independent teams can focus on a single business process, without
needing to understand the entire application.
Microservices also provide individual components that suit your business needs, giving
you flexibility without rewriting your entire workflow. Different teams can use the
programming languages and frameworks of their choice to work with their
microservice.
For long-running workflows you can use Standard Workflows with AWS Fargate
integration to orchestrate applications running in containers. For short-duration,
high-volume workflows that need an immediate response, [Synchronous Express Workflows](./choosing-workflow-type.html#concepts-express-synchronous) are ideal.
One example are web-based or mobile applications, which require the completion of a
series of steps before they return a response. You can directly trigger a Synchronous
Express Workflows from Amazon API Gateway, and the connection is held open until the workflow
completes or timeouts. For short duration workflows that do not require an immediate
response, Step Functions provides Asynchronous Express Workflows.
Examples of some API orchestrations that use Step Functions include:
**Synchronous or real-time workflows**
* Change a value in a record; such as updating an employee’s last name and making
the change immediately visible.
* Update an order during checkout, for example, adding, removing, or changing the
quantity of an item; then immediately showing the updated cart to your
customer.
* Run a quick processing job and immediately return the result back to the
requester.
**Container Orchestration**
* Run jobs on Kubernetes with Amazon Elastic Kubernetes Service or on Amazon Elastic Container Service (ECS) with Fargate and
integrate with other AWS services, such as sending notifications with Amazon SNS, as
part of the same workflow.
## IT and security automation
With Step Functions, you can create workflows that automatically scale and react to errors in
your workflow. Your workflows can automatically [retry failed tasks](./concepts-error-handling.html#error-handling-retrying-after-an-error) and use [an exponential backoff](./concepts-error-handling.html#error-handling-examples) to handle errors.
Error handling is essential in IT automation scenarios to manage complex and
time-consuming operations, such as upgrading and patching software, deploying security
updates to address vulnerabilities, selecting infrastructure, synchronizing data, and
routing support tickets. By automating repetitive and time-consuming tasks, your
organization can complete routine operations quickly and consistently at scale. Your
focus can shift to strategic efforts such as feature development, complex support
requests, and innovation while meeting your operational demands.
When human intervention is required for the workflow to proceed, for example approving a
substantial credit increase, you can define branching logic in Step Functions, so that requests
under a limit are automatically approved, and requests of the limit require human
approval. When human approval is required, Step Functions can pause the workflow, wait for a
human response, then continue the workflow after a response is received.
Some examples automation workflows include the following:
**IT automation**
* Auto-remediate incidents such as open SSH ports, low disk space, or public
access granted to an Amazon S3 bucket.
* Automate the deployment of AWS CloudFormation StackSets.
**Security automation**
* Automate the response to a scenario where a user and user access key has
been exposed.
* Auto-remediate security incident responses according to policy actions, such as
restricting action to specific ARNs.
* Warn employees of phishing emails within seconds of receiving them.
**Human Approval**
* Automate machine learning model training, then get approval of the model by a data
scientist before deploying the updated model.
* Automate customer feedback routing based on sentiment analysis so negative
comments are quickly escalated for review.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
What is Step Functions?
Getting started tutorial
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.