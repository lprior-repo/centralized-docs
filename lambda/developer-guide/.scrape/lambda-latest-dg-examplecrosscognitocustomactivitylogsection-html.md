---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_CognitoCustomActivityLog_section.html
title: Write custom activity data with a Lambda function after Amazon Cognito user authentication using an AWS SDK
word_count: 3273
filtered: true
elements_removed: 0
density_score: 0.87
---

Write custom activity data with a Lambda function after Amazon Cognito user authentication using an AWS SDK - AWS Lambda
Write custom activity data with a Lambda function after Amazon Cognito user authentication using an AWS SDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_CognitoCustomActivityLog_section)
# Write custom activity data with a Lambda function after Amazon Cognito user authentication using an AWS SDK
The following code example shows how to write custom activity data with a Lambda function after Amazon Cognito user authentication.
* Use administrator functions to add a user to a user pool.
* Configure a user pool to call a Lambda function for the `PostAuthentication` trigger.
* Sign the new user in to Amazon Cognito.
* The Lambda function writes custom information to CloudWatch Logs and to an DynamoDB table.
* Get and display custom data from the DynamoDB table, then clean up resources.
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
// ActivityLog separates the steps of this scenario into individual functions so that
// they are simpler to read and understand.
type ActivityLog struct {
helper IScenarioHelper
questioner demotools.IQuestioner
resources Resources
cognitoActor \*actions.CognitoActions
}
// NewActivityLog constructs a new activity log runner.
func NewActivityLog(sdkConfig aws.Config, questioner demotools.IQuestioner, helper IScenarioHelper) ActivityLog {
scenario := ActivityLog{
helper: helper,
questioner: questioner,
resources: Resources{},
cognitoActor: &amp;actions.CognitoActions{CognitoClient: cognitoidentityprovider.NewFromConfig(sdkConfig)},
}
scenario.resources.init(scenario.cognitoActor, questioner)
return scenario
}
// AddUserToPool selects a user from the known users table and uses administrator credentials to add the user to the user pool.
func (runner \*ActivityLog) AddUserToPool(ctx context.Context, userPoolId string, tableName string) (string, string) {
log.Println("To facilitate this example, let's add a user to the user pool using administrator privileges.")
users, err := runner.helper.GetKnownUsers(ctx, tableName)
if err != nil {
panic(err)
}
user := users.Users[0]
log.Printf("Adding known user %v to the user pool.\\n", user.UserName)
err = runner.cognitoActor.AdminCreateUser(ctx, userPoolId, user.UserName, user.UserEmail)
if err != nil {
panic(err)
}
pwSet := false
password := runner.questioner.AskPassword("\\nEnter a password that has at least eight characters, uppercase, lowercase, numbers and symbols.\\n"+
"(the password will not display as you type):", 8)
for !pwSet {
log.Printf("\\nSetting password for user '%v'.\\n", user.UserName)
err = runner.cognitoActor.AdminSetUserPassword(ctx, userPoolId, user.UserName, password)
if err != nil {
var invalidPassword \*types.InvalidPasswordException
if errors.As(err, &amp;&amp;invalidPassword) {
password = runner.questioner.AskPassword("\\nEnter another password:", 8)
} else {
panic(err)
}
} else {
pwSet = true
}
}
log.Println(strings.Repeat("-", 88))
return user.UserName, password
}
// AddActivityLogTrigger adds a Lambda handler as an invocation target for the PostAuthentication trigger.
func (runner \*ActivityLog) AddActivityLogTrigger(ctx context.Context, userPoolId string, activityLogArn string) {
log.Println("Let's add a Lambda function to handle the PostAuthentication trigger from Cognito.\\n" +
"This trigger happens after a user is authenticated, and lets your function take action, such as logging\\n" +
"the outcome.")
err := runner.cognitoActor.UpdateTriggers(
ctx, userPoolId,
actions.TriggerInfo{Trigger: actions.PostAuthentication, HandlerArn: aws.String(activityLogArn)})
if err != nil {
panic(err)
}
runner.resources.triggers = append(runner.resources.triggers, actions.PostAuthentication)
log.Printf("Lambda function %v added to user pool %v to handle PostAuthentication Cognito trigger.\\n",
activityLogArn, userPoolId)
log.Println(strings.Repeat("-", 88))
}
// SignInUser signs in as the specified user.
func (runner \*ActivityLog) SignInUser(ctx context.Context, clientId string, userName string, password string) {
log.Printf("Now we'll sign in user %v and check the results in the logs and the DynamoDB table.", userName)
runner.questioner.Ask("Press Enter when you're ready.")
authResult, err := runner.cognitoActor.SignIn(ctx, clientId, userName, password)
if err != nil {
panic(err)
}
log.Println("Sign in successful.",
"The PostAuthentication Lambda handler writes custom information to CloudWatch Logs.")
runner.resources.userAccessTokens = append(runner.resources.userAccessTokens, \*authResult.AccessToken)
}
// GetKnownUserLastLogin gets the login info for a user from the Amazon DynamoDB table and displays it.
func (runner \*ActivityLog) GetKnownUserLastLogin(ctx context.Context, tableName string, userName string) {
log.Println("The PostAuthentication handler also writes login data to the DynamoDB table.")
runner.questioner.Ask("Press Enter when you're ready to continue.")
users, err := runner.helper.GetKnownUsers(ctx, tableName)
if err != nil {
panic(err)
}
for \_, user := range users.Users {
if user.UserName == userName {
log.Println("The last login info for the user in the known users table is:")
log.Printf("\\t%+v", \*user.LastLogin)
}
}
log.Println(strings.Repeat("-", 88))
}
// Run runs the scenario.
func (runner \*ActivityLog) Run(ctx context.Context, stackName string) {
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
userName, password := runner.AddUserToPool(ctx, stackOutputs["UserPoolId"], stackOutputs["TableName"])
runner.AddActivityLogTrigger(ctx, stackOutputs["UserPoolId"], stackOutputs["ActivityLogFunctionArn"])
runner.SignInUser(ctx, stackOutputs["UserPoolClientId"], userName, password)
runner.helper.ListRecentLogEvents(ctx, stackOutputs["ActivityLogFunction"])
runner.GetKnownUserLastLogin(ctx, stackOutputs["TableName"], userName)
runner.resources.Cleanup(ctx)
log.Println(strings.Repeat("-", 88))
log.Println("Thanks for watching!")
log.Println(strings.Repeat("-", 88))
}
`
```
Handle the `PostAuthentication` trigger with a Lambda function.
```
`
import (
"context"
"fmt"
"log"
"os"
"time"
"github.com/aws/aws-lambda-go/events"
"github.com/aws/aws-lambda-go/lambda"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/config"
"github.com/aws/aws-sdk-go-v2/feature/dynamodb/attributevalue"
"github.com/aws/aws-sdk-go-v2/service/dynamodb"
dynamodbtypes "github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)
const TABLE\_NAME = "TABLE\_NAME"
// LoginInfo defines structured login data that can be marshalled to a DynamoDB format.
type LoginInfo struct {
UserPoolId string `dynamodbav:"UserPoolId"`
ClientId string `dynamodbav:"ClientId"`
Time string `dynamodbav:"Time"`
}
// UserInfo defines structured user data that can be marshalled to a DynamoDB format.
type UserInfo struct {
UserName string `dynamodbav:"UserName"`
UserEmail string `dynamodbav:"UserEmail"`
LastLogin LoginInfo `dynamodbav:"LastLogin"`
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
// HandleRequest handles the PostAuthentication event by writing custom data to the logs and
// to an Amazon DynamoDB table.
func (h \*handler) HandleRequest(ctx context.Context, event events.CognitoEventUserPoolsPostAuthentication) (events.CognitoEventUserPoolsPostAuthentication, error) {
log.Printf("Received post authentication trigger from %v for user '%v'", event.TriggerSource, event.UserName)
tableName := os.Getenv(TABLE\_NAME)
user := UserInfo{
UserName: event.UserName,
UserEmail: event.Request.UserAttributes["email"],
LastLogin: LoginInfo{
UserPoolId: event.UserPoolID,
ClientId: event.CallerContext.ClientID,
Time: time.Now().Format(time.UnixDate),
},
}
// Write to CloudWatch Logs.
fmt.Printf("%#v", user)
// Also write to an external system. This examples uses DynamoDB to demonstrate.
userMap, err := attributevalue.MarshalMap(user)
if err != nil {
log.Printf("Couldn't marshal to DynamoDB map. Here's why: %v\\n", err)
} else if len(userMap) == 0 {
log.Printf("User info marshaled to an empty map.")
} else {
\_, err := h.dynamoClient.PutItem(ctx, &amp;&amp;dynamodb.PutItemInput{
Item: userMap,
TableName: aws.String(tableName),
})
if err != nil {
log.Printf("Couldn't write to DynamoDB. Here's why: %v\\n", err)
} else {
log.Printf("Wrote user info to DynamoDB table %v.\\n", tableName)
}
}
return event, nil
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
* [AdminCreateUser](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.AdminCreateUser)
* [AdminSetUserPassword](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.AdminSetUserPassword)
* [DeleteUser](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.DeleteUser)
* [InitiateAuth](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.InitiateAuth)
* [UpdateUserPool](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.UpdateUserPool)
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Use the Neptune API to query graph data
Serverless examples
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.