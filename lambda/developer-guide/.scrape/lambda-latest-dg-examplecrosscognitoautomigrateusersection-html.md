---
url: https://docs.aws.amazon.com/lambda/latest/dg/example_cross_CognitoAutoMigrateUser_section.html
title: Automatically migrate known Amazon Cognito users with a Lambda function using an AWS SDK
word_count: 3434
filtered: true
elements_removed: 0
density_score: 0.87
---

Automatically migrate known Amazon Cognito users with a Lambda function using an AWS SDK - AWS Lambda
Automatically migrate known Amazon Cognito users with a Lambda function using an AWS SDK - AWS Lambda
[](https://docs.aws.amazon.com/pdfs/lambda/latest/dg/lambda-dg.pdf#example_cross_CognitoAutoMigrateUser_section)
# Automatically migrate known Amazon Cognito users with a Lambda function using an AWS SDK
The following code example shows how to automatically migrate known Amazon Cognito users with a Lambda function.
* Configure a user pool to call a Lambda function for the `MigrateUser` trigger.
* Sign in to Amazon Cognito with a username and email that is not in the user pool.
* The Lambda function scans a DynamoDB table and automatically migrates known users to the user pool.
* Perform the forgot password flow to reset the password for the migrated user.
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
"fmt"
"log"
"strings"
"user\_pools\_and\_lambda\_triggers/actions"
"github.com/aws/aws-sdk-go-v2/aws"
"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider"
"github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider/types"
"github.com/awsdocs/aws-doc-sdk-examples/gov2/demotools"
)
// MigrateUser separates the steps of this scenario into individual functions so that
// they are simpler to read and understand.
type MigrateUser struct {
helper IScenarioHelper
questioner demotools.IQuestioner
resources Resources
cognitoActor \*actions.CognitoActions
}
// NewMigrateUser constructs a new migrate user runner.
func NewMigrateUser(sdkConfig aws.Config, questioner demotools.IQuestioner, helper IScenarioHelper) MigrateUser {
scenario := MigrateUser{
helper: helper,
questioner: questioner,
resources: Resources{},
cognitoActor: &amp;actions.CognitoActions{CognitoClient: cognitoidentityprovider.NewFromConfig(sdkConfig)},
}
scenario.resources.init(scenario.cognitoActor, questioner)
return scenario
}
// AddMigrateUserTrigger adds a Lambda handler as an invocation target for the MigrateUser trigger.
func (runner \*MigrateUser) AddMigrateUserTrigger(ctx context.Context, userPoolId string, functionArn string) {
log.Printf("Let's add a Lambda function to handle the MigrateUser trigger from Cognito.\\n" +
"This trigger happens when an unknown user signs in, and lets your function take action before Cognito\\n" +
"rejects the user.\\n\\n")
err := runner.cognitoActor.UpdateTriggers(
ctx, userPoolId,
actions.TriggerInfo{Trigger: actions.UserMigration, HandlerArn: aws.String(functionArn)})
if err != nil {
panic(err)
}
log.Printf("Lambda function %v added to user pool %v to handle the MigrateUser trigger.\\n",
functionArn, userPoolId)
log.Println(strings.Repeat("-", 88))
}
// SignInUser adds a new user to the known users table and signs that user in to Amazon Cognito.
func (runner \*MigrateUser) SignInUser(ctx context.Context, usersTable string, clientId string) (bool, actions.User) {
log.Println("Let's sign in a user to your Cognito user pool. When the username and email matches an entry in the\\n" +
"DynamoDB known users table, the email is automatically verified and the user is migrated to the Cognito user pool.")
user := actions.User{}
user.UserName = runner.questioner.Ask("\\nEnter a username:")
user.UserEmail = runner.questioner.Ask("\\nEnter an email that you own. This email will be used to confirm user migration\\n" +
"during this example:")
runner.helper.AddKnownUser(ctx, usersTable, user)
var err error
var resetRequired \*types.PasswordResetRequiredException
var authResult \*types.AuthenticationResultType
signedIn := false
for !signedIn &amp;&amp;&amp;&amp; resetRequired == nil {
log.Printf("Signing in to Cognito as user '%v'. The expected result is a PasswordResetRequiredException.\\n\\n", user.UserName)
authResult, err = runner.cognitoActor.SignIn(ctx, clientId, user.UserName, "\_")
if err != nil {
if errors.As(err, &amp;resetRequired) {
log.Printf("\\nUser '%v' is not in the Cognito user pool but was found in the DynamoDB known users table.\\n"+
"User migration is started and a password reset is required.", user.UserName)
} else {
panic(err)
}
} else {
log.Printf("User '%v' successfully signed in. This is unexpected and probably means you have not\\n"+
"cleaned up a previous run of this scenario, so the user exist in the Cognito user pool.\\n"+
"You can continue this example and select to clean up resources, or manually remove\\n"+
"the user from your user pool and try again.", user.UserName)
runner.resources.userAccessTokens = append(runner.resources.userAccessTokens, \*authResult.AccessToken)
signedIn = true
}
}
log.Println(strings.Repeat("-", 88))
return resetRequired != nil, user
}
// ResetPassword starts a password recovery flow.
func (runner \*MigrateUser) ResetPassword(ctx context.Context, clientId string, user actions.User) {
wantCode := runner.questioner.AskBool(fmt.Sprintf("In order to migrate the user to Cognito, you must be able to receive a confirmation\\n"+
"code by email at %v. Do you want to send a code (y/n)?", user.UserEmail), "y")
if !wantCode {
log.Println("To complete this example and successfully migrate a user to Cognito, you must enter an email\\n" +
"you own that can receive a confirmation code.")
return
}
codeDelivery, err := runner.cognitoActor.ForgotPassword(ctx, clientId, user.UserName)
if err != nil {
panic(err)
}
log.Printf("\\nA confirmation code has been sent to %v.", \*codeDelivery.Destination)
code := runner.questioner.Ask("Check your email and enter it here:")
confirmed := false
password := runner.questioner.AskPassword("\\nEnter a password that has at least eight characters, uppercase, lowercase, numbers and symbols.\\n"+
"(the password will not display as you type):", 8)
for !confirmed {
log.Printf("\\nConfirming password reset for user '%v'.\\n", user.UserName)
err = runner.cognitoActor.ConfirmForgotPassword(ctx, clientId, code, user.UserName, password)
if err != nil {
var invalidPassword \*types.InvalidPasswordException
if errors.As(err, &amp;&amp;invalidPassword) {
password = runner.questioner.AskPassword("\\nEnter another password:", 8)
} else {
panic(err)
}
} else {
confirmed = true
}
}
log.Printf("User '%v' successfully confirmed and migrated.\\n", user.UserName)
log.Println("Signing in with your username and password...")
authResult, err := runner.cognitoActor.SignIn(ctx, clientId, user.UserName, password)
if err != nil {
panic(err)
}
log.Printf("Successfully signed in. Your access token starts with: %v...\\n", (\*authResult.AccessToken)[:10])
runner.resources.userAccessTokens = append(runner.resources.userAccessTokens, \*authResult.AccessToken)
log.Println(strings.Repeat("-", 88))
}
// Run runs the scenario.
func (runner \*MigrateUser) Run(ctx context.Context, stackName string) {
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
runner.AddMigrateUserTrigger(ctx, stackOutputs["UserPoolId"], stackOutputs["MigrateUserFunctionArn"])
runner.resources.triggers = append(runner.resources.triggers, actions.UserMigration)
resetNeeded, user := runner.SignInUser(ctx, stackOutputs["TableName"], stackOutputs["UserPoolClientId"])
if resetNeeded {
runner.helper.ListRecentLogEvents(ctx, stackOutputs["MigrateUserFunction"])
runner.ResetPassword(ctx, stackOutputs["UserPoolClientId"], user)
}
runner.resources.Cleanup(ctx)
log.Println(strings.Repeat("-", 88))
log.Println("Thanks for watching!")
log.Println(strings.Repeat("-", 88))
}
`
```
Handle the `MigrateUser` trigger with a Lambda function.
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
"github.com/aws/aws-sdk-go-v2/feature/dynamodb/expression"
"github.com/aws/aws-sdk-go-v2/service/dynamodb"
)
const TABLE\_NAME = "TABLE\_NAME"
// UserInfo defines structured user data that can be marshalled to a DynamoDB format.
type UserInfo struct {
UserName string `dynamodbav:"UserName"`
UserEmail string `dynamodbav:"UserEmail"`
}
type handler struct {
dynamoClient \*dynamodb.Client
}
// HandleRequest handles the MigrateUser event by looking up a user in an Amazon DynamoDB table and
// specifying whether they should be migrated to the user pool.
func (h \*handler) HandleRequest(ctx context.Context, event events.CognitoEventUserPoolsMigrateUser) (events.CognitoEventUserPoolsMigrateUser, error) {
log.Printf("Received migrate trigger from %v for user '%v'", event.TriggerSource, event.UserName)
if event.TriggerSource != "UserMigration\_Authentication" {
return event, nil
}
tableName := os.Getenv(TABLE\_NAME)
user := UserInfo{
UserName: event.UserName,
}
log.Printf("Looking up user '%v' in table %v.\\n", user.UserName, tableName)
filterEx := expression.Name("UserName").Equal(expression.Value(user.UserName))
expr, err := expression.NewBuilder().WithFilter(filterEx).Build()
if err != nil {
log.Printf("Error building expression to query for user '%v'.\\n", user.UserName)
return event, err
}
output, err := h.dynamoClient.Scan(ctx, &amp;&amp;dynamodb.ScanInput{
TableName: aws.String(tableName),
FilterExpression: expr.Filter(),
ExpressionAttributeNames: expr.Names(),
ExpressionAttributeValues: expr.Values(),
})
if err != nil {
log.Printf("Error looking up user '%v'.\\n", user.UserName)
return event, err
}
if len(output.Items) == 0 {
log.Printf("User '%v' not found, not migrating user.\\n", user.UserName)
return event, err
}
var users []UserInfo
err = attributevalue.UnmarshalListOfMaps(output.Items, &amp;&amp;users)
if err != nil {
log.Printf("Couldn't unmarshal DynamoDB items. Here's why: %v\\n", err)
return event, err
}
user = users[0]
log.Printf("UserName '%v' found with email %v. User is migrated and must reset password.\\n", user.UserName, user.UserEmail)
event.CognitoEventUserPoolsMigrateUserResponse.UserAttributes = map[string]string{
"email": user.UserEmail,
"email\_verified": "true", // email\_verified is required for the forgot password flow.
}
event.CognitoEventUserPoolsMigrateUserResponse.FinalUserStatus = "RESET\_REQUIRED"
event.CognitoEventUserPoolsMigrateUserResponse.MessageAction = "SUPPRESS"
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
* [ConfirmForgotPassword](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.ConfirmForgotPassword)
* [DeleteUser](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.DeleteUser)
* [ForgotPassword](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.ForgotPassword)
* [InitiateAuth](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.InitiateAuth)
* [SignUp](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.SignUp)
* [UpdateUserPool](https://pkg.go.dev/github.com/aws/aws-sdk-go-v2/service/cognitoidentityprovider#Client.UpdateUserPool)
For a complete list of AWS SDK developer guides and code examples, see
[Using Lambda with an AWS SDK](./sdk-general-information-section.html).
This topic also includes information about getting started and details about previous SDK versions.
[Document Conventions](https://docs.aws.amazon.com/general/latest/gr/docconventions.html)
Automatically confirm known users with a Lambda function
Create a REST API to track COVID-19 data
Did this page help you? - Yes
Thanks for letting us know we're doing a good job!
If you've got a moment, please tell us what we did right so we can do more of it.
Did this page help you? - No
Thanks for letting us know this page needs work. We're sorry we let you down.
If you've got a moment, please tell us how we can make the documentation better.