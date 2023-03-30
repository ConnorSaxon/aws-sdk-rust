// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListThingsInBillingGroup`](crate::client::fluent_builders::ListThingsInBillingGroup) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListThingsInBillingGroup::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`billing_group_name(impl Into<String>)`](crate::client::fluent_builders::ListThingsInBillingGroup::billing_group_name) / [`set_billing_group_name(Option<String>)`](crate::client::fluent_builders::ListThingsInBillingGroup::set_billing_group_name): <p>The name of the billing group.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListThingsInBillingGroup::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListThingsInBillingGroup::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListThingsInBillingGroup::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListThingsInBillingGroup::set_max_results): <p>The maximum number of results to return per request.</p>
                            /// - On success, responds with [`ListThingsInBillingGroupOutput`](crate::output::ListThingsInBillingGroupOutput) with field(s):
    ///   - [`things(Option<Vec<String>>)`](crate::output::ListThingsInBillingGroupOutput::things): <p>A list of things in the billing group.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListThingsInBillingGroupOutput::next_token): <p>The token to use to get the next set of results. Will not be returned if operation has returned all results.</p>
                            /// - On failure, responds with [`SdkError<ListThingsInBillingGroupError>`](crate::error::ListThingsInBillingGroupError)
    pub fn list_things_in_billing_group(&self) -> crate::client::fluent_builders::ListThingsInBillingGroup {
                                crate::client::fluent_builders::ListThingsInBillingGroup::new(self.handle.clone())
                            }
}

