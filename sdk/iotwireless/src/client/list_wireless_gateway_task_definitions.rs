// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListWirelessGatewayTaskDefinitions`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::set_max_results): <p>The maximum number of results to return in this operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::set_next_token): <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    ///   - [`task_definition_type(WirelessGatewayTaskDefinitionType)`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::task_definition_type) / [`set_task_definition_type(Option<WirelessGatewayTaskDefinitionType>)`](crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::set_task_definition_type): <p>A filter to list only the wireless gateway task definitions that use this task definition type.</p>
                            /// - On success, responds with [`ListWirelessGatewayTaskDefinitionsOutput`](crate::output::ListWirelessGatewayTaskDefinitionsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListWirelessGatewayTaskDefinitionsOutput::next_token): <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    ///   - [`task_definitions(Option<Vec<UpdateWirelessGatewayTaskEntry>>)`](crate::output::ListWirelessGatewayTaskDefinitionsOutput::task_definitions): <p>The list of task definitions.</p>
                            /// - On failure, responds with [`SdkError<ListWirelessGatewayTaskDefinitionsError>`](crate::error::ListWirelessGatewayTaskDefinitionsError)
    pub fn list_wireless_gateway_task_definitions(&self) -> crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions {
                                crate::client::fluent_builders::ListWirelessGatewayTaskDefinitions::new(self.handle.clone())
                            }
}

