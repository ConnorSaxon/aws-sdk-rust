// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateRoutingProfileQueues`](crate::client::fluent_builders::DisassociateRoutingProfileQueues) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateRoutingProfileQueues::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::DisassociateRoutingProfileQueues::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`routing_profile_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateRoutingProfileQueues::routing_profile_id) / [`set_routing_profile_id(Option<String>)`](crate::client::fluent_builders::DisassociateRoutingProfileQueues::set_routing_profile_id): <p>The identifier of the routing profile.</p>
    ///   - [`queue_references(Vec<RoutingProfileQueueReference>)`](crate::client::fluent_builders::DisassociateRoutingProfileQueues::queue_references) / [`set_queue_references(Option<Vec<RoutingProfileQueueReference>>)`](crate::client::fluent_builders::DisassociateRoutingProfileQueues::set_queue_references): <p>The queues to disassociate from this routing profile.</p>
                            /// - On success, responds with [`DisassociateRoutingProfileQueuesOutput`](crate::output::DisassociateRoutingProfileQueuesOutput)
                            /// - On failure, responds with [`SdkError<DisassociateRoutingProfileQueuesError>`](crate::error::DisassociateRoutingProfileQueuesError)
    pub fn disassociate_routing_profile_queues(&self) -> crate::client::fluent_builders::DisassociateRoutingProfileQueues {
                                crate::client::fluent_builders::DisassociateRoutingProfileQueues::new(self.handle.clone())
                            }
}

