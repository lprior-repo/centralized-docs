---
url: https://docs.aws.amazon.com/step-functions/latest/dg/example-csv-parse-dist-map.html
title: example csv parse dist map.html
word_count: 554
filtered: true
elements_removed: 0
density_score: 0.79
---

How Step Functions parses input CSV files - AWS Step Functions
How Step Functions parses input CSV files - AWS Step Functions
[](https://docs.aws.amazon.com/pdfs/step-functions/latest/dg/step-functions-dg.pdf#example-csv-parse-dist-map)
###### Managing state and transforming data
Learn about [Passing data between states with variables](./workflow-variables.html) and [Transforming data with JSONata](./transforming-data.html).
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
######
Example of parsing an input CSV file
Say that you have provided a CSV file named ``myCSVInput.csv`` that contains one row as input. Then, you've stored
this file in an Amazon S3 bucket that's named ``amzn-s3-demo-bucket``. The CSV file is as follows.
```
`abc,123,"This string contains commas, a double quotation marks (""), and a newline (
)",{""MyKey"":""MyValue""},"[1,2,3]"`
```
The following state machine reads this CSV file and uses [ItemSelector (Map)](./input-output-itemselector.html) to
convert the data types of some of the fields.
```
`{
"StartAt": "Map",
"States": {
"Map": {
"Type": "Map",
"ItemProcessor": {
"ProcessorConfig": {
"Mode": "DISTRIBUTED",
"ExecutionType": "STANDARD"
},
"StartAt": "Pass",
"States": {
"Pass": {
"Type": "Pass",
"End": true
}
}
},
"End": true,
"Label": "Map",
"MaxConcurrency": 1000,
"ItemReader": {
"Resource": "arn:aws:states:::s3:getObject",
"ReaderConfig": {
"InputType": "CSV",
"CSVHeaderLocation": "GIVEN",
"CSVHeaders": [
"MyLetters",
"MyNumbers",
"MyString",
"MyObject",
"MyArray"
]
},
"Parameters": {
"Bucket": "`amzn-s3-demo-bucket`",
"Key": "`myCSVInput.csv`"
}
},
"ItemSelector": {
"MyLetters.$": "$$.Map.Item.Value.MyLetters",
"MyNumbers.$": "States.StringToJson($$.Map.Item.Value.MyNumbers)",
"MyString.$": "$$.Map.Item.Value.MyString",
"MyObject.$": "States.StringToJson($$.Map.Item.Value.MyObject)",
"MyArray.$": "States.StringToJson($$.Map.Item.Value.MyArray)"
}
}
}
}`
```
When you run this state machine, it produces the following output.
```
`[
{
"MyNumbers": 123,
"MyObject": {
"MyKey": "MyValue"
},
"MyString": "This string contains commas, a double quote (\\"), and a newline (\\n)",
"MyLetters": "abc",
"MyArray": [
1,
2,
3
]
}
]`
```
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
ResultWriter
Integrating services
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.