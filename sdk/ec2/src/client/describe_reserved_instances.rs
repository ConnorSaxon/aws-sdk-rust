// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeReservedInstances`](crate::client::fluent_builders::DescribeReservedInstances) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeReservedInstances::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeReservedInstances::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>availability-zone</code> - The Availability Zone where the Reserved Instance can be used.</p> </li>   <li> <p> <code>duration</code> - The duration of the Reserved Instance (one year or three years), in seconds (<code>31536000</code> | <code>94608000</code>).</p> </li>   <li> <p> <code>end</code> - The time when the Reserved Instance expires (for example, 2015-08-07T11:54:42.000Z).</p> </li>   <li> <p> <code>fixed-price</code> - The purchase price of the Reserved Instance (for example, 9800.0).</p> </li>   <li> <p> <code>instance-type</code> - The instance type that is covered by the reservation.</p> </li>   <li> <p> <code>scope</code> - The scope of the Reserved Instance (<code>Region</code> or <code>Availability Zone</code>).</p> </li>   <li> <p> <code>product-description</code> - The Reserved Instance product platform description. Instances that include <code>(Amazon VPC)</code> in the product platform description will only be displayed to EC2-Classic account holders and are for use with Amazon VPC (<code>Linux/UNIX</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>SUSE Linux</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Red Hat Enterprise Linux</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>Red Hat Enterprise Linux with HA (Amazon VPC)</code> | <code>Windows</code> | <code>Windows (Amazon VPC)</code> | <code>Windows with SQL Server Standard</code> | <code>Windows with SQL Server Standard (Amazon VPC)</code> | <code>Windows with SQL Server Web</code> | <code>Windows with SQL Server Web (Amazon VPC)</code> | <code>Windows with SQL Server Enterprise</code> | <code>Windows with SQL Server Enterprise (Amazon VPC)</code>).</p> </li>   <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instance.</p> </li>   <li> <p> <code>start</code> - The time at which the Reserved Instance purchase request was placed (for example, 2014-08-07T11:54:42.000Z).</p> </li>   <li> <p> <code>state</code> - The state of the Reserved Instance (<code>payment-pending</code> | <code>active</code> | <code>payment-failed</code> | <code>retired</code>).</p> </li>   <li> <p> <code>tag:     <key></key></code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>usage-price</code> - The usage price of the Reserved Instance, per hour (for example, 0.84).</p> </li>  </ul>
    ///   - [`offering_class(OfferingClassType)`](crate::client::fluent_builders::DescribeReservedInstances::offering_class) / [`set_offering_class(Option<OfferingClassType>)`](crate::client::fluent_builders::DescribeReservedInstances::set_offering_class): <p>Describes whether the Reserved Instance is Standard or Convertible.</p>
    ///   - [`reserved_instances_ids(Vec<String>)`](crate::client::fluent_builders::DescribeReservedInstances::reserved_instances_ids) / [`set_reserved_instances_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeReservedInstances::set_reserved_instances_ids): <p>One or more Reserved Instance IDs.</p>  <p>Default: Describes all your Reserved Instances, or only those otherwise specified.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeReservedInstances::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeReservedInstances::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`offering_type(OfferingTypeValues)`](crate::client::fluent_builders::DescribeReservedInstances::offering_type) / [`set_offering_type(Option<OfferingTypeValues>)`](crate::client::fluent_builders::DescribeReservedInstances::set_offering_type): <p>The Reserved Instance offering type. If you are using tools that predate the 2011-11-01 API version, you only have access to the <code>Medium Utilization</code> Reserved Instance offering type.</p>
                            /// - On success, responds with [`DescribeReservedInstancesOutput`](crate::output::DescribeReservedInstancesOutput) with field(s):
    ///   - [`reserved_instances(Option<Vec<ReservedInstances>>)`](crate::output::DescribeReservedInstancesOutput::reserved_instances): <p>A list of Reserved Instances.</p>
                            /// - On failure, responds with [`SdkError<DescribeReservedInstancesError>`](crate::error::DescribeReservedInstancesError)
    pub fn describe_reserved_instances(&self) -> crate::client::fluent_builders::DescribeReservedInstances {
                                crate::client::fluent_builders::DescribeReservedInstances::new(self.handle.clone())
                            }
}

