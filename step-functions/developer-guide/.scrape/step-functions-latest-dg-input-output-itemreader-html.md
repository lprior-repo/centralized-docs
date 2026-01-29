---
url: https://docs.aws.amazon.com/step-functions/latest/dg/input-output-itemreader.html
title: ItemReader (Map)
word_count: 4526
filtered: true
elements_removed: 0
density_score: 0.82
---

ItemReader (Map) - AWS Step Functions
ItemReader (Map) - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#input-output-itemreader)
[Contents of the ItemReader field](#itemreader-field-contents)[Processing nested data sets (updated Sep 11, 2025)](#itemreader-flatten)[Examples of datasets](#itemreader-examples-map)[IAM policy recommendations for datasets](#itemreader-iam-policies)
# ItemReader (Map)
The `ItemReader` field is a JSON object, which specifies a dataset and its location. A *Distributed Map state* uses this dataset as its input.
The following example shows the syntax of the `ItemReader` field in a **JSONPath-based** workflow, for a dataset
in a text delimited file that's stored in an Amazon S3 bucket.
```
`"ItemReader": {
"ReaderConfig": {
"InputType": "CSV",
"CSVHeaderLocation": "FIRST\_ROW"
},
"Resource": "arn:aws:states:::s3:getObject",
"Parameters": {
"Bucket": "`amzn-s3-demo-bucket`",
"Key": "`csvDataset/ratings.csv`",
"VersionId": "BcK42coT2jE1234VHLUvBV1yLNod2OEt"
}
}`
```
In the following **JSONata-based**
workflow, note that `Parameters` is replaced with **Arguments**.
```
`"ItemReader": {
"ReaderConfig": {
"InputType": "CSV",
"CSVHeaderLocation": "FIRST\_ROW"
},
"Resource": "arn:aws:states:::s3:getObject",
"Arguments": {
"Bucket": "`amzn-s3-demo-bucket`",
"Key": "`csvDataset/ratings.csv`"
"VersionId": "BcK42coT2jE1234VHLUvBV1yLNod2OEt"
}
}`
```
## Contents of the ItemReader field
Depending on your dataset, the contents of the `ItemReader` field varies. For example, if your dataset is a JSON array passed from a
previous step in the workflow, the `ItemReader` field is omitted. If your dataset is an Amazon S3 data source, this field contains the following
sub-fields.
**`Resource`**
The Amazon S3 API integration action that Step Functions will use, such as
`arn:aws:states:::s3:getObject`
**`Arguments (JSONata) or `Parameters` (JSONPath)`**
A JSON object that specifies the Amazon S3 bucket name and object key that the
dataset is stored in.
If the bucket has versioning enabled, you can also provide the Amazon S3 object
version.
**
`**ReaderConfig**`**
A JSON object that specifies the following details:
* `InputType`
Accepts one of the following values: `CSV`,
`JSON`, `JSONL`, `PARQUET`,
`MANIFEST`.
Specifies the type of Amazon S3 data source, such as a text delimited
file (`CSV`), object, JSON file, JSON Lines, Parquet
file, Athena manifest, or an Amazon S3 inventory list. In Workflow Studio, you can
select an input type from **S3 item
source**.
Most input types which use `S3GetObject` retrieval also
support `ExpectedBucketOwner` and `VersionId`
fields in their parameters. Parquet files are the one exception
which does not support `VersionId`.
Input files support the following external compression types:
GZIP, ZSTD.
Example file names: `myObject.jsonl.gz` and
`myObject.csv.zstd`.
Note: Parquet files are a binary file type that are internally
compressed. GZIP, ZSTD, and Snappy compression are supported.
* `Transformation`
*Optional*. Value will be either
or `NONE` or `LOAD\_AND\_FLATTEN`.
If not specified, `NONE` will be assumed. When set to
`LOAD\_AND\_FLATTEN`, you must also set
`InputType`.
Default behavior, map will iterate over **metadata objects** returned from calls to
`S3:ListObjectsV2`. When set to
`LOAD\_AND\_FLATTEN`, map will read and process the actual
**data objects** referenced in the list
of results.
* `ManifestType`
*Optional*. Value will be either or
`ATHENA\_DATA` or `S3\_INVENTORY`.
Note: If set to `S3\_INVENTORY`, you must **not** also specify `InputType`
because the type is assumed to be `CSV`.
* `CSVDelimiter`
You can specify this field when `InputType` is `CSV` or
`MANIFEST`.
Accepts one of the following values: `COMMA` (default),
`PIPE`, `SEMICOLON`, `SPACE`,
`TAB`.
###### Note
With the `CSVDelimiter` field, `ItemReader` can process files that are delimited by characters other than a comma. References to "CSV files" also includes files that use alternative delimiters specified by the `CSVDelimiter` field.
* `CSVHeaderLocation`
You can specify this field when `InputType` is
`CSV` or `MANIFEST`.
Accepts one of the following values to specify the location of the column
header:
* `FIRST\_ROW` – Use this option if the first line of the file is the header.
* `GIVEN` – Use this option to specify the header within the
state machine definition.
For example, if your file contains the following
data.
```
`1,307,3.5,1256677221
1,481,3.5,1256677456
1,1091,1.5,1256677471
...`
```
You might provide the following JSON array as a CSV header:
```
`"ItemReader": {
"ReaderConfig": {
"InputType": "CSV",
**"CSVHeaderLocation": "GIVEN",
"CSVHeaders": [
`"userId"`,
`"movieId"`,
`"rating"`,
`"timestamp"`
]**
}
}`
```
###### Note
You can specify a limit of up to 100,000,000 after which the `Distributed Map`
stops reading items.
###### Requirements for account and region
Your Amazon S3 buckets must be in the same AWS account and AWS Region as your state machine.
Note that even though your state machine may be able to access files in buckets across different AWS accounts that are in the same AWS Region, Step Functions only supports listing objects in Amazon S3 buckets that are in *both* the same AWS account and the same AWS Region as the state machine.
## Processing nested data sets (updated Sep 11, 2025)
With the new `Transformation` parameter, you can specify a value of
`LOAD\_AND\_FLATTEN` and the map will read the **actual
**data objects referenced in the list of results from a call to
`S3:ListObjectsV2`.
Prior to this release, you would need to create nested Distributed Maps to **retrieve** the metadata and then **process** the actual data. The first map would iterate over the **metadata** returned by `S3:ListObjectsV2` and invoke
child workflows. Another map within each child state machine would read the **actual data **from individual files. With the transformation
option, you can accomplish both steps at once.
Imagine you want to run a daily audit on the past 24 log files your system produces
hourly and stores in Amazon S3. Your Distributed Map state can list the log files with
`S3:ListObjectsV2`, then iterate over either the
*metadata* of each object, or it can now load and analyze the
**actual data **objects stored in your Amazon S3 bucket.
Using the `LOAD\_AND\_FLATTEN` option can increase scalability, reduce open
Map Run counts, and process multiple objects concurrently. Athena and Amazon EMR jobs
typically generate output that can be processed with the new configuration.
The following is an example of the parameters in an `ItemReader`
definition:
```
`{
"QueryLanguage": "JSONata",
"States": {
...
"Map": {
...
"ItemReader": {
"Resource": "arn:aws:states:::s3:listObjectsV2",
"ReaderConfig": {
// InputType is required if Transformation is LOAD\_AND\_FLATTEN.
"InputType": "CSV | JSON | JSONL | PARQUET",
// Transformation is OPTIONAL and defaults to NONE if not present
"Transformation": "NONE | LOAD\_AND\_FLATTEN"
},
"Arguments": {
"Bucket": "amzn-s3-demo-bucket1",
"Prefix": "{% $states.input.PrefixKey %}"
}
},
...
}
}`
```
## Examples of datasets
You can specify one of the following options as your dataset:
* [JSON data from a previous step](#itemsource-json-array)
* [A list of Amazon S3 objects](#itemsource-example-s3-object-data)
* [Amazon S3 objects transformed by
LOAD\_AND\_FLATTEN](#itemsource-example-s3-object-data-flatten)
* [JSON file in an Amazon S3 bucket](#itemsource-example-json-data)
* [JSON Lines file in an Amazon S3 bucket](#itemsource-example-json-lines-data)
* [CSV file in an Amazon S3 bucket](#itemsource-example-csv-data)
* [Parquet file in an Amazon S3 bucket](#itemsource-example-parquet-data)
* [Athena manifest (process
multiple items)](#itemsource-example-athena-manifest-data)
* [Amazon S3 inventory (process multiple
items)](#itemsource-example-s3-inventory)
###### Note
Step Functions needs appropriate permissions to access the Amazon S3 datasets that you use. For information about IAM policies for the datasets, see [IAM policy recommendations for datasets](#itemreader-iam-policies).
A *Distributed Map state* can accept a JSON input passed from a previous step in the workflow.
The input can be a JSON array, a JSON object, or an array within a node of a JSON object.
Step Functions will iterate directly over the elements of an array, or the key-value
pairs of a JSON object.
To select a specific node that contains a nested JSON array or object from the input, you can use the
`[ItemsPath (Map, JSONPath only)](./input-output-itemspath.html)` or use a JSONata expression in
the `Items` field for JSONata states.
To process individual items, the *Distributed Map state* starts a child workflow execution for each
item. The following tabs show examples of the input passed to the
`Map` state and the corresponding input to a child workflow
execution.
###### Note
The `ItemReader` field is not needed when your dataset is JSON data from
a previous step.
Input passed to the Map state
Consider the following JSON array of three items.
```
`"facts": [
{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
},
{
"verdict": "false",
"statement\_date": "6/7/2022",
"statement\_source": "television"
},
{
"verdict": "mostly-true",
"statement\_date": "5/18/2016",
"statement\_source": "news"
}
]`
```
Input passed to a child workflow execution
The *Distributed Map state* starts three child workflow executions. Each execution receives an array item as input. The following example shows the input
received by a child workflow execution.
```
`{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
}`
```
A *Distributed Map state* can iterate over the objects that are stored in an Amazon S3 bucket. When the
workflow execution reaches the `Map` state, Step Functions invokes the [ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html) API action, which returns an array of the Amazon S3 **object metadata**. In this array, each item contains data,
such as **ETag** and **Key**, for the actual
data stored in the bucket.
To process individual items in the array, the *Distributed Map state* starts a child workflow
execution. For example, suppose that your Amazon S3 bucket contains 100 images. Then,
the array returned after invoking the `ListObjectsV2` API action
contains 100 metadata items. The *Distributed Map state* then starts 100 child workflow
executions to process each item.
To process data objects directly, without nested workflows, you can choose the
LOAD\_AND\_FLATTEN Transformation option to process items **directly**.
###### Note
* Step Functions will also include an item for each **folder
**created in the Amazon S3 bucket using the Amazon S3 **console**. The folder items result in starting
extra child workflow executions.
To avoid creating a extra child workflow executions for each
folder, we recommend that you use the AWS CLI to create folders. For
more information, see [High-level Amazon S3 commands](https://docs.aws.amazon.com/cli/latest/userguide/cli-services-s3-commands.html#using-s3-commands-managing-buckets-creating) in the
*AWS Command Line Interface User Guide*.
* Step Functions needs appropriate permissions to access the Amazon S3 datasets that you use. For information about IAM policies for the datasets, see [IAM policy recommendations for datasets](#itemreader-iam-policies).
The following tabs show examples of the `ItemReader` field syntax and the input passed to a child workflow execution for this dataset.
ItemReader syntax
In this example, you've organized your data, which includes images, JSON files, and objects, within a prefix named `processData`
in an Amazon S3 bucket named `amzn-s3-demo-bucket`.
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:listObjectsV2",
"Parameters": {
"Bucket": "`amzn-s3-demo-bucket`",
"Prefix": "`processData`"
}
}`
```
Input passed to a child workflow execution
The *Distributed Map state* starts as many child workflow executions as the number of
metadata items present in the Amazon S3 bucket. The following example
shows the input received by a child workflow execution.
```
`{
"Etag": "\\"05704fbdccb224cb01c59005bebbad28\\"",
"Key": "`processData`/`images`/`n02085620\_1073.jpg`",
"LastModified": 1668699881,
"Size": 34910,
"StorageClass": "STANDARD"
}`
```
With enhanced support for S3 ListObjectsV2 as an input source in Distributed Map, your state
machines can read and process multiple **data objects
**from Amazon S3 buckets directly, eliminating the need for nested maps to
process the metadata!
With the `LOAD\_AND\_FLATTEN` option, your state machine will do the following:
* Read the **actual content** of each object listed by Amazon S3
`ListObjectsV2` call.
* Parse the content based on InputType (CSV, JSON, JSONL, Parquet).
* Create items from the file contents (rows/records) rather than metadata.
With the transformation option, you no longer need nested Distributed Maps to process the
metadata. Using the LOAD\_AND\_FLATTEN option increases scalability, reduces
active map run counts, and processes multiple objects concurrently.
The following configuration shows the setting for an
`ItemReader`:
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:listObjectsV2",
"ReaderConfig": {
"InputType": "JSON",
"Transformation": "LOAD\_AND\_FLATTEN"
},
"Arguments": {
"Bucket": "`S3\_BUCKET\_NAME`",
"Prefix": "`S3\_BUCKET\_PREFIX`"
}
}`
```
###### Bucket prefix recommendation
We recommend including a trailing slash on your prefix. For example, if you select data with a prefix of `folder1`, your state machine will process both `folder1/myData.csv` and `folder10/myData.csv`. Using `folder1/` will strictly process only one folder.
A *Distributed Map state* can accept a JSON file that's stored in an Amazon S3 bucket as a dataset. The
JSON file must contain an array or JSON object.
When the workflow execution reaches the `Map` state, Step Functions invokes the
[GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html) API action to fetch the specified JSON file.
If the JSON file contains a nested object structure, you can select the
specific node with your data set with an `ItemsPointer`. For example,
the following configuration would extract a nested list of *featured products* in *inventory*.
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:getObject",
"ReaderConfig": {
"InputType": "JSON",
"ItemsPointer": "`/inventory/products/featured`"
},
"Arguments": {
"Bucket": "amzn-s3-demo-bucket",
"Key": "nested-data-file.json"
}
}`
```
The `Map` state then iterates over each item in the array and
starts a child workflow execution for each item. For example, if your JSON file
contains 1000 array items, the `Map` state starts 1000 child workflow
executions.
###### Note
* The execution input used to start a child workflow execution can't exceed 256 KiB. However, Step Functions supports reading an item of up to 8 MB from a text delimited file, JSON, or JSON Lines file if you then apply the optional `ItemSelector` field to reduce the item's size.
* Step Functions supports 10 GB as the maximum size of an individual file in Amazon S3.
* Step Functions needs appropriate permissions to access the Amazon S3 datasets that you use. For information about IAM policies for the datasets, see [IAM policy recommendations for datasets](#itemreader-iam-policies).
The following tabs show examples of the `ItemReader` field syntax and the input passed to a child workflow execution for this dataset.
For this example, imagine you have a JSON file named ``factcheck.json``. You've stored this file within a
prefix named ``jsonDataset`` in an Amazon S3 bucket. The following is an example of the JSON dataset.
```
`[
{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
},
{
"verdict": "false",
"statement\_date": "6/7/2022",
"statement\_source": "television"
},
{
"verdict": "mostly-true",
"statement\_date": "5/18/2016",
"statement\_source": "news"
},
...
]`
```
ItemReader syntax
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:getObject",
"ReaderConfig": {
"InputType": "JSON"
},
"Parameters": {
"Bucket": "`amzn-s3-demo-bucket`",
"Key": "`jsonDataset/factcheck.json`"
}
}`
```
Input to a child workflow execution
The *Distributed Map state* starts as many child workflow executions as the number of array items present in the JSON file. The following example shows
the input received by a child workflow execution.
```
`{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
}`
```
A *Distributed Map state* can accept a JSON Lines file that's stored in an Amazon S3 bucket as a dataset.
###### Note
* The execution input used to start a child workflow execution can't exceed 256 KiB. However, Step Functions supports reading an item of up to 8 MB from a text delimited file, JSON, or JSON Lines file if you then apply the optional `ItemSelector` field to reduce the item's size.
* Step Functions supports 10 GB as the maximum size of an individual file in Amazon S3.
* Step Functions needs appropriate permissions to access the Amazon S3 datasets that you use. For information about IAM policies for the datasets, see [IAM policy recommendations for datasets](#itemreader-iam-policies).
The following tabs show examples of the `ItemReader` field syntax and the input passed to a child workflow execution for this dataset.
For this example, imagine you have a JSON Lines file named ``factcheck.jsonl``. You've stored this file
within a prefix named ``jsonlDataset`` in an Amazon S3 bucket. The following is an example of the file's
contents.
```
`{"verdict": "true", "statement\_date": "6/11/2008", "statement\_source": "speech"}
{"verdict": "false", "statement\_date": "6/7/2022", "statement\_source": "television"}
{"verdict": "mostly-true", "statement\_date": "5/18/2016", "statement\_source": "news"}`
```
ItemReader syntax
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:getObject",
"ReaderConfig": {
"InputType": "JSONL"
},
"Parameters": {
"Bucket": "`amzn-s3-demo-bucket`",
"Key": "`jsonlDataset/factcheck.jsonl`"
}
}`
```
Input to a child workflow execution
The *Distributed Map state* starts as many child workflow executions as the number of lines present in the JSONL file. The following example shows the
input received by a child workflow execution.
```
`{
"verdict": "true",
"statement\_date": "6/11/2008",
"statement\_source": "speech"
}`
```
###### Note
With the `CSVDelimiter` field, `ItemReader` can process files that are delimited by characters other than a comma. References to "CSV files" also includes files that use alternative delimiters specified by the `CSVDelimiter` field.
A *Distributed Map state* can accept a text delimited file that's stored in an Amazon S3 bucket as a dataset. If you use a text delimited file as your dataset, you
need to specify a column header. For information about how to specify a header, see [Contents of the ItemReader field](#itemreader-field-contents).
Step Functions parses text delimited files based on the following rules:
* The delimiter that separates fields is specified by `CSVDelimiter` in *ReaderConfig*. The delimiter
defaults to `COMMA`.
* Newlines are a delimiter that separates **records**.
* Fields are treated as strings. For data type conversions, use the `[States.StringToJson](./intrinsic-functions.html#stringtojson)` intrinsic
function in [ItemSelector (Map)](./input-output-itemselector.html).
* Double quotation marks (" ") are not required to enclose strings. However, strings
that are enclosed by double quotation marks can contain commas and newlines without
acting as record delimiters.
* You can preserve double quotes by repeating them.
* Backslashes (\\) are another way to escape special characters. Backslashes only work
with other backslashes, double quotation marks, and the configured field separator such
as comma or pipe. A backslash followed by any other character is silently removed.
* You can preserve backslashes by repeating them. For example:
```
`path,size
C:\\\\Program Files\\\\MyApp.exe,6534512`
```
* Backslashes that escape double quotation marks (`\\"`), only work when included in pairs, so we recommend escaping double quotation marks by repeating them: `""`.
* If the number of fields in a row is **less** than the
number of fields in the header, Step Functions provides **empty
strings** for the missing values.
* If the number of fields in a row is **more** than the
number of fields in the header, Step Functions **skips** the
additional fields.
For more information about how Step Functions parses a text delimited file, see [Example of parsing an input CSV file](./example-csv-parse-dist-map.html#example-csv-parse).
When the workflow execution reaches the `Map` state, Step Functions invokes the [GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html) API action to fetch the specified file. The `Map` state then iterates over
each row in the file and starts a child workflow execution to process the items in each row. For example, suppose that you provide a text delimited
file that contains 100 rows as input. Then, the interpreter passes each row to the `Map` state. The `Map` state processes items
in serial order, starting after the header row.
###### Note
* The execution input used to start a child workflow execution can't exceed 256 KiB. However, Step Functions supports reading an item of up to 8 MB from a text delimited file, JSON, or JSON Lines file if you then apply the optional `ItemSelector` field to reduce the item's size.
* Step Functions supports 10 GB as the maximum size of an individual file in Amazon S3.
* Step Functions needs appropriate permissions to access the Amazon S3 datasets that you use. For information about IAM policies for the datasets, see [IAM policy recommendations for datasets](#itemreader-iam-policies).
The following tabs show examples of the `ItemReader` field syntax and the input passed to a child workflow execution for this dataset.
ItemReader syntax
For example, say that you have a CSV file named ``ratings.csv``. Then, you've stored this file within a
prefix that's named ``csvDataset`` in an Amazon S3 bucket.
```
`"ItemReader": {
"ReaderConfig": {
"InputType": "CSV",
"CSVHeaderLocation": "FIRST\_ROW",
"CSVDelimiter": "PIPE"
},
"Resource": "arn:aws:states:::s3:getObject",
"Parameters": {
"Bucket": "`amzn-s3-demo-bucket`",
"Key": "`csvDataset/ratings.csv`"
}
}`
```
Input to a child workflow execution
The *Distributed Map state* starts as many child workflow executions as the number of rows present in the CSV file, excluding the header row, if in the
file. The following example shows the input received by a child workflow execution.
```
`{
"rating": "3.5",
"movieId": "307",
"userId": "1",
"timestamp": "1256677221"
}`
```
Parquet files can be used as an input source. Apache Parquet files stored in Amazon S3 provide efficient columnar data processing at scale.
When using Parquet files, the following conditions apply:
* 256MB is the maximum row-group size, and 5MB is the maximum footer size. If you provide input
files that exceed either limit, your state machine will return a runtime
error.
* The `VersionId` field is **not** supported for `InputType=Parquet`.
* Internal GZIP, ZSTD, and Snappy data compression are natively supported. No filename
extensions are necessary.
The following shows an example ASL configuration for `InputType` set to Parquet:
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:getObject",
"ReaderConfig": {
"InputType": "PARQUET"
},
"Arguments": {
"Bucket": "amzn-s3-demo-bucket",
"Key": "my-parquet-data-file-1.parquet"
}
}`
```
###### Large scale job processing
For extremely large scale jobs, Step Functions will use many input readers. Readers interleave their processing, which might result in some readers pausing while others progress. Intermittent progress is expected behavior at scale.
You can use the Athena manifest files, generated from `UNLOAD` query results, to
specify the **source** of data files for your Map
state. You set `ManifestType` to `ATHENA\_DATA`, and
`InputType` to either `CSV`, `JSONL`, or
`Parquet`.
When running an `UNLOAD` query, Athena generates a data manifest
file in addition to the actual data objects. The manifest file provides a
structured CSV list of the data files. Both the manifest and the data files are
saved to your Athena query result location in Amazon S3.
```
`UNLOAD (&lt;&lt;YOUR\_SELECT\_QUERY&gt;&gt;) TO 'S3\_URI\_FOR\_STORING\_DATA\_OBJECT' WITH (format = 'JSON')`
```
Conceptual overview of the process, in brief:
1. Select your data from a Table using an `UNLOAD` query
in Athena.
2. Athena will generate a manifest file (CSV) and the data objects in
Amazon S3.
3. Configure Step Functions to read the manifest file and process the
input.
The feature can process CSV, JSONL, and Parquet output formats from Athena. All
objects referenced in a single manifest file must be the same InputType format.
Note that CSV objects exported by an `UNLOAD` query do **not **include header in the first line. See
`CSVHeaderLocation` if you need to provide column headers.
The map context will also include a `$states.context.Map.Item.Source` so you can customize processing based on the source of the data.
The following is an example configuration of an `ItemReader` configured to use
an Athena manifest:
```
`"ItemReader": {
"Resource": "arn:aws:states:::s3:getObject",
"ReaderConfig": {
"ManifestType": "ATHENA\_DATA",
"InputType": "CSV | JSONL | PARQUET"
},
"Arguments": {
"Bucket": "&lt;&lt;S3\_BUCKET\_NAME&gt;&gt;",
"Key": "&lt;&lt;S3\_KEY\_PREFIX&gt;&gt;&lt;&lt;QUERY\_ID&gt;&gt;-manifest.csv"
}
}`
```
###### Using the Athena manifest pattern in Workflow Studio
A common scenario for data processing applies a Map to data sourced from an Athena UNLOAD query. The Map invokes a Lambda function to process each item described in the Athena manifest. Step Functions Workflow Studio provides a ready-made pattern that combines all of these components into block you an drag onto your state machine canvas.
A *Distributed Map state* can accept an Amazon S3 inventory manifest file that's stored in an Amazon S3 bucket as a dataset.
When the workflow execution reaches the `Map` state, Step Functions invokes the
[GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html) API action to fetch the specified Amazon S3 inventory manifest
file.
By default, the `Map` state then iterates over the **objects** in the inventory to return an array of Amazon S3
inventory object metadata.
If you specify ManifestType is S3\_INVENTORY then InputType cannot be
specified.
###### Note
* Step Functions supports 10 GB as the maximum size of an individual file in an Amazon S3 inventory report after decompression. However, Step Functions can process more than 10 GB if each individual file is under 10 GB.
* Step Functions needs appropriate permissions to access the Amazon S3 datasets that you use. For information about IAM policies for the datasets, see [IAM policy recommendations for datasets](#itemreader-iam-policies).
The following is an example of an inventory file in CSV format. This file includes the objects named `csvDataset` and
`imageDataset`, which are stored in an Amazon S3 bucket that's named `amzn-s3-demo-source-bucket`.
```
`"amzn-s3-demo-source-bucket","csvDataset/","0","2022-11-16T00:27:19.000Z"
"amzn-s3-demo-source-bucket","csvDataset/titles.csv","3399671","2022-11-16T00:29:32.000Z"
"amzn-s3-demo-source-bucket","imageDataset/","0","2022-11-15T20:00:44.000Z"
"amzn-s3-demo-source-bucket","imageDataset/n02085620\_10074.jpg","27034","2022-11-15T20:02:16.000Z"
...`
```
###### Important
Step Functions doesn't support a user-defined Amazon S3 inventory report as a dataset.
The output format of your Amazon S3 inventory report must be CSV.
For more information about Amazon S3 inventories and how to set them up, see
[Amazon S3
Inventory](https://docs.aws.amazon.com/AmazonS3/latest/userguide/storage-inventory.html).
The following example of an Amazon S3 inventory manifest file shows the CSV headers for
the inventory object metadata.
```
`{
"sourceBucket" : "`amzn-s3-demo-source-bucket`",
"destinationBucket" : "arn:aws:s3:::`amzn-s3-demo-inventory`",
"version" : "2016-11-30",
"creationTimestamp" : "1668560400000",
"fileFormat" : "CSV",
"fileSchema" : "Bucket, Key, Size, LastModifiedDate",
"files" : [ {
"key" : "`amzn-s3-demo-bucket`/`destination-prefix`/data/`20e55de8-9c21-45d4-99b9-46c732000228.csv.gz`",
"size" : 7300,
"MD5checksum" : "a7ff4a1d4164c3cd55851055ec8f6b20"
} ]
}
`
```
The following tabs show examples of the `ItemReader` field syntax and the input passed to a child workflow execution for this dataset.
ItemReader syntax
```
`"ItemReader": {
"ReaderConfig": {
"InputType": "MANIFEST"
},
"Resource": "arn:aws:states:::s3:getObject",
"Parameters": {
"Bucket": "`amzn-s3-demo-destination-bucket`",
"Key": "`destination-prefix`/`amzn-s3-demo-bucket`/`config-id`/`YYYY-MM-DDTHH-MMZ`/manifest.json"
}
}`
```
Input to a child workflow execution
```
`{
"LastModifiedDate": "2022-11-16T00:29:32.000Z",
"Bucket": "amzn-s3-demo-source-bucket",
"Size": "3399671",
"Key": "csvDataset/titles.csv"
}`
```
Depending on the fields you selected while configuring the Amazon S3 inventory
report, the contents of your `manifest.json` file may
vary from the example.
## IAM policy recommendations for datasets
When you create workflows with the Step Functions console, Step Functions can automatically generate IAM policies based on the resources in your workflow definition. Generated policies include the least privileges necessary to allow the state machine role to invoke the `[StartExecution](https://docs.aws.amazon.com/step-functions/latest/apireference/API_StartExecution.html)` API action for the *Distributed Map state* and access AWS resources, such as Amazon S3 buckets and objects, and Lambda functions.
We recommend including only the necessary permissiosn in your IAM policies. For example, if your workflow includes a `Map` state in Distributed mode, scope your policies down to the specific Amazon S3 bucket and folder that contains your data.
###### Important
If you specify an Amazon S3 bucket and object, or prefix, with a [reference path](./amazon-states-language-paths.html#amazon-states-language-reference-paths) to an existing key-value pair in your *Distributed Map state* input, make sure that you update the IAM policies for your workflow. Scope the policies down to the bucket and object names the path resolves to at runtime.
The following examples show techniques for granting the least privileges required to access your
Amazon S3 datasets using the [ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html) and [GetObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html) API
actions.
###### Example condition using an Amazon S3 object as a dataset
The following condition grants the least privileges to access
objects in a ``processImages`` folder of an
Amazon S3 bucket.
```
`"Resource": [ "arn:aws:s3:::`amzn-s3-demo-bucket`" ],
"Condition": {
"StringLike": {
"s3:prefix": [ "`processImages`" ]
}
}`
```
###### Example using a CSV file as a dataset
The following example shows the actions required to access a CSV file named ``ratings.csv``.
```
`"Action": [ "s3:GetObject" ],
"Resource": [
"arn:aws:s3:::`amzn-s3-demo-bucket`/`csvDataset`/`ratings.csv`"
]`
```
###### Example using an Amazon S3 inventory as a dataset
The following shows example resources for an Amazon S3 inventory manifest and data files.
```
`"Resource": [
"arn:aws:s3:::myPrefix/`amzn-s3-demo-bucket`/myConfig-id/`YYYY-MM-DDTHH-MMZ`/manifest.json",
"arn:aws:s3:::myPrefix/`amzn-s3-demo-bucket`/myConfig-id/data/\*"
]`
```
###### Example using ListObjectsV2 to restrict to a folder prefix
When using [ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html), two policies
will be generated. One is needed to allow **listing** the
contents of the bucket (`ListBucket`) and another policy will allow **retrieving objects** in the bucket (`GetObject`).
The following show example actions, resources, and a condition:
```
`"Action": [ "s3:ListBucket" ],
"Resource": [ "arn:aws:s3:::`amzn-s3-demo-bucket`" ],
"Condition": {
"StringLike": {
"s3:prefix": [ "/path/to/your/json/" ]
}
}`
```
```
`"Action": [ "s3:GetObject" ],
"Resource": [ "arn:aws:s3:::`amzn-s3-demo-bucket`/path/to/your/json/\*" ]`
```
Note that `GetObject` will not be scoped and you will use a wildcard
(`\*`) for the object.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Map state configuration
ItemsPath (JSONPath)
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.