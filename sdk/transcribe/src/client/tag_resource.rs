// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`TagResource`](crate::client::fluent_builders::TagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::TagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::TagResource::set_resource_arn): <p>The Amazon Resource Name (ARN) of the resource you want to tag. ARNs have the format <code>arn:partition:service:region:account-id:resource-type/resource-id</code>.</p>  <p>For example, <code>arn:aws:transcribe:us-west-2:111122223333:transcription-job/transcription-job-name</code>.</p>  <p>Valid values for <code>resource-type</code> are: <code>transcription-job</code>, <code>medical-transcription-job</code>, <code>vocabulary</code>, <code>medical-vocabulary</code>, <code>vocabulary-filter</code>, and <code>language-model</code>.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::TagResource::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::TagResource::set_tags): <p>Adds one or more custom tags, each in the form of a key:value pair, to the specified resource.</p>  <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging resources</a>.</p>
                            /// - On success, responds with [`TagResourceOutput`](crate::output::TagResourceOutput)
                            /// - On failure, responds with [`SdkError<TagResourceError>`](crate::error::TagResourceError)
    pub fn tag_resource(&self) -> crate::client::fluent_builders::TagResource {
                                crate::client::fluent_builders::TagResource::new(self.handle.clone())
                            }
}

