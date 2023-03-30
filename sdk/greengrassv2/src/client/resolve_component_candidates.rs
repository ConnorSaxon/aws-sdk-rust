// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ResolveComponentCandidates`](crate::client::fluent_builders::ResolveComponentCandidates) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`platform(ComponentPlatform)`](crate::client::fluent_builders::ResolveComponentCandidates::platform) / [`set_platform(Option<ComponentPlatform>)`](crate::client::fluent_builders::ResolveComponentCandidates::set_platform): <p>The platform to use to resolve compatible components.</p>
    ///   - [`component_candidates(Vec<ComponentCandidate>)`](crate::client::fluent_builders::ResolveComponentCandidates::component_candidates) / [`set_component_candidates(Option<Vec<ComponentCandidate>>)`](crate::client::fluent_builders::ResolveComponentCandidates::set_component_candidates): <p>The list of components to resolve.</p>
                            /// - On success, responds with [`ResolveComponentCandidatesOutput`](crate::output::ResolveComponentCandidatesOutput) with field(s):
    ///   - [`resolved_component_versions(Option<Vec<ResolvedComponentVersion>>)`](crate::output::ResolveComponentCandidatesOutput::resolved_component_versions): <p>A list of components that meet the requirements that you specify in the request. This list includes each component's recipe that you can use to install the component.</p>
                            /// - On failure, responds with [`SdkError<ResolveComponentCandidatesError>`](crate::error::ResolveComponentCandidatesError)
    pub fn resolve_component_candidates(&self) -> crate::client::fluent_builders::ResolveComponentCandidates {
                                crate::client::fluent_builders::ResolveComponentCandidates::new(self.handle.clone())
                            }
}

