// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAssociatedRoute53HealthChecks`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::set_max_results): <p>The number of objects that you want to return with this call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::set_next_token): <p>The token that identifies which batch of results you want to see.</p>
    ///   - [`routing_control_arn(impl Into<String>)`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::routing_control_arn) / [`set_routing_control_arn(Option<String>)`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::set_routing_control_arn): <p>The Amazon Resource Name (ARN) of the routing control.</p>
                            /// - On success, responds with [`ListAssociatedRoute53HealthChecksOutput`](crate::output::ListAssociatedRoute53HealthChecksOutput) with field(s):
    ///   - [`health_check_ids(Option<Vec<String>>)`](crate::output::ListAssociatedRoute53HealthChecksOutput::health_check_ids): <p>Identifiers for the health checks.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAssociatedRoute53HealthChecksOutput::next_token): <p>Next token for listing health checks.</p>
                            /// - On failure, responds with [`SdkError<ListAssociatedRoute53HealthChecksError>`](crate::error::ListAssociatedRoute53HealthChecksError)
    pub fn list_associated_route53_health_checks(&self) -> crate::client::fluent_builders::ListAssociatedRoute53HealthChecks {
                                crate::client::fluent_builders::ListAssociatedRoute53HealthChecks::new(self.handle.clone())
                            }
}

