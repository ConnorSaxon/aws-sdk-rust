// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateAlias`](crate::client::fluent_builders::UpdateAlias) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alias_name(impl Into<String>)`](crate::client::fluent_builders::UpdateAlias::alias_name) / [`set_alias_name(Option<String>)`](crate::client::fluent_builders::UpdateAlias::set_alias_name): <p>Identifies the alias that is changing its KMS key. This value must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>. You cannot use <code>UpdateAlias</code> to change the alias name.</p>
    ///   - [`target_key_id(impl Into<String>)`](crate::client::fluent_builders::UpdateAlias::target_key_id) / [`set_target_key_id(Option<String>)`](crate::client::fluent_builders::UpdateAlias::set_target_key_id): <p>Identifies the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed key</a> to associate with the alias. You don't have permission to associate an alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed key</a>.</p>  <p>The KMS key must be in the same Amazon Web Services account and Region as the alias. Also, the new target KMS key must be the same type as the current target KMS key (both symmetric or both asymmetric or both HMAC) and they must have the same key usage. </p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>  <p>To verify that the alias is mapped to the correct KMS key, use <code>ListAliases</code>.</p>
                            /// - On success, responds with [`UpdateAliasOutput`](crate::output::UpdateAliasOutput)
                            /// - On failure, responds with [`SdkError<UpdateAliasError>`](crate::error::UpdateAliasError)
    pub fn update_alias(&self) -> crate::client::fluent_builders::UpdateAlias {
                                crate::client::fluent_builders::UpdateAlias::new(self.handle.clone())
                            }
}

