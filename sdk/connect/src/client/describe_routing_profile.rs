// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeRoutingProfile`](crate::client::fluent_builders::DescribeRoutingProfile) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::DescribeRoutingProfile::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::DescribeRoutingProfile::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`routing_profile_id(impl Into<String>)`](crate::client::fluent_builders::DescribeRoutingProfile::routing_profile_id) / [`set_routing_profile_id(Option<String>)`](crate::client::fluent_builders::DescribeRoutingProfile::set_routing_profile_id): <p>The identifier of the routing profile.</p>
                            /// - On success, responds with [`DescribeRoutingProfileOutput`](crate::output::DescribeRoutingProfileOutput) with field(s):
    ///   - [`routing_profile(Option<RoutingProfile>)`](crate::output::DescribeRoutingProfileOutput::routing_profile): <p>The routing profile.</p>
                            /// - On failure, responds with [`SdkError<DescribeRoutingProfileError>`](crate::error::DescribeRoutingProfileError)
    pub fn describe_routing_profile(&self) -> crate::client::fluent_builders::DescribeRoutingProfile {
                                crate::client::fluent_builders::DescribeRoutingProfile::new(self.handle.clone())
                            }
}

