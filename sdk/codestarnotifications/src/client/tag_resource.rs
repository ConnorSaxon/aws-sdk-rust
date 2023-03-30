// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TagResource`](crate::client::fluent_builders::TagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::TagResource::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::TagResource::set_arn): <p>The Amazon Resource Name (ARN) of the notification rule to tag.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::TagResource::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::TagResource::set_tags): <p>The list of tags to associate with the resource. Tag key names cannot start with "<code>aws</code>".</p>
                            /// - On success, responds with [`TagResourceOutput`](crate::output::TagResourceOutput) with field(s):
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::TagResourceOutput::tags): <p>The list of tags associated with the resource.</p>
                            /// - On failure, responds with [`SdkError<TagResourceError>`](crate::error::TagResourceError)
    pub fn tag_resource(&self) -> crate::client::fluent_builders::TagResource {
                                crate::client::fluent_builders::TagResource::new(self.handle.clone())
                            }
}

