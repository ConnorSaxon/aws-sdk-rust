// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRestApi`](crate::client::fluent_builders::GetRestApi) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::GetRestApi::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::GetRestApi::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
                            /// - On success, responds with [`GetRestApiOutput`](crate::output::GetRestApiOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::GetRestApiOutput::id): <p>The API's identifier. This identifier is unique across all of your APIs in API Gateway.</p>
    ///   - [`name(Option<String>)`](crate::output::GetRestApiOutput::name): <p>The API's name.</p>
    ///   - [`description(Option<String>)`](crate::output::GetRestApiOutput::description): <p>The API's description.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::output::GetRestApiOutput::created_date): <p>The timestamp when the API was created.</p>
    ///   - [`version(Option<String>)`](crate::output::GetRestApiOutput::version): <p>A version identifier for the API.</p>
    ///   - [`warnings(Option<Vec<String>>)`](crate::output::GetRestApiOutput::warnings): <p>The warning messages reported when <code>failonwarnings</code> is turned on during API import.</p>
    ///   - [`binary_media_types(Option<Vec<String>>)`](crate::output::GetRestApiOutput::binary_media_types): <p>The list of binary media types supported by the RestApi. By default, the RestApi supports only UTF-8-encoded text payloads.</p>
    ///   - [`minimum_compression_size(Option<i32>)`](crate::output::GetRestApiOutput::minimum_compression_size): <p>A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.</p>
    ///   - [`api_key_source(Option<ApiKeySourceType>)`](crate::output::GetRestApiOutput::api_key_source): <p>The source of the API key for metering requests according to a usage plan. Valid values are: &gt;<code>HEADER</code> to read the API key from the <code>X-API-Key</code> header of a request. <code>AUTHORIZER</code> to read the API key from the <code>UsageIdentifierKey</code> from a custom authorizer.</p>
    ///   - [`endpoint_configuration(Option<EndpointConfiguration>)`](crate::output::GetRestApiOutput::endpoint_configuration): <p>The endpoint configuration of this RestApi showing the endpoint types of the API. </p>
    ///   - [`policy(Option<String>)`](crate::output::GetRestApiOutput::policy): <p>A stringified JSON policy document that applies to this RestApi regardless of the caller and Method configuration.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::GetRestApiOutput::tags): <p>The collection of tags. Each tag element is associated with a given resource.</p>
    ///   - [`disable_execute_api_endpoint(bool)`](crate::output::GetRestApiOutput::disable_execute_api_endpoint): <p>Specifies whether clients can invoke your API by using the default <code>execute-api</code> endpoint. By default, clients can invoke your API with the default <code>https://{api_id}.execute-api.{region}.amazonaws.com</code> endpoint. To require that clients use a custom domain name to invoke your API, disable the default endpoint.</p>
                            /// - On failure, responds with [`SdkError<GetRestApiError>`](crate::error::GetRestApiError)
    pub fn get_rest_api(&self) -> crate::client::fluent_builders::GetRestApi {
                                crate::client::fluent_builders::GetRestApi::new(self.handle.clone())
                            }
}

