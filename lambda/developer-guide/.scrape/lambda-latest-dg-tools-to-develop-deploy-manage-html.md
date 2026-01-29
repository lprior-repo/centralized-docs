---
url: https://docs.aws.amazon.com/lambda/latest/dg/tools-to-develop-deploy-manage.html
title: Development tools for Lambda
word_count: 1082
filtered: true
elements_removed: 0
density_score: 0.89
---

Development tools for Lambda - AWS Lambda
Development tools for Lambda - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#tools-to-develop-deploy-manage)
[Local development tools](#local-development-tools)[Infrastructure as Code (IaC) tools](#iac-tools)[GitHub Actions tools](#github-actions-tools)[Powertools for AWS Lambda](#development-powertools)[Workflow and event management tools](#workflow-event-management-tools)
# Development tools for Lambda
You have access to a variety of tools that increase productivity and ease-of-use throughout the entire development lifecycle. This section provides information on tools that help many Lambda customers design, develop, and manage their applications. From local development in your IDE to deploying and managing complex serverless applications, these tools help you streamline your workflow, improve code quality, and accelerate the development of robust Lambda-based solutions.
* **Local development**—Write and test Lambda functions faster in your preferred development environment. The AWS Toolkit for VS Code enables local function development, debugging, and testing with direct deployment capabilities to Lambda.
* **Infrastructure as Code (IaC)**—Deploy and manage serverless applications consistently from local testing to production. AWS SAM, AWS CDK, and CloudFormation let you define and manage your serverless infrastructure through code for consistent, version-controlled deployments.
* **GitHub Actions**—Automate Lambda deployments directly from your code repository. GitHub Actions allows you to set up workflows that automatically deploy your Lambda functions whenever you push code or configuration changes, simplifying your CI/CD pipeline.
* **Powertools for AWS Lambda**—Build production-ready serverless applications with less custom code. Powertools for AWS Lambda (also referred to as Powertools for AWS) is an open-source developer toolkit that simplifies implementing serverless best practices such as observability, parameter retrieval, and idempotency across Python, TypeScript, Java, and .NET.
* **Workflows and events**—Coordinate Lambda functions with AWS services, APIs, and external systems. Lambda provides two orchestration options: [Lambda durable functions](./durable-functions.html) for application-centric orchestration using standard programming languages within Lambda, and [AWS Step Functions](./with-step-functions.html) for workflow-centric orchestration with visual design across multiple services. Amazon EventBridge provides event management capabilities for event-driven architectures. For help choosing an orchestration approach, see [Durable functions or Step Functions](./durable-step-functions.html).
## Local development tools
Local development environments enable you to work offline and leverage advanced IDE features while iterating quickly on your Lambda functions. These tools help you debug complex functions and develop in environments with limited connectivity. They also support team collaboration and integration with version control systems.
For more information on developing Lambda functions locally, see [Developing Lambda functions locally with VS Code](./foundation-iac-local-development.html). This page describes how to move Lambda function development from the AWS console to Visual Studio Code, which provides a rich development environment with features like debugging and code completion. To make the transition, you need to set up the AWS Toolkit for Visual Studio Code and credentials, after which you can use advanced features in VS Code while maintaining the ability to deploy directly to AWS.
Local development for Lambda provides several key capabilities:
* Use Visual Studio Code integration with the Lambda console
* Configure local Lambda development environments
* Debug and test functions locally
* Apply best practices for local function management
For more information, see [Developing Lambda functions locally with VS Code](./foundation-iac-local-development.html).
## Infrastructure as Code (IaC) tools
With Infrastructure as Code (IaC) tools, you can define and manage your serverless architecture using code. This approach helps maintain consistency across environments, lets you control your infrastructure versions, and facilitates DevOps practices. IaC is especially valuable for automating deployments, ensuring consistent environments, and managing multi-region deployments.
Key IaC tools and concepts for Lambda include frameworks for template creation, deployment management, and best practices for serverless infrastructure:
* Core IaC principles for Lambda development
* CloudFormation, AWS SAM, and AWS CDK capabilities
* Tool selection criteria and comparison
* Best practices for Lambda IaC implementation
Whether you're working independently on a small project or as part of a large team managing enterprise-scale serverless applications, these development and deployment tools can help you write, deploy, and manage your Lambda functions more effectively.
For more information, see [Using Lambda with infrastructure as code (IaC)](./foundation-iac.html).
## GitHub Actions tools
GitHub Actions provides automated deployment capabilities for your Lambda functions directly from your code repository. By creating workflow files in your repository, you can automatically deploy Lambda functions whenever code or configuration changes are pushed, streamlining your continuous integration and continuous deployment (CI/CD) pipeline. The Deploy Lambda Function action offers a declarative YAML interface that simplifies the deployment process, handles AWS credentials through OpenID Connect (OIDC), and supports various deployment scenarios including code updates, configuration changes, and dry run validations. This integration enables teams to maintain a consistent and automated deployment process while leveraging their existing GitHub workflows.
For more information, see [Using GitHub Actions to deploy Lambda functions](./deploying-github-actions.html).
## Powertools for AWS Lambda
Powertools for AWS is an open-source developer toolkit that helps you implement serverless best practices with minimal custom code. Available for Python, TypeScript/Node.js, Java, and .NET, it provides utility functions, decorators,
and middleware that streamline common Lambda development tasks. The toolkit includes built-in observability features like structured logging, tracing, and metrics collection, such as utilities for parameter retrieval, secrets management,
and idempotency patterns. These tools align with AWS well-architected best practices and help developers build production-ready serverless applications more efficiently. By reducing boilerplate code and standardizing common patterns,
Powertools for AWS enables teams to focus on business logic while maintaining consistent implementation of serverless best practices across their applications.
For more information, see [Powertools for AWS Lambda](./powertools-for-lambda.html).
## Workflow and event management tools
Lambda applications can be used in orchestration of complex workflows and handling of various events. AWS provides specialized tools to help you manage these aspects of serverless development. Learn about AWS Step Functions for workflow orchestration and Amazon EventBridge for event management, and how to integrate them with your Lambda functions. These tools can significantly enhance the scalability and reliability of your serverless applications by providing robust state management and event-driven architectures. By leveraging these services, you can build more sophisticated and resilient Lambda-based solutions that can handle complex business processes and react to a wide range of system and application events.
For more information, see [Managing Lambda workflows and events](./workflow-event-management.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Creating an Order Processing System with Lambda Durable Functions
Local development
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.