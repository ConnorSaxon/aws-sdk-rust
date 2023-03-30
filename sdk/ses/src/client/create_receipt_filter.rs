// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateReceiptFilter`](crate::client::fluent_builders::CreateReceiptFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filter(ReceiptFilter)`](crate::client::fluent_builders::CreateReceiptFilter::filter) / [`set_filter(Option<ReceiptFilter>)`](crate::client::fluent_builders::CreateReceiptFilter::set_filter): <p>A data structure that describes the IP address filter to create, which consists of a name, an IP address range, and whether to allow or block mail from it.</p>
                            /// - On success, responds with [`CreateReceiptFilterOutput`](crate::output::CreateReceiptFilterOutput)
                            /// - On failure, responds with [`SdkError<CreateReceiptFilterError>`](crate::error::CreateReceiptFilterError)
    pub fn create_receipt_filter(&self) -> crate::client::fluent_builders::CreateReceiptFilter {
                                crate::client::fluent_builders::CreateReceiptFilter::new(self.handle.clone())
                            }
}

