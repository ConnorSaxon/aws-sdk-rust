// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateFromMasterAccount`](crate::client::fluent_builders::DisassociateFromMasterAccount) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DisassociateFromMasterAccount::send) it.
                            /// - On success, responds with [`DisassociateFromMasterAccountOutput`](crate::output::DisassociateFromMasterAccountOutput)
                            /// - On failure, responds with [`SdkError<DisassociateFromMasterAccountError>`](crate::error::DisassociateFromMasterAccountError)
    #[deprecated(note = "This API has been deprecated, use DisassociateFromAdministratorAccount API instead.")]
    pub fn disassociate_from_master_account(&self) -> crate::client::fluent_builders::DisassociateFromMasterAccount {
                                crate::client::fluent_builders::DisassociateFromMasterAccount::new(self.handle.clone())
                            }
}

