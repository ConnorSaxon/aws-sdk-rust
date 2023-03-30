// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetConfigurationSet`](crate::client::fluent_builders::GetConfigurationSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::GetConfigurationSet::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::GetConfigurationSet::set_configuration_set_name): <p>The name of the configuration set.</p>
                            /// - On success, responds with [`GetConfigurationSetOutput`](crate::output::GetConfigurationSetOutput) with field(s):
    ///   - [`configuration_set_name(Option<String>)`](crate::output::GetConfigurationSetOutput::configuration_set_name): <p>The name of the configuration set.</p>
    ///   - [`tracking_options(Option<TrackingOptions>)`](crate::output::GetConfigurationSetOutput::tracking_options): <p>An object that defines the open and click tracking options for emails that you send using the configuration set.</p>
    ///   - [`delivery_options(Option<DeliveryOptions>)`](crate::output::GetConfigurationSetOutput::delivery_options): <p>An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.</p>
    ///   - [`reputation_options(Option<ReputationOptions>)`](crate::output::GetConfigurationSetOutput::reputation_options): <p>An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.</p>
    ///   - [`sending_options(Option<SendingOptions>)`](crate::output::GetConfigurationSetOutput::sending_options): <p>An object that defines whether or not Amazon SES can send email that you send using the configuration set.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::GetConfigurationSetOutput::tags): <p>An array of objects that define the tags (keys and values) that are associated with the configuration set.</p>
    ///   - [`suppression_options(Option<SuppressionOptions>)`](crate::output::GetConfigurationSetOutput::suppression_options): <p>An object that contains information about the suppression list preferences for your account.</p>
    ///   - [`vdm_options(Option<VdmOptions>)`](crate::output::GetConfigurationSetOutput::vdm_options): <p>An object that contains information about the VDM preferences for your configuration set.</p>
                            /// - On failure, responds with [`SdkError<GetConfigurationSetError>`](crate::error::GetConfigurationSetError)
    pub fn get_configuration_set(&self) -> crate::client::fluent_builders::GetConfigurationSet {
                                crate::client::fluent_builders::GetConfigurationSet::new(self.handle.clone())
                            }
}

