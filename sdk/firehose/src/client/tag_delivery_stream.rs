// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TagDeliveryStream`](crate::client::fluent_builders::TagDeliveryStream) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`delivery_stream_name(impl Into<String>)`](crate::client::fluent_builders::TagDeliveryStream::delivery_stream_name) / [`set_delivery_stream_name(Option<String>)`](crate::client::fluent_builders::TagDeliveryStream::set_delivery_stream_name): <p>The name of the delivery stream to which you want to add the tags.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::TagDeliveryStream::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::TagDeliveryStream::set_tags): <p>A set of key-value pairs to use to create the tags.</p>
                            /// - On success, responds with [`TagDeliveryStreamOutput`](crate::output::TagDeliveryStreamOutput)
                            /// - On failure, responds with [`SdkError<TagDeliveryStreamError>`](crate::error::TagDeliveryStreamError)
    pub fn tag_delivery_stream(&self) -> crate::client::fluent_builders::TagDeliveryStream {
                                crate::client::fluent_builders::TagDeliveryStream::new(self.handle.clone())
                            }
}

