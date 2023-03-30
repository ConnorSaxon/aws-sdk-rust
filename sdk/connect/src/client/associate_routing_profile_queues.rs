// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateRoutingProfileQueues`](crate::client::fluent_builders::AssociateRoutingProfileQueues) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::AssociateRoutingProfileQueues::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::AssociateRoutingProfileQueues::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`routing_profile_id(impl Into<String>)`](crate::client::fluent_builders::AssociateRoutingProfileQueues::routing_profile_id) / [`set_routing_profile_id(Option<String>)`](crate::client::fluent_builders::AssociateRoutingProfileQueues::set_routing_profile_id): <p>The identifier of the routing profile.</p>
    ///   - [`queue_configs(Vec<RoutingProfileQueueConfig>)`](crate::client::fluent_builders::AssociateRoutingProfileQueues::queue_configs) / [`set_queue_configs(Option<Vec<RoutingProfileQueueConfig>>)`](crate::client::fluent_builders::AssociateRoutingProfileQueues::set_queue_configs): <p>The queues to associate with this routing profile.</p>
                            /// - On success, responds with [`AssociateRoutingProfileQueuesOutput`](crate::output::AssociateRoutingProfileQueuesOutput)
                            /// - On failure, responds with [`SdkError<AssociateRoutingProfileQueuesError>`](crate::error::AssociateRoutingProfileQueuesError)
    pub fn associate_routing_profile_queues(&self) -> crate::client::fluent_builders::AssociateRoutingProfileQueues {
                                crate::client::fluent_builders::AssociateRoutingProfileQueues::new(self.handle.clone())
                            }
}

