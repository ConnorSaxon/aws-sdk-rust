// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ModifyVerifiedAccessInstance`](crate::client::fluent_builders::ModifyVerifiedAccessInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`verified_access_instance_id(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::verified_access_instance_id) / [`set_verified_access_instance_id(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::set_verified_access_instance_id): <p>The ID of the Amazon Web Services Verified Access instance.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::set_description): <p>A description for the Amazon Web Services Verified Access instance.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::ModifyVerifiedAccessInstance::set_client_token): <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
                            /// - On success, responds with [`ModifyVerifiedAccessInstanceOutput`](crate::output::ModifyVerifiedAccessInstanceOutput) with field(s):
    ///   - [`verified_access_instance(Option<VerifiedAccessInstance>)`](crate::output::ModifyVerifiedAccessInstanceOutput::verified_access_instance): <p>The ID of the Amazon Web Services Verified Access instance.</p>
                            /// - On failure, responds with [`SdkError<ModifyVerifiedAccessInstanceError>`](crate::error::ModifyVerifiedAccessInstanceError)
    pub fn modify_verified_access_instance(&self) -> crate::client::fluent_builders::ModifyVerifiedAccessInstance {
                                crate::client::fluent_builders::ModifyVerifiedAccessInstance::new(self.handle.clone())
                            }
}

