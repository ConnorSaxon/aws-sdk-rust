// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                    pub use aws_smithy_client::Builder;
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                    pub(crate) conf: crate::Config,
                }

                /// Client for Amazon WorkSpaces
                    ///
                    /// Client for invoking operations on Amazon WorkSpaces. Each operation on Amazon WorkSpaces is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_workspaces::Client::new(&shared_config);
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
                        /// let config = aws_sdk_workspaces::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_workspaces::Client::from_conf(config);
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

mod associate_connection_alias;

mod associate_ip_groups;

mod authorize_ip_rules;

mod copy_workspace_image;

mod create_connect_client_add_in;

mod create_connection_alias;

mod create_ip_group;

mod create_standby_workspaces;

mod create_tags;

mod create_updated_workspace_image;

mod create_workspace_bundle;

mod create_workspace_image;

mod create_workspaces;

mod delete_client_branding;

mod delete_connect_client_add_in;

mod delete_connection_alias;

mod delete_ip_group;

mod delete_tags;

mod delete_workspace_bundle;

mod delete_workspace_image;

mod deregister_workspace_directory;

mod describe_account;

mod describe_account_modifications;

mod describe_client_branding;

mod describe_client_properties;

mod describe_connect_client_add_ins;

mod describe_connection_alias_permissions;

mod describe_connection_aliases;

mod describe_ip_groups;

mod describe_tags;

mod describe_workspace_bundles;

mod describe_workspace_directories;

mod describe_workspace_image_permissions;

mod describe_workspace_images;

mod describe_workspace_snapshots;

mod describe_workspaces;

mod describe_workspaces_connection_status;

mod disassociate_connection_alias;

mod disassociate_ip_groups;

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

mod import_client_branding;

mod import_workspace_image;

mod list_available_management_cidr_ranges;

mod migrate_workspace;

mod modify_account;

mod modify_certificate_based_auth_properties;

mod modify_client_properties;

mod modify_saml_properties;

mod modify_selfservice_permissions;

mod modify_workspace_access_properties;

mod modify_workspace_creation_properties;

mod modify_workspace_properties;

mod modify_workspace_state;

mod reboot_workspaces;

mod rebuild_workspaces;

mod register_workspace_directory;

mod restore_workspace;

mod revoke_ip_rules;

mod start_workspaces;

mod stop_workspaces;

mod terminate_workspaces;

mod update_connect_client_add_in;

mod update_connection_alias_permission;

mod update_rules_of_ip_group;

mod update_workspace_bundle;

mod update_workspace_image_permission;

