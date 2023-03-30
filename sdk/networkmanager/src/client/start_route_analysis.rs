// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartRouteAnalysis`](crate::client::fluent_builders::StartRouteAnalysis) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`global_network_id(impl Into<String>)`](crate::client::fluent_builders::StartRouteAnalysis::global_network_id) / [`set_global_network_id(Option<String>)`](crate::client::fluent_builders::StartRouteAnalysis::set_global_network_id): <p>The ID of the global network.</p>
    ///   - [`source(RouteAnalysisEndpointOptionsSpecification)`](crate::client::fluent_builders::StartRouteAnalysis::source) / [`set_source(Option<RouteAnalysisEndpointOptionsSpecification>)`](crate::client::fluent_builders::StartRouteAnalysis::set_source): <p>The source from which traffic originates.</p>
    ///   - [`destination(RouteAnalysisEndpointOptionsSpecification)`](crate::client::fluent_builders::StartRouteAnalysis::destination) / [`set_destination(Option<RouteAnalysisEndpointOptionsSpecification>)`](crate::client::fluent_builders::StartRouteAnalysis::set_destination): <p>The destination.</p>
    ///   - [`include_return_path(bool)`](crate::client::fluent_builders::StartRouteAnalysis::include_return_path) / [`set_include_return_path(bool)`](crate::client::fluent_builders::StartRouteAnalysis::set_include_return_path): <p>Indicates whether to analyze the return path. The default is <code>false</code>.</p>
    ///   - [`use_middleboxes(bool)`](crate::client::fluent_builders::StartRouteAnalysis::use_middleboxes) / [`set_use_middleboxes(bool)`](crate::client::fluent_builders::StartRouteAnalysis::set_use_middleboxes): <p>Indicates whether to include the location of middlebox appliances in the route analysis. The default is <code>false</code>.</p>
                            /// - On success, responds with [`StartRouteAnalysisOutput`](crate::output::StartRouteAnalysisOutput) with field(s):
    ///   - [`route_analysis(Option<RouteAnalysis>)`](crate::output::StartRouteAnalysisOutput::route_analysis): <p>The route analysis.</p>
                            /// - On failure, responds with [`SdkError<StartRouteAnalysisError>`](crate::error::StartRouteAnalysisError)
    pub fn start_route_analysis(&self) -> crate::client::fluent_builders::StartRouteAnalysis {
                                crate::client::fluent_builders::StartRouteAnalysis::new(self.handle.clone())
                            }
}

