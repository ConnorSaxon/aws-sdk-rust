// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDeliveryStreams`](crate::client::fluent_builders::ListDeliveryStreams) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListDeliveryStreams::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListDeliveryStreams::set_limit): <p>The maximum number of delivery streams to list. The default value is 10.</p>
    ///   - [`delivery_stream_type(DeliveryStreamType)`](crate::client::fluent_builders::ListDeliveryStreams::delivery_stream_type) / [`set_delivery_stream_type(Option<DeliveryStreamType>)`](crate::client::fluent_builders::ListDeliveryStreams::set_delivery_stream_type): <p>The delivery stream type. This can be one of the following values:</p>  <ul>   <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li>   <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li>  </ul>  <p>This parameter is optional. If this parameter is omitted, delivery streams of all types are returned.</p>
    ///   - [`exclusive_start_delivery_stream_name(impl Into<String>)`](crate::client::fluent_builders::ListDeliveryStreams::exclusive_start_delivery_stream_name) / [`set_exclusive_start_delivery_stream_name(Option<String>)`](crate::client::fluent_builders::ListDeliveryStreams::set_exclusive_start_delivery_stream_name): <p>The list of delivery streams returned by this call to <code>ListDeliveryStreams</code> will start with the delivery stream whose name comes alphabetically immediately after the name you specify in <code>ExclusiveStartDeliveryStreamName</code>.</p>
                            /// - On success, responds with [`ListDeliveryStreamsOutput`](crate::output::ListDeliveryStreamsOutput) with field(s):
    ///   - [`delivery_stream_names(Option<Vec<String>>)`](crate::output::ListDeliveryStreamsOutput::delivery_stream_names): <p>The names of the delivery streams.</p>
    ///   - [`has_more_delivery_streams(Option<bool>)`](crate::output::ListDeliveryStreamsOutput::has_more_delivery_streams): <p>Indicates whether there are more delivery streams available to list.</p>
                            /// - On failure, responds with [`SdkError<ListDeliveryStreamsError>`](crate::error::ListDeliveryStreamsError)
    pub fn list_delivery_streams(&self) -> crate::client::fluent_builders::ListDeliveryStreams {
                                crate::client::fluent_builders::ListDeliveryStreams::new(self.handle.clone())
                            }
}

