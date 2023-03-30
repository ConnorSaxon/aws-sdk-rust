// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateStack`](crate::client::fluent_builders::CreateStack) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stack_name(impl Into<String>)`](crate::client::fluent_builders::CreateStack::stack_name) / [`set_stack_name(Option<String>)`](crate::client::fluent_builders::CreateStack::set_stack_name): <p>The name that's associated with the stack. The name must be unique in the Region in which you are creating the stack.</p> <note>   <p>A stack name can contain only alphanumeric characters (case sensitive) and hyphens. It must start with an alphabetical character and can't be longer than 128 characters.</p>  </note>
    ///   - [`template_body(impl Into<String>)`](crate::client::fluent_builders::CreateStack::template_body) / [`set_template_body(Option<String>)`](crate::client::fluent_builders::CreateStack::set_template_body): <p>Structure containing the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template anatomy</a> in the CloudFormation User Guide.</p>  <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>
    ///   - [`template_url(impl Into<String>)`](crate::client::fluent_builders::CreateStack::template_url) / [`set_template_url(Option<String>)`](crate::client::fluent_builders::CreateStack::set_template_url): <p>Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that's located in an Amazon S3 bucket or a Systems Manager document. For more information, go to the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-anatomy.html">Template anatomy</a> in the CloudFormation User Guide.</p>  <p>Conditional: You must specify either the <code>TemplateBody</code> or the <code>TemplateURL</code> parameter, but not both.</p>
    ///   - [`parameters(Vec<Parameter>)`](crate::client::fluent_builders::CreateStack::parameters) / [`set_parameters(Option<Vec<Parameter>>)`](crate::client::fluent_builders::CreateStack::set_parameters): <p>A list of <code>Parameter</code> structures that specify input parameters for the stack. For more information, see the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_Parameter.html">Parameter</a> data type.</p>
    ///   - [`disable_rollback(bool)`](crate::client::fluent_builders::CreateStack::disable_rollback) / [`set_disable_rollback(Option<bool>)`](crate::client::fluent_builders::CreateStack::set_disable_rollback): <p>Set to <code>true</code> to disable rollback of the stack if stack creation failed. You can specify either <code>DisableRollback</code> or <code>OnFailure</code>, but not both.</p>  <p>Default: <code>false</code> </p>
    ///   - [`rollback_configuration(RollbackConfiguration)`](crate::client::fluent_builders::CreateStack::rollback_configuration) / [`set_rollback_configuration(Option<RollbackConfiguration>)`](crate::client::fluent_builders::CreateStack::set_rollback_configuration): <p>The rollback triggers for CloudFormation to monitor during stack creation and updating operations, and for the specified monitoring period afterwards.</p>
    ///   - [`timeout_in_minutes(i32)`](crate::client::fluent_builders::CreateStack::timeout_in_minutes) / [`set_timeout_in_minutes(Option<i32>)`](crate::client::fluent_builders::CreateStack::set_timeout_in_minutes): <p>The amount of time that can pass before the stack status becomes CREATE_FAILED; if <code>DisableRollback</code> is not set or is set to <code>false</code>, the stack will be rolled back.</p>
    ///   - [`notification_ar_ns(Vec<String>)`](crate::client::fluent_builders::CreateStack::notification_ar_ns) / [`set_notification_ar_ns(Option<Vec<String>>)`](crate::client::fluent_builders::CreateStack::set_notification_ar_ns): <p>The Amazon Simple Notification Service (Amazon SNS) topic ARNs to publish stack related events. You can find your Amazon SNS topic ARNs using the Amazon SNS console or your Command Line Interface (CLI).</p>
    ///   - [`capabilities(Vec<Capability>)`](crate::client::fluent_builders::CreateStack::capabilities) / [`set_capabilities(Option<Vec<Capability>>)`](crate::client::fluent_builders::CreateStack::set_capabilities): <p>In some cases, you must explicitly acknowledge that your stack template contains certain capabilities in order for CloudFormation to create the stack.</p>  <ul>   <li> <p> <code>CAPABILITY_IAM</code> and <code>CAPABILITY_NAMED_IAM</code> </p> <p>Some stack templates might include resources that can affect permissions in your Amazon Web Services account; for example, by creating new Identity and Access Management (IAM) users. For those stacks, you must explicitly acknowledge this by specifying one of these capabilities.</p> <p>The following IAM resources require you to specify either the <code>CAPABILITY_IAM</code> or <code>CAPABILITY_NAMED_IAM</code> capability.</p>    <ul>     <li> <p>If you have IAM resources, you can specify either capability.</p> </li>     <li> <p>If you have IAM resources with custom names, you <i>must</i> specify <code>CAPABILITY_NAMED_IAM</code>.</p> </li>     <li> <p>If you don't specify either of these capabilities, CloudFormation returns an <code>InsufficientCapabilities</code> error.</p> </li>    </ul> <p>If your stack template contains these resources, we recommend that you review all permissions associated with them and edit their permissions if necessary.</p>    <ul>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html"> AWS::IAM::AccessKey</a> </p> </li>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html"> AWS::IAM::Group</a> </p> </li>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html">AWS::IAM::InstanceProfile</a> </p> </li>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html"> AWS::IAM::Policy</a> </p> </li>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html"> AWS::IAM::Role</a> </p> </li>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html"> AWS::IAM::User</a> </p> </li>     <li> <p> <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html">AWS::IAM::UserToGroupAddition</a> </p> </li>    </ul> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html#capabilities">Acknowledging IAM Resources in CloudFormation Templates</a>.</p> </li>   <li> <p> <code>CAPABILITY_AUTO_EXPAND</code> </p> <p>Some template contain macros. Macros perform custom processing on templates; this can include simple actions like find-and-replace operations, all the way to extensive transformations of entire templates. Because of this, users typically create a change set from the processed template, so that they can review the changes resulting from the macros before actually creating the stack. If your stack template contains one or more macros, and you choose to create a stack directly from the processed template, without first reviewing the resulting changes in a change set, you must acknowledge this capability. This includes the <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/create-reusable-transform-function-snippets-and-add-to-your-template-with-aws-include-transform.html">AWS::Include</a> and <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-aws-serverless.html">AWS::Serverless</a> transforms, which are macros hosted by CloudFormation.</p> <p>If you want to create a stack from a stack template that contains macros <i>and</i> nested stacks, you must create the stack directly from the template using this capability.</p> <important>     <p>You should only create stacks directly from a stack template that contains macros if you know what processing the macro performs.</p>     <p>Each macro relies on an underlying Lambda service function for processing stack templates. Be aware that the Lambda function owner can update the function operation without CloudFormation being notified.</p>    </important> <p>For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros.html">Using CloudFormation macros to perform custom processing on templates</a>.</p> </li>  </ul>
    ///   - [`resource_types(Vec<String>)`](crate::client::fluent_builders::CreateStack::resource_types) / [`set_resource_types(Option<Vec<String>>)`](crate::client::fluent_builders::CreateStack::set_resource_types): <p>The template resource types that you have permissions to work with for this create stack action, such as <code>AWS::EC2::Instance</code>, <code>AWS::EC2::*</code>, or <code>Custom::MyCustomInstance</code>. Use the following syntax to describe template resource types: <code>AWS::*</code> (for all Amazon Web Services resources), <code>Custom::*</code> (for all custom resources), <code>Custom::<i>logical_ID</i> </code> (for a specific custom resource), <code>AWS::<i>service_name</i>::*</code> (for all resources of a particular Amazon Web Services service), and <code>AWS::<i>service_name</i>::<i>resource_logical_ID</i> </code> (for a specific Amazon Web Services resource).</p>  <p>If the list of resource types doesn't include a resource that you're creating, the stack creation fails. By default, CloudFormation grants permissions to all resource types. Identity and Access Management (IAM) uses this parameter for CloudFormation-specific condition keys in IAM policies. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-iam-template.html">Controlling Access with Identity and Access Management</a>.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateStack::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateStack::set_role_arn): <p>The Amazon Resource Name (ARN) of an Identity and Access Management (IAM) role that CloudFormation assumes to create the stack. CloudFormation uses the role's credentials to make calls on your behalf. CloudFormation always uses this role for all future operations on the stack. Provided that users have permission to operate on the stack, CloudFormation uses this role even if the users don't have permission to pass it. Ensure that the role grants least privilege.</p>  <p>If you don't specify a value, CloudFormation uses the role that was previously associated with the stack. If no role is available, CloudFormation uses a temporary session that's generated from your user credentials.</p>
    ///   - [`on_failure(OnFailure)`](crate::client::fluent_builders::CreateStack::on_failure) / [`set_on_failure(Option<OnFailure>)`](crate::client::fluent_builders::CreateStack::set_on_failure): <p>Determines what action will be taken if stack creation fails. This must be one of: <code>DO_NOTHING</code>, <code>ROLLBACK</code>, or <code>DELETE</code>. You can specify either <code>OnFailure</code> or <code>DisableRollback</code>, but not both.</p>  <p>Default: <code>ROLLBACK</code> </p>
    ///   - [`stack_policy_body(impl Into<String>)`](crate::client::fluent_builders::CreateStack::stack_policy_body) / [`set_stack_policy_body(Option<String>)`](crate::client::fluent_builders::CreateStack::set_stack_policy_body): <p>Structure containing the stack policy body. For more information, go to <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/protect-stack-resources.html"> Prevent Updates to Stack Resources</a> in the <i>CloudFormation User Guide</i>. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    ///   - [`stack_policy_url(impl Into<String>)`](crate::client::fluent_builders::CreateStack::stack_policy_url) / [`set_stack_policy_url(Option<String>)`](crate::client::fluent_builders::CreateStack::set_stack_policy_url): <p>Location of a file containing the stack policy. The URL must point to a policy (maximum size: 16 KB) located in an S3 bucket in the same Region as the stack. You can specify either the <code>StackPolicyBody</code> or the <code>StackPolicyURL</code> parameter, but not both.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateStack::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateStack::set_tags): <p>Key-value pairs to associate with this stack. CloudFormation also propagates these tags to the resources created in the stack. A maximum number of 50 tags can be specified.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateStack::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateStack::set_client_request_token): <p>A unique identifier for this <code>CreateStack</code> request. Specify this token if you plan to retry requests so that CloudFormation knows that you're not attempting to create a stack with the same name. You might retry <code>CreateStack</code> requests to ensure that CloudFormation successfully received them.</p>  <p>All events initiated by a given stack operation are assigned the same client request token, which you can use to track operations. For example, if you execute a <code>CreateStack</code> operation with the token <code>token1</code>, then all the <code>StackEvents</code> generated by that operation will have <code>ClientRequestToken</code> set as <code>token1</code>.</p>  <p>In the console, stack operations display the client request token on the Events tab. Stack operations that are initiated from the console use the token format <i>Console-StackOperation-ID</i>, which helps you easily identify the stack operation . For example, if you create a stack using the console, each stack event would be assigned the same token in the following format: <code>Console-CreateStack-7f59c3cf-00d2-40c7-b2ff-e75db0987002</code>.</p>
    ///   - [`enable_termination_protection(bool)`](crate::client::fluent_builders::CreateStack::enable_termination_protection) / [`set_enable_termination_protection(Option<bool>)`](crate::client::fluent_builders::CreateStack::set_enable_termination_protection): <p>Whether to enable termination protection on the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-protect-stacks.html">Protecting a Stack From Being Deleted</a> in the <i>CloudFormation User Guide</i>. Termination protection is deactivated on stacks by default.</p>  <p>For <a href="http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-nested-stacks.html">nested stacks</a>, termination protection is set on the root stack and can't be changed directly on the nested stack.</p>
                            /// - On success, responds with [`CreateStackOutput`](crate::output::CreateStackOutput) with field(s):
    ///   - [`stack_id(Option<String>)`](crate::output::CreateStackOutput::stack_id): <p>Unique identifier of the stack.</p>
                            /// - On failure, responds with [`SdkError<CreateStackError>`](crate::error::CreateStackError)
    pub fn create_stack(&self) -> crate::client::fluent_builders::CreateStack {
                                crate::client::fluent_builders::CreateStack::new(self.handle.clone())
                            }
}

