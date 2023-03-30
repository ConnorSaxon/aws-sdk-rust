// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeApplicationFleetAssociations`](crate::client::fluent_builders::DescribeApplicationFleetAssociations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`fleet_name(impl Into<String>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::fleet_name) / [`set_fleet_name(Option<String>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::set_fleet_name): <p>The name of the fleet.</p>
    ///   - [`application_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::application_arn) / [`set_application_arn(Option<String>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::set_application_arn): <p>The ARN of the application.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::set_max_results): <p>The maximum size of each page of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeApplicationFleetAssociations::set_next_token): <p>The pagination token used to retrieve the next page of results for this operation.</p>
                            /// - On success, responds with [`DescribeApplicationFleetAssociationsOutput`](crate::output::DescribeApplicationFleetAssociationsOutput) with field(s):
    ///   - [`application_fleet_associations(Option<Vec<ApplicationFleetAssociation>>)`](crate::output::DescribeApplicationFleetAssociationsOutput::application_fleet_associations): <p>The application fleet associations in the list.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeApplicationFleetAssociationsOutput::next_token): <p>The pagination token used to retrieve the next page of results for this operation.</p>
                            /// - On failure, responds with [`SdkError<DescribeApplicationFleetAssociationsError>`](crate::error::DescribeApplicationFleetAssociationsError)
    pub fn describe_application_fleet_associations(&self) -> crate::client::fluent_builders::DescribeApplicationFleetAssociations {
                                crate::client::fluent_builders::DescribeApplicationFleetAssociations::new(self.handle.clone())
                            }
}

