// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetGrant`](crate::client::fluent_builders::GetGrant) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`grant_arn(impl Into<String>)`](crate::client::fluent_builders::GetGrant::grant_arn) / [`set_grant_arn(Option<String>)`](crate::client::fluent_builders::GetGrant::set_grant_arn): <p>Amazon Resource Name (ARN) of the grant.</p>
    ///   - [`version(impl Into<String>)`](crate::client::fluent_builders::GetGrant::version) / [`set_version(Option<String>)`](crate::client::fluent_builders::GetGrant::set_version): <p>Grant version.</p>
                            /// - On success, responds with [`GetGrantOutput`](crate::output::GetGrantOutput) with field(s):
    ///   - [`grant(Option<Grant>)`](crate::output::GetGrantOutput::grant): <p>Grant details.</p>
                            /// - On failure, responds with [`SdkError<GetGrantError>`](crate::error::GetGrantError)
    pub fn get_grant(&self) -> crate::client::fluent_builders::GetGrant {
                                crate::client::fluent_builders::GetGrant::new(self.handle.clone())
                            }
}

