// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutAutoScalingPolicy`](crate::client::fluent_builders::PutAutoScalingPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl Into<String>)`](crate::client::fluent_builders::PutAutoScalingPolicy::cluster_id) / [`set_cluster_id(Option<String>)`](crate::client::fluent_builders::PutAutoScalingPolicy::set_cluster_id): <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    ///   - [`instance_group_id(impl Into<String>)`](crate::client::fluent_builders::PutAutoScalingPolicy::instance_group_id) / [`set_instance_group_id(Option<String>)`](crate::client::fluent_builders::PutAutoScalingPolicy::set_instance_group_id): <p>Specifies the ID of the instance group to which the automatic scaling policy is applied.</p>
    ///   - [`auto_scaling_policy(AutoScalingPolicy)`](crate::client::fluent_builders::PutAutoScalingPolicy::auto_scaling_policy) / [`set_auto_scaling_policy(Option<AutoScalingPolicy>)`](crate::client::fluent_builders::PutAutoScalingPolicy::set_auto_scaling_policy): <p>Specifies the definition of the automatic scaling policy.</p>
                            /// - On success, responds with [`PutAutoScalingPolicyOutput`](crate::output::PutAutoScalingPolicyOutput) with field(s):
    ///   - [`cluster_id(Option<String>)`](crate::output::PutAutoScalingPolicyOutput::cluster_id): <p>Specifies the ID of a cluster. The instance group to which the automatic scaling policy is applied is within this cluster.</p>
    ///   - [`instance_group_id(Option<String>)`](crate::output::PutAutoScalingPolicyOutput::instance_group_id): <p>Specifies the ID of the instance group to which the scaling policy is applied.</p>
    ///   - [`auto_scaling_policy(Option<AutoScalingPolicyDescription>)`](crate::output::PutAutoScalingPolicyOutput::auto_scaling_policy): <p>The automatic scaling policy definition.</p>
    ///   - [`cluster_arn(Option<String>)`](crate::output::PutAutoScalingPolicyOutput::cluster_arn): <p>The Amazon Resource Name (ARN) of the cluster.</p>
                            /// - On failure, responds with [`SdkError<PutAutoScalingPolicyError>`](crate::error::PutAutoScalingPolicyError)
    pub fn put_auto_scaling_policy(&self) -> crate::client::fluent_builders::PutAutoScalingPolicy {
                                crate::client::fluent_builders::PutAutoScalingPolicy::new(self.handle.clone())
                            }
}

