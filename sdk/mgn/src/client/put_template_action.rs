// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutTemplateAction`](crate::client::fluent_builders::PutTemplateAction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`launch_configuration_template_id(impl Into<String>)`](crate::client::fluent_builders::PutTemplateAction::launch_configuration_template_id) / [`set_launch_configuration_template_id(Option<String>)`](crate::client::fluent_builders::PutTemplateAction::set_launch_configuration_template_id): <p>Launch configuration template ID.</p>
    ///   - [`action_name(impl Into<String>)`](crate::client::fluent_builders::PutTemplateAction::action_name) / [`set_action_name(Option<String>)`](crate::client::fluent_builders::PutTemplateAction::set_action_name): <p>Template post migration custom action name.</p>
    ///   - [`document_identifier(impl Into<String>)`](crate::client::fluent_builders::PutTemplateAction::document_identifier) / [`set_document_identifier(Option<String>)`](crate::client::fluent_builders::PutTemplateAction::set_document_identifier): <p>Template post migration custom action document identifier.</p>
    ///   - [`order(i32)`](crate::client::fluent_builders::PutTemplateAction::order) / [`set_order(i32)`](crate::client::fluent_builders::PutTemplateAction::set_order): <p>Template post migration custom action order.</p>
    ///   - [`action_id(impl Into<String>)`](crate::client::fluent_builders::PutTemplateAction::action_id) / [`set_action_id(Option<String>)`](crate::client::fluent_builders::PutTemplateAction::set_action_id): <p>Template post migration custom action ID.</p>
    ///   - [`document_version(impl Into<String>)`](crate::client::fluent_builders::PutTemplateAction::document_version) / [`set_document_version(Option<String>)`](crate::client::fluent_builders::PutTemplateAction::set_document_version): <p>Template post migration custom action document version.</p>
    ///   - [`active(bool)`](crate::client::fluent_builders::PutTemplateAction::active) / [`set_active(Option<bool>)`](crate::client::fluent_builders::PutTemplateAction::set_active): <p>Template post migration custom action active status.</p>
    ///   - [`timeout_seconds(i32)`](crate::client::fluent_builders::PutTemplateAction::timeout_seconds) / [`set_timeout_seconds(i32)`](crate::client::fluent_builders::PutTemplateAction::set_timeout_seconds): <p>Template post migration custom action timeout in seconds.</p>
    ///   - [`must_succeed_for_cutover(bool)`](crate::client::fluent_builders::PutTemplateAction::must_succeed_for_cutover) / [`set_must_succeed_for_cutover(Option<bool>)`](crate::client::fluent_builders::PutTemplateAction::set_must_succeed_for_cutover): <p>Template post migration custom action must succeed for cutover.</p>
    ///   - [`parameters(HashMap<String, Vec<SsmParameterStoreParameter>>)`](crate::client::fluent_builders::PutTemplateAction::parameters) / [`set_parameters(Option<HashMap<String, Vec<SsmParameterStoreParameter>>>)`](crate::client::fluent_builders::PutTemplateAction::set_parameters): <p>Template post migration custom action parameters.</p>
    ///   - [`operating_system(impl Into<String>)`](crate::client::fluent_builders::PutTemplateAction::operating_system) / [`set_operating_system(Option<String>)`](crate::client::fluent_builders::PutTemplateAction::set_operating_system): <p>Operating system eligible for this template post migration custom action.</p>
                            /// - On success, responds with [`PutTemplateActionOutput`](crate::output::PutTemplateActionOutput) with field(s):
    ///   - [`action_id(Option<String>)`](crate::output::PutTemplateActionOutput::action_id): <p>Template post migration custom action ID.</p>
    ///   - [`action_name(Option<String>)`](crate::output::PutTemplateActionOutput::action_name): <p>Template post migration custom action name.</p>
    ///   - [`document_identifier(Option<String>)`](crate::output::PutTemplateActionOutput::document_identifier): <p>Template post migration custom action document identifier.</p>
    ///   - [`order(i32)`](crate::output::PutTemplateActionOutput::order): <p>Template post migration custom action order.</p>
    ///   - [`document_version(Option<String>)`](crate::output::PutTemplateActionOutput::document_version): <p>Template post migration custom action document version.</p>
    ///   - [`active(Option<bool>)`](crate::output::PutTemplateActionOutput::active): <p>Template post migration custom action active status.</p>
    ///   - [`timeout_seconds(i32)`](crate::output::PutTemplateActionOutput::timeout_seconds): <p>Template post migration custom action timeout in seconds.</p>
    ///   - [`must_succeed_for_cutover(Option<bool>)`](crate::output::PutTemplateActionOutput::must_succeed_for_cutover): <p>Template post migration custom action must succeed for cutover.</p>
    ///   - [`parameters(Option<HashMap<String, Vec<SsmParameterStoreParameter>>>)`](crate::output::PutTemplateActionOutput::parameters): <p>Template post migration custom action parameters.</p>
    ///   - [`operating_system(Option<String>)`](crate::output::PutTemplateActionOutput::operating_system): <p>Operating system eligible for this template post migration custom action.</p>
                            /// - On failure, responds with [`SdkError<PutTemplateActionError>`](crate::error::PutTemplateActionError)
    pub fn put_template_action(&self) -> crate::client::fluent_builders::PutTemplateAction {
                                crate::client::fluent_builders::PutTemplateAction::new(self.handle.clone())
                            }
}

