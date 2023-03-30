// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`secret_id(impl Into<String>)`](crate::client::fluent_builders::UntagResource::secret_id) / [`set_secret_id(Option<String>)`](crate::client::fluent_builders::UntagResource::set_secret_id): <p>The ARN or name of the secret.</p>  <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_tag_keys): <p>A list of tag key names to remove from the secret. You don't specify the value. Both the key and its associated value are removed.</p>  <p>This parameter requires a JSON text string argument.</p>  <p>For storing multiple values, we recommend that you use a JSON text string argument and specify key/value pairs. For more information, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-parameters.html">Specifying parameter values for the Amazon Web Services CLI</a> in the Amazon Web Services CLI User Guide.</p>
                            /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)
                            /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> crate::client::fluent_builders::UntagResource {
                                crate::client::fluent_builders::UntagResource::new(self.handle.clone())
                            }
}

