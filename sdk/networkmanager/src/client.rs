// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                    pub use aws_smithy_client::Builder;
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                    pub(crate) conf: crate::Config,
                }

                /// Client for AWS Network Manager
                    ///
                    /// Client for invoking operations on AWS Network Manager. Each operation on AWS Network Manager is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_networkmanager::Client::new(&shared_config);
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
                        /// let config = aws_sdk_networkmanager::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_networkmanager::Client::from_conf(config);
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

mod accept_attachment;

mod associate_connect_peer;

mod associate_customer_gateway;

mod associate_link;

mod associate_transit_gateway_connect_peer;

mod create_connect_attachment;

mod create_connect_peer;

mod create_connection;

mod create_core_network;

mod create_device;

mod create_global_network;

mod create_link;

mod create_site;

mod create_site_to_site_vpn_attachment;

mod create_transit_gateway_peering;

mod create_transit_gateway_route_table_attachment;

mod create_vpc_attachment;

mod delete_attachment;

mod delete_connect_peer;

mod delete_connection;

mod delete_core_network;

mod delete_core_network_policy_version;

mod delete_device;

mod delete_global_network;

mod delete_link;

mod delete_peering;

mod delete_resource_policy;

mod delete_site;

mod deregister_transit_gateway;

mod describe_global_networks;

mod disassociate_connect_peer;

mod disassociate_customer_gateway;

mod disassociate_link;

mod disassociate_transit_gateway_connect_peer;

mod execute_core_network_change_set;

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

mod get_connect_attachment;

mod get_connect_peer;

mod get_connect_peer_associations;

mod get_connections;

mod get_core_network;

mod get_core_network_change_events;

mod get_core_network_change_set;

mod get_core_network_policy;

mod get_customer_gateway_associations;

mod get_devices;

mod get_link_associations;

mod get_links;

mod get_network_resource_counts;

mod get_network_resource_relationships;

mod get_network_resources;

mod get_network_routes;

mod get_network_telemetry;

mod get_resource_policy;

mod get_route_analysis;

mod get_site_to_site_vpn_attachment;

mod get_sites;

mod get_transit_gateway_connect_peer_associations;

mod get_transit_gateway_peering;

mod get_transit_gateway_registrations;

mod get_transit_gateway_route_table_attachment;

mod get_vpc_attachment;

mod list_attachments;

mod list_connect_peers;

mod list_core_network_policy_versions;

mod list_core_networks;

mod list_organization_service_access_status;

mod list_peerings;

mod list_tags_for_resource;

mod put_core_network_policy;

mod put_resource_policy;

mod register_transit_gateway;

mod reject_attachment;

mod restore_core_network_policy_version;

mod start_organization_service_access_update;

mod start_route_analysis;

mod tag_resource;

mod untag_resource;

mod update_connection;

mod update_core_network;

mod update_device;

mod update_global_network;

mod update_link;

mod update_network_resource_metadata;

mod update_site;

mod update_vpc_attachment;

