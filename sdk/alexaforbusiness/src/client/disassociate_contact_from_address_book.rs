// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateContactFromAddressBook`](crate::client::fluent_builders::DisassociateContactFromAddressBook) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`contact_arn(impl Into<String>)`](crate::client::fluent_builders::DisassociateContactFromAddressBook::contact_arn) / [`set_contact_arn(Option<String>)`](crate::client::fluent_builders::DisassociateContactFromAddressBook::set_contact_arn): <p>The ARN of the contact to disassociate from an address book.</p>
    ///   - [`address_book_arn(impl Into<String>)`](crate::client::fluent_builders::DisassociateContactFromAddressBook::address_book_arn) / [`set_address_book_arn(Option<String>)`](crate::client::fluent_builders::DisassociateContactFromAddressBook::set_address_book_arn): <p>The ARN of the address from which to disassociate the contact.</p>
                            /// - On success, responds with [`DisassociateContactFromAddressBookOutput`](crate::output::DisassociateContactFromAddressBookOutput)
                            /// - On failure, responds with [`SdkError<DisassociateContactFromAddressBookError>`](crate::error::DisassociateContactFromAddressBookError)
    pub fn disassociate_contact_from_address_book(&self) -> crate::client::fluent_builders::DisassociateContactFromAddressBook {
                                crate::client::fluent_builders::DisassociateContactFromAddressBook::new(self.handle.clone())
                            }
}

