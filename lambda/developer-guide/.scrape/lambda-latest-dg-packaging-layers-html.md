---
url: https://docs.aws.amazon.com/lambda/latest/dg/packaging-layers.html
title: Packaging your layer content
word_count: 625
filtered: true
elements_removed: 0
density_score: 0.87
---

Packaging your layer content - AWS Lambda
Packaging your layer content - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#packaging-layers)
[Layer paths for each Lambda runtime](#packaging-layers-paths)
# Packaging your layer content
A Lambda layer is a .zip file archive that contains supplementary code or data.
Layers usually contain library dependencies, a [custom runtime](./runtimes-custom.html),
or configuration files.
This section explains how to properly package your layer content. For more
conceptual information about layers and why you might consider using them, see
[Managing Lambda dependencies with layers](./chapter-layers.html).
The first step to creating a layer is to bundle all of your layer content into a .zip file archive. Because Lambda functions run on
[Amazon Linux](https://docs.aws.amazon.com/linux/al2023/ug/what-is-amazon-linux.html), your layer content must be able to compile and build in a Linux environment.
To ensure that your layer content works properly in a Linux environment,
we recommend creating your layer content using a tool like
[Docker](https://docs.docker.com/get-docker).
## Layer paths for each Lambda runtime
When you add a layer to a function, Lambda loads the layer content into the
`/opt` directory of that execution environment. For each Lambda runtime,
the `PATH` variable already includes specific folder paths within the
`/opt` directory. To ensure that Lambda picks up your layer content,
your layer .zip file should have its dependencies
in one of the following folder paths:
|Runtime|Path|
|
Node.js
|
`nodejs/node\_modules`
|
|
`nodejs/node18/node\_modules` (`NODE\_PATH`)
|
|
`nodejs/node20/node\_modules` (`NODE\_PATH`)
|
|
`nodejs/node22/node\_modules` (`NODE\_PATH`)
|
|
Python
|
`python`
|
|
`python/lib/`python3.x`/site-packages` (site directories)
|
|
Java
|
`java/lib` (`CLASSPATH`)
|
|
Ruby
|
`ruby/gems/3.4.0` (`GEM\_PATH`)
|
|
`ruby/lib` (`RUBYLIB`)
|
|
All runtimes
|
`bin` (`PATH`)
|
|
`lib` (`LD\_LIBRARY\_PATH`)
|
The following examples show how you can structure the folders in your layer .zip archive.
Node.js
###### Example file structure for the AWS X-Ray SDK for Node.js
```
`xray-sdk.zip
└ nodejs/node\_modules/aws-xray-sdk`
```
Python
```
`python/` *# Required top-level directory*
└── requests/
└── boto3/
└── numpy/
└── (dependencies of the other packages)
```
Ruby
###### Example file structure for the JSON gem
```
`json.zip
└ ruby/gems/3.4.0/
| build\_info
| cache
| doc
| extensions
| gems
| └ json-2.1.0
└ specifications
└ json-2.1.0.gemspec`
```
Java
###### Example file structure for the Jackson JAR file
```
`layer\_content.zip
└ java
└ lib
└ jackson-core-2.17.0.jar
└ &lt;&lt;other potential dependencies&gt;&gt;
└ ...`
```
All
###### Example file structure for the jq library
```
`jq.zip
└ bin/jq`
```
For language-specific instructions on packaging, creating, and adding
a layer, refer to the following pages:
* **Node.js** – [Working with layers for Node.js Lambda functions](./nodejs-layers.html)
* **Python** – [Working with layers for Python Lambda functions](./python-layers.html)
* **Ruby** – [Working with layers for Ruby Lambda functions](./ruby-layers.html)
* **Java** – [Working with layers for Java Lambda functions](./java-layers.html)
We recommend **against** using layers to manage dependencies for Lambda functions written in Go and Rust. This is
because Lambda functions written in these languages compile into a single executable, which you provide to Lambda when you deploy your function. This
executable contains your compiled function code, along with all of its dependencies. Using layers not only complicates this process, but also leads to
increased cold start times because your functions need to manually load extra assemblies into memory during the init phase.
To use external dependencies with Go and Rust Lambda functions, include them directly in your deployment package.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Lambda layers
Creating and deleting layers
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.