// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Fluent builder constructing a request to `InvokeEndpoint`.
/// 
/// <p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. </p> 
/// <p>For an overview of Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p> 
/// <p>Amazon SageMaker strips all POST headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax. </p> 
/// <p>Calls to <code>InvokeEndpoint</code> are authenticated by using Amazon Web Services Signature Version 4. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests (Amazon Web Services Signature Version 4)</a> in the <i>Amazon S3 API Reference</i>.</p> 
/// <p>A customer's model containers must respond to requests within 60 seconds. The model itself can have a maximum processing time of 60 seconds before responding to invocations. If your model is going to take 50-60 seconds of processing time, the SDK socket timeout should be set to be 70 seconds.</p> <note> 
/// <p>Endpoints are scoped to an individual account, and are not public. The URL does not contain the account ID, but Amazon SageMaker determines the account ID from the authentication token that is supplied by the caller.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct InvokeEndpoint {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::invoke_endpoint_input::Builder
            }
impl InvokeEndpoint  {
    /// Creates a new `InvokeEndpoint`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::InvokeEndpoint, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::InvokeEndpointError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::InvokeEndpointOutput, aws_smithy_http::result::SdkError<crate::error::InvokeEndpointError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
    pub fn endpoint_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.endpoint_name(input.into());
        self
    }
    /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
    pub fn set_endpoint_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_name(input);
        self
    }
    /// <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p> 
    /// <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    pub fn body(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.body(input);
        self
    }
    /// <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p> 
    /// <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    pub fn set_body(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_body(input);
        self
    }
    /// <p>The MIME type of the input data in the request body.</p>
    pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content_type(input.into());
        self
    }
    /// <p>The MIME type of the input data in the request body.</p>
    pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content_type(input);
        self
    }
    /// <p>The desired MIME type of the inference in the response.</p>
    pub fn accept(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.accept(input.into());
        self
    }
    /// <p>The desired MIME type of the inference in the response.</p>
    pub fn set_accept(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_accept(input);
        self
    }
    /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p> 
    /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p> 
    /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
    pub fn custom_attributes(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.custom_attributes(input.into());
        self
    }
    /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p> 
    /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p> 
    /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK.</p>
    pub fn set_custom_attributes(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_custom_attributes(input);
        self
    }
    /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
    pub fn target_model(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_model(input.into());
        self
    }
    /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
    pub fn set_target_model(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_model(input);
        self
    }
    /// <p>Specify the production variant to send the inference request to when invoking an endpoint that is running two or more variants. Note that this parameter overrides the default behavior for the endpoint, which is to distribute the invocation traffic based on the variant weights.</p> 
    /// <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in production</a> </p>
    pub fn target_variant(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_variant(input.into());
        self
    }
    /// <p>Specify the production variant to send the inference request to when invoking an endpoint that is running two or more variants. Note that this parameter overrides the default behavior for the endpoint, which is to distribute the invocation traffic based on the variant weights.</p> 
    /// <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in production</a> </p>
    pub fn set_target_variant(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_variant(input);
        self
    }
    /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation, this parameter specifies the host name of the container to invoke.</p>
    pub fn target_container_hostname(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_container_hostname(input.into());
        self
    }
    /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation, this parameter specifies the host name of the container to invoke.</p>
    pub fn set_target_container_hostname(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_container_hostname(input);
        self
    }
    /// <p>If you provide a value, it is added to the captured data when you enable data capture on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture Data</a>.</p>
    pub fn inference_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.inference_id(input.into());
        self
    }
    /// <p>If you provide a value, it is added to the captured data when you enable data capture on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture Data</a>.</p>
    pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_inference_id(input);
        self
    }
    /// <p>An optional JMESPath expression used to override the <code>EnableExplanations</code> parameter of the <code>ClarifyExplainerConfig</code> API. See the <a href="https://docs.aws.amazon.com/clarify-online-explainability-create-endpoint.html#clarify-online-exaplainability-create-endpoint-enable">EnableExplanations</a> section in the developer guide for more information. </p>
    pub fn enable_explanations(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.enable_explanations(input.into());
        self
    }
    /// <p>An optional JMESPath expression used to override the <code>EnableExplanations</code> parameter of the <code>ClarifyExplainerConfig</code> API. See the <a href="https://docs.aws.amazon.com/clarify-online-explainability-create-endpoint.html#clarify-online-exaplainability-create-endpoint-enable">EnableExplanations</a> section in the developer guide for more information. </p>
    pub fn set_enable_explanations(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_enable_explanations(input);
        self
    }
}

/// Fluent builder constructing a request to `InvokeEndpointAsync`.
/// 
/// <p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint in an asynchronous manner.</p> 
/// <p>Inference requests sent to this API are enqueued for asynchronous processing. The processing of the inference request may or may not complete before the you receive a response from this API. The response from this API will not contain the result of the inference request but contain information about where you can locate it.</p> 
/// <p>Amazon SageMaker strips all <code>POST</code> headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax.</p> 
/// <p>Calls to <code>InvokeEndpointAsync</code> are authenticated by using Amazon Web Services Signature Version 4. For information, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests (Amazon Web Services Signature Version 4)</a> in the <i>Amazon S3 API Reference</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct InvokeEndpointAsync {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::invoke_endpoint_async_input::Builder
            }
impl InvokeEndpointAsync  {
    /// Creates a new `InvokeEndpointAsync`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::InvokeEndpointAsync, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::InvokeEndpointAsyncError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::InvokeEndpointAsyncOutput, aws_smithy_http::result::SdkError<crate::error::InvokeEndpointAsyncError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html"> <code>CreateEndpoint</code> </a> API.</p>
    pub fn endpoint_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.endpoint_name(input.into());
        self
    }
    /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateEndpoint.html"> <code>CreateEndpoint</code> </a> API.</p>
    pub fn set_endpoint_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_name(input);
        self
    }
    /// <p>The MIME type of the input data in the request body.</p>
    pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content_type(input.into());
        self
    }
    /// <p>The MIME type of the input data in the request body.</p>
    pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content_type(input);
        self
    }
    /// <p>The desired MIME type of the inference in the response.</p>
    pub fn accept(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.accept(input.into());
        self
    }
    /// <p>The desired MIME type of the inference in the response.</p>
    pub fn set_accept(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_accept(input);
        self
    }
    /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p> 
    /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID</code>: in your post-processing function. </p> 
    /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK. </p>
    pub fn custom_attributes(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.custom_attributes(input.into());
        self
    }
    /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p> 
    /// <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID</code>: in your post-processing function. </p> 
    /// <p>This feature is currently supported in the Amazon Web Services SDKs but not in the Amazon SageMaker Python SDK. </p>
    pub fn set_custom_attributes(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_custom_attributes(input);
        self
    }
    /// <p>The identifier for the inference request. Amazon SageMaker will generate an identifier for you if none is specified. </p>
    pub fn inference_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.inference_id(input.into());
        self
    }
    /// <p>The identifier for the inference request. Amazon SageMaker will generate an identifier for you if none is specified. </p>
    pub fn set_inference_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_inference_id(input);
        self
    }
    /// <p>The Amazon S3 URI where the inference request payload is stored.</p>
    pub fn input_location(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.input_location(input.into());
        self
    }
    /// <p>The Amazon S3 URI where the inference request payload is stored.</p>
    pub fn set_input_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_input_location(input);
        self
    }
    /// <p>Maximum age in seconds a request can be in the queue before it is marked as expired.</p>
    pub fn request_ttl_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.request_ttl_seconds(input);
        self
    }
    /// <p>Maximum age in seconds a request can be in the queue before it is marked as expired.</p>
    pub fn set_request_ttl_seconds(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_request_ttl_seconds(input);
        self
    }
}

