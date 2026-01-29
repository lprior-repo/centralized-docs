---
url: https://docs.aws.amazon.com/apigateway/latest/api/API_DomainNameAccessAssociation.html
title: DomainNameAccessAssociation
word_count: 154
filtered: true
elements_removed: 0
density_score: 0.82
---

DomainNameAccessAssociation - Amazon API Gateway
DomainNameAccessAssociation - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/api/apigw-api.pdf#API_DomainNameAccessAssociation)
[Contents](#API_DomainNameAccessAssociation_Contents)[See Also](#API_DomainNameAccessAssociation_SeeAlso)
# DomainNameAccessAssociation
Represents a domain name access association between an access association source and a private custom domain name. With a domain name access association, an access association source can invoke a private custom domain name while isolated from the public internet.
## Contents
**
accessAssociationSource
**
The identifier of the domain name access association source. For a VPCE, the value is the VPC endpoint ID.
Type: String
Required: No
**
accessAssociationSourceType
**
The type of the domain name access association source.
Type: String
Valid Values: `VPCE`
Required: No
**
domainNameAccessAssociationArn
**
The ARN of the domain name access association resource.
Type: String
Required: No
**
domainNameArn
**
The ARN of the domain name.
Type: String
Required: No
**
tags
**
The collection of tags. Each tag element is associated with a given resource.
Type: String to string map
Required: No