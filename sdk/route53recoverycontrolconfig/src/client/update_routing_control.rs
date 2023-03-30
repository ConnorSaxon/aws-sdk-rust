// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateRoutingControl`](crate::client::fluent_builders::UpdateRoutingControl) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`routing_control_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateRoutingControl::routing_control_arn) / [`set_routing_control_arn(Option<String>)`](crate::client::fluent_builders::UpdateRoutingControl::set_routing_control_arn): <p>The Amazon Resource Name (ARN) of the routing control.</p>
    ///   - [`routing_control_name(impl Into<String>)`](crate::client::fluent_builders::UpdateRoutingControl::routing_control_name) / [`set_routing_control_name(Option<String>)`](crate::client::fluent_builders::UpdateRoutingControl::set_routing_control_name): <p>The name of the routing control.</p>
                            /// - On success, responds with [`UpdateRoutingControlOutput`](crate::output::UpdateRoutingControlOutput) with field(s):
    ///   - [`routing_control(Option<RoutingControl>)`](crate::output::UpdateRoutingControlOutput::routing_control): <p>The routing control that was updated.</p>
                            /// - On failure, responds with [`SdkError<UpdateRoutingControlError>`](crate::error::UpdateRoutingControlError)
    pub fn update_routing_control(&self) -> crate::client::fluent_builders::UpdateRoutingControl {
                                crate::client::fluent_builders::UpdateRoutingControl::new(self.handle.clone())
                            }
}

