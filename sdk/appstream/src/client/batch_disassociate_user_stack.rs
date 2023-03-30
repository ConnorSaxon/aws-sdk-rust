// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDisassociateUserStack`](crate::client::fluent_builders::BatchDisassociateUserStack) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_stack_associations(Vec<UserStackAssociation>)`](crate::client::fluent_builders::BatchDisassociateUserStack::user_stack_associations) / [`set_user_stack_associations(Option<Vec<UserStackAssociation>>)`](crate::client::fluent_builders::BatchDisassociateUserStack::set_user_stack_associations): <p>The list of UserStackAssociation objects.</p>
                            /// - On success, responds with [`BatchDisassociateUserStackOutput`](crate::output::BatchDisassociateUserStackOutput) with field(s):
    ///   - [`errors(Option<Vec<UserStackAssociationError>>)`](crate::output::BatchDisassociateUserStackOutput::errors): <p>The list of UserStackAssociationError objects.</p>
                            /// - On failure, responds with [`SdkError<BatchDisassociateUserStackError>`](crate::error::BatchDisassociateUserStackError)
    pub fn batch_disassociate_user_stack(&self) -> crate::client::fluent_builders::BatchDisassociateUserStack {
                                crate::client::fluent_builders::BatchDisassociateUserStack::new(self.handle.clone())
                            }
}

