---
url: https://docs.aws.amazon.com/lambda/latest/dg/ddb-rds-database-decision.html
title: Select a database service for your Lambda-based applications
word_count: 1095
filtered: true
elements_removed: 0
density_score: 0.87
---

Select a database service for your Lambda-based applications - AWS Lambda
Select a database service for your Lambda-based applications - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#ddb-rds-database-decision)
[What are your choices](#w2aad101d101c19b9)[Basic recommendations](#w2aad101d101c19c11)[What to consider](#w2aad101d101c19c13)[Select a database](#w2aad101d101c19c15)
# Select a database service for your Lambda-based applications
Many serverless applications need to store and retrieve data. AWS offers multiple database options that work with Lambda functions. Two of the most popular choices are Amazon DynamoDB,
a NoSQL database service, and Amazon RDS, a traditional relational database solution. The following sections explain the key differences between these services when using them with Lambda
and help you select the right database service for your serverless application.
To learn more about the other database services offered by AWS, and to understand their use cases and tradeoffs more generally, see
[Choosing an AWS database service](https://docs.aws.amazon.com/decision-guides/latest/databases-on-aws-how-to-choose/databases-on-aws-how-to-choose.html). All of the
AWS database services are compatible with Lambda, but not all of them may be suited to your particular use case.
## What are your choices when selecting a database service with Lambda?
AWS offers multiple database services. For serverless applications, two of the most popular choices are DynamoDB and Amazon RDS.
* **DynamoDB** is a fully managed NoSQL database service optimized for serverless applications. It provides seamless scaling and consistent
single-digit millisecond performance at any scale.
* **Amazon RDS** is a managed relational database service that supports multiple database engines including MySQL and PostgreSQL. It provides
familiar SQL capabilities with managed infrastructure.
## Recommendations if you already know your requirements
If you're already clear on your requirements, here are our basic recommendations:
We recommend [DynamoDB](./with-ddb.html) for serverless applications that need consistent low-latency performance, automatic scaling, and don't require complex joins or transactions. It's
particularly well-suited for Lambda-based applications due to its serverless nature.
[Amazon RDS](./services-rds.html) is a better choice when you need complex SQL queries, joins, or have existing applications using relational databases. However, be aware that connecting Lambda functions
to Amazon RDS requires additional configuration and can impact cold start times.
## What to consider when selecting a database service
When selecting between DynamoDB and Amazon RDS for your Lambda applications, consider these factors:
* Connection management and cold starts
* Data access patterns
* Query complexity
* Data consistency requirements
* Scaling characteristics
* Cost model
By understanding these factors, you can select the option that best meets the needs of your particular use case.
* DynamoDB uses an HTTP API for all operations. Lambda functions can make immediate requests without maintaining connections, resulting in better cold start performance.
Each request is authenticated using AWS credentials without connection overhead.
* Amazon RDS requires managing connection pools since it uses traditional database connections. This can impact cold starts as new Lambda instances need to establish connections.
You'll need to implement connection pooling strategies and potentially use [Amazon RDS Proxy](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-proxy.html) to
manage connections effectively. Note that using Amazon RDS Proxy incurs additional costs.
* DynamoDB works best with known access patterns and single-table designs. It's ideal for Lambda applications that need consistent low-latency access to data based on primary keys or secondary indexes.
* Amazon RDS provides flexibility for complex queries and changing access patterns. It's better suited when your Lambda functions need to perform unique, tailored queries or complex joins across multiple tables.
* DynamoDB excels at simple, key-based operations and predefined access patterns. Complex queries must be designed around index structures, and joins must be handled in application code.
* Amazon RDS supports complex SQL queries with joins, subqueries, and aggregations. This can simplify your Lambda function code when complex data operations are needed.
* DynamoDB offers both eventual and strong consistency options, with strong consistency available for single-item reads. Transactions are supported but with some limitations.
* Amazon RDS provides full atomicity, consistency, isolation, and durability (ACID) compliance and complex transaction support. If your Lambda functions require complex transactions
or strong consistency across multiple records, Amazon RDS might be more suitable.
* DynamoDB scales automatically with your workload. It can handle sudden spikes in traffic from Lambda functions without pre-provisioning. You can use on-demand capacity mode to
pay only for what you use, perfectly matching Lambda's scaling model.
* Amazon RDS has fixed capacity based on the instance size you choose. If multiple Lambda functions try to connect simultaneously, you may exceed your connection quota.
You need to carefully manage connection pools and potentially implement retry logic.
* DynamoDB's pricing aligns well with serverless applications. With on-demand capacity, you pay only for the actual reads and writes performed by your Lambda functions. There are no charges for idle time.
* Amazon RDS charges for the running instance regardless of usage. This can be less cost-effective for sporadic workloads that can be typical in serverless applications. However, it might be more economical
for high-throughput workloads with consistent usage.
## Getting started with your chosen database service
Now that you've read about the criteria for selecting between DynamoDB and Amazon RDS and the key differences between them, you can select the option that best
matches your needs and use the following resources to get started using it.
DynamoDB
###### Get started with DynamoDB with the following resources
* For an introduction to the DynamoDB service, read [What is DynamoDB?](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html) in
the *Amazon DynamoDB Developer Guide*.
* Follow the tutorial [Using Lambda with API Gateway](./services-apigateway-tutorial.html) to see an example of using a Lambda function to perform CRUD operations on a
DynamoDB table in response to an API request.
* Read [Programming with DynamoDB and the AWS SDKs](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.html) in the
*Amazon DynamoDB Developer Guide* to learn more about how to access DynamoDB from within your Lambda function by using one of the AWS SDKs.
Amazon RDS
###### Get started with Amazon RDS with the following resources
* For an introduction to the Amazon RDS service, read [What is Amazon Relational Database Service (Amazon RDS)?](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Welcome.html) in
the *Amazon Relational Database Service User Guide*.
* Follow the tutorial [Using a Lambda function to access an Amazon RDS database](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-lambda-tutorial.html) in
the *Amazon Relational Database Service User Guide*.
* Learn more about using Lambda with Amazon RDS by reading [Using AWS Lambda with Amazon RDS](./services-rds.html).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
RDS
S3
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.