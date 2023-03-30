// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRoutingControls`](crate::client::fluent_builders::ListRoutingControls) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRoutingControls::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`control_panel_arn(impl Into<String>)`](crate::client::fluent_builders::ListRoutingControls::control_panel_arn) / [`set_control_panel_arn(Option<String>)`](crate::client::fluent_builders::ListRoutingControls::set_control_panel_arn): <p>The Amazon Resource Name (ARN) of the control panel of the routing controls to list.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRoutingControls::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRoutingControls::set_next_token): <p>The token for the next set of results. You receive this token from a previous call.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRoutingControls::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListRoutingControls::set_max_results): <p>The number of routing controls objects that you want to return with this call. The default value is 500.</p>
                            /// - On success, responds with [`ListRoutingControlsOutput`](crate::output::ListRoutingControlsOutput) with field(s):
    ///   - [`routing_controls(Option<Vec<RoutingControl>>)`](crate::output::ListRoutingControlsOutput::routing_controls): <p>The list of routing controls.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListRoutingControlsOutput::next_token): <p>The token for the next set of results. You receive this token from a previous call.</p>
                            /// - On failure, responds with [`SdkError<ListRoutingControlsError>`](crate::error::ListRoutingControlsError)
    pub fn list_routing_controls(&self) -> crate::client::fluent_builders::ListRoutingControls {
                                crate::client::fluent_builders::ListRoutingControls::new(self.handle.clone())
                            }
}

