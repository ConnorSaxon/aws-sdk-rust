// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeregisterCompute`](crate::client::fluent_builders::DeregisterCompute) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::client::fluent_builders::DeregisterCompute::fleet_id) / [`set_fleet_id(Option<String>)`](crate::client::fluent_builders::DeregisterCompute::set_fleet_id): <p>&gt;A unique identifier for the fleet the compute resource is registered to.</p>
    ///   - [`compute_name(impl Into<String>)`](crate::client::fluent_builders::DeregisterCompute::compute_name) / [`set_compute_name(Option<String>)`](crate::client::fluent_builders::DeregisterCompute::set_compute_name): <p>The name of the compute resource you want to delete.</p>
                            /// - On success, responds with [`DeregisterComputeOutput`](crate::output::DeregisterComputeOutput)
                            /// - On failure, responds with [`SdkError<DeregisterComputeError>`](crate::error::DeregisterComputeError)
    pub fn deregister_compute(&self) -> crate::client::fluent_builders::DeregisterCompute {
                                crate::client::fluent_builders::DeregisterCompute::new(self.handle.clone())
                            }
}

