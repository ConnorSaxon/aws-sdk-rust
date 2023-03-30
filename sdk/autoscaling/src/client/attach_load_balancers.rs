// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AttachLoadBalancers`](crate::client::fluent_builders::AttachLoadBalancers) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::AttachLoadBalancers::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::AttachLoadBalancers::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`load_balancer_names(Vec<String>)`](crate::client::fluent_builders::AttachLoadBalancers::load_balancer_names) / [`set_load_balancer_names(Option<Vec<String>>)`](crate::client::fluent_builders::AttachLoadBalancers::set_load_balancer_names): <p>The names of the load balancers. You can specify up to 10 load balancers.</p>
                            /// - On success, responds with [`AttachLoadBalancersOutput`](crate::output::AttachLoadBalancersOutput)
                            /// - On failure, responds with [`SdkError<AttachLoadBalancersError>`](crate::error::AttachLoadBalancersError)
    pub fn attach_load_balancers(&self) -> crate::client::fluent_builders::AttachLoadBalancers {
                                crate::client::fluent_builders::AttachLoadBalancers::new(self.handle.clone())
                            }
}

