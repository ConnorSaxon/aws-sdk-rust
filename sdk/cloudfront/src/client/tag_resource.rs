// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TagResource`](crate::client::fluent_builders::TagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource(impl Into<String>)`](crate::client::fluent_builders::TagResource::resource) / [`set_resource(Option<String>)`](crate::client::fluent_builders::TagResource::set_resource): <p>An ARN of a CloudFront resource.</p>
    ///   - [`tags(Tags)`](crate::client::fluent_builders::TagResource::tags) / [`set_tags(Option<Tags>)`](crate::client::fluent_builders::TagResource::set_tags): <p>A complex type that contains zero or more <code>Tag</code> elements.</p>
                            /// - On success, responds with [`TagResourceOutput`](crate::output::TagResourceOutput)
                            /// - On failure, responds with [`SdkError<TagResourceError>`](crate::error::TagResourceError)
    pub fn tag_resource(&self) -> crate::client::fluent_builders::TagResource {
                                crate::client::fluent_builders::TagResource::new(self.handle.clone())
                            }
}

