// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeInventoryDeletions`](crate::client::fluent_builders::DescribeInventoryDeletions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeInventoryDeletions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`deletion_id(impl Into<String>)`](crate::client::fluent_builders::DescribeInventoryDeletions::deletion_id) / [`set_deletion_id(Option<String>)`](crate::client::fluent_builders::DescribeInventoryDeletions::set_deletion_id): <p>Specify the delete inventory ID for which you want information. This ID was returned by the <code>DeleteInventory</code> operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeInventoryDeletions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeInventoryDeletions::set_next_token): <p>A token to start the list. Use this token to get the next set of results. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeInventoryDeletions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeInventoryDeletions::set_max_results): <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
                            /// - On success, responds with [`DescribeInventoryDeletionsOutput`](crate::output::DescribeInventoryDeletionsOutput) with field(s):
    ///   - [`inventory_deletions(Option<Vec<InventoryDeletionStatusItem>>)`](crate::output::DescribeInventoryDeletionsOutput::inventory_deletions): <p>A list of status items for deleted inventory.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeInventoryDeletionsOutput::next_token): <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
                            /// - On failure, responds with [`SdkError<DescribeInventoryDeletionsError>`](crate::error::DescribeInventoryDeletionsError)
    pub fn describe_inventory_deletions(&self) -> crate::client::fluent_builders::DescribeInventoryDeletions {
                                crate::client::fluent_builders::DescribeInventoryDeletions::new(self.handle.clone())
                            }
}

