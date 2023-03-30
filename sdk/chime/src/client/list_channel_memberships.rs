// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListChannelMemberships`](crate::client::fluent_builders::ListChannelMemberships) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListChannelMemberships::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::ListChannelMemberships::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::ListChannelMemberships::set_channel_arn): <p>The maximum number of channel memberships that you want returned.</p>
    ///   - [`r#type(ChannelMembershipType)`](crate::client::fluent_builders::ListChannelMemberships::type) / [`set_type(Option<ChannelMembershipType>)`](crate::client::fluent_builders::ListChannelMemberships::set_type): <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListChannelMemberships::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListChannelMemberships::set_max_results): <p>The maximum number of channel memberships that you want returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListChannelMemberships::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListChannelMemberships::set_next_token): <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::ListChannelMemberships::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::ListChannelMemberships::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
                            /// - On success, responds with [`ListChannelMembershipsOutput`](crate::output::ListChannelMembershipsOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::output::ListChannelMembershipsOutput::channel_arn): <p>The ARN of the channel.</p>
    ///   - [`channel_memberships(Option<Vec<ChannelMembershipSummary>>)`](crate::output::ListChannelMembershipsOutput::channel_memberships): <p>The information for the requested channel memberships.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListChannelMembershipsOutput::next_token): <p>The token passed by previous API calls until all requested channel memberships are returned.</p>
                            /// - On failure, responds with [`SdkError<ListChannelMembershipsError>`](crate::error::ListChannelMembershipsError)
    pub fn list_channel_memberships(&self) -> crate::client::fluent_builders::ListChannelMemberships {
                                crate::client::fluent_builders::ListChannelMemberships::new(self.handle.clone())
                            }
}

