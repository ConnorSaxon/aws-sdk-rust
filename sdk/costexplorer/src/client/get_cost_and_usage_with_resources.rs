// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCostAndUsageWithResources`](crate::client::fluent_builders::GetCostAndUsageWithResources) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`time_period(DateInterval)`](crate::client::fluent_builders::GetCostAndUsageWithResources::time_period) / [`set_time_period(Option<DateInterval>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::set_time_period): <p>Sets the start and end dates for retrieving Amazon Web Services costs. The range must be within the last 14 days (the start date cannot be earlier than 14 days ago). The start date is inclusive, but the end date is exclusive. For example, if <code>start</code> is <code>2017-01-01</code> and <code>end</code> is <code>2017-05-01</code>, then the cost and usage data is retrieved from <code>2017-01-01</code> up to and including <code>2017-04-30</code> but not including <code>2017-05-01</code>.</p>
    ///   - [`granularity(Granularity)`](crate::client::fluent_builders::GetCostAndUsageWithResources::granularity) / [`set_granularity(Option<Granularity>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::set_granularity): <p>Sets the Amazon Web Services cost granularity to <code>MONTHLY</code>, <code>DAILY</code>, or <code>HOURLY</code>. If <code>Granularity</code> isn't set, the response object doesn't include the <code>Granularity</code>, <code>MONTHLY</code>, <code>DAILY</code>, or <code>HOURLY</code>. </p>
    ///   - [`filter(Expression)`](crate::client::fluent_builders::GetCostAndUsageWithResources::filter) / [`set_filter(Option<Expression>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::set_filter): <p>Filters Amazon Web Services costs by different dimensions. For example, you can specify <code>SERVICE</code> and <code>LINKED_ACCOUNT</code> and get the costs that are associated with that account's usage of that service. You can nest <code>Expression</code> objects to define any combination of dimension filters. For more information, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a>. </p>  <p>The <code>GetCostAndUsageWithResources</code> operation requires that you either group by or filter by a <code>ResourceId</code>. It requires the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Expression.html">Expression</a> <code>"SERVICE = Amazon Elastic Compute Cloud - Compute"</code> in the filter.</p>  <p>Valid values for <code>MatchOptions</code> for <code>Dimensions</code> are <code>EQUALS</code> and <code>CASE_SENSITIVE</code>.</p>  <p>Valid values for <code>MatchOptions</code> for <code>CostCategories</code> and <code>Tags</code> are <code>EQUALS</code>, <code>ABSENT</code>, and <code>CASE_SENSITIVE</code>. Default values are <code>EQUALS</code> and <code>CASE_SENSITIVE</code>.</p>
    ///   - [`metrics(Vec<String>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::metrics) / [`set_metrics(Option<Vec<String>>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::set_metrics): <p>Which metrics are returned in the query. For more information about blended and unblended rates, see <a href="http://aws.amazon.com/premiumsupport/knowledge-center/blended-rates-intro/">Why does the "blended" annotation appear on some line items in my bill?</a>. </p>  <p>Valid values are <code>AmortizedCost</code>, <code>BlendedCost</code>, <code>NetAmortizedCost</code>, <code>NetUnblendedCost</code>, <code>NormalizedUsageAmount</code>, <code>UnblendedCost</code>, and <code>UsageQuantity</code>. </p> <note>   <p>If you return the <code>UsageQuantity</code> metric, the service aggregates all usage numbers without taking the units into account. For example, if you aggregate <code>usageQuantity</code> across all of Amazon EC2, the results aren't meaningful because Amazon EC2 compute hours and data transfer are measured in different units (for example, hour or GB). To get more meaningful <code>UsageQuantity</code> metrics, filter by <code>UsageType</code> or <code>UsageTypeGroups</code>. </p>  </note>  <p> <code>Metrics</code> is required for <code>GetCostAndUsageWithResources</code> requests.</p>
    ///   - [`group_by(Vec<GroupDefinition>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::group_by) / [`set_group_by(Option<Vec<GroupDefinition>>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::set_group_by): <p>You can group Amazon Web Services costs using up to two different groups: <code>DIMENSION</code>, <code>TAG</code>, <code>COST_CATEGORY</code>.</p>
    ///   - [`next_page_token(impl Into<String>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::next_page_token) / [`set_next_page_token(Option<String>)`](crate::client::fluent_builders::GetCostAndUsageWithResources::set_next_page_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
                            /// - On success, responds with [`GetCostAndUsageWithResourcesOutput`](crate::output::GetCostAndUsageWithResourcesOutput) with field(s):
    ///   - [`next_page_token(Option<String>)`](crate::output::GetCostAndUsageWithResourcesOutput::next_page_token): <p>The token for the next set of retrievable results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    ///   - [`group_definitions(Option<Vec<GroupDefinition>>)`](crate::output::GetCostAndUsageWithResourcesOutput::group_definitions): <p>The groups that are specified by the <code>Filter</code> or <code>GroupBy</code> parameters in the request.</p>
    ///   - [`results_by_time(Option<Vec<ResultByTime>>)`](crate::output::GetCostAndUsageWithResourcesOutput::results_by_time): <p>The time period that's covered by the results in the response.</p>
    ///   - [`dimension_value_attributes(Option<Vec<DimensionValuesWithAttributes>>)`](crate::output::GetCostAndUsageWithResourcesOutput::dimension_value_attributes): <p>The attributes that apply to a specific dimension value. For example, if the value is a linked account, the attribute is that account name.</p>
                            /// - On failure, responds with [`SdkError<GetCostAndUsageWithResourcesError>`](crate::error::GetCostAndUsageWithResourcesError)
    pub fn get_cost_and_usage_with_resources(&self) -> crate::client::fluent_builders::GetCostAndUsageWithResources {
                                crate::client::fluent_builders::GetCostAndUsageWithResources::new(self.handle.clone())
                            }
}

