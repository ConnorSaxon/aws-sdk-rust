// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateDevEndpoint`](crate::client::fluent_builders::UpdateDevEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`endpoint_name(impl Into<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::endpoint_name) / [`set_endpoint_name(Option<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_endpoint_name): <p>The name of the <code>DevEndpoint</code> to be updated.</p>
    ///   - [`public_key(impl Into<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::public_key) / [`set_public_key(Option<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_public_key): <p>The public key for the <code>DevEndpoint</code> to use.</p>
    ///   - [`add_public_keys(Vec<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::add_public_keys) / [`set_add_public_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_add_public_keys): <p>The list of public keys for the <code>DevEndpoint</code> to use.</p>
    ///   - [`delete_public_keys(Vec<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::delete_public_keys) / [`set_delete_public_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_delete_public_keys): <p>The list of public keys to be deleted from the <code>DevEndpoint</code>.</p>
    ///   - [`custom_libraries(DevEndpointCustomLibraries)`](crate::client::fluent_builders::UpdateDevEndpoint::custom_libraries) / [`set_custom_libraries(Option<DevEndpointCustomLibraries>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_custom_libraries): <p>Custom Python or Java libraries to be loaded in the <code>DevEndpoint</code>.</p>
    ///   - [`update_etl_libraries(bool)`](crate::client::fluent_builders::UpdateDevEndpoint::update_etl_libraries) / [`set_update_etl_libraries(bool)`](crate::client::fluent_builders::UpdateDevEndpoint::set_update_etl_libraries): <p> <code>True</code> if the list of custom libraries to be loaded in the development endpoint needs to be updated, or <code>False</code> if otherwise.</p>
    ///   - [`delete_arguments(Vec<String>)`](crate::client::fluent_builders::UpdateDevEndpoint::delete_arguments) / [`set_delete_arguments(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_delete_arguments): <p>The list of argument keys to be deleted from the map of arguments used to configure the <code>DevEndpoint</code>.</p>
    ///   - [`add_arguments(HashMap<String, String>)`](crate::client::fluent_builders::UpdateDevEndpoint::add_arguments) / [`set_add_arguments(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdateDevEndpoint::set_add_arguments): <p>The map of arguments to add the map of arguments used to configure the <code>DevEndpoint</code>.</p>  <p>Valid arguments are:</p>  <ul>   <li> <p> <code>"--enable-glue-datacatalog": ""</code> </p> </li>  </ul>  <p>You can specify a version of Python support for development endpoints by using the <code>Arguments</code> parameter in the <code>CreateDevEndpoint</code> or <code>UpdateDevEndpoint</code> APIs. If no arguments are provided, the version defaults to Python 2.</p>
                            /// - On success, responds with [`UpdateDevEndpointOutput`](crate::output::UpdateDevEndpointOutput)
                            /// - On failure, responds with [`SdkError<UpdateDevEndpointError>`](crate::error::UpdateDevEndpointError)
    pub fn update_dev_endpoint(&self) -> crate::client::fluent_builders::UpdateDevEndpoint {
                                crate::client::fluent_builders::UpdateDevEndpoint::new(self.handle.clone())
                            }
}

