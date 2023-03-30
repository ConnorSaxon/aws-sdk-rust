// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`Untag`](crate::client::fluent_builders::Untag) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::Untag::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::Untag::set_arn): <p>The ARN of the resource group from which to remove tags. The command removed both the specified keys and any values associated with those keys.</p>
    ///   - [`keys(Vec<String>)`](crate::client::fluent_builders::Untag::keys) / [`set_keys(Option<Vec<String>>)`](crate::client::fluent_builders::Untag::set_keys): <p>The keys of the tags to be removed.</p>
                            /// - On success, responds with [`UntagOutput`](crate::output::UntagOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::UntagOutput::arn): <p>The ARN of the resource group from which tags have been removed.</p>
    ///   - [`keys(Option<Vec<String>>)`](crate::output::UntagOutput::keys): <p>The keys of the tags that were removed.</p>
                            /// - On failure, responds with [`SdkError<UntagError>`](crate::error::UntagError)
    pub fn untag(&self) -> crate::client::fluent_builders::Untag {
                                crate::client::fluent_builders::Untag::new(self.handle.clone())
                            }
}

