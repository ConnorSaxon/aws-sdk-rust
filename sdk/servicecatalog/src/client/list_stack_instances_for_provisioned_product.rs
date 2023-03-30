// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListStackInstancesForProvisionedProduct`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::accept_language) / [`set_accept_language(Option<String>)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::set_accept_language): <p>The language code.</p>  <ul>   <li> <p> <code>en</code> - English (default)</p> </li>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul>
    ///   - [`provisioned_product_id(impl Into<String>)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::provisioned_product_id) / [`set_provisioned_product_id(Option<String>)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::set_provisioned_product_id): <p>The identifier of the provisioned product.</p>
    ///   - [`page_token(impl Into<String>)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::page_token) / [`set_page_token(Option<String>)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::set_page_token): <p>The page token for the next set of results. To retrieve the first set of results, use null.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::page_size) / [`set_page_size(i32)`](crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::set_page_size): <p>The maximum number of items to return with this call.</p>
                            /// - On success, responds with [`ListStackInstancesForProvisionedProductOutput`](crate::output::ListStackInstancesForProvisionedProductOutput) with field(s):
    ///   - [`stack_instances(Option<Vec<StackInstance>>)`](crate::output::ListStackInstancesForProvisionedProductOutput::stack_instances): <p>List of stack instances.</p>
    ///   - [`next_page_token(Option<String>)`](crate::output::ListStackInstancesForProvisionedProductOutput::next_page_token): <p>The page token to use to retrieve the next set of results. If there are no additional results, this value is null.</p>
                            /// - On failure, responds with [`SdkError<ListStackInstancesForProvisionedProductError>`](crate::error::ListStackInstancesForProvisionedProductError)
    pub fn list_stack_instances_for_provisioned_product(&self) -> crate::client::fluent_builders::ListStackInstancesForProvisionedProduct {
                                crate::client::fluent_builders::ListStackInstancesForProvisionedProduct::new(self.handle.clone())
                            }
}

