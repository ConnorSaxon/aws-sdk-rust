// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteIndex`](crate::client::fluent_builders::DeleteIndex) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::DeleteIndex::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::DeleteIndex::set_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you want to delete.</p>
                            /// - On success, responds with [`DeleteIndexOutput`](crate::output::DeleteIndexOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DeleteIndexOutput::arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you successfully started the deletion process.</p> <note>   <p>This operation is asynchronous. To check its status, call the <code>GetIndex</code> operation.</p>  </note>
    ///   - [`state(Option<IndexState>)`](crate::output::DeleteIndexOutput::state): <p>Indicates the current state of the index. </p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::output::DeleteIndexOutput::last_updated_at): <p>The date and time when you last updated this index.</p>
                            /// - On failure, responds with [`SdkError<DeleteIndexError>`](crate::error::DeleteIndexError)
    pub fn delete_index(&self) -> crate::client::fluent_builders::DeleteIndex {
                                crate::client::fluent_builders::DeleteIndex::new(self.handle.clone())
                            }
}

