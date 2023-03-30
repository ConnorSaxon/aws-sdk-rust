// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RequestSpotInstances`](crate::client::fluent_builders::RequestSpotInstances) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zone_group(impl Into<String>)`](crate::client::fluent_builders::RequestSpotInstances::availability_zone_group) / [`set_availability_zone_group(Option<String>)`](crate::client::fluent_builders::RequestSpotInstances::set_availability_zone_group): <p>The user-specified name for a logical grouping of requests.</p>  <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>  <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>  <p>Default: Instances are launched in any available Availability Zone.</p>
    ///   - [`block_duration_minutes(i32)`](crate::client::fluent_builders::RequestSpotInstances::block_duration_minutes) / [`set_block_duration_minutes(Option<i32>)`](crate::client::fluent_builders::RequestSpotInstances::set_block_duration_minutes): <p>Deprecated.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::RequestSpotInstances::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::RequestSpotInstances::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::RequestSpotInstances::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::RequestSpotInstances::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_count(i32)`](crate::client::fluent_builders::RequestSpotInstances::instance_count) / [`set_instance_count(Option<i32>)`](crate::client::fluent_builders::RequestSpotInstances::set_instance_count): <p>The maximum number of Spot Instances to launch.</p>  <p>Default: 1</p>
    ///   - [`launch_group(impl Into<String>)`](crate::client::fluent_builders::RequestSpotInstances::launch_group) / [`set_launch_group(Option<String>)`](crate::client::fluent_builders::RequestSpotInstances::set_launch_group): <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>  <p>Default: Instances are launched and terminated individually</p>
    ///   - [`launch_specification(RequestSpotLaunchSpecification)`](crate::client::fluent_builders::RequestSpotInstances::launch_specification) / [`set_launch_specification(Option<RequestSpotLaunchSpecification>)`](crate::client::fluent_builders::RequestSpotInstances::set_launch_specification): <p>The launch specification.</p>
    ///   - [`spot_price(impl Into<String>)`](crate::client::fluent_builders::RequestSpotInstances::spot_price) / [`set_spot_price(Option<String>)`](crate::client::fluent_builders::RequestSpotInstances::set_spot_price): <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>   <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>  </important>
    ///   - [`r#type(SpotInstanceType)`](crate::client::fluent_builders::RequestSpotInstances::type) / [`set_type(Option<SpotInstanceType>)`](crate::client::fluent_builders::RequestSpotInstances::set_type): <p>The Spot Instance request type.</p>  <p>Default: <code>one-time</code> </p>
    ///   - [`valid_from(DateTime)`](crate::client::fluent_builders::RequestSpotInstances::valid_from) / [`set_valid_from(Option<DateTime>)`](crate::client::fluent_builders::RequestSpotInstances::set_valid_from): <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>  <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    ///   - [`valid_until(DateTime)`](crate::client::fluent_builders::RequestSpotInstances::valid_until) / [`set_valid_until(Option<DateTime>)`](crate::client::fluent_builders::RequestSpotInstances::set_valid_until): <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>  <ul>   <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>   <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>  </ul>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::RequestSpotInstances::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::RequestSpotInstances::set_tag_specifications): <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    ///   - [`instance_interruption_behavior(InstanceInterruptionBehavior)`](crate::client::fluent_builders::RequestSpotInstances::instance_interruption_behavior) / [`set_instance_interruption_behavior(Option<InstanceInterruptionBehavior>)`](crate::client::fluent_builders::RequestSpotInstances::set_instance_interruption_behavior): <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
                            /// - On success, responds with [`RequestSpotInstancesOutput`](crate::output::RequestSpotInstancesOutput) with field(s):
    ///   - [`spot_instance_requests(Option<Vec<SpotInstanceRequest>>)`](crate::output::RequestSpotInstancesOutput::spot_instance_requests): <p>One or more Spot Instance requests.</p>
                            /// - On failure, responds with [`SdkError<RequestSpotInstancesError>`](crate::error::RequestSpotInstancesError)
    pub fn request_spot_instances(&self) -> crate::client::fluent_builders::RequestSpotInstances {
                                crate::client::fluent_builders::RequestSpotInstances::new(self.handle.clone())
                            }
}

