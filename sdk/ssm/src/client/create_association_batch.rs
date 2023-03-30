// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAssociationBatch`](crate::client::fluent_builders::CreateAssociationBatch) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`entries(Vec<CreateAssociationBatchRequestEntry>)`](crate::client::fluent_builders::CreateAssociationBatch::entries) / [`set_entries(Option<Vec<CreateAssociationBatchRequestEntry>>)`](crate::client::fluent_builders::CreateAssociationBatch::set_entries): <p>One or more associations.</p>
                            /// - On success, responds with [`CreateAssociationBatchOutput`](crate::output::CreateAssociationBatchOutput) with field(s):
    ///   - [`successful(Option<Vec<AssociationDescription>>)`](crate::output::CreateAssociationBatchOutput::successful): <p>Information about the associations that succeeded.</p>
    ///   - [`failed(Option<Vec<FailedCreateAssociation>>)`](crate::output::CreateAssociationBatchOutput::failed): <p>Information about the associations that failed.</p>
                            /// - On failure, responds with [`SdkError<CreateAssociationBatchError>`](crate::error::CreateAssociationBatchError)
    pub fn create_association_batch(&self) -> crate::client::fluent_builders::CreateAssociationBatch {
                                crate::client::fluent_builders::CreateAssociationBatch::new(self.handle.clone())
                            }
}

