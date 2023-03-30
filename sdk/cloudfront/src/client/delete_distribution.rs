// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteDistribution`](crate::client::fluent_builders::DeleteDistribution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteDistribution::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteDistribution::set_id): <p>The distribution ID.</p>
    ///   - [`if_match(impl Into<String>)`](crate::client::fluent_builders::DeleteDistribution::if_match) / [`set_if_match(Option<String>)`](crate::client::fluent_builders::DeleteDistribution::set_if_match): <p>The value of the <code>ETag</code> header that you received when you disabled the distribution. For example: <code>E2QWRUHAPOMQZL</code>.</p>
                            /// - On success, responds with [`DeleteDistributionOutput`](crate::output::DeleteDistributionOutput)
                            /// - On failure, responds with [`SdkError<DeleteDistributionError>`](crate::error::DeleteDistributionError)
    pub fn delete_distribution(&self) -> crate::client::fluent_builders::DeleteDistribution {
                                crate::client::fluent_builders::DeleteDistribution::new(self.handle.clone())
                            }
}

