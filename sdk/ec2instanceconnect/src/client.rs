// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    pub(crate) client: aws_smithy_client::Client<C, M, R>,
    pub(crate) conf: crate::Config,
}

/// Client for AWS EC2 Instance Connect
///
/// Client for invoking operations on AWS EC2 Instance Connect. Each operation on AWS EC2 Instance Connect is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_ec2instanceconnect::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_ec2instanceconnect::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_ec2instanceconnect::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `SendSerialConsoleSSHPublicKey` operation.
    ///
    /// See [`SendSerialConsoleSSHPublicKey`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey) for more information about the
    /// operation and its arguments.
    pub fn send_serial_console_ssh_public_key(
        &self,
    ) -> fluent_builders::SendSerialConsoleSSHPublicKey<C, M, R> {
        fluent_builders::SendSerialConsoleSSHPublicKey::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `SendSSHPublicKey` operation.
    ///
    /// See [`SendSSHPublicKey`](crate::client::fluent_builders::SendSSHPublicKey) for more information about the
    /// operation and its arguments.
    pub fn send_ssh_public_key(&self) -> fluent_builders::SendSSHPublicKey<C, M, R> {
        fluent_builders::SendSSHPublicKey::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `SendSerialConsoleSSHPublicKey`.
    ///
    /// <p>Pushes an SSH public key to the specified EC2 instance. The key remains for 60 seconds, which gives you 60 seconds to establish a serial console connection to the instance using SSH. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-serial-console.html">EC2 Serial Console</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendSerialConsoleSSHPublicKey<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_serial_console_ssh_public_key_input::Builder,
    }
    impl<C, M, R> SendSerialConsoleSSHPublicKey<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `SendSerialConsoleSSHPublicKey`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendSerialConsoleSshPublicKeyOutput,
            aws_smithy_http::result::SdkError<crate::error::SendSerialConsoleSSHPublicKeyError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendSerialConsoleSshPublicKeyInputOperationOutputAlias,
                crate::output::SendSerialConsoleSshPublicKeyOutput,
                crate::error::SendSerialConsoleSSHPublicKeyError,
                crate::input::SendSerialConsoleSshPublicKeyInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(input.into());
            self
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
        /// <p>Default: 0</p>
        pub fn serial_port(mut self, input: i32) -> Self {
            self.inner = self.inner.serial_port(input);
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
        /// <p>Default: 0</p>
        pub fn set_serial_port(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_serial_port(input);
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.ssh_public_key(input.into());
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_ssh_public_key(input);
            self
        }
    }
    /// Fluent builder constructing a request to `SendSSHPublicKey`.
    ///
    /// <p>Pushes an SSH public key to the specified EC2 instance for use by the specified user. The key remains for 60 seconds. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Connect-using-EC2-Instance-Connect.html">Connect to your Linux instance using EC2 Instance Connect</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendSSHPublicKey<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::send_ssh_public_key_input::Builder,
    }
    impl<C, M, R> SendSSHPublicKey<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `SendSSHPublicKey`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendSshPublicKeyOutput,
            aws_smithy_http::result::SdkError<crate::error::SendSSHPublicKeyError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::SendSshPublicKeyInputOperationOutputAlias,
                crate::output::SendSshPublicKeyOutput,
                crate::error::SendSSHPublicKeyError,
                crate::input::SendSshPublicKeyInputOperationRetryAlias,
            >,
        {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(input.into());
            self
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn instance_os_user(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_os_user(input.into());
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn set_instance_os_user(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_instance_os_user(input);
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.ssh_public_key(input.into());
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_ssh_public_key(input);
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone(input.into());
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn set_availability_zone(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_availability_zone(input);
            self
        }
    }
}

impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
