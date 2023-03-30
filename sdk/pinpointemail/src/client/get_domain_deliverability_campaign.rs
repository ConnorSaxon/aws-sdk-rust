// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDomainDeliverabilityCampaign`](crate::client::fluent_builders::GetDomainDeliverabilityCampaign) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`campaign_id(impl Into<String>)`](crate::client::fluent_builders::GetDomainDeliverabilityCampaign::campaign_id) / [`set_campaign_id(Option<String>)`](crate::client::fluent_builders::GetDomainDeliverabilityCampaign::set_campaign_id): <p>The unique identifier for the campaign. Amazon Pinpoint automatically generates and assigns this identifier to a campaign. This value is not the same as the campaign identifier that Amazon Pinpoint assigns to campaigns that you create and manage by using the Amazon Pinpoint API or the Amazon Pinpoint console.</p>
                            /// - On success, responds with [`GetDomainDeliverabilityCampaignOutput`](crate::output::GetDomainDeliverabilityCampaignOutput) with field(s):
    ///   - [`domain_deliverability_campaign(Option<DomainDeliverabilityCampaign>)`](crate::output::GetDomainDeliverabilityCampaignOutput::domain_deliverability_campaign): <p>An object that contains the deliverability data for the campaign.</p>
                            /// - On failure, responds with [`SdkError<GetDomainDeliverabilityCampaignError>`](crate::error::GetDomainDeliverabilityCampaignError)
    pub fn get_domain_deliverability_campaign(&self) -> crate::client::fluent_builders::GetDomainDeliverabilityCampaign {
                                crate::client::fluent_builders::GetDomainDeliverabilityCampaign::new(self.handle.clone())
                            }
}

