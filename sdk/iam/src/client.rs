// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                    pub use aws_smithy_client::Builder;
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                    pub(crate) conf: crate::Config,
                }

                /// Client for AWS Identity and Access Management
                    ///
                    /// Client for invoking operations on AWS Identity and Access Management. Each operation on AWS Identity and Access Management is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_iam::Client::new(&shared_config);
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
                        /// let config = aws_sdk_iam::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_iam::Client::from_conf(config);
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

mod add_client_id_to_open_id_connect_provider;

mod add_role_to_instance_profile;

mod add_user_to_group;

mod attach_group_policy;

mod attach_role_policy;

mod attach_user_policy;

mod change_password;

mod create_access_key;

mod create_account_alias;

mod create_group;

mod create_instance_profile;

mod create_login_profile;

mod create_open_id_connect_provider;

mod create_policy;

mod create_policy_version;

mod create_role;

mod create_saml_provider;

mod create_service_linked_role;

mod create_service_specific_credential;

mod create_user;

mod create_virtual_mfa_device;

mod deactivate_mfa_device;

mod delete_access_key;

mod delete_account_alias;

mod delete_account_password_policy;

mod delete_group;

mod delete_group_policy;

mod delete_instance_profile;

mod delete_login_profile;

mod delete_open_id_connect_provider;

mod delete_policy;

mod delete_policy_version;

mod delete_role;

mod delete_role_permissions_boundary;

mod delete_role_policy;

mod delete_saml_provider;

mod delete_server_certificate;

mod delete_service_linked_role;

mod delete_service_specific_credential;

mod delete_signing_certificate;

mod delete_ssh_public_key;

mod delete_user;

mod delete_user_permissions_boundary;

mod delete_user_policy;

mod delete_virtual_mfa_device;

mod detach_group_policy;

mod detach_role_policy;

mod detach_user_policy;

mod enable_mfa_device;

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

mod generate_credential_report;

mod generate_organizations_access_report;

mod generate_service_last_accessed_details;

mod get_access_key_last_used;

mod get_account_authorization_details;

mod get_account_password_policy;

mod get_account_summary;

mod get_context_keys_for_custom_policy;

mod get_context_keys_for_principal_policy;

mod get_credential_report;

mod get_group;

mod get_group_policy;

mod get_instance_profile;

mod get_login_profile;

mod get_open_id_connect_provider;

mod get_organizations_access_report;

mod get_policy;

mod get_policy_version;

mod get_role;

mod get_role_policy;

mod get_saml_provider;

mod get_server_certificate;

mod get_service_last_accessed_details;

mod get_service_last_accessed_details_with_entities;

mod get_service_linked_role_deletion_status;

mod get_ssh_public_key;

mod get_user;

mod get_user_policy;

mod list_access_keys;

mod list_account_aliases;

mod list_attached_group_policies;

mod list_attached_role_policies;

mod list_attached_user_policies;

mod list_entities_for_policy;

mod list_group_policies;

mod list_groups;

mod list_groups_for_user;

mod list_instance_profile_tags;

mod list_instance_profiles;

mod list_instance_profiles_for_role;

mod list_mfa_device_tags;

mod list_mfa_devices;

mod list_open_id_connect_provider_tags;

mod list_open_id_connect_providers;

mod list_policies;

mod list_policies_granting_service_access;

mod list_policy_tags;

mod list_policy_versions;

mod list_role_policies;

mod list_role_tags;

mod list_roles;

mod list_saml_provider_tags;

mod list_saml_providers;

mod list_server_certificate_tags;

mod list_server_certificates;

mod list_service_specific_credentials;

mod list_signing_certificates;

mod list_ssh_public_keys;

mod list_user_policies;

mod list_user_tags;

mod list_users;

mod list_virtual_mfa_devices;

mod put_group_policy;

mod put_role_permissions_boundary;

mod put_role_policy;

mod put_user_permissions_boundary;

mod put_user_policy;

mod remove_client_id_from_open_id_connect_provider;

mod remove_role_from_instance_profile;

mod remove_user_from_group;

mod reset_service_specific_credential;

mod resync_mfa_device;

mod set_default_policy_version;

mod set_security_token_service_preferences;

mod simulate_custom_policy;

mod simulate_principal_policy;

mod tag_instance_profile;

mod tag_mfa_device;

mod tag_open_id_connect_provider;

mod tag_policy;

mod tag_role;

mod tag_saml_provider;

mod tag_server_certificate;

mod tag_user;

mod untag_instance_profile;

mod untag_mfa_device;

mod untag_open_id_connect_provider;

mod untag_policy;

mod untag_role;

mod untag_saml_provider;

mod untag_server_certificate;

mod untag_user;

mod update_access_key;

mod update_account_password_policy;

mod update_assume_role_policy;

mod update_group;

mod update_login_profile;

mod update_open_id_connect_provider_thumbprint;

mod update_role;

mod update_role_description;

mod update_saml_provider;

mod update_server_certificate;

mod update_service_specific_credential;

mod update_signing_certificate;

mod update_ssh_public_key;

mod update_user;

mod upload_server_certificate;

mod upload_signing_certificate;

mod upload_ssh_public_key;

