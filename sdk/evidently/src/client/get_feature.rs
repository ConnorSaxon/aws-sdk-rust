// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetFeature`](crate::client::fluent_builders::GetFeature) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project(impl Into<String>)`](crate::client::fluent_builders::GetFeature::project) / [`set_project(Option<String>)`](crate::client::fluent_builders::GetFeature::set_project): <p>The name or ARN of the project that contains the feature.</p>
    ///   - [`feature(impl Into<String>)`](crate::client::fluent_builders::GetFeature::feature) / [`set_feature(Option<String>)`](crate::client::fluent_builders::GetFeature::set_feature): <p>The name of the feature that you want to retrieve information for.</p>
                            /// - On success, responds with [`GetFeatureOutput`](crate::output::GetFeatureOutput) with field(s):
    ///   - [`feature(Option<Feature>)`](crate::output::GetFeatureOutput::feature): <p>A structure containing the configuration details of the feature.</p>
                            /// - On failure, responds with [`SdkError<GetFeatureError>`](crate::error::GetFeatureError)
    pub fn get_feature(&self) -> crate::client::fluent_builders::GetFeature {
                                crate::client::fluent_builders::GetFeature::new(self.handle.clone())
                            }
}

