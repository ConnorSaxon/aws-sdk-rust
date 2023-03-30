// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMedia`](crate::client::fluent_builders::GetMedia) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::client::fluent_builders::GetMedia::stream_name) / [`set_stream_name(Option<String>)`](crate::client::fluent_builders::GetMedia::set_stream_name): <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
    ///   - [`stream_arn(impl Into<String>)`](crate::client::fluent_builders::GetMedia::stream_arn) / [`set_stream_arn(Option<String>)`](crate::client::fluent_builders::GetMedia::set_stream_arn): <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
    ///   - [`start_selector(StartSelector)`](crate::client::fluent_builders::GetMedia::start_selector) / [`set_start_selector(Option<StartSelector>)`](crate::client::fluent_builders::GetMedia::set_start_selector): <p>Identifies the starting chunk to get from the specified stream. </p>
                            /// - On success, responds with [`GetMediaOutput`](crate::output::GetMediaOutput) with field(s):
    ///   - [`content_type(Option<String>)`](crate::output::GetMediaOutput::content_type): <p>The content type of the requested media.</p>
    ///   - [`payload(ByteStream)`](crate::output::GetMediaOutput::payload): <p> The payload Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see . The chunks that Kinesis Video Streams returns in the <code>GetMedia</code> call also include the following additional Matroska (MKV) tags: </p>  <ul>   <li> <p>AWS_KINESISVIDEO_CONTINUATION_TOKEN (UTF-8 string) - In the event your <code>GetMedia</code> call terminates, you can use this continuation token in your next request to get the next chunk where the last request terminated.</p> </li>   <li> <p>AWS_KINESISVIDEO_MILLIS_BEHIND_NOW (UTF-8 string) - Client applications can use this tag value to determine how far behind the chunk returned in the response is from the latest chunk on the stream. </p> </li>   <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p> </li>   <li> <p>AWS_KINESISVIDEO_SERVER_TIMESTAMP - Server timestamp of the fragment.</p> </li>   <li> <p>AWS_KINESISVIDEO_PRODUCER_TIMESTAMP - Producer timestamp of the fragment.</p> </li>  </ul>  <p>The following tags will be present if an error occurs:</p>  <ul>   <li> <p>AWS_KINESISVIDEO_ERROR_CODE - String description of an error that caused GetMedia to stop.</p> </li>   <li> <p>AWS_KINESISVIDEO_ERROR_ID: Integer code of the error.</p> </li>  </ul>  <p>The error codes are as follows:</p>  <ul>   <li> <p>3002 - Error writing to the stream</p> </li>   <li> <p>4000 - Requested fragment is not found</p> </li>   <li> <p>4500 - Access denied for the stream's KMS key</p> </li>   <li> <p>4501 - Stream's KMS key is disabled</p> </li>   <li> <p>4502 - Validation error on the stream's KMS key</p> </li>   <li> <p>4503 - KMS key specified in the stream is unavailable</p> </li>   <li> <p>4504 - Invalid usage of the KMS key specified in the stream</p> </li>   <li> <p>4505 - Invalid state of the KMS key specified in the stream</p> </li>   <li> <p>4506 - Unable to find the KMS key specified in the stream</p> </li>   <li> <p>5000 - Internal error</p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<GetMediaError>`](crate::error::GetMediaError)
    pub fn get_media(&self) -> crate::client::fluent_builders::GetMedia {
                                crate::client::fluent_builders::GetMedia::new(self.handle.clone())
                            }
}

