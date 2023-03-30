// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResourcesInProtectionGroup`](crate::client::fluent_builders::ListResourcesInProtectionGroup) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListResourcesInProtectionGroup::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`protection_group_id(impl Into<String>)`](crate::client::fluent_builders::ListResourcesInProtectionGroup::protection_group_id) / [`set_protection_group_id(Option<String>)`](crate::client::fluent_builders::ListResourcesInProtectionGroup::set_protection_group_id): <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListResourcesInProtectionGroup::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListResourcesInProtectionGroup::set_next_token): <p>When you request a list of objects from Shield Advanced, if the response does not include all of the remaining available objects, Shield Advanced includes a <code>NextToken</code> value in the response. You can retrieve the next batch of objects by requesting the list again and providing the token that was returned by the prior call in your request. </p>  <p>You can indicate the maximum number of objects that you want Shield Advanced to return for a single call with the <code>MaxResults</code> setting. Shield Advanced will not return more than <code>MaxResults</code> objects, but may return fewer, even if more objects are still available.</p>  <p>Whenever more objects remain that Shield Advanced has not yet returned to you, the response will include a <code>NextToken</code> value.</p>  <p>On your first call to a list operation, leave this setting empty.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListResourcesInProtectionGroup::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListResourcesInProtectionGroup::set_max_results): <p>The greatest number of objects that you want Shield Advanced to return to the list request. Shield Advanced might return fewer objects than you indicate in this setting, even if more objects are available. If there are more objects remaining, Shield Advanced will always also return a <code>NextToken</code> value in the response.</p>  <p>The default setting is 20.</p>
                            /// - On success, responds with [`ListResourcesInProtectionGroupOutput`](crate::output::ListResourcesInProtectionGroupOutput) with field(s):
    ///   - [`resource_arns(Option<Vec<String>>)`](crate::output::ListResourcesInProtectionGroupOutput::resource_arns): <p>The Amazon Resource Names (ARNs) of the resources that are included in the protection group.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListResourcesInProtectionGroupOutput::next_token): <p>When you request a list of objects from Shield Advanced, if the response does not include all of the remaining available objects, Shield Advanced includes a <code>NextToken</code> value in the response. You can retrieve the next batch of objects by requesting the list again and providing the token that was returned by the prior call in your request. </p>  <p>You can indicate the maximum number of objects that you want Shield Advanced to return for a single call with the <code>MaxResults</code> setting. Shield Advanced will not return more than <code>MaxResults</code> objects, but may return fewer, even if more objects are still available.</p>  <p>Whenever more objects remain that Shield Advanced has not yet returned to you, the response will include a <code>NextToken</code> value.</p>
                            /// - On failure, responds with [`SdkError<ListResourcesInProtectionGroupError>`](crate::error::ListResourcesInProtectionGroupError)
    pub fn list_resources_in_protection_group(&self) -> crate::client::fluent_builders::ListResourcesInProtectionGroup {
                                crate::client::fluent_builders::ListResourcesInProtectionGroup::new(self.handle.clone())
                            }
}

