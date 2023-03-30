// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ResolveAlias`](crate::client::fluent_builders::ResolveAlias) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alias_id(impl Into<String>)`](crate::client::fluent_builders::ResolveAlias::alias_id) / [`set_alias_id(Option<String>)`](crate::client::fluent_builders::ResolveAlias::set_alias_id): <p>The unique identifier of the alias that you want to retrieve a fleet ID for. You can use either the alias ID or ARN value.</p>
                            /// - On success, responds with [`ResolveAliasOutput`](crate::output::ResolveAliasOutput) with field(s):
    ///   - [`fleet_id(Option<String>)`](crate::output::ResolveAliasOutput::fleet_id): <p>The fleet identifier that the alias is pointing to.</p>
    ///   - [`fleet_arn(Option<String>)`](crate::output::ResolveAliasOutput::fleet_arn): <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) associated with the GameLift fleet resource that this alias points to. </p>
                            /// - On failure, responds with [`SdkError<ResolveAliasError>`](crate::error::ResolveAliasError)
    pub fn resolve_alias(&self) -> crate::client::fluent_builders::ResolveAlias {
                                crate::client::fluent_builders::ResolveAlias::new(self.handle.clone())
                            }
}

