---
url: https://docs.aws.amazon.com/apigateway/latest/developerguide/api-as-kinesis-proxy-export-swagger-with-extensions.html
title: OpenAPI
word_count: 1726
filtered: true
elements_removed: 0
density_score: 0.93
---

OpenAPI definitions of a sample API as a Kinesis proxy - Amazon API Gateway
OpenAPI definitions of a sample API as a Kinesis proxy - Amazon API Gateway
[](https://docs.aws.amazon.com/pdfs/apigateway/latest/developerguide/apigateway-dg.pdf#api-as-kinesis-proxy-export-swagger-with-extensions)
# OpenAPI
definitions of a sample API as a Kinesis proxy
Following are OpenAPI definitions for the sample API as a Kinesis proxy used in this
tutorial.
OpenAPI 3.0
```
`{
"openapi": "3.0.0",
"info": {
"title": "KinesisProxy",
"version": "2016-03-31T18:25:32Z"
},
"paths": {
"/streams/{stream-name}/sharditerator": {
"get": {
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
},
{
"name": "shard-id",
"in": "query",
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/GetShardIterator",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"ShardId\\": \\"$input.params('shard-id')\\",\\n \\"ShardIteratorType\\": \\"TRIM\_HORIZON\\",\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}/records": {
"get": {
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
},
{
"name": "Shard-Iterator",
"in": "header",
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/GetRecords",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"ShardIterator\\": \\"$input.params('Shard-Iterator')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
},
"put": {
"parameters": [
{
"name": "Content-Type",
"in": "header",
"schema": {
"type": "string"
}
},
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
}
],
"requestBody": {
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/PutRecordsMethodRequestPayload"
}
},
"application/x-amz-json-1.1": {
"schema": {
"$ref": "#/components/schemas/PutRecordsMethodRequestPayload"
}
}
},
"required": true
},
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/PutRecords",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\",\\n \\"Records\\": [\\n {\\n \\"Data\\": \\"$util.base64Encode($elem.data)\\",\\n \\"PartitionKey\\": \\"$elem.partition-key\\"\\n }#if($foreach.hasNext),#end\\n ]\\n}",
"application/x-amz-json-1.1": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\",\\n \\"records\\" : [\\n {\\n \\"Data\\" : \\"$elem.data\\",\\n \\"PartitionKey\\" : \\"$elem.partition-key\\"\\n }#if($foreach.hasNext),#end\\n ]\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}": {
"get": {
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/DescribeStream",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
},
"post": {
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/CreateStream",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"ShardCount\\": 5,\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
},
"delete": {
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"headers": {
"Content-Type": {
"schema": {
"type": "string"
}
}
},
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
},
"400": {
"description": "400 response",
"headers": {
"Content-Type": {
"schema": {
"type": "string"
}
}
},
"content": {}
},
"500": {
"description": "500 response",
"headers": {
"Content-Type": {
"schema": {
"type": "string"
}
}
},
"content": {}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/DeleteStream",
"responses": {
"4\\\\d{2}": {
"statusCode": "400",
"responseParameters": {
"method.response.header.Content-Type": "integration.response.header.Content-Type"
}
},
"default": {
"statusCode": "200",
"responseParameters": {
"method.response.header.Content-Type": "integration.response.header.Content-Type"
}
},
"5\\\\d{2}": {
"statusCode": "500",
"responseParameters": {
"method.response.header.Content-Type": "integration.response.header.Content-Type"
}
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}/record": {
"put": {
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"schema": {
"type": "string"
}
}
],
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/PutRecord",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\",\\n \\"Data\\": \\"$util.base64Encode($input.json('$.Data'))\\",\\n \\"PartitionKey\\": \\"$input.path('$.PartitionKey')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams": {
"get": {
"responses": {
"200": {
"description": "200 response",
"content": {
"application/json": {
"schema": {
"$ref": "#/components/schemas/Empty"
}
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/ListStreams",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
}
},
"components": {
"schemas": {
"Empty": {
"type": "object"
},
"PutRecordsMethodRequestPayload": {
"type": "object",
"properties": {
"records": {
"type": "array",
"items": {
"type": "object",
"properties": {
"data": {
"type": "string"
},
"partition-key": {
"type": "string"
}
}
}
}
}
}
}
}
}`
```
OpenAPI 2.0
```
`{
"swagger": "2.0",
"info": {
"version": "2016-03-31T18:25:32Z",
"title": "KinesisProxy"
},
"basePath": "/test",
"schemes": [
"https"
],
"paths": {
"/streams": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/ListStreams",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/DescribeStream",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
},
"post": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/CreateStream",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"ShardCount\\": 5,\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
},
"delete": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
},
"headers": {
"Content-Type": {
"type": "string"
}
}
},
"400": {
"description": "400 response",
"headers": {
"Content-Type": {
"type": "string"
}
}
},
"500": {
"description": "500 response",
"headers": {
"Content-Type": {
"type": "string"
}
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/DeleteStream",
"responses": {
"4\\\\d{2}": {
"statusCode": "400",
"responseParameters": {
"method.response.header.Content-Type": "integration.response.header.Content-Type"
}
},
"default": {
"statusCode": "200",
"responseParameters": {
"method.response.header.Content-Type": "integration.response.header.Content-Type"
}
},
"5\\\\d{2}": {
"statusCode": "500",
"responseParameters": {
"method.response.header.Content-Type": "integration.response.header.Content-Type"
}
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}/record": {
"put": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/PutRecord",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\",\\n \\"Data\\": \\"$util.base64Encode($input.json('$.Data'))\\",\\n \\"PartitionKey\\": \\"$input.path('$.PartitionKey')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}/records": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "Shard-Iterator",
"in": "header",
"required": false,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/GetRecords",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"ShardIterator\\": \\"$input.params('Shard-Iterator')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
},
"put": {
"consumes": [
"application/json",
"application/x-amz-json-1.1"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "Content-Type",
"in": "header",
"required": false,
"type": "string"
},
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
},
{
"in": "body",
"name": "PutRecordsMethodRequestPayload",
"required": true,
"schema": {
"$ref": "#/definitions/PutRecordsMethodRequestPayload"
}
},
{
"in": "body",
"name": "PutRecordsMethodRequestPayload",
"required": true,
"schema": {
"$ref": "#/definitions/PutRecordsMethodRequestPayload"
}
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/PutRecords",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\",\\n \\"Records\\": [\\n {\\n \\"Data\\": \\"$util.base64Encode($elem.data)\\",\\n \\"PartitionKey\\": \\"$elem.partition-key\\"\\n }#if($foreach.hasNext),#end\\n ]\\n}",
"application/x-amz-json-1.1": "{\\n \\"StreamName\\": \\"$input.params('stream-name')\\",\\n \\"records\\" : [\\n {\\n \\"Data\\" : \\"$elem.data\\",\\n \\"PartitionKey\\" : \\"$elem.partition-key\\"\\n }#if($foreach.hasNext),#end\\n ]\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
},
"/streams/{stream-name}/sharditerator": {
"get": {
"consumes": [
"application/json"
],
"produces": [
"application/json"
],
"parameters": [
{
"name": "stream-name",
"in": "path",
"required": true,
"type": "string"
},
{
"name": "shard-id",
"in": "query",
"required": false,
"type": "string"
}
],
"responses": {
"200": {
"description": "200 response",
"schema": {
"$ref": "#/definitions/Empty"
}
}
},
"x-amazon-apigateway-integration": {
"type": "aws",
"credentials": "arn:aws:iam::123456789012:role/apigAwsProxyRole",
"uri": "arn:aws:apigateway:us-east-1:kinesis:action/GetShardIterator",
"responses": {
"default": {
"statusCode": "200"
}
},
"requestParameters": {
"integration.request.header.Content-Type": "'application/x-amz-json-1.1'"
},
"requestTemplates": {
"application/json": "{\\n \\"ShardId\\": \\"$input.params('shard-id')\\",\\n \\"ShardIteratorType\\": \\"TRIM\_HORIZON\\",\\n \\"StreamName\\": \\"$input.params('stream-name')\\"\\n}"
},
"passthroughBehavior": "when\_no\_match",
"httpMethod": "POST"
}
}
}
},
"definitions": {
"Empty": {
"type": "object"
},
"PutRecordsMethodRequestPayload": {
"type": "object",
"properties": {
"records": {
"type": "array",
"items": {
"type": "object",
"properties": {
"data": {
"type": "string"
},
"partition-key": {
"type": "string"
}
}
}
}
}
}
}
}`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Tutorial: Create a REST API as an Amazon Kinesis proxy
Tutorial: Create a REST API using
AWS SDKs or the AWS CLI
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.