// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateTagsForDomain`](crate::client::fluent_builders::UpdateTagsForDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::UpdateTagsForDomain::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::UpdateTagsForDomain::set_domain_name): <p>The domain for which you want to add or update tags.</p>
    ///   - [`tags_to_update(Vec<Tag>)`](crate::client::fluent_builders::UpdateTagsForDomain::tags_to_update) / [`set_tags_to_update(Option<Vec<Tag>>)`](crate::client::fluent_builders::UpdateTagsForDomain::set_tags_to_update): <p>A list of the tag keys and values that you want to add or update. If you specify a key that already exists, the corresponding value will be replaced.</p>
                            /// - On success, responds with [`UpdateTagsForDomainOutput`](crate::output::UpdateTagsForDomainOutput)
                            /// - On failure, responds with [`SdkError<UpdateTagsForDomainError>`](crate::error::UpdateTagsForDomainError)
    pub fn update_tags_for_domain(&self) -> crate::client::fluent_builders::UpdateTagsForDomain {
                                crate::client::fluent_builders::UpdateTagsForDomain::new(self.handle.clone())
                            }
}

