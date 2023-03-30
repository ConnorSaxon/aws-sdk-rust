// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAssociation`](crate::client::fluent_builders::CreateAssociation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_name): <p>The name of the SSM Command document or Automation runbook that contains the configuration information for the managed node.</p>  <p>You can specify Amazon Web Services-predefined documents, documents you created, or a document that is shared with you from another account.</p>  <p>For Systems Manager documents (SSM documents) that are shared with you from other Amazon Web Services accounts, you must specify the complete SSM document ARN, in the following format:</p>  <p> <code>arn:<i>partition</i>:ssm:<i>region</i>:<i>account-id</i>:document/<i>document-name</i> </code> </p>  <p>For example:</p>  <p> <code>arn:aws:ssm:us-east-2:12345678912:document/My-Shared-Document</code> </p>  <p>For Amazon Web Services-predefined documents and SSM documents you created in your account, you only need to specify the document name. For example, <code>AWS-ApplyPatchBaseline</code> or <code>My-Document</code>.</p>
    ///   - [`document_version(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::document_version) / [`set_document_version(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_document_version): <p>The document version you want to associate with the target(s). Can be a specific version or the default version.</p> <important>   <p>State Manager doesn't support running associations that use a new version of a document if that document is shared from another account. State Manager always runs the <code>default</code> version of a document if shared from another account, even though the Systems Manager console shows that a new version was processed. If you want to run an association using a new version of a document shared form another account, you must set the document version to <code>default</code>.</p>  </important>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_instance_id): <p>The managed node ID.</p> <note>   <p> <code>InstanceId</code> has been deprecated. To specify a managed node ID for an association, use the <code>Targets</code> parameter. Requests that include the parameter <code>InstanceID</code> with Systems Manager documents (SSM documents) that use schema version 2.0 or later will fail. In addition, if you use the parameter <code>InstanceId</code>, you can't use the parameters <code>AssociationName</code>, <code>DocumentVersion</code>, <code>MaxErrors</code>, <code>MaxConcurrency</code>, <code>OutputLocation</code>, or <code>ScheduleExpression</code>. To use these parameters, you must use the <code>Targets</code> parameter.</p>  </note>
    ///   - [`parameters(HashMap<String, Vec<String>>)`](crate::client::fluent_builders::CreateAssociation::parameters) / [`set_parameters(Option<HashMap<String, Vec<String>>>)`](crate::client::fluent_builders::CreateAssociation::set_parameters): <p>The parameters for the runtime configuration of the document.</p>
    ///   - [`targets(Vec<Target>)`](crate::client::fluent_builders::CreateAssociation::targets) / [`set_targets(Option<Vec<Target>>)`](crate::client::fluent_builders::CreateAssociation::set_targets): <p>The targets for the association. You can target managed nodes by using tags, Amazon Web Services resource groups, all managed nodes in an Amazon Web Services account, or individual managed node IDs. You can target all managed nodes in an Amazon Web Services account by specifying the <code>InstanceIds</code> key with a value of <code>*</code>. For more information about choosing targets for an association, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-state-manager-targets-and-rate-controls.html">Using targets and rate controls with State Manager associations</a> in the <i>Amazon Web Services Systems Manager User Guide</i>.</p>
    ///   - [`schedule_expression(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::schedule_expression) / [`set_schedule_expression(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_schedule_expression): <p>A cron expression when the association will be applied to the target(s).</p>
    ///   - [`output_location(InstanceAssociationOutputLocation)`](crate::client::fluent_builders::CreateAssociation::output_location) / [`set_output_location(Option<InstanceAssociationOutputLocation>)`](crate::client::fluent_builders::CreateAssociation::set_output_location): <p>An Amazon Simple Storage Service (Amazon S3) bucket where you want to store the output details of the request.</p>
    ///   - [`association_name(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::association_name) / [`set_association_name(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_association_name): <p>Specify a descriptive name for the association.</p>
    ///   - [`automation_target_parameter_name(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::automation_target_parameter_name) / [`set_automation_target_parameter_name(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_automation_target_parameter_name): <p>Choose the parameter that will define how your automation will branch out. This target is required for associations that use an Automation runbook and target resources by using rate controls. Automation is a capability of Amazon Web Services Systems Manager.</p>
    ///   - [`max_errors(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::max_errors) / [`set_max_errors(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_max_errors): <p>The number of errors that are allowed before the system stops sending requests to run the association on additional targets. You can specify either an absolute number of errors, for example 10, or a percentage of the target set, for example 10%. If you specify 3, for example, the system stops sending requests when the fourth error is received. If you specify 0, then the system stops sending requests after the first error is returned. If you run an association on 50 managed nodes and set <code>MaxError</code> to 10%, then the system stops sending the request when the sixth error is received.</p>  <p>Executions that are already running an association when <code>MaxErrors</code> is reached are allowed to complete, but some of these executions may fail as well. If you need to ensure that there won't be more than max-errors failed executions, set <code>MaxConcurrency</code> to 1 so that executions proceed one at a time.</p>
    ///   - [`max_concurrency(impl Into<String>)`](crate::client::fluent_builders::CreateAssociation::max_concurrency) / [`set_max_concurrency(Option<String>)`](crate::client::fluent_builders::CreateAssociation::set_max_concurrency): <p>The maximum number of targets allowed to run the association at the same time. You can specify a number, for example 10, or a percentage of the target set, for example 10%. The default value is 100%, which means all targets run the association at the same time.</p>  <p>If a new managed node starts and attempts to run an association while Systems Manager is running <code>MaxConcurrency</code> associations, the association is allowed to run. During the next association interval, the new managed node will process its association within the limit specified for <code>MaxConcurrency</code>.</p>
    ///   - [`compliance_severity(AssociationComplianceSeverity)`](crate::client::fluent_builders::CreateAssociation::compliance_severity) / [`set_compliance_severity(Option<AssociationComplianceSeverity>)`](crate::client::fluent_builders::CreateAssociation::set_compliance_severity): <p>The severity level to assign to the association.</p>
    ///   - [`sync_compliance(AssociationSyncCompliance)`](crate::client::fluent_builders::CreateAssociation::sync_compliance) / [`set_sync_compliance(Option<AssociationSyncCompliance>)`](crate::client::fluent_builders::CreateAssociation::set_sync_compliance): <p>The mode for generating association compliance. You can specify <code>AUTO</code> or <code>MANUAL</code>. In <code>AUTO</code> mode, the system uses the status of the association execution to determine the compliance status. If the association execution runs successfully, then the association is <code>COMPLIANT</code>. If the association execution doesn't run successfully, the association is <code>NON-COMPLIANT</code>.</p>  <p>In <code>MANUAL</code> mode, you must specify the <code>AssociationId</code> as a parameter for the <code>PutComplianceItems</code> API operation. In this case, compliance data isn't managed by State Manager. It is managed by your direct call to the <code>PutComplianceItems</code> API operation.</p>  <p>By default, all associations use <code>AUTO</code> mode.</p>
    ///   - [`apply_only_at_cron_interval(bool)`](crate::client::fluent_builders::CreateAssociation::apply_only_at_cron_interval) / [`set_apply_only_at_cron_interval(bool)`](crate::client::fluent_builders::CreateAssociation::set_apply_only_at_cron_interval): <p>By default, when you create a new association, the system runs it immediately after it is created and then according to the schedule you specified. Specify this option if you don't want an association to run immediately after you create it. This parameter isn't supported for rate expressions.</p>
    ///   - [`calendar_names(Vec<String>)`](crate::client::fluent_builders::CreateAssociation::calendar_names) / [`set_calendar_names(Option<Vec<String>>)`](crate::client::fluent_builders::CreateAssociation::set_calendar_names): <p>The names or Amazon Resource Names (ARNs) of the Change Calendar type documents you want to gate your associations under. The associations only run when that change calendar is open. For more information, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-change-calendar">Amazon Web Services Systems Manager Change Calendar</a>.</p>
    ///   - [`target_locations(Vec<TargetLocation>)`](crate::client::fluent_builders::CreateAssociation::target_locations) / [`set_target_locations(Option<Vec<TargetLocation>>)`](crate::client::fluent_builders::CreateAssociation::set_target_locations): <p>A location is a combination of Amazon Web Services Regions and Amazon Web Services accounts where you want to run the association. Use this action to create an association in multiple Regions and multiple accounts.</p>
    ///   - [`schedule_offset(i32)`](crate::client::fluent_builders::CreateAssociation::schedule_offset) / [`set_schedule_offset(Option<i32>)`](crate::client::fluent_builders::CreateAssociation::set_schedule_offset): <p>Number of days to wait after the scheduled day to run an association. For example, if you specified a cron schedule of <code>cron(0 0 ? * THU#2 *)</code>, you could specify an offset of 3 to run the association each Sunday after the second Thursday of the month. For more information about cron schedules for associations, see <a href="https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html">Reference: Cron and rate expressions for Systems Manager</a> in the <i>Amazon Web Services Systems Manager User Guide</i>. </p> <note>   <p>To use offsets, you must specify the <code>ApplyOnlyAtCronInterval</code> parameter. This option tells the system not to run an association immediately after you create it. </p>  </note>
    ///   - [`target_maps(Vec<HashMap<String, Vec<String>>>)`](crate::client::fluent_builders::CreateAssociation::target_maps) / [`set_target_maps(Option<Vec<HashMap<String, Vec<String>>>>)`](crate::client::fluent_builders::CreateAssociation::set_target_maps): <p>A key-value mapping of document parameters to target resources. Both Targets and TargetMaps can't be specified together.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateAssociation::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateAssociation::set_tags): <p>Adds or overwrites one or more tags for a State Manager association. <i>Tags</i> are metadata that you can assign to your Amazon Web Services resources. Tags enable you to categorize your resources in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. </p>
    ///   - [`alarm_configuration(AlarmConfiguration)`](crate::client::fluent_builders::CreateAssociation::alarm_configuration) / [`set_alarm_configuration(Option<AlarmConfiguration>)`](crate::client::fluent_builders::CreateAssociation::set_alarm_configuration): <p>The details for the CloudWatch alarm you want to apply to an automation or command.</p>
                            /// - On success, responds with [`CreateAssociationOutput`](crate::output::CreateAssociationOutput) with field(s):
    ///   - [`association_description(Option<AssociationDescription>)`](crate::output::CreateAssociationOutput::association_description): <p>Information about the association.</p>
                            /// - On failure, responds with [`SdkError<CreateAssociationError>`](crate::error::CreateAssociationError)
    pub fn create_association(&self) -> crate::client::fluent_builders::CreateAssociation {
                                crate::client::fluent_builders::CreateAssociation::new(self.handle.clone())
                            }
}

