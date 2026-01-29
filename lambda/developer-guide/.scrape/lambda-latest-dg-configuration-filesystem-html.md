---
url: https://docs.aws.amazon.com/lambda/latest/dg/configuration-filesystem.html
title: Configuring file system access for Lambda functions
word_count: 870
filtered: true
elements_removed: 0
density_score: 0.86
---

Configuring file system access for Lambda functions - AWS Lambda
Configuring file system access for Lambda functions - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#configuration-filesystem)
[Supported Regions](#configuration-filesystem-supported-regions)[Execution role and user permissions](#configuration-filesystem-permissions)[Configuring a file system and access point](#configuration-filesystem-setup)[Connecting to a file system (console)](#configuration-filesystem-config)
# Configuring file system access for Lambda functions
You can configure a function to mount an Amazon Elastic File System (Amazon EFS) file system to a local directory. With Amazon EFS, your
function code can access and modify shared resources safely and at high concurrency.
## Supported Regions
Amazon EFS for Lambda is available in all [commercial Regions](https://docs.aws.amazon.com/general/latest/gr/glos-chap.html#region) except Asia Pacific (New Zealand), Asia Pacific (Taipei), Asia Pacific (Malaysia), Mexico (Central), Asia Pacific (Thailand), and Canada West (Calgary).
###### Sections
* [Execution role and user permissions](#configuration-filesystem-permissions)
* [Configuring a file system and access point](#configuration-filesystem-setup)
* [Connecting to a file system (console)](#configuration-filesystem-config)
## Execution role and user permissions
If the file system doesn't have a user-configured AWS Identity and Access Management (IAM) policy, EFS uses a default policy that grants full access to any client that can connect to the file system using a file system mount target.
If the file system has a user-configured IAM policy, your function's
execution role must have the correct `elasticfilesystem` permissions.
###### Execution role permissions
* **elasticfilesystem:ClientMount**
* **elasticfilesystem:ClientWrite (not required for read-only
connections)**
These permissions are included in the **AmazonElasticFileSystemClientReadWriteAccess**
managed policy. Additionally, your execution role must have the [permissions
required to connect to the file system's VPC](./configuration-vpc.html#configuration-vpc-permissions).
When you configure a file system, Lambda uses your permissions to verify mount targets. To configure a function
to connect to a file system, your user needs the following permissions:
## Configuring a file system and access point
Create a file system in Amazon EFS with a mount target in every Availability Zone that your function connects to.
For performance and resilience, use at least two Availability Zones. For example, in a simple configuration you
could have a VPC with two private subnets in separate Availability Zones. The function connects to both subnets
and a mount target is available in each. Ensure that NFS traffic (port 2049) is allowed by the security groups
used by the function and mount targets.
###### Note
When you create a file system, you choose a performance mode that can't be changed later. **General
purpose** mode has lower latency, and **Max I/O** mode supports a higher maximum
throughput and IOPS. For help choosing, see [Amazon EFS
performance](https://docs.aws.amazon.com/efs/latest/ug/performance.html) in the *Amazon Elastic File System User Guide*.
An access point connects each instance of the function to the right mount target for the Availability Zone it
connects to. For best performance, create an access point with a non-root path, and limit the number of files that
you create in each directory. The
following example creates a directory named `my-function` on the file system and sets the owner ID to
1001 with standard directory permissions (755).
###### Example access point configuration
* **Name** – `files`
* **User ID** – `1001`
* **Group ID** – `1001`
* **Path** – `/my-function`
* **Permissions** – `755`
* **Owner user ID** – `1001`
* **Group user ID** – `1001`
When a function uses the access point, it is given user ID 1001 and has full access to the directory.
For more information, see the following topics in the *Amazon Elastic File System User Guide*:
* [Creating resources for Amazon EFS](https://docs.aws.amazon.com/efs/latest/ug/creating-using.html)
* [Working with users, groups, and
permissions](https://docs.aws.amazon.com/efs/latest/ug/accessing-fs-nfs-permissions.html)
## Connecting to a file system (console)
A function connects to a file system over the local network in a VPC. The subnets that your function connects to
can be the same subnets that contain mount points for your file system, or subnets in the same Availability Zone
that can route NFS traffic (port 2049) to the file system.
###### Note
If your function is not already connected to a VPC, see [Giving Lambda functions access to resources in an Amazon VPC](./configuration-vpc.html).
###### To configure file system access
1. Open the [Functions page](https://console.aws.amazon.com/lambda/home#/functions) of the Lambda console.
2. Choose a function.
3. Choose **Configuration** and then choose **File systems**.
4. Under **File system**, choose **Add file system**.
5. Configure the following properties:
* **EFS file system** – The access point for a file system in the same VPC.
* **Local mount path** – The location where the file system is mounted on the
Lambda function, starting with `/mnt/`.
###### Pricing
Amazon EFS charges for storage and throughput, with rates that vary by storage class. For details, see [Amazon EFS pricing](https://aws.amazon.com/efs/pricing).
Lambda charges for data transfer between VPCs. This only applies if your function's VPC is peered to another
VPC with a file system. The rates are the same as for Amazon EC2 data transfer between VPCs in the same Region. For
details, see [Lambda pricing](https://aws.amazon.com/lambda/pricing).
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Inbound networking
Aliases
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.