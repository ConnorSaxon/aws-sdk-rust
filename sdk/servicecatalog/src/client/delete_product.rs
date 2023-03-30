// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteProduct`](crate::client::fluent_builders::DeleteProduct) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::DeleteProduct::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::DeleteProduct::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteProduct::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteProduct::set_id): <p>The product identifier.</p>
                            /// - On success, responds with [`DeleteProductOutput`](crate::output::DeleteProductOutput)
                            /// - On failure, responds with [`SdkError<DeleteProductError>`](crate::error::DeleteProductError)
    pub fn delete_product(&self) -> crate::client::fluent_builders::DeleteProduct {
                                crate::client::fluent_builders::DeleteProduct::new(self.handle.clone())
                            }
}

