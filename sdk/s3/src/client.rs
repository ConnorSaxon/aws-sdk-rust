// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                    pub use aws_smithy_client::Builder;
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                    pub(crate) conf: crate::Config,
                }

                /// Client for Amazon Simple Storage Service
                    ///
                    /// Client for invoking operations on Amazon Simple Storage Service. Each operation on Amazon Simple Storage Service is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_s3::Client::new(&shared_config);
                        ///     // invoke an operation
                        ///     /* let rsp = client
                        ///         .<operation_name>().
                        ///         .<param>("some value")
                        ///         .send().await; */
                        /// # }
                        /// ```
                        /// **Constructing a client with custom configuration**
                        /// ```rust,no_run
                        /// use aws_config::retry::RetryConfig;
                        /// # async fn docs() {
                        /// let shared_config = aws_config::load_from_env().await;
                        /// let config = aws_sdk_s3::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_s3::Client::from_conf(config);
                        /// # }
                #[derive(std::fmt::Debug)]
                pub struct Client {
                    handle: std::sync::Arc<Handle>
                }

                impl std::clone::Clone for Client {
                    fn clone(&self) -> Self {
                        Self { handle: self.handle.clone() }
                    }
                }

                impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                    fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                        Self::with_config(client, crate::Config::builder().build())
                    }
                }

                impl Client {
                    /// Creates a client with the given service configuration.
                    pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                        Self {
                            handle: std::sync::Arc::new(Handle {
                                client,
                                conf,
                            })
                        }
                    }

                    /// Returns the client's configuration.
                    pub fn conf(&self) -> &crate::Config {
                        &self.handle.conf
                    }
                }

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
    
                        let connector = conf.http_connector().and_then(|c| {
                            let timeout_config = conf
                                .timeout_config()
                                .cloned()
                                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                            let connector_settings = aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                                &timeout_config,
                            );
                            c.connector(&connector_settings, conf.sleep_impl())
                        });
    
                        let builder = aws_smithy_client::Builder::new();
    
                        let builder = match connector {
                            // Use provided connector
                            Some(c) => builder.connector(c),
                            None =>{
                                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                                {
                                    // Use default connector based on enabled features
                                    builder.dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                                }
                                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                                {
                                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                                }
                            }
                        };
                        let mut builder = builder
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

mod abort_multipart_upload;

mod complete_multipart_upload;

mod copy_object;

mod create_bucket;

mod create_multipart_upload;

mod delete_bucket;

mod delete_bucket_analytics_configuration;

mod delete_bucket_cors;

mod delete_bucket_encryption;

mod delete_bucket_intelligent_tiering_configuration;

mod delete_bucket_inventory_configuration;

mod delete_bucket_lifecycle;

mod delete_bucket_metrics_configuration;

mod delete_bucket_ownership_controls;

mod delete_bucket_policy;

mod delete_bucket_replication;

mod delete_bucket_tagging;

mod delete_bucket_website;

mod delete_object;

mod delete_object_tagging;

mod delete_objects;

mod delete_public_access_block;

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

mod get_bucket_accelerate_configuration;

mod get_bucket_acl;

mod get_bucket_analytics_configuration;

mod get_bucket_cors;

mod get_bucket_encryption;

mod get_bucket_intelligent_tiering_configuration;

mod get_bucket_inventory_configuration;

mod get_bucket_lifecycle_configuration;

mod get_bucket_location;

mod get_bucket_logging;

mod get_bucket_metrics_configuration;

mod get_bucket_notification_configuration;

mod get_bucket_ownership_controls;

mod get_bucket_policy;

mod get_bucket_policy_status;

mod get_bucket_replication;

mod get_bucket_request_payment;

mod get_bucket_tagging;

mod get_bucket_versioning;

mod get_bucket_website;

mod get_object;

mod get_object_acl;

mod get_object_attributes;

mod get_object_legal_hold;

mod get_object_lock_configuration;

mod get_object_retention;

mod get_object_tagging;

mod get_object_torrent;

mod get_public_access_block;

mod head_bucket;

mod head_object;

mod list_bucket_analytics_configurations;

mod list_bucket_intelligent_tiering_configurations;

mod list_bucket_inventory_configurations;

mod list_bucket_metrics_configurations;

mod list_buckets;

mod list_multipart_uploads;

mod list_object_versions;

mod list_objects;

mod list_objects_v2;

mod list_parts;

mod put_bucket_accelerate_configuration;

mod put_bucket_acl;

mod put_bucket_analytics_configuration;

mod put_bucket_cors;

mod put_bucket_encryption;

mod put_bucket_intelligent_tiering_configuration;

mod put_bucket_inventory_configuration;

mod put_bucket_lifecycle_configuration;

mod put_bucket_logging;

mod put_bucket_metrics_configuration;

mod put_bucket_notification_configuration;

mod put_bucket_ownership_controls;

mod put_bucket_policy;

mod put_bucket_replication;

mod put_bucket_request_payment;

mod put_bucket_tagging;

mod put_bucket_versioning;

mod put_bucket_website;

mod put_object;

mod put_object_acl;

mod put_object_legal_hold;

mod put_object_lock_configuration;

mod put_object_retention;

mod put_object_tagging;

mod put_public_access_block;

mod restore_object;

mod select_object_content;

mod upload_part;

mod upload_part_copy;

mod write_get_object_response;

