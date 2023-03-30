// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForDomain`](crate::client::fluent_builders::ListTagsForDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::ListTagsForDomain::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::ListTagsForDomain::set_domain_name): <p>The domain for which you want to get a list of tags.</p>
                            /// - On success, responds with [`ListTagsForDomainOutput`](crate::output::ListTagsForDomainOutput) with field(s):
    ///   - [`tag_list(Option<Vec<Tag>>)`](crate::output::ListTagsForDomainOutput::tag_list): <p>A list of the tags that are associated with the specified domain.</p>
                            /// - On failure, responds with [`SdkError<ListTagsForDomainError>`](crate::error::ListTagsForDomainError)
    pub fn list_tags_for_domain(&self) -> crate::client::fluent_builders::ListTagsForDomain {
                                crate::client::fluent_builders::ListTagsForDomain::new(self.handle.clone())
                            }
}

