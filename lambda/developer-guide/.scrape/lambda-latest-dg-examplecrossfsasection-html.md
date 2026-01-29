---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_FSA_section.html
title: Create an application that analyzes customer feedback and synthesizes audio
word_count: 1242
filtered: true
elements_removed: 0
density_score: 0.87
---

Create an application that analyzes customer feedback and synthesizes audio - AWS Lambda
Create an application that analyzes customer feedback and synthesizes audio - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_FSA_section)
# Create an application that analyzes customer feedback and synthesizes audio
The following code examples show how to create an application that analyzes customer comment cards, translates them from their original language, determines their sentiment, and generates an audio file from the translated text.
.NET
**SDK for .NET**
This example application analyzes and stores customer feedback cards. Specifically,
it fulfills the need of a fictitious hotel in New York City. The hotel receives feedback
from guests in various languages in the form of physical comment cards. That feedback
is uploaded into the app through a web client.
After an image of a comment card is uploaded, the following steps occur:
* Text is extracted from the image using Amazon Textract.
* Amazon Comprehend determines the sentiment of the extracted text and its language.
* The extracted text is translated to English using Amazon Translate.
* Amazon Polly synthesizes an audio file from the extracted text.
The full app can be deployed with the AWS CDK. For source code and deployment
instructions, see the project in [
GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/dotnetv3/cross-service/FeedbackSentimentAnalyzer).
###### Services used in this example
* Amazon Comprehend
* Lambda
* Amazon Polly
* Amazon Textract
* Amazon Translate
Java
**SDK for Java 2.x**
This example application analyzes and stores customer feedback cards. Specifically,
it fulfills the need of a fictitious hotel in New York City. The hotel receives feedback
from guests in various languages in the form of physical comment cards. That feedback
is uploaded into the app through a web client.
After an image of a comment card is uploaded, the following steps occur:
* Text is extracted from the image using Amazon Textract.
* Amazon Comprehend determines the sentiment of the extracted text and its language.
* The extracted text is translated to English using Amazon Translate.
* Amazon Polly synthesizes an audio file from the extracted text.
The full app can be deployed with the AWS CDK. For source code and deployment
instructions, see the project in [
GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javav2/usecases/creating_fsa_app).
###### Services used in this example
* Amazon Comprehend
* Lambda
* Amazon Polly
* Amazon Textract
* Amazon Translate
JavaScript
**SDK for JavaScript (v3)**
This example application analyzes and stores customer feedback cards. Specifically,
it fulfills the need of a fictitious hotel in New York City. The hotel receives feedback
from guests in various languages in the form of physical comment cards. That feedback
is uploaded into the app through a web client.
After an image of a comment card is uploaded, the following steps occur:
* Text is extracted from the image using Amazon Textract.
* Amazon Comprehend determines the sentiment of the extracted text and its language.
* The extracted text is translated to English using Amazon Translate.
* Amazon Polly synthesizes an audio file from the extracted text.
The full app can be deployed with the AWS CDK. For source code and deployment
instructions, see the project in [
GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/cross-services/feedback-sentiment-analyzer). The following excerpts show how the AWS SDK for JavaScript is used inside of Lambda functions.
```
`import {
ComprehendClient,
DetectDominantLanguageCommand,
DetectSentimentCommand,
} from "@aws-sdk/client-comprehend";
/\*\*
\* Determine the language and sentiment of the extracted text.
\*
\* @param {{ source\_text: string}} extractTextOutput
\*/
export const handler = async (extractTextOutput) =&gt;&gt; {
const comprehendClient = new ComprehendClient({});
const detectDominantLanguageCommand = new DetectDominantLanguageCommand({
Text: extractTextOutput.source\_text,
});
// The source language is required for sentiment analysis and
// translation in the next step.
const { Languages } = await comprehendClient.send(
detectDominantLanguageCommand,
);
const languageCode = Languages[0].LanguageCode;
const detectSentimentCommand = new DetectSentimentCommand({
Text: extractTextOutput.source\_text,
LanguageCode: languageCode,
});
const { Sentiment } = await comprehendClient.send(detectSentimentCommand);
return {
sentiment: Sentiment,
language\_code: languageCode,
};
};
`
```
```
`import {
DetectDocumentTextCommand,
TextractClient,
} from "@aws-sdk/client-textract";
/\*\*
\* Fetch the S3 object from the event and analyze it using Amazon Textract.
\*
\* @param {import("@types/aws-lambda").EventBridgeEvent&lt;&lt;"Object Created"&gt;&gt;} eventBridgeS3Event
\*/
export const handler = async (eventBridgeS3Event) =&gt;&gt; {
const textractClient = new TextractClient();
const detectDocumentTextCommand = new DetectDocumentTextCommand({
Document: {
S3Object: {
Bucket: eventBridgeS3Event.bucket,
Name: eventBridgeS3Event.object,
},
},
});
// Textract returns a list of blocks. A block can be a line, a page, word, etc.
// Each block also contains geometry of the detected text.
// For more information on the Block type, see https://docs.aws.amazon.com/textract/latest/dg/API\_Block.html.
const { Blocks } = await textractClient.send(detectDocumentTextCommand);
// For the purpose of this example, we are only interested in words.
const extractedWords = Blocks.filter((b) =&gt; b.BlockType === "WORD").map(
(b) =&gt; b.Text,
);
return extractedWords.join(" ");
};
`
```
```
`import { PollyClient, SynthesizeSpeechCommand } from "@aws-sdk/client-polly";
import { S3Client } from "@aws-sdk/client-s3";
import { Upload } from "@aws-sdk/lib-storage";
/\*\*
\* Synthesize an audio file from text.
\*
\* @param {{ bucket: string, translated\_text: string, object: string}} sourceDestinationConfig
\*/
export const handler = async (sourceDestinationConfig) =&gt;&gt; {
const pollyClient = new PollyClient({});
const synthesizeSpeechCommand = new SynthesizeSpeechCommand({
Engine: "neural",
Text: sourceDestinationConfig.translated\_text,
VoiceId: "Ruth",
OutputFormat: "mp3",
});
const { AudioStream } = await pollyClient.send(synthesizeSpeechCommand);
const audioKey = `${sourceDestinationConfig.object}.mp3`;
// Store the audio file in S3.
const s3Client = new S3Client();
const upload = new Upload({
client: s3Client,
params: {
Bucket: sourceDestinationConfig.bucket,
Key: audioKey,
Body: AudioStream,
ContentType: "audio/mp3",
},
});
await upload.done();
return audioKey;
};
`
```
```
`import {
TranslateClient,
TranslateTextCommand,
} from "@aws-sdk/client-translate";
/\*\*
\* Translate the extracted text to English.
\*
\* @param {{ extracted\_text: string, source\_language\_code: string}} textAndSourceLanguage
\*/
export const handler = async (textAndSourceLanguage) =&gt;&gt; {
const translateClient = new TranslateClient({});
const translateCommand = new TranslateTextCommand({
SourceLanguageCode: textAndSourceLanguage.source\_language\_code,
TargetLanguageCode: "en",
Text: textAndSourceLanguage.extracted\_text,
});
const { TranslatedText } = await translateClient.send(translateCommand);
return { translated\_text: TranslatedText };
};
`
```
###### Services used in this example
* Amazon Comprehend
* Lambda
* Amazon Polly
* Amazon Textract
* Amazon Translate
Ruby
**SDK for Ruby**
This example application analyzes and stores customer feedback cards. Specifically,
it fulfills the need of a fictitious hotel in New York City. The hotel receives feedback
from guests in various languages in the form of physical comment cards. That feedback
is uploaded into the app through a web client.
After an image of a comment card is uploaded, the following steps occur:
* Text is extracted from the image using Amazon Textract.
* Amazon Comprehend determines the sentiment of the extracted text and its language.
* The extracted text is translated to English using Amazon Translate.
* Amazon Polly synthesizes an audio file from the extracted text.
The full app can be deployed with the AWS CDK. For source code and deployment
instructions, see the project in [
GitHub](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/ruby/cross_service_examples/feedback_sentiment_analyzer).
###### Services used in this example
* Amazon Comprehend
* Lambda
* Amazon Polly
* Amazon Textract
* Amazon Translate
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Create a websocket chat application
Invoke a Lambda function from a browser
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.