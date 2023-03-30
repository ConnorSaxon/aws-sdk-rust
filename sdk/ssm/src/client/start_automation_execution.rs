// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartAutomationExecution`](crate::client::fluent_builders::StartAutomationExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`document_name(impl Into<String>)`](crate::client::fluent_builders::StartAutomationExecution::document_name) / [`set_document_name(Option<String>)`](crate::client::fluent_builders::StartAutomationExecution::set_document_name): <p>The name of the SSM document to run. This can be a public document or a custom document. To run a shared document belonging to another account, specify the document ARN. For more information about how to use shared documents, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/ssm-using-shared.html">Using shared SSM documents</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
    ///   - [`document_version(impl Into<String>)`](crate::client::fluent_builders::StartAutomationExecution::document_version) / [`set_document_version(Option<String>)`](crate::client::fluent_builders::StartAutomationExecution::set_document_version): <p>The version of the Automation runbook to use for this execution.</p>
    ///   - [`parameters(HashMap<String, Vec<String>>)`](crate::client::fluent_builders::StartAutomationExecution::parameters) / [`set_parameters(Option<HashMap<String, Vec<String>>>)`](crate::client::fluent_builders::StartAutomationExecution::set_parameters): <p>A key-value map of execution parameters, which match the declared parameters in the Automation runbook.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::StartAutomationExecution::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::StartAutomationExecution::set_client_token): <p>User-provided idempotency token. The token must be unique, is case insensitive, enforces the UUID format, and can't be reused.</p>
    ///   - [`mode(ExecutionMode)`](crate::client::fluent_builders::StartAutomationExecution::mode) / [`set_mode(Option<ExecutionMode>)`](crate::client::fluent_builders::StartAutomationExecution::set_mode): <p>The execution mode of the automation. Valid modes include the following: Auto and Interactive. The default mode is Auto.</p>
    ///   - [`target_parameter_name(impl Into<String>)`](crate::client::fluent_builders::StartAutomationExecution::target_parameter_name) / [`set_target_parameter_name(Option<String>)`](crate::client::fluent_builders::StartAutomationExecution::set_target_parameter_name): <p>The name of the parameter used as the target resource for the rate-controlled execution. Required if you specify targets.</p>
    ///   - [`targets(Vec<Target>)`](crate::client::fluent_builders::StartAutomationExecution::targets) / [`set_targets(Option<Vec<Target>>)`](crate::client::fluent_builders::StartAutomationExecution::set_targets): <p>A key-value mapping to target resources. Required if you specify TargetParameterName.</p>
    ///   - [`target_maps(Vec<HashMap<String, Vec<String>>>)`](crate::client::fluent_builders::StartAutomationExecution::target_maps) / [`set_target_maps(Option<Vec<HashMap<String, Vec<String>>>>)`](crate::client::fluent_builders::StartAutomationExecution::set_target_maps): <p>A key-value mapping of document parameters to target resources. Both Targets and TargetMaps can't be specified together.</p>
    ///   - [`max_concurrency(impl Into<String>)`](crate::client::fluent_builders::StartAutomationExecution::max_concurrency) / [`set_max_concurrency(Option<String>)`](crate::client::fluent_builders::StartAutomationExecution::set_max_concurrency): <p>The maximum number of targets allowed to run this task in parallel. You can specify a number, such as 10, or a percentage, such as 10%. The default value is <code>10</code>.</p>
    ///   - [`max_errors(impl Into<String>)`](crate::client::fluent_builders::StartAutomationExecution::max_errors) / [`set_max_errors(Option<String>)`](crate::client::fluent_builders::StartAutomationExecution::set_max_errors): <p>The number of errors that are allowed before the system stops running the automation on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops running the automation when the fourth error is received. If you specify 0, then the system stops running the automation on additional targets after the first error result is returned. If you run an automation on 50 resources and set max-errors to 10%, then the system stops running the automation on additional targets when the sixth error is received.</p>  <p>Executions that are already running an automation when max-errors is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set max-concurrency to 1 so the executions proceed one at a time.</p>
    ///   - [`target_locations(Vec<TargetLocation>)`](crate::client::fluent_builders::StartAutomationExecution::target_locations) / [`set_target_locations(Option<Vec<TargetLocation>>)`](crate::client::fluent_builders::StartAutomationExecution::set_target_locations): <p>A location is a combination of Amazon Web Services Regions and/or Amazon Web Services accounts where you want to run the automation. Use this operation to start an automation in multiple Amazon Web Services Regions and multiple Amazon Web Services accounts. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-automation-multiple-accounts-and-regions.html">Running Automation workflows in multiple Amazon Web Services Regions and Amazon Web Services accounts</a> in the <i>Amazon Web Services Systems Manager User Guide</i>. </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::StartAutomationExecution::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::StartAutomationExecution::set_tags): <p>Optional metadata that you assign to a resource. You can specify a maximum of five tags for an automation. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment. For example, you might want to tag an automation to identify an environment or operating system. In this case, you could specify the following key-value pairs:</p>  <ul>   <li> <p> <code>Key=environment,Value=test</code> </p> </li>   <li> <p> <code>Key=OS,Value=Windows</code> </p> </li>  </ul> <note>   <p>To add tags to an existing automation, use the <code>AddTagsToResource</code> operation.</p>  </note>
    ///   - [`alarm_configuration(AlarmConfiguration)`](crate::client::fluent_builders::StartAutomationExecution::alarm_configuration) / [`set_alarm_configuration(Option<AlarmConfiguration>)`](crate::client::fluent_builders::StartAutomationExecution::set_alarm_configuration): <p>The CloudWatch alarm you want to apply to your automation.</p>
                            /// - On success, responds with [`StartAutomationExecutionOutput`](crate::output::StartAutomationExecutionOutput) with field(s):
    ///   - [`automation_execution_id(Option<String>)`](crate::output::StartAutomationExecutionOutput::automation_execution_id): <p>The unique ID of a newly scheduled automation execution.</p>
                            /// - On failure, responds with [`SdkError<StartAutomationExecutionError>`](crate::error::StartAutomationExecutionError)
    pub fn start_automation_execution(&self) -> crate::client::fluent_builders::StartAutomationExecution {
                                crate::client::fluent_builders::StartAutomationExecution::new(self.handle.clone())
                            }
}

