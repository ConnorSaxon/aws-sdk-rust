// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateConfigurationTemplate`](crate::client::fluent_builders::UpdateConfigurationTemplate) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::application_name) / [`set_application_name(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::set_application_name): <p>The name of the application associated with the configuration template to update.</p>  <p> If no application is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error. </p>
    ///   - [`template_name(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::template_name) / [`set_template_name(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::set_template_name): <p>The name of the configuration template to update.</p>  <p> If no configuration template is found with this name, <code>UpdateConfigurationTemplate</code> returns an <code>InvalidParameterValue</code> error. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::set_description): <p>A new description for the configuration.</p>
    ///   - [`option_settings(Vec<ConfigurationOptionSetting>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::option_settings) / [`set_option_settings(Option<Vec<ConfigurationOptionSetting>>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::set_option_settings): <p>A list of configuration option settings to update with the new specified option value.</p>
    ///   - [`options_to_remove(Vec<OptionSpecification>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::options_to_remove) / [`set_options_to_remove(Option<Vec<OptionSpecification>>)`](crate::client::fluent_builders::UpdateConfigurationTemplate::set_options_to_remove): <p>A list of configuration options to remove from the configuration set.</p>  <p> Constraint: You can remove only <code>UserDefined</code> configuration options. </p>
                            /// - On success, responds with [`UpdateConfigurationTemplateOutput`](crate::output::UpdateConfigurationTemplateOutput) with field(s):
    ///   - [`solution_stack_name(Option<String>)`](crate::output::UpdateConfigurationTemplateOutput::solution_stack_name): <p>The name of the solution stack this configuration set uses.</p>
    ///   - [`platform_arn(Option<String>)`](crate::output::UpdateConfigurationTemplateOutput::platform_arn): <p>The ARN of the platform version.</p>
    ///   - [`application_name(Option<String>)`](crate::output::UpdateConfigurationTemplateOutput::application_name): <p>The name of the application associated with this configuration set.</p>
    ///   - [`template_name(Option<String>)`](crate::output::UpdateConfigurationTemplateOutput::template_name): <p> If not <code>null</code>, the name of the configuration template for this configuration set. </p>
    ///   - [`description(Option<String>)`](crate::output::UpdateConfigurationTemplateOutput::description): <p>Describes this configuration set.</p>
    ///   - [`environment_name(Option<String>)`](crate::output::UpdateConfigurationTemplateOutput::environment_name): <p> If not <code>null</code>, the name of the environment for this configuration set. </p>
    ///   - [`deployment_status(Option<ConfigurationDeploymentStatus>)`](crate::output::UpdateConfigurationTemplateOutput::deployment_status): <p> If this configuration set is associated with an environment, the <code>DeploymentStatus</code> parameter indicates the deployment status of this configuration set: </p>  <ul>   <li> <p> <code>null</code>: This configuration is not associated with a running environment.</p> </li>   <li> <p> <code>pending</code>: This is a draft configuration that is not deployed to the associated environment but is in the process of deploying.</p> </li>   <li> <p> <code>deployed</code>: This is the configuration that is currently deployed to the associated running environment.</p> </li>   <li> <p> <code>failed</code>: This is a draft configuration that failed to successfully deploy.</p> </li>  </ul>
    ///   - [`date_created(Option<DateTime>)`](crate::output::UpdateConfigurationTemplateOutput::date_created): <p>The date (in UTC time) when this configuration set was created.</p>
    ///   - [`date_updated(Option<DateTime>)`](crate::output::UpdateConfigurationTemplateOutput::date_updated): <p>The date (in UTC time) when this configuration set was last modified.</p>
    ///   - [`option_settings(Option<Vec<ConfigurationOptionSetting>>)`](crate::output::UpdateConfigurationTemplateOutput::option_settings): <p>A list of the configuration options and their values in this configuration set.</p>
                            /// - On failure, responds with [`SdkError<UpdateConfigurationTemplateError>`](crate::error::UpdateConfigurationTemplateError)
    pub fn update_configuration_template(&self) -> crate::client::fluent_builders::UpdateConfigurationTemplate {
                                crate::client::fluent_builders::UpdateConfigurationTemplate::new(self.handle.clone())
                            }
}

