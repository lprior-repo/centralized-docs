---
url: https://docs.aws.amazon.com/lambda/latest/api/API_FileSystemConfig.html
title: FileSystemConfig
word_count: 98
filtered: true
elements_removed: 0
density_score: 0.93
---

FileSystemConfig - AWS Lambda
FileSystemConfig - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/api/lambda-api.pdf#API_FileSystemConfig)
[Contents](#API_FileSystemConfig_Contents)[See Also](#API_FileSystemConfig_SeeAlso)
# FileSystemConfig
Details about the connection between a Lambda function and an [Amazon EFS file system](https://docs.aws.amazon.com/lambda/latest/dg/configuration-filesystem.html).
## Contents
**
Arn
**
The Amazon Resource Name (ARN) of the Amazon EFS access point that provides access to the file
system.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 200.
Pattern: `arn:aws[a-zA-Z-]\*:elasticfilesystem:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\\d{1}:\\d{12}:access-point/fsap-[a-f0-9]{17}`
Required: Yes
**
LocalMountPath
**
The path where the function can access the file system, starting with `/mnt/`.
Type: String
Length Constraints: Minimum length of 0. Maximum length of 160.
Pattern: `/mnt/[a-zA-Z0-9-\_.]+`
Required: Yes