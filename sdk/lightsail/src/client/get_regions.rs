// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRegions`](crate::client::fluent_builders::GetRegions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`include_availability_zones(bool)`](crate::client::fluent_builders::GetRegions::include_availability_zones) / [`set_include_availability_zones(Option<bool>)`](crate::client::fluent_builders::GetRegions::set_include_availability_zones): <p>A Boolean value indicating whether to also include Availability Zones in your get regions request. Availability Zones are indicated with a letter: e.g., <code>us-east-2a</code>.</p>
    ///   - [`include_relational_database_availability_zones(bool)`](crate::client::fluent_builders::GetRegions::include_relational_database_availability_zones) / [`set_include_relational_database_availability_zones(Option<bool>)`](crate::client::fluent_builders::GetRegions::set_include_relational_database_availability_zones): <p>A Boolean value indicating whether to also include Availability Zones for databases in your get regions request. Availability Zones are indicated with a letter (e.g., <code>us-east-2a</code>).</p>
                            /// - On success, responds with [`GetRegionsOutput`](crate::output::GetRegionsOutput) with field(s):
    ///   - [`regions(Option<Vec<Region>>)`](crate::output::GetRegionsOutput::regions): <p>An array of key-value pairs containing information about your get regions request.</p>
                            /// - On failure, responds with [`SdkError<GetRegionsError>`](crate::error::GetRegionsError)
    pub fn get_regions(&self) -> crate::client::fluent_builders::GetRegions {
                                crate::client::fluent_builders::GetRegions::new(self.handle.clone())
                            }
}

