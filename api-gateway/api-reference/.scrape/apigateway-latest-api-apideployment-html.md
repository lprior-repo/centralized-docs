---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_Deployment.html
title: Deployment
word_count: 121
filtered: true
elements_removed: 0
density_score: 0.83
---

Deployment - Amazon API Gateway
Deployment - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_Deployment)
[Contents](#API_Deployment_Contents)[See Also](#API_Deployment_SeeAlso)
# Deployment
An immutable representation of a RestApi resource that can be called by users using Stages. A deployment must be associated with a Stage for it to be callable over the Internet.
## Contents
**
apiSummary
**
A summary of the RestApi at the date and time that the deployment resource was created.
Type: String to string to [MethodSnapshot](./API_MethodSnapshot.html) object map map
Required: No
**
createdDate
**
The date and time that the deployment resource was created.
Type: Timestamp
Required: No
**
description
**
The description for the deployment resource.
Type: String
Required: No
**
id
**
The identifier for the deployment resource.
Type: String
Required: No