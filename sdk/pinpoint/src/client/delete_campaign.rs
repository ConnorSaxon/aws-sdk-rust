// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteCampaign`](crate::client::fluent_builders::DeleteCampaign) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::DeleteCampaign::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::DeleteCampaign::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`campaign_id(impl Into<String>)`](crate::client::fluent_builders::DeleteCampaign::campaign_id) / [`set_campaign_id(Option<String>)`](crate::client::fluent_builders::DeleteCampaign::set_campaign_id): <p>The unique identifier for the campaign.</p>
                            /// - On success, responds with [`DeleteCampaignOutput`](crate::output::DeleteCampaignOutput) with field(s):
    ///   - [`campaign_response(Option<CampaignResponse>)`](crate::output::DeleteCampaignOutput::campaign_response): <p>Provides information about the status, configuration, and other settings for a campaign.</p>
                            /// - On failure, responds with [`SdkError<DeleteCampaignError>`](crate::error::DeleteCampaignError)
    pub fn delete_campaign(&self) -> crate::client::fluent_builders::DeleteCampaign {
                                crate::client::fluent_builders::DeleteCampaign::new(self.handle.clone())
                            }
}

