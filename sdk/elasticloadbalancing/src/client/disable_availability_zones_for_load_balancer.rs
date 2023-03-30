// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisableAvailabilityZonesForLoadBalancer`](crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl Into<String>)`](crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer::set_load_balancer_name): <p>The name of the load balancer.</p>
    ///   - [`availability_zones(Vec<String>)`](crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer::availability_zones) / [`set_availability_zones(Option<Vec<String>>)`](crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer::set_availability_zones): <p>The Availability Zones.</p>
                            /// - On success, responds with [`DisableAvailabilityZonesForLoadBalancerOutput`](crate::output::DisableAvailabilityZonesForLoadBalancerOutput) with field(s):
    ///   - [`availability_zones(Option<Vec<String>>)`](crate::output::DisableAvailabilityZonesForLoadBalancerOutput::availability_zones): <p>The remaining Availability Zones for the load balancer.</p>
                            /// - On failure, responds with [`SdkError<DisableAvailabilityZonesForLoadBalancerError>`](crate::error::DisableAvailabilityZonesForLoadBalancerError)
    pub fn disable_availability_zones_for_load_balancer(&self) -> crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer {
                                crate::client::fluent_builders::DisableAvailabilityZonesForLoadBalancer::new(self.handle.clone())
                            }
}

