// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeReservedInstancesOfferings`](crate::client::fluent_builders::DescribeReservedInstancesOfferings) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`availability_zone(impl Into<String>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::availability_zone) / [`set_availability_zone(Option<String>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_availability_zone): <p>The Availability Zone in which the Reserved Instance can be used.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>availability-zone</code> - The Availability Zone where the Reserved Instance can be used.</p> </li>   <li> <p> <code>duration</code> - The duration of the Reserved Instance (for example, one year or three years), in seconds (<code>31536000</code> | <code>94608000</code>).</p> </li>   <li> <p> <code>fixed-price</code> - The purchase price of the Reserved Instance (for example, 9800.0).</p> </li>   <li> <p> <code>instance-type</code> - The instance type that is covered by the reservation.</p> </li>   <li> <p> <code>marketplace</code> - Set to <code>true</code> to show only Reserved Instance Marketplace offerings. When this filter is not used, which is the default behavior, all offerings from both Amazon Web Services and the Reserved Instance Marketplace are listed.</p> </li>   <li> <p> <code>product-description</code> - The Reserved Instance product platform description. Instances that include <code>(Amazon VPC)</code> in the product platform description will only be displayed to EC2-Classic account holders and are for use with Amazon VPC. (<code>Linux/UNIX</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>SUSE Linux</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Red Hat Enterprise Linux</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>Red Hat Enterprise Linux with HA (Amazon VPC)</code> | <code>Windows</code> | <code>Windows (Amazon VPC)</code> | <code>Windows with SQL Server Standard</code> | <code>Windows with SQL Server Standard (Amazon VPC)</code> | <code>Windows with SQL Server Web</code> | <code> Windows with SQL Server Web (Amazon VPC)</code> | <code>Windows with SQL Server Enterprise</code> | <code>Windows with SQL Server Enterprise (Amazon VPC)</code>) </p> </li>   <li> <p> <code>reserved-instances-offering-id</code> - The Reserved Instances offering ID.</p> </li>   <li> <p> <code>scope</code> - The scope of the Reserved Instance (<code>Availability Zone</code> or <code>Region</code>).</p> </li>   <li> <p> <code>usage-price</code> - The usage price of the Reserved Instance, per hour (for example, 0.84).</p> </li>  </ul>
    ///   - [`include_marketplace(bool)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::include_marketplace) / [`set_include_marketplace(Option<bool>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_include_marketplace): <p>Include Reserved Instance Marketplace offerings in the response.</p>
    ///   - [`instance_type(InstanceType)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::instance_type) / [`set_instance_type(Option<InstanceType>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_instance_type): <p>The instance type that the reservation will cover (for example, <code>m1.small</code>). For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    ///   - [`max_duration(i64)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::max_duration) / [`set_max_duration(Option<i64>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_max_duration): <p>The maximum duration (in seconds) to filter when searching for offerings.</p>  <p>Default: 94608000 (3 years)</p>
    ///   - [`max_instance_count(i32)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::max_instance_count) / [`set_max_instance_count(Option<i32>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_max_instance_count): <p>The maximum number of instances to filter when searching for offerings.</p>  <p>Default: 20</p>
    ///   - [`min_duration(i64)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::min_duration) / [`set_min_duration(Option<i64>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_min_duration): <p>The minimum duration (in seconds) to filter when searching for offerings.</p>  <p>Default: 2592000 (1 month)</p>
    ///   - [`offering_class(OfferingClassType)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::offering_class) / [`set_offering_class(Option<OfferingClassType>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_offering_class): <p>The offering class of the Reserved Instance. Can be <code>standard</code> or <code>convertible</code>.</p>
    ///   - [`product_description(RiProductDescription)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::product_description) / [`set_product_description(Option<RiProductDescription>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_product_description): <p>The Reserved Instance product platform description. Instances that include <code>(Amazon VPC)</code> in the description are for use with Amazon VPC.</p>
    ///   - [`reserved_instances_offering_ids(Vec<String>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::reserved_instances_offering_ids) / [`set_reserved_instances_offering_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_reserved_instances_offering_ids): <p>One or more Reserved Instances offering IDs.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_tenancy(Tenancy)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::instance_tenancy) / [`set_instance_tenancy(Option<Tenancy>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_instance_tenancy): <p>The tenancy of the instances covered by the reservation. A Reserved Instance with a tenancy of <code>dedicated</code> is applied to instances that run in a VPC on single-tenant hardware (i.e., Dedicated Instances).</p>  <p> <b>Important:</b> The <code>host</code> value cannot be used with this parameter. Use the <code>default</code> or <code>dedicated</code> values only.</p>  <p>Default: <code>default</code> </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_max_results): <p>The maximum number of results to return for the request in a single page. The remaining results of the initial request can be seen by sending another request with the returned <code>NextToken</code> value. The maximum is 100.</p>  <p>Default: 100</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_next_token): <p>The token to retrieve the next page of results.</p>
    ///   - [`offering_type(OfferingTypeValues)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::offering_type) / [`set_offering_type(Option<OfferingTypeValues>)`](crate::client::fluent_builders::DescribeReservedInstancesOfferings::set_offering_type): <p>The Reserved Instance offering type. If you are using tools that predate the 2011-11-01 API version, you only have access to the <code>Medium Utilization</code> Reserved Instance offering type. </p>
                            /// - On success, responds with [`DescribeReservedInstancesOfferingsOutput`](crate::output::DescribeReservedInstancesOfferingsOutput) with field(s):
    ///   - [`reserved_instances_offerings(Option<Vec<ReservedInstancesOffering>>)`](crate::output::DescribeReservedInstancesOfferingsOutput::reserved_instances_offerings): <p>A list of Reserved Instances offerings.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeReservedInstancesOfferingsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<DescribeReservedInstancesOfferingsError>`](crate::error::DescribeReservedInstancesOfferingsError)
    pub fn describe_reserved_instances_offerings(&self) -> crate::client::fluent_builders::DescribeReservedInstancesOfferings {
                                crate::client::fluent_builders::DescribeReservedInstancesOfferings::new(self.handle.clone())
                            }
}

