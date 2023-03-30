// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateTagOption`](crate::client::fluent_builders::UpdateTagOption) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::UpdateTagOption::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::UpdateTagOption::set_id): <p>The TagOption identifier.</p>
    ///   - [`value(impl Into<String>)`](crate::client::fluent_builders::UpdateTagOption::value) / [`set_value(Option<String>)`](crate::client::fluent_builders::UpdateTagOption::set_value): <p>The updated value.</p>
    ///   - [`active(bool)`](crate::client::fluent_builders::UpdateTagOption::active) / [`set_active(Option<bool>)`](crate::client::fluent_builders::UpdateTagOption::set_active): <p>The updated active state.</p>
                            /// - On success, responds with [`UpdateTagOptionOutput`](crate::output::UpdateTagOptionOutput) with field(s):
    ///   - [`tag_option_detail(Option<TagOptionDetail>)`](crate::output::UpdateTagOptionOutput::tag_option_detail): <p>Information about the TagOption.</p>
                            /// - On failure, responds with [`SdkError<UpdateTagOptionError>`](crate::error::UpdateTagOptionError)
    pub fn update_tag_option(&self) -> crate::client::fluent_builders::UpdateTagOption {
                                crate::client::fluent_builders::UpdateTagOption::new(self.handle.clone())
                            }
}

