// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateIPSet`](crate::client::fluent_builders::UpdateIPSet) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::UpdateIPSet::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::UpdateIPSet::set_detector_id): <p>The detectorID that specifies the GuardDuty service whose IPSet you want to update.</p>
    ///   - [`ip_set_id(impl Into<String>)`](crate::client::fluent_builders::UpdateIPSet::ip_set_id) / [`set_ip_set_id(Option<String>)`](crate::client::fluent_builders::UpdateIPSet::set_ip_set_id): <p>The unique ID that specifies the IPSet that you want to update.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateIPSet::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateIPSet::set_name): <p>The unique ID that specifies the IPSet that you want to update.</p>
    ///   - [`location(impl Into<String>)`](crate::client::fluent_builders::UpdateIPSet::location) / [`set_location(Option<String>)`](crate::client::fluent_builders::UpdateIPSet::set_location): <p>The updated URI of the file that contains the IPSet. </p>
    ///   - [`activate(bool)`](crate::client::fluent_builders::UpdateIPSet::activate) / [`set_activate(bool)`](crate::client::fluent_builders::UpdateIPSet::set_activate): <p>The updated Boolean value that specifies whether the IPSet is active or not.</p>
                            /// - On success, responds with [`UpdateIpSetOutput`](crate::output::UpdateIpSetOutput)
                            /// - On failure, responds with [`SdkError<UpdateIPSetError>`](crate::error::UpdateIPSetError)
    pub fn update_ip_set(&self) -> crate::client::fluent_builders::UpdateIPSet {
                                crate::client::fluent_builders::UpdateIPSet::new(self.handle.clone())
                            }
}

