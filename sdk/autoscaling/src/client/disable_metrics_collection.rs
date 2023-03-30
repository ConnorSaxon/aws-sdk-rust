// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisableMetricsCollection`](crate::client::fluent_builders::DisableMetricsCollection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::DisableMetricsCollection::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::DisableMetricsCollection::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`metrics(Vec<String>)`](crate::client::fluent_builders::DisableMetricsCollection::metrics) / [`set_metrics(Option<Vec<String>>)`](crate::client::fluent_builders::DisableMetricsCollection::set_metrics): <p>Identifies the metrics to disable.</p>  <p>You can specify one or more of the following metrics:</p>  <ul>   <li> <p> <code>GroupMinSize</code> </p> </li>   <li> <p> <code>GroupMaxSize</code> </p> </li>   <li> <p> <code>GroupDesiredCapacity</code> </p> </li>   <li> <p> <code>GroupInServiceInstances</code> </p> </li>   <li> <p> <code>GroupPendingInstances</code> </p> </li>   <li> <p> <code>GroupStandbyInstances</code> </p> </li>   <li> <p> <code>GroupTerminatingInstances</code> </p> </li>   <li> <p> <code>GroupTotalInstances</code> </p> </li>   <li> <p> <code>GroupInServiceCapacity</code> </p> </li>   <li> <p> <code>GroupPendingCapacity</code> </p> </li>   <li> <p> <code>GroupStandbyCapacity</code> </p> </li>   <li> <p> <code>GroupTerminatingCapacity</code> </p> </li>   <li> <p> <code>GroupTotalCapacity</code> </p> </li>   <li> <p> <code>WarmPoolDesiredCapacity</code> </p> </li>   <li> <p> <code>WarmPoolWarmedCapacity</code> </p> </li>   <li> <p> <code>WarmPoolPendingCapacity</code> </p> </li>   <li> <p> <code>WarmPoolTerminatingCapacity</code> </p> </li>   <li> <p> <code>WarmPoolTotalCapacity</code> </p> </li>   <li> <p> <code>GroupAndWarmPoolDesiredCapacity</code> </p> </li>   <li> <p> <code>GroupAndWarmPoolTotalCapacity</code> </p> </li>  </ul>  <p>If you omit this property, all metrics are disabled.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-cloudwatch-monitoring.html#as-group-metrics">Auto Scaling group metrics</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
                            /// - On success, responds with [`DisableMetricsCollectionOutput`](crate::output::DisableMetricsCollectionOutput)
                            /// - On failure, responds with [`SdkError<DisableMetricsCollectionError>`](crate::error::DisableMetricsCollectionError)
    pub fn disable_metrics_collection(&self) -> crate::client::fluent_builders::DisableMetricsCollection {
                                crate::client::fluent_builders::DisableMetricsCollection::new(self.handle.clone())
                            }
}

