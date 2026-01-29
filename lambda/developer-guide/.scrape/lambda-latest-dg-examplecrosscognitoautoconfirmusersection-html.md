---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_CognitoAutoConfirmUser_section.html
title: Automatically confirm known Amazon Cognito users with a Lambda function using an AWS SDK
word_count: 5456
filtered: true
elements_removed: 0
density_score: 0.85
---

Automatically confirm known Amazon Cognito users with a Lambda function using an AWS SDK - AWS Lambda
Automatically confirm known Amazon Cognito users with a Lambda function using an AWS SDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_CognitoAutoConfirmUser_section)
# Automatically confirm known Amazon Cognito users with a Lambda function using an AWS SDK
The following code examples show how to automatically confirm known Amazon Cognito users with a Lambda function.
* Configure a user pool to call a Lambda function for the `PreSignUp` trigger.
* Sign up a user with Amazon Cognito.
* The Lambda function scans a DynamoDB table and automatically confirms known users.
* Sign in as the new user, then clean up resources.
Go
**SDK for Go V2**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/gov2/workflows/user_pools_and_lambda_triggers#code-examples).
Run an interactive scenario at a command prompt.
```
`
import (
"context"
"errors"
"log"
"strings"
"user\_pools\_and\_lambda\_triggers/actions"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider"
"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider/types"
"github.com/awsdocs/aws-doc-sdk-examples/gov2/demotools"
)
// AutoConfirm separates the steps of this scenario into individual functions so that
// they are simpler to read and understand.
type AutoConfirm struct {
helper IScenarioHelper
questioner demotools.IQuestioner
resources Resources
cognitoActor \*actions.CognitoActions
}
// NewAutoConfirm constructs a new auto confirm runner.
func NewAutoConfirm(sdkConfig aws.Config, questioner demotools.IQuestioner, helper IScenarioHelper) AutoConfirm {
scenario := AutoConfirm{
helper: helper,
questioner: questioner,
resources: Resources{},
cognitoActor: &amp;actions.CognitoActions{CognitoClient: cognitoidentityprovider.NewFromConfig(sdkConfig)},
}
scenario.resources.init(scenario.cognitoActor, questioner)
return scenario
}
// AddPreSignUpTrigger adds a Lambda handler as an invocation target for the PreSignUp trigger.
func (runner \*AutoConfirm) AddPreSignUpTrigger(ctx context.Context, userPoolId string, functionArn string) {
log.Printf("Let's add a Lambda function to handle the PreSignUp trigger from Cognito.\\n" +
"This trigger happens when a user signs up, and lets your function take action before the main Cognito\\n" +
"sign up processing occurs.\\n")
err := runner.cognitoActor.UpdateTriggers(
ctx, userPoolId,
actions.TriggerInfo{Trigger: actions.PreSignUp, HandlerArn: aws.String(functionArn)})
if err != nil {
panic(err)
}
log.Printf("Lambda function %v added to user pool %v to handle the PreSignUp trigger.\\n",
functionArn, userPoolId)
}
// SignUpUser signs up a user from the known user table with a password you specify.
func (runner \*AutoConfirm) SignUpUser(ctx context.Context, clientId string, usersTable string) (string, string) {
log.Println("Let's sign up a user to your Cognito user pool. When the user's email matches an email in the\\n" +
"DynamoDB known users table, it is automatically verified and the user is confirmed.")
knownUsers, err := runner.helper.GetKnownUsers(ctx, usersTable)
if err != nil {
panic(err)
}
userChoice := runner.questioner.AskChoice("Which user do you want to use?\\n", knownUsers.UserNameList())
user := knownUsers.Users[userChoice]
var signedUp bool
var userConfirmed bool
password := runner.questioner.AskPassword("Enter a password that has at least eight characters, uppercase, lowercase, numbers and symbols.\\n"+
"(the password will not display as you type):", 8)
for !signedUp {
log.Printf("Signing up user '%v' with email '%v' to Cognito.\\n", user.UserName, user.UserEmail)
userConfirmed, err = runner.cognitoActor.SignUp(ctx, clientId, user.UserName, password, user.UserEmail)
if err != nil {
var invalidPassword \*types.InvalidPasswordException
if errors.As(err, &amp;&amp;invalidPassword) {
password = runner.questioner.AskPassword("Enter another password:", 8)
} else {
panic(err)
}
} else {
signedUp = true
}
}
log.Printf("User %v signed up, confirmed = %v.\\n", user.UserName, userConfirmed)
log.Println(strings.Repeat("-", 88))
return user.UserName, password
}
// SignInUser signs in a user.
func (runner \*AutoConfirm) SignInUser(ctx context.Context, clientId string, userName string, password string) string {
runner.questioner.Ask("Press Enter when you're ready to continue.")
log.Printf("Let's sign in as %v...\\n", userName)
authResult, err := runner.cognitoActor.SignIn(ctx, clientId, userName, password)
if err != nil {
panic(err)
}
log.Printf("Successfully signed in. Your access token starts with: %v...\\n", (\*authResult.AccessToken)[:10])
log.Println(strings.Repeat("-", 88))
return \*authResult.AccessToken
}
// Run runs the scenario.
func (runner \*AutoConfirm) Run(ctx context.Context, stackName string) {
defer func() {
if r := recover(); r != nil {
log.Println("Something went wrong with the demo.")
runner.resources.Cleanup(ctx)
}
}()
log.Println(strings.Repeat("-", 88))
log.Printf("Welcome\\n")
log.Println(strings.Repeat("-", 88))
stackOutputs, err := runner.helper.GetStackOutputs(ctx, stackName)
if err != nil {
panic(err)
}
runner.resources.userPoolId = stackOutputs["UserPoolId"]
runner.helper.PopulateUserTable(ctx, stackOutputs["TableName"])
runner.AddPreSignUpTrigger(ctx, stackOutputs["UserPoolId"], stackOutputs["AutoConfirmFunctionArn"])
runner.resources.triggers = append(runner.resources.triggers, actions.PreSignUp)
userName, password := runner.SignUpUser(ctx, stackOutputs["UserPoolClientId"], stackOutputs["TableName"])
runner.helper.ListRecentLogEvents(ctx, stackOutputs["AutoConfirmFunction"])
runner.resources.userAccessTokens = append(runner.resources.userAccessTokens,
runner.SignInUser(ctx, stackOutputs["UserPoolClientId"], userName, password))
runner.resources.Cleanup(ctx)
log.Println(strings.Repeat("-", 88))
log.Println("Thanks for watching!")
log.Println(strings.Repeat("-", 88))
}
`
```
Handle the `PreSignUp` trigger with a Lambda function.
```
`
import (
"context"
"log"
"os"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/config"
"github.com/aws/aws-sdk-go-v2/feature/dynamodb/attributevalue"
"github.com/aws/aws-sdk-go-v2/service/dynamodb"
dynamodbtypes "github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)
const TABLE\_NAME = "TABLE\_NAME"
// UserInfo defines structured user data that can be marshalled to a DynamoDB format.
type UserInfo struct {
UserName string `dynamodbav:"UserName"`
UserEmail string `dynamodbav:"UserEmail"`
}
// GetKey marshals the user email value to a DynamoDB key format.
func (user UserInfo) GetKey() map[string]dynamodbtypes.AttributeValue {
userEmail, err := attributevalue.Marshal(user.UserEmail)
if err != nil {
panic(err)
}
return map[string]dynamodbtypes.AttributeValue{"UserEmail": userEmail}
}
type handler struct {
dynamoClient \*dynamodb.Client
}
// HandleRequest handles the PreSignUp event by looking up a user in an Amazon DynamoDB table and
// specifying whether they should be confirmed and verified.
func (h \*handler) HandleRequest(ctx context.Context, event events.CognitoEventUserPoolsPreSignup) (events.CognitoEventUserPoolsPreSignup, error) {
log.Printf("Received presignup from %v for user '%v'", event.TriggerSource, event.UserName)
if event.TriggerSource != "PreSignUp\_SignUp" {
// Other trigger sources, such as PreSignUp\_AdminInitiateAuth, ignore the response from this handler.
return event, nil
}
tableName := os.Getenv(TABLE\_NAME)
user := UserInfo{
UserEmail: event.Request.UserAttributes["email"],
}
log.Printf("Looking up email %v in table %v.\\n", user.UserEmail, tableName)
output, err := h.dynamoClient.GetItem(ctx, &amp;&amp;dynamodb.GetItemInput{
Key: user.GetKey(),
TableName: aws.String(tableName),
})
if err != nil {
log.Printf("Error looking up email %v.\\n", user.UserEmail)
return event, err
}
if output.Item == nil {
log.Printf("Email %v not found. Email verification is required.\\n", user.UserEmail)
return event, err
}
err = attributevalue.UnmarshalMap(output.Item, &amp;&amp;user)
if err != nil {
log.Printf("Couldn't unmarshal DynamoDB item. Here's why: %v\\n", err)
return event, err
}
if user.UserName != event.UserName {
log.Printf("UserEmail %v found, but stored UserName '%v' does not match supplied UserName '%v'. Verification is required.\\n",
user.UserEmail, user.UserName, event.UserName)
} else {
log.Printf("UserEmail %v found with matching UserName %v. User is confirmed.\\n", user.UserEmail, user.UserName)
event.Response.AutoConfirmUser = true
event.Response.AutoVerifyEmail = true
}
return event, err
}
func main() {
ctx := context.Background()
sdkConfig, err := config.LoadDefaultConfig(ctx)
if err != nil {
log.Panicln(err)
}
h := handler{
dynamoClient: dynamodb.NewFromConfig(sdkConfig),
}
lambda.Start(h.HandleRequest)
}
`
```
Create a struct that performs common tasks.
```
`
import (
"context"
"log"
"strings"
"time"
"user\_pools\_and\_lambda\_triggers/actions"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/cloudformation"
"github.com/aws/aws-sdk-go-v2/service/cloudwatchlogs"
"github.com/aws/aws-sdk-go-v2/service/dynamodb"
"github.com/awsdocs/aws-doc-sdk-examples/gov2/demotools"
)
// IScenarioHelper defines common functions used by the workflows in this example.
type IScenarioHelper interface {
Pause(secs int)
GetStackOutputs(ctx context.Context, stackName string) (actions.StackOutputs, error)
PopulateUserTable(ctx context.Context, tableName string)
GetKnownUsers(ctx context.Context, tableName string) (actions.UserList, error)
AddKnownUser(ctx context.Context, tableName string, user actions.User)
ListRecentLogEvents(ctx context.Context, functionName string)
}
// ScenarioHelper contains AWS wrapper structs used by the workflows in this example.
type ScenarioHelper struct {
questioner demotools.IQuestioner
dynamoActor \*actions.DynamoActions
cfnActor \*actions.CloudFormationActions
cwlActor \*actions.CloudWatchLogsActions
isTestRun bool
}
// NewScenarioHelper constructs a new scenario helper.
func NewScenarioHelper(sdkConfig aws.Config, questioner demotools.IQuestioner) ScenarioHelper {
scenario := ScenarioHelper{
questioner: questioner,
dynamoActor: &amp;actions.DynamoActions{DynamoClient: dynamodb.NewFromConfig(sdkConfig)},
cfnActor: &amp;actions.CloudFormationActions{CfnClient: cloudformation.NewFromConfig(sdkConfig)},
cwlActor: &amp;actions.CloudWatchLogsActions{CwlClient: cloudwatchlogs.NewFromConfig(sdkConfig)},
}
return scenario
}
// Pause waits for the specified number of seconds.
func (helper ScenarioHelper) Pause(secs int) {
if !helper.isTestRun {
time.Sleep(time.Duration(secs) \* time.Second)
}
}
// GetStackOutputs gets the outputs from the specified CloudFormation stack in a structured format.
func (helper ScenarioHelper) GetStackOutputs(ctx context.Context, stackName string) (actions.StackOutputs, error) {
return helper.cfnActor.GetOutputs(ctx, stackName), nil
}
// PopulateUserTable fills the known user table with example data.
func (helper ScenarioHelper) PopulateUserTable(ctx context.Context, tableName string) {
log.Printf("First, let's add some users to the DynamoDB %v table we'll use for this example.\\n", tableName)
err := helper.dynamoActor.PopulateTable(ctx, tableName)
if err != nil {
panic(err)
}
}
// GetKnownUsers gets the users from the known users table in a structured format.
func (helper ScenarioHelper) GetKnownUsers(ctx context.Context, tableName string) (actions.UserList, error) {
knownUsers, err := helper.dynamoActor.Scan(ctx, tableName)
if err != nil {
log.Printf("Couldn't get known users from table %v. Here's why: %v\\n", tableName, err)
}
return knownUsers, err
}
// AddKnownUser adds a user to the known users table.
func (helper ScenarioHelper) AddKnownUser(ctx context.Context, tableName string, user actions.User) {
log.Printf("Adding user '%v' with email '%v' to the DynamoDB known users table...\\n",
user.UserName, user.UserEmail)
err := helper.dynamoActor.AddUser(ctx, tableName, user)
if err != nil {
panic(err)
}
}
// ListRecentLogEvents gets the most recent log stream and events for the specified Lambda function and displays them.
func (helper ScenarioHelper) ListRecentLogEvents(ctx context.Context, functionName string) {
log.Println("Waiting a few seconds to let Lambda write to CloudWatch Logs...")
helper.Pause(10)
log.Println("Okay, let's check the logs to find what's happened recently with your Lambda function.")
logStream, err := helper.cwlActor.GetLatestLogStream(ctx, functionName)
if err != nil {
panic(err)
}
log.Printf("Getting some recent events from log stream %v\\n", \*logStream.LogStreamName)
events, err := helper.cwlActor.GetLogEvents(ctx, functionName, \*logStream.LogStreamName, 10)
if err != nil {
panic(err)
}
for \_, event := range events {
log.Printf("\\t%v", \*event.Message)
}
log.Println(strings.Repeat("-", 88))
}
`
```
Create a struct that wraps Amazon Cognito actions.
```
`
import (
"context"
"errors"
"log"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider"
"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider/types"
)
type CognitoActions struct {
CognitoClient \*cognitoidentityprovider.Client
}
// Trigger and TriggerInfo define typed data for updating an Amazon Cognito trigger.
type Trigger int
const (
PreSignUp Trigger = iota
UserMigration
PostAuthentication
)
type TriggerInfo struct {
Trigger Trigger
HandlerArn \*string
}
// UpdateTriggers adds or removes Lambda triggers for a user pool. When a trigger is specified with a `nil` value,
// it is removed from the user pool.
func (actor CognitoActions) UpdateTriggers(ctx context.Context, userPoolId string, triggers ...TriggerInfo) error {
output, err := actor.CognitoClient.DescribeUserPool(ctx, &amp;cognitoidentityprovider.DescribeUserPoolInput{
UserPoolId: aws.String(userPoolId),
})
if err != nil {
log.Printf("Couldn't get info about user pool %v. Here's why: %v\\n", userPoolId, err)
return err
}
lambdaConfig := output.UserPool.LambdaConfig
for \_, trigger := range triggers {
switch trigger.Trigger {
case PreSignUp:
lambdaConfig.PreSignUp = trigger.HandlerArn
case UserMigration:
lambdaConfig.UserMigration = trigger.HandlerArn
case PostAuthentication:
lambdaConfig.PostAuthentication = trigger.HandlerArn
}
}
\_, err = actor.CognitoClient.UpdateUserPool(ctx, &amp;&amp;cognitoidentityprovider.UpdateUserPoolInput{
UserPoolId: aws.String(userPoolId),
LambdaConfig: lambdaConfig,
})
if err != nil {
log.Printf("Couldn't update user pool %v. Here's why: %v\\n", userPoolId, err)
}
return err
}
// SignUp signs up a user with Amazon Cognito.
func (actor CognitoActions) SignUp(ctx context.Context, clientId string, userName string, password string, userEmail string) (bool, error) {
confirmed := false
output, err := actor.CognitoClient.SignUp(ctx, &amp;cognitoidentityprovider.SignUpInput{
ClientId: aws.String(clientId),
Password: aws.String(password),
Username: aws.String(userName),
UserAttributes: []types.AttributeType{
{Name: aws.String("email"), Value: aws.String(userEmail)},
},
})
if err != nil {
var invalidPassword \*types.InvalidPasswordException
if errors.As(err, &amp;&amp;invalidPassword) {
log.Println(\*invalidPassword.Message)
} else {
log.Printf("Couldn't sign up user %v. Here's why: %v\\n", userName, err)
}
} else {
confirmed = output.UserConfirmed
}
return confirmed, err
}
// SignIn signs in a user to Amazon Cognito using a username and password authentication flow.
func (actor CognitoActions) SignIn(ctx context.Context, clientId string, userName string, password string) (\*types.AuthenticationResultType, error) {
var authResult \*types.AuthenticationResultType
output, err := actor.CognitoClient.InitiateAuth(ctx, &amp;&amp;cognitoidentityprovider.InitiateAuthInput{
AuthFlow: "USER\_PASSWORD\_AUTH",
ClientId: aws.String(clientId),
AuthParameters: map[string]string{"USERNAME": userName, "PASSWORD": password},
})
if err != nil {
var resetRequired \*types.PasswordResetRequiredException
if errors.As(err, &amp;&amp;resetRequired) {
log.Println(\*resetRequired.Message)
} else {
log.Printf("Couldn't sign in user %v. Here's why: %v\\n", userName, err)
}
} else {
authResult = output.AuthenticationResult
}
return authResult, err
}
// ForgotPassword starts a password recovery flow for a user. This flow typically sends a confirmation code
// to the user's configured notification destination, such as email.
func (actor CognitoActions) ForgotPassword(ctx context.Context, clientId string, userName string) (\*types.CodeDeliveryDetailsType, error) {
output, err := actor.CognitoClient.ForgotPassword(ctx, &amp;cognitoidentityprovider.ForgotPasswordInput{
ClientId: aws.String(clientId),
Username: aws.String(userName),
})
if err != nil {
log.Printf("Couldn't start password reset for user '%v'. Here;s why: %v\\n", userName, err)
}
return output.CodeDeliveryDetails, err
}
// ConfirmForgotPassword confirms a user with a confirmation code and a new password.
func (actor CognitoActions) ConfirmForgotPassword(ctx context.Context, clientId string, code string, userName string, password string) error {
\_, err := actor.CognitoClient.ConfirmForgotPassword(ctx, &amp;&amp;cognitoidentityprovider.ConfirmForgotPasswordInput{
ClientId: aws.String(clientId),
ConfirmationCode: aws.String(code),
Password: aws.String(password),
Username: aws.String(userName),
})
if err != nil {
var invalidPassword \*types.InvalidPasswordException
if errors.As(err, &amp;&amp;invalidPassword) {
log.Println(\*invalidPassword.Message)
} else {
log.Printf("Couldn't confirm user %v. Here's why: %v", userName, err)
}
}
return err
}
// DeleteUser removes a user from the user pool.
func (actor CognitoActions) DeleteUser(ctx context.Context, userAccessToken string) error {
\_, err := actor.CognitoClient.DeleteUser(ctx, &amp;&amp;cognitoidentityprovider.DeleteUserInput{
AccessToken: aws.String(userAccessToken),
})
if err != nil {
log.Printf("Couldn't delete user. Here's why: %v\\n", err)
}
return err
}
// AdminCreateUser uses administrator credentials to add a user to a user pool. This method leaves the user
// in a state that requires they enter a new password next time they sign in.
func (actor CognitoActions) AdminCreateUser(ctx context.Context, userPoolId string, userName string, userEmail string) error {
\_, err := actor.CognitoClient.AdminCreateUser(ctx, &amp;&amp;cognitoidentityprovider.AdminCreateUserInput{
UserPoolId: aws.String(userPoolId),
Username: aws.String(userName),
MessageAction: types.MessageActionTypeSuppress,
UserAttributes: []types.AttributeType{{Name: aws.String("email"), Value: aws.String(userEmail)}},
})
if err != nil {
var userExists \*types.UsernameExistsException
if errors.As(err, &amp;&amp;userExists) {
log.Printf("User %v already exists in the user pool.", userName)
err = nil
} else {
log.Printf("Couldn't create user %v. Here's why: %v\\n", userName, err)
}
}
return err
}
// AdminSetUserPassword uses administrator credentials to set a password for a user without requiring a
// temporary password.
func (actor CognitoActions) AdminSetUserPassword(ctx context.Context, userPoolId string, userName string, password string) error {
\_, err := actor.CognitoClient.AdminSetUserPassword(ctx, &amp;&amp;cognitoidentityprovider.AdminSetUserPasswordInput{
Password: aws.String(password),
UserPoolId: aws.String(userPoolId),
Username: aws.String(userName),
Permanent: true,
})
if err != nil {
var invalidPassword \*types.InvalidPasswordException
if errors.As(err, &amp;&amp;invalidPassword) {
log.Println(\*invalidPassword.Message)
} else {
log.Printf("Couldn't set password for user %v. Here's why: %v\\n", userName, err)
}
}
return err
}
`
```
Create a struct that wraps DynamoDB actions.
```
`
import (
"context"
"fmt"
"log"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/feature/dynamodb/attributevalue"
"github.com/aws/aws-sdk-go-v2/service/dynamodb"
"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)
// DynamoActions encapsulates the Amazon Simple Notification Service (Amazon SNS) actions
// used in the examples.
type DynamoActions struct {
DynamoClient \*dynamodb.Client
}
// User defines structured user data.
type User struct {
UserName string
UserEmail string
LastLogin \*LoginInfo `dynamodbav:",omitempty"`
}
// LoginInfo defines structured custom login data.
type LoginInfo struct {
UserPoolId string
ClientId string
Time string
}
// UserList defines a list of users.
type UserList struct {
Users []User
}
// UserNameList returns the usernames contained in a UserList as a list of strings.
func (users \*UserList) UserNameList() []string {
names := make([]string, len(users.Users))
for i := 0; i &lt; len(users.Users); i++ {
names[i] = users.Users[i].UserName
}
return names
}
// PopulateTable adds a set of test users to the table.
func (actor DynamoActions) PopulateTable(ctx context.Context, tableName string) error {
var err error
var item map[string]types.AttributeValue
var writeReqs []types.WriteRequest
for i := 1; i &lt; 4; i++ {
item, err = attributevalue.MarshalMap(User{UserName: fmt.Sprintf("test\_user\_%v", i), UserEmail: fmt.Sprintf("test\_email\_%v@example.com", i)})
if err != nil {
log.Printf("Couldn't marshall user into DynamoDB format. Here's why: %v\\n", err)
return err
}
writeReqs = append(writeReqs, types.WriteRequest{PutRequest: &amp;types.PutRequest{Item: item}})
}
\_, err = actor.DynamoClient.BatchWriteItem(ctx, &amp;&amp;dynamodb.BatchWriteItemInput{
RequestItems: map[string][]types.WriteRequest{tableName: writeReqs},
})
if err != nil {
log.Printf("Couldn't populate table %v with users. Here's why: %v\\n", tableName, err)
}
return err
}
// Scan scans the table for all items.
func (actor DynamoActions) Scan(ctx context.Context, tableName string) (UserList, error) {
var userList UserList
output, err := actor.DynamoClient.Scan(ctx, &amp;dynamodb.ScanInput{
TableName: aws.String(tableName),
})
if err != nil {
log.Printf("Couldn't scan table %v for items. Here's why: %v\\n", tableName, err)
} else {
err = attributevalue.UnmarshalListOfMaps(output.Items, &amp;userList.Users)
if err != nil {
log.Printf("Couldn't unmarshal items into users. Here's why: %v\\n", err)
}
}
return userList, err
}
// AddUser adds a user item to a table.
func (actor DynamoActions) AddUser(ctx context.Context, tableName string, user User) error {
userItem, err := attributevalue.MarshalMap(user)
if err != nil {
log.Printf("Couldn't marshall user to item. Here's why: %v\\n", err)
}
\_, err = actor.DynamoClient.PutItem(ctx, &amp;&amp;dynamodb.PutItemInput{
Item: userItem,
TableName: aws.String(tableName),
})
if err != nil {
log.Printf("Couldn't put item in table %v. Here's why: %v", tableName, err)
}
return err
}
`
```
Create a struct that wraps CloudWatch Logs actions.
```
`
import (
"context"
"fmt"
"log"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/cloudwatchlogs"
"github.com/aws/aws-sdk-go-v2/service/cloudwatchlogs/types"
)
type CloudWatchLogsActions struct {
CwlClient \*cloudwatchlogs.Client
}
// GetLatestLogStream gets the most recent log stream for a Lambda function.
func (actor CloudWatchLogsActions) GetLatestLogStream(ctx context.Context, functionName string) (types.LogStream, error) {
var logStream types.LogStream
logGroupName := fmt.Sprintf("/aws/lambda/%s", functionName)
output, err := actor.CwlClient.DescribeLogStreams(ctx, &amp;cloudwatchlogs.DescribeLogStreamsInput{
Descending: aws.Bool(true),
Limit: aws.Int32(1),
LogGroupName: aws.String(logGroupName),
OrderBy: types.OrderByLastEventTime,
})
if err != nil {
log.Printf("Couldn't get log streams for log group %v. Here's why: %v\\n", logGroupName, err)
} else {
logStream = output.LogStreams[0]
}
return logStream, err
}
// GetLogEvents gets the most recent eventCount events from the specified log stream.
func (actor CloudWatchLogsActions) GetLogEvents(ctx context.Context, functionName string, logStreamName string, eventCount int32) (
[]types.OutputLogEvent, error) {
var events []types.OutputLogEvent
logGroupName := fmt.Sprintf("/aws/lambda/%s", functionName)
output, err := actor.CwlClient.GetLogEvents(ctx, &amp;cloudwatchlogs.GetLogEventsInput{
LogStreamName: aws.String(logStreamName),
Limit: aws.Int32(eventCount),
LogGroupName: aws.String(logGroupName),
})
if err != nil {
log.Printf("Couldn't get log event for log stream %v. Here's why: %v\\n", logStreamName, err)
} else {
events = output.Events
}
return events, err
}
`
```
Create a struct that wraps CloudFormation actions.
```
`
import (
"context"
"log"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/cloudformation"
)
// StackOutputs defines a map of outputs from a specific stack.
type StackOutputs map[string]string
type CloudFormationActions struct {
CfnClient \*cloudformation.Client
}
// GetOutputs gets the outputs from a CloudFormation stack and puts them into a structured format.
func (actor CloudFormationActions) GetOutputs(ctx context.Context, stackName string) StackOutputs {
output, err := actor.CfnClient.DescribeStacks(ctx, &amp;cloudformation.DescribeStacksInput{
StackName: aws.String(stackName),
})
if err != nil || len(output.Stacks) == 0 {
log.Panicf("Couldn't find a CloudFormation stack named %v. Here's why: %v\\n", stackName, err)
}
stackOutputs := StackOutputs{}
for \_, out := range output.Stacks[0].Outputs {
stackOutputs[\*out.OutputKey] = \*out.OutputValue
}
return stackOutputs
}
`
```
Clean up resources.
```
`
import (
"context"
"log"
"user\_pools\_and\_lambda\_triggers/actions"
"github.com/awsdocs/aws-doc-sdk-examples/gov2/demotools"
)
// Resources keeps track of AWS resources created during an example and handles
// cleanup when the example finishes.
type Resources struct {
userPoolId string
userAccessTokens []string
triggers []actions.Trigger
cognitoActor \*actions.CognitoActions
questioner demotools.IQuestioner
}
func (resources \*Resources) init(cognitoActor \*actions.CognitoActions, questioner demotools.IQuestioner) {
resources.userAccessTokens = []string{}
resources.triggers = []actions.Trigger{}
resources.cognitoActor = cognitoActor
resources.questioner = questioner
}
// Cleanup deletes all AWS resources created during an example.
func (resources \*Resources) Cleanup(ctx context.Context) {
defer func() {
if r := recover(); r != nil {
log.Printf("Something went wrong during cleanup.\\n%v\\n", r)
log.Println("Use the AWS Management Console to remove any remaining resources \\n" +
"that were created for this scenario.")
}
}()
wantDelete := resources.questioner.AskBool("Do you want to remove all of the AWS resources that were created "+
"during this demo (y/n)?", "y")
if wantDelete {
for \_, accessToken := range resources.userAccessTokens {
err := resources.cognitoActor.DeleteUser(ctx, accessToken)
if err != nil {
log.Println("Couldn't delete user during cleanup.")
panic(err)
}
log.Println("Deleted user.")
}
triggerList := make([]actions.TriggerInfo, len(resources.triggers))
for i := 0; i &lt; len(resources.triggers); i++ {
triggerList[i] = actions.TriggerInfo{Trigger: resources.triggers[i], HandlerArn: nil}
}
err := resources.cognitoActor.UpdateTriggers(ctx, resources.userPoolId, triggerList...)
if err != nil {
log.Println("Couldn't update Cognito triggers during cleanup.")
panic(err)
}
log.Println("Removed Cognito triggers from user pool.")
} else {
log.Println("Be sure to remove resources when you're done with them to avoid unexpected charges!")
}
}
`
```
* For API details, see the following topics in *AWS SDK for Go API Reference*.
* [DeleteUser](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.DeleteUser)
* [InitiateAuth](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.InitiateAuth)
* [SignUp](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.SignUp)
* [UpdateUserPool](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.UpdateUserPool)
JavaScript
**SDK for JavaScript (v3)**
###### Note
There's more on GitHub. Find the complete example and learn how to set up and run in the
[AWS Code
Examples Repository](https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/javascriptv3/example_code/cross-services/wkflw-pools-triggers#code-examples).
Configure an interactive "Scenario" run. The JavaScript (v3) examples
share a Scenario runner to streamline complex examples. The complete
source code is on GitHub.
```
`import { AutoConfirm } from "./scenario-auto-confirm.js";
/\*\*
\* The context is passed to every scenario. Scenario steps
\* will modify the context.
\*/
const context = {
errors: [],
users: [
{
UserName: "test\_user\_1",
UserEmail: "test\_email\_1@example.com",
},
{
UserName: "test\_user\_2",
UserEmail: "test\_email\_2@example.com",
},
{
UserName: "test\_user\_3",
UserEmail: "test\_email\_3@example.com",
},
],
};
/\*\*
\* Three Scenarios are created for the workflow. A Scenario is an orchestration class
\* that simplifies running a series of steps.
\*/
export const scenarios = {
// Demonstrate automatically confirming known users in a database.
"auto-confirm": AutoConfirm(context),
};
// Call function if run directly
import { fileURLToPath } from "node:url";
import { parseScenarioArgs } from "@aws-doc-sdk-examples/lib/scenario/index.js";
if (process.argv[1] === fileURLToPath(import.meta.url)) {
parseScenarioArgs(scenarios, {
name: "Cognito user pools and triggers",
description:
"Demonstrate how to use the AWS SDKs to customize Amazon Cognito authentication behavior.",
});
}
`
```
This Scenario demonstrates auto-confirming a known user.
It orchestrates the example steps.
```
`import { wait } from "@aws-doc-sdk-examples/lib/utils/util-timers.js";
import {
Scenario,
ScenarioAction,
ScenarioInput,
ScenarioOutput,
} from "@aws-doc-sdk-examples/lib/scenario/scenario.js";
import {
getStackOutputs,
logCleanUpReminder,
promptForStackName,
promptForStackRegion,
skipWhenErrors,
} from "./steps-common.js";
import { populateTable } from "./actions/dynamodb-actions.js";
import {
addPreSignUpHandler,
deleteUser,
getUser,
signIn,
signUpUser,
} from "./actions/cognito-actions.js";
import {
getLatestLogStreamForLambda,
getLogEvents,
} from "./actions/cloudwatch-logs-actions.js";
/\*\*
\* @typedef {{
\* errors: Error[],
\* password: string,
\* users: { UserName: string, UserEmail: string }[],
\* selectedUser?: string,
\* stackName?: string,
\* stackRegion?: string,
\* token?: string,
\* confirmDeleteSignedInUser?: boolean,
\* TableName?: string,
\* UserPoolClientId?: string,
\* UserPoolId?: string,
\* UserPoolArn?: string,
\* AutoConfirmHandlerArn?: string,
\* AutoConfirmHandlerName?: string
\* }} State
\*/
const greeting = new ScenarioOutput(
"greeting",
(/\*\* @type {State} \*/ state) =&gt;&gt; `This demo will populate some users into the \\
database created as part of the "${state.stackName}" stack. \\
Then the AutoConfirmHandler will be linked to the PreSignUp \\
trigger from Cognito. Finally, you will choose a user to sign up.`,
{ skipWhen: skipWhenErrors },
);
const logPopulatingUsers = new ScenarioOutput(
"logPopulatingUsers",
"Populating the DynamoDB table with some users.",
{ skipWhenErrors: skipWhenErrors },
);
const logPopulatingUsersComplete = new ScenarioOutput(
"logPopulatingUsersComplete",
"Done populating users.",
{ skipWhen: skipWhenErrors },
);
const populateUsers = new ScenarioAction(
"populateUsers",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
const [\_, err] = await populateTable({
region: state.stackRegion,
tableName: state.TableName,
items: state.users,
});
if (err) {
state.errors.push(err);
}
},
{
skipWhen: skipWhenErrors,
},
);
const logSetupSignUpTrigger = new ScenarioOutput(
"logSetupSignUpTrigger",
"Setting up the PreSignUp trigger for the Cognito User Pool.",
{ skipWhen: skipWhenErrors },
);
const setupSignUpTrigger = new ScenarioAction(
"setupSignUpTrigger",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
const [\_, err] = await addPreSignUpHandler({
region: state.stackRegion,
userPoolId: state.UserPoolId,
handlerArn: state.AutoConfirmHandlerArn,
});
if (err) {
state.errors.push(err);
}
},
{
skipWhen: skipWhenErrors,
},
);
const logSetupSignUpTriggerComplete = new ScenarioOutput(
"logSetupSignUpTriggerComplete",
(
/\*\* @type {State} \*/ state,
) =&gt;&gt; `The lambda function "${state.AutoConfirmHandlerName}" \\
has been configured as the PreSignUp trigger handler for the user pool "${state.UserPoolId}".`,
{ skipWhen: skipWhenErrors },
);
const selectUser = new ScenarioInput(
"selectedUser",
"Select a user to sign up.",
{
type: "select",
choices: (/\*\* @type {State} \*/ state) =&gt;&gt; state.users.map((u) =&gt;&gt; u.UserName),
skipWhen: skipWhenErrors,
default: (/\*\* @type {State} \*/ state) =&gt;&gt; state.users[0].UserName,
},
);
const checkIfUserAlreadyExists = new ScenarioAction(
"checkIfUserAlreadyExists",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
const [user, err] = await getUser({
region: state.stackRegion,
userPoolId: state.UserPoolId,
username: state.selectedUser,
});
if (err?.name === "UserNotFoundException") {
// Do nothing. We're not expecting the user to exist before
// sign up is complete.
return;
}
if (err) {
state.errors.push(err);
return;
}
if (user) {
state.errors.push(
new Error(
`The user "${state.selectedUser}" already exists in the user pool "${state.UserPoolId}".`,
),
);
}
},
{
skipWhen: skipWhenErrors,
},
);
const createPassword = new ScenarioInput(
"password",
"Enter a password that has at least eight characters, uppercase, lowercase, numbers and symbols.",
{ type: "password", skipWhen: skipWhenErrors, default: "Abcd1234!" },
);
const logSignUpExistingUser = new ScenarioOutput(
"logSignUpExistingUser",
(/\*\* @type {State} \*/ state) =&gt;&gt; `Signing up user "${state.selectedUser}".`,
{ skipWhen: skipWhenErrors },
);
const signUpExistingUser = new ScenarioAction(
"signUpExistingUser",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
const signUp = (password) =&gt;
signUpUser({
region: state.stackRegion,
userPoolClientId: state.UserPoolClientId,
username: state.selectedUser,
email: state.users.find((u) =&gt;&gt; u.UserName === state.selectedUser)
.UserEmail,
password,
});
let [\_, err] = await signUp(state.password);
while (err?.name === "InvalidPasswordException") {
console.warn("The password you entered was invalid.");
await createPassword.handle(state);
[\_, err] = await signUp(state.password);
}
if (err) {
state.errors.push(err);
}
},
{ skipWhen: skipWhenErrors },
);
const logSignUpExistingUserComplete = new ScenarioOutput(
"logSignUpExistingUserComplete",
(/\*\* @type {State} \*/ state) =&gt;&gt;
`"${state.selectedUser} was signed up successfully.`,
{ skipWhen: skipWhenErrors },
);
const logLambdaLogs = new ScenarioAction(
"logLambdaLogs",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
console.log(
"Waiting a few seconds to let Lambda write to CloudWatch Logs...\\n",
);
await wait(10);
const [logStream, logStreamErr] = await getLatestLogStreamForLambda({
functionName: state.AutoConfirmHandlerName,
region: state.stackRegion,
});
if (logStreamErr) {
state.errors.push(logStreamErr);
return;
}
console.log(
`Getting some recent events from log stream "${logStream.logStreamName}"`,
);
const [logEvents, logEventsErr] = await getLogEvents({
functionName: state.AutoConfirmHandlerName,
region: state.stackRegion,
eventCount: 10,
logStreamName: logStream.logStreamName,
});
if (logEventsErr) {
state.errors.push(logEventsErr);
return;
}
console.log(logEvents.map((ev) =&gt;&gt; `\\t${ev.message}`).join(""));
},
{ skipWhen: skipWhenErrors },
);
const logSignInUser = new ScenarioOutput(
"logSignInUser",
(/\*\* @type {State} \*/ state) =&gt;&gt; `Let's sign in as ${state.selectedUser}`,
{ skipWhen: skipWhenErrors },
);
const signInUser = new ScenarioAction(
"signInUser",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
const [response, err] = await signIn({
region: state.stackRegion,
clientId: state.UserPoolClientId,
username: state.selectedUser,
password: state.password,
});
if (err?.name === "PasswordResetRequiredException") {
state.errors.push(new Error("Please reset your password."));
return;
}
if (err) {
state.errors.push(err);
return;
}
state.token = response?.AuthenticationResult?.AccessToken;
},
{ skipWhen: skipWhenErrors },
);
const logSignInUserComplete = new ScenarioOutput(
"logSignInUserComplete",
(/\*\* @type {State} \*/ state) =&gt;&gt;
`Successfully signed in. Your access token starts with: ${state.token.slice(0, 11)}`,
{ skipWhen: skipWhenErrors },
);
const confirmDeleteSignedInUser = new ScenarioInput(
"confirmDeleteSignedInUser",
"Do you want to delete the currently signed in user?",
{ type: "confirm", skipWhen: skipWhenErrors },
);
const deleteSignedInUser = new ScenarioAction(
"deleteSignedInUser",
async (/\*\* @type {State} \*/ state) =&gt;&gt; {
const [\_, err] = await deleteUser({
region: state.stackRegion,
accessToken: state.token,
});
if (err) {
state.errors.push(err);
}
},
{
skipWhen: (/\*\* @type {State} \*/ state) =&gt;&gt;
skipWhenErrors(state) || !state.confirmDeleteSignedInUser,
},
);
const logErrors = new ScenarioOutput(
"logErrors",
(/\*\* @type {State}\*/ state) =&gt;&gt; {
const errorList = state.errors
.map((err) =&gt; ` - ${err.name}: ${err.message}`)
.join("\\n");
return `Scenario errors found:\\n${errorList}`;
},
{
// Don't log errors when there aren't any!
skipWhen: (/\*\* @type {State} \*/ state) =&gt;&gt; state.errors.length === 0,
},
);
export const AutoConfirm = (context) =&gt;&gt;
new Scenario(
"AutoConfirm",
[
promptForStackName,
promptForStackRegion,
getStackOutputs,
greeting,
logPopulatingUsers,
populateUsers,
logPopulatingUsersComplete,
logSetupSignUpTrigger,
setupSignUpTrigger,
logSetupSignUpTriggerComplete,
selectUser,
checkIfUserAlreadyExists,
createPassword,
logSignUpExistingUser,
signUpExistingUser,
logSignUpExistingUserComplete,
logLambdaLogs,
logSignInUser,
signInUser,
logSignInUserComplete,
confirmDeleteSignedInUser,
deleteSignedInUser,
logCleanUpReminder,
logErrors,
],
context,
);
`
```
These are steps that are shared with other Scenarios.
```
`import {
ScenarioAction,
ScenarioInput,
ScenarioOutput,
} from "@aws-doc-sdk-examples/lib/scenario/scenario.js";
import { getCfnOutputs } from "@aws-doc-sdk-examples/lib/sdk/cfn-outputs.js";
export const skipWhenErrors = (state) =&gt; state.errors.length &gt; 0;
export const getStackOutputs = new ScenarioAction(
"getStackOutputs",
async (state) =&gt; {
if (!state.stackName || !state.stackRegion) {
state.errors.push(
new Error(
"No stack name or region provided. The stack name and \\
region are required to fetch CFN outputs relevant to this example.",
),
);
return;
}
const outputs = await getCfnOutputs(state.stackName, state.stackRegion);
Object.assign(state, outputs);
},
);
export const promptForStackName = new ScenarioInput(
"stackName",
"Enter the name of the stack you deployed earlier.",
{ type: "input", default: "PoolsAndTriggersStack" },
);
export const promptForStackRegion = new ScenarioInput(
"stackRegion",
"Enter the region of the stack you deployed earlier.",
{ type: "input", default: "us-east-1" },
);
export const logCleanUpReminder = new ScenarioOutput(
"logCleanUpReminder",
"All done. Remember to run 'cdk destroy' to teardown the stack.",
{ skipWhen: skipWhenErrors },
);
`
```
A handler for the `PreSignUp` trigger with a Lambda function.
```
`import type { PreSignUpTriggerEvent, Handler } from "aws-lambda";
import type { UserRepository } from "./user-repository";
import { DynamoDBUserRepository } from "./user-repository";
export class PreSignUpHandler {
private userRepository: UserRepository;
constructor(userRepository: UserRepository) {
this.userRepository = userRepository;
}
private isPreSignUpTriggerSource(event: PreSignUpTriggerEvent): boolean {
return event.triggerSource === "PreSignUp\_SignUp";
}
private getEventUserEmail(event: PreSignUpTriggerEvent): string {
return event.request.userAttributes.email;
}
async handlePreSignUpTriggerEvent(
event: PreSignUpTriggerEvent,
): Promise&lt;PreSignUpTriggerEvent&gt; {
console.log(
`Received presignup from ${event.triggerSource} for user '${event.userName}'`,
);
if (!this.isPreSignUpTriggerSource(event)) {
return event;
}
const eventEmail = this.getEventUserEmail(event);
console.log(`Looking up email ${eventEmail}.`);
const storedUserInfo =
await this.userRepository.getUserInfoByEmail(eventEmail);
if (!storedUserInfo) {
console.log(
`Email ${eventEmail} not found. Email verification is required.`,
);
return event;
}
if (storedUserInfo.UserName !== event.userName) {
console.log(
`UserEmail ${eventEmail} found, but stored UserName '${storedUserInfo.UserName}' does not match supplied UserName '${event.userName}'. Verification is required.`,
);
} else {
console.log(
`UserEmail ${eventEmail} found with matching UserName ${storedUserInfo.UserName}. User is confirmed.`,
);
event.response.autoConfirmUser = true;
event.response.autoVerifyEmail = true;
}
return event;
}
}
const createPreSignUpHandler = (): PreSignUpHandler =&gt; {
const tableName = process.env.TABLE\_NAME;
if (!tableName) {
throw new Error("TABLE\_NAME environment variable is not set");
}
const userRepository = new DynamoDBUserRepository(tableName);
return new PreSignUpHandler(userRepository);
};
export const handler: Handler = async (event: PreSignUpTriggerEvent) =&gt;&gt; {
const preSignUpHandler = createPreSignUpHandler();
return preSignUpHandler.handlePreSignUpTriggerEvent(event);
};
`
```
Module of CloudWatch Logs actions.
```
`
import {
CloudWatchLogsClient,
GetLogEventsCommand,
OrderBy,
paginateDescribeLogStreams,
} from "@aws-sdk/client-cloudwatch-logs";
/\*\*
\* Get the latest log stream for a Lambda function.
\* @param {{ functionName: string, region: string }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cloudwatch-logs").LogStream | null, unknown]&gt;&gt;}
\*/
export const getLatestLogStreamForLambda = async ({ functionName, region }) =&gt; {
try {
const logGroupName = `/aws/lambda/${functionName}`;
const cwlClient = new CloudWatchLogsClient({ region });
const paginator = paginateDescribeLogStreams(
{ client: cwlClient },
{
descending: true,
limit: 1,
orderBy: OrderBy.LastEventTime,
logGroupName,
},
);
for await (const page of paginator) {
return [page.logStreams[0], null];
}
} catch (err) {
return [null, err];
}
};
/\*\*
\* Get the log events for a Lambda function's log stream.
\* @param {{
\* functionName: string,
\* logStreamName: string,
\* eventCount: number,
\* region: string
\* }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cloudwatch-logs").OutputLogEvent[] | null, unknown]&gt;&gt;}
\*/
export const getLogEvents = async ({
functionName,
logStreamName,
eventCount,
region,
}) =&gt; {
try {
const cwlClient = new CloudWatchLogsClient({ region });
const logGroupName = `/aws/lambda/${functionName}`;
const response = await cwlClient.send(
new GetLogEventsCommand({
logStreamName: logStreamName,
limit: eventCount,
logGroupName: logGroupName,
}),
);
return [response.events, null];
} catch (err) {
return [null, err];
}
};
`
```
Module of Amazon Cognito actions.
```
`
import {
AdminGetUserCommand,
CognitoIdentityProviderClient,
DeleteUserCommand,
InitiateAuthCommand,
SignUpCommand,
UpdateUserPoolCommand,
} from "@aws-sdk/client-cognito-identity-provider";
/\*\*
\* Connect a Lambda function to the PreSignUp trigger for a Cognito user pool
\* @param {{ region: string, userPoolId: string, handlerArn: string }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cognito-identity-provider").UpdateUserPoolCommandOutput | null, unknown]&gt;&gt;}
\*/
export const addPreSignUpHandler = async ({
region,
userPoolId,
handlerArn,
}) =&gt; {
try {
const cognitoClient = new CognitoIdentityProviderClient({
region,
});
const command = new UpdateUserPoolCommand({
UserPoolId: userPoolId,
LambdaConfig: {
PreSignUp: handlerArn,
},
});
const response = await cognitoClient.send(command);
return [response, null];
} catch (err) {
return [null, err];
}
};
/\*\*
\* Attempt to register a user to a user pool with a given username and password.
\* @param {{
\* region: string,
\* userPoolClientId: string,
\* username: string,
\* email: string,
\* password: string
\* }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cognito-identity-provider").SignUpCommandOutput | null, unknown]&gt;&gt;}
\*/
export const signUpUser = async ({
region,
userPoolClientId,
username,
email,
password,
}) =&gt; {
try {
const cognitoClient = new CognitoIdentityProviderClient({
region,
});
const response = await cognitoClient.send(
new SignUpCommand({
ClientId: userPoolClientId,
Username: username,
Password: password,
UserAttributes: [{ Name: "email", Value: email }],
}),
);
return [response, null];
} catch (err) {
return [null, err];
}
};
/\*\*
\* Sign in a user to Amazon Cognito using a username and password authentication flow.
\* @param {{ region: string, clientId: string, username: string, password: string }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cognito-identity-provider").InitiateAuthCommandOutput | null, unknown]&gt;&gt;}
\*/
export const signIn = async ({ region, clientId, username, password }) =&gt; {
try {
const cognitoClient = new CognitoIdentityProviderClient({ region });
const response = await cognitoClient.send(
new InitiateAuthCommand({
AuthFlow: "USER\_PASSWORD\_AUTH",
ClientId: clientId,
AuthParameters: { USERNAME: username, PASSWORD: password },
}),
);
return [response, null];
} catch (err) {
return [null, err];
}
};
/\*\*
\* Retrieve an existing user from a user pool.
\* @param {{ region: string, userPoolId: string, username: string }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cognito-identity-provider").AdminGetUserCommandOutput | null, unknown]&gt;&gt;}
\*/
export const getUser = async ({ region, userPoolId, username }) =&gt; {
try {
const cognitoClient = new CognitoIdentityProviderClient({ region });
const response = await cognitoClient.send(
new AdminGetUserCommand({
UserPoolId: userPoolId,
Username: username,
}),
);
return [response, null];
} catch (err) {
return [null, err];
}
};
/\*\*
\* Delete the signed-in user. Useful for allowing a user to delete their
\* own profile.
\* @param {{ region: string, accessToken: string }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/client-cognito-identity-provider").DeleteUserCommandOutput | null, unknown]&gt;&gt;}
\*/
export const deleteUser = async ({ region, accessToken }) =&gt; {
try {
const client = new CognitoIdentityProviderClient({ region });
const response = await client.send(
new DeleteUserCommand({ AccessToken: accessToken }),
);
return [response, null];
} catch (err) {
return [null, err];
}
};
`
```
Module of DynamoDB actions.
```
`
import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import {
BatchWriteCommand,
DynamoDBDocumentClient,
} from "@aws-sdk/lib-dynamodb";
/\*\*
\* Populate a DynamoDB table with provide items.
\* @param {{ region: string, tableName: string, items: Record&lt;&lt;string, unknown&gt;&gt;[] }} config
\* @returns {Promise&lt;&lt;[import("@aws-sdk/lib-dynamodb").BatchWriteCommandOutput | null, unknown]&gt;&gt;}
\*/
export const populateTable = async ({ region, tableName, items }) =&gt; {
try {
const ddbClient = new DynamoDBClient({ region });
const docClient = DynamoDBDocumentClient.from(ddbClient);
const response = await docClient.send(
new BatchWriteCommand({
RequestItems: {
[tableName]: items.map((item) =&gt; ({
PutRequest: {
Item: item,
},
})),
},
}),
);
return [response, null];
} catch (err) {
return [null, err];
}
};
`
```
* For API details, see the following topics in *AWS SDK for JavaScript API Reference*.
* [DeleteUser](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/cognito-identity-provider/command/DeleteUserCommand)
* [InitiateAuth](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/cognito-identity-provider/command/InitiateAuthCommand)
* [SignUp](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/cognito-identity-provider/command/SignUpCommand)
* [UpdateUserPool](https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/client/cognito-identity-provider/command/UpdateUserPoolCommand)
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Scenarios
Automatically migrate known users with a Lambda function
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.