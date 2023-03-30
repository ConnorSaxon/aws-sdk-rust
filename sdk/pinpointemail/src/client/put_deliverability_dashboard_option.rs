// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutDeliverabilityDashboardOption`](crate::client::fluent_builders::PutDeliverabilityDashboardOption) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dashboard_enabled(bool)`](crate::client::fluent_builders::PutDeliverabilityDashboardOption::dashboard_enabled) / [`set_dashboard_enabled(bool)`](crate::client::fluent_builders::PutDeliverabilityDashboardOption::set_dashboard_enabled): <p>Specifies whether to enable the Deliverability dashboard for your Amazon Pinpoint account. To enable the dashboard, set this value to <code>true</code>.</p>
    ///   - [`subscribed_domains(Vec<DomainDeliverabilityTrackingOption>)`](crate::client::fluent_builders::PutDeliverabilityDashboardOption::subscribed_domains) / [`set_subscribed_domains(Option<Vec<DomainDeliverabilityTrackingOption>>)`](crate::client::fluent_builders::PutDeliverabilityDashboardOption::set_subscribed_domains): <p>An array of objects, one for each verified domain that you use to send email and enabled the Deliverability dashboard for.</p>
                            /// - On success, responds with [`PutDeliverabilityDashboardOptionOutput`](crate::output::PutDeliverabilityDashboardOptionOutput)
                            /// - On failure, responds with [`SdkError<PutDeliverabilityDashboardOptionError>`](crate::error::PutDeliverabilityDashboardOptionError)
    pub fn put_deliverability_dashboard_option(&self) -> crate::client::fluent_builders::PutDeliverabilityDashboardOption {
                                crate::client::fluent_builders::PutDeliverabilityDashboardOption::new(self.handle.clone())
                            }
}

