// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdatePublicSharingSettings`](crate::client::fluent_builders::UpdatePublicSharingSettings) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::UpdatePublicSharingSettings::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::UpdatePublicSharingSettings::set_aws_account_id): <p>The Amazon Web Services account ID associated with your Amazon QuickSight subscription.</p>
    ///   - [`public_sharing_enabled(bool)`](crate::client::fluent_builders::UpdatePublicSharingSettings::public_sharing_enabled) / [`set_public_sharing_enabled(bool)`](crate::client::fluent_builders::UpdatePublicSharingSettings::set_public_sharing_enabled): <p>A Boolean value that indicates whether public sharing is turned on for an Amazon QuickSight account.</p>
                            /// - On success, responds with [`UpdatePublicSharingSettingsOutput`](crate::output::UpdatePublicSharingSettingsOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::UpdatePublicSharingSettingsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::output::UpdatePublicSharingSettingsOutput::status): <p>The HTTP status of the request.</p>
                            /// - On failure, responds with [`SdkError<UpdatePublicSharingSettingsError>`](crate::error::UpdatePublicSharingSettingsError)
    pub fn update_public_sharing_settings(&self) -> crate::client::fluent_builders::UpdatePublicSharingSettings {
                                crate::client::fluent_builders::UpdatePublicSharingSettings::new(self.handle.clone())
                            }
}

