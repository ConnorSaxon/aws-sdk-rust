// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTrafficPolicyInstancesByHostedZone`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl Into<String>)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::set_hosted_zone_id): <p>The ID of the hosted zone that you want to list traffic policy instances for.</p>
    ///   - [`traffic_policy_instance_name_marker(impl Into<String>)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::traffic_policy_instance_name_marker) / [`set_traffic_policy_instance_name_marker(Option<String>)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::set_traffic_policy_instance_name_marker): <p>If the value of <code>IsTruncated</code> in the previous response is true, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancename</code>, specify the value of <code>TrafficPolicyInstanceNameMarker</code> from the previous response, which is the name of the first traffic policy instance in the next group of traffic policy instances.</p>  <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    ///   - [`traffic_policy_instance_type_marker(RrType)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::traffic_policy_instance_type_marker) / [`set_traffic_policy_instance_type_marker(Option<RrType>)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::set_traffic_policy_instance_type_marker): <p>If the value of <code>IsTruncated</code> in the previous response is true, you have more traffic policy instances. To get more traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>trafficpolicyinstancetype</code>, specify the value of <code>TrafficPolicyInstanceTypeMarker</code> from the previous response, which is the type of the first traffic policy instance in the next group of traffic policy instances.</p>  <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::set_max_items): <p>The maximum number of traffic policy instances to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance that Amazon Route 53 will return if you submit another request.</p>
                            /// - On success, responds with [`ListTrafficPolicyInstancesByHostedZoneOutput`](crate::output::ListTrafficPolicyInstancesByHostedZoneOutput) with field(s):
    ///   - [`traffic_policy_instances(Option<Vec<TrafficPolicyInstance>>)`](crate::output::ListTrafficPolicyInstancesByHostedZoneOutput::traffic_policy_instances): <p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request. </p>
    ///   - [`traffic_policy_instance_name_marker(Option<String>)`](crate::output::ListTrafficPolicyInstancesByHostedZoneOutput::traffic_policy_instance_name_marker): <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of traffic policy instances.</p>
    ///   - [`traffic_policy_instance_type_marker(Option<RrType>)`](crate::output::ListTrafficPolicyInstancesByHostedZoneOutput::traffic_policy_instance_type_marker): <p>If <code>IsTruncated</code> is true, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance in the next group of traffic policy instances.</p>
    ///   - [`is_truncated(bool)`](crate::output::ListTrafficPolicyInstancesByHostedZoneOutput::is_truncated): <p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get the next group of traffic policy instances by submitting another <code>ListTrafficPolicyInstancesByHostedZone</code> request and specifying the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> in the corresponding request parameters.</p>
    ///   - [`max_items(Option<i32>)`](crate::output::ListTrafficPolicyInstancesByHostedZoneOutput::max_items): <p>The value that you specified for the <code>MaxItems</code> parameter in the <code>ListTrafficPolicyInstancesByHostedZone</code> request that produced the current response.</p>
                            /// - On failure, responds with [`SdkError<ListTrafficPolicyInstancesByHostedZoneError>`](crate::error::ListTrafficPolicyInstancesByHostedZoneError)
    pub fn list_traffic_policy_instances_by_hosted_zone(&self) -> crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone {
                                crate::client::fluent_builders::ListTrafficPolicyInstancesByHostedZone::new(self.handle.clone())
                            }
}

