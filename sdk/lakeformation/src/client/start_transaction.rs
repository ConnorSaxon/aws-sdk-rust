// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartTransaction`](crate::client::fluent_builders::StartTransaction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`transaction_type(TransactionType)`](crate::client::fluent_builders::StartTransaction::transaction_type) / [`set_transaction_type(Option<TransactionType>)`](crate::client::fluent_builders::StartTransaction::set_transaction_type): <p>Indicates whether this transaction should be read only or read and write. Writes made using a read-only transaction ID will be rejected. Read-only transactions do not need to be committed. </p>
                            /// - On success, responds with [`StartTransactionOutput`](crate::output::StartTransactionOutput) with field(s):
    ///   - [`transaction_id(Option<String>)`](crate::output::StartTransactionOutput::transaction_id): <p>An opaque identifier for the transaction.</p>
                            /// - On failure, responds with [`SdkError<StartTransactionError>`](crate::error::StartTransactionError)
    pub fn start_transaction(&self) -> crate::client::fluent_builders::StartTransaction {
                                crate::client::fluent_builders::StartTransaction::new(self.handle.clone())
                            }
}

