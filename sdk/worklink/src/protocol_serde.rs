// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_associate_domain;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_associate_website_authorization_provider;

pub(crate) mod shape_associate_website_certificate_authority;

pub(crate) mod shape_create_fleet;

pub(crate) mod shape_delete_fleet;

pub(crate) mod shape_describe_audit_stream_configuration;

pub(crate) mod shape_describe_company_network_configuration;

pub(crate) mod shape_describe_device;

pub(crate) mod shape_describe_device_policy_configuration;

pub(crate) mod shape_describe_domain;

pub(crate) mod shape_describe_fleet_metadata;

pub(crate) mod shape_describe_identity_provider_configuration;

pub(crate) mod shape_describe_website_certificate_authority;

pub(crate) mod shape_disassociate_domain;

pub(crate) mod shape_disassociate_website_authorization_provider;

pub(crate) mod shape_disassociate_website_certificate_authority;

pub(crate) mod shape_list_devices;

pub(crate) mod shape_list_domains;

pub(crate) mod shape_list_fleets;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_website_authorization_providers;

pub(crate) mod shape_list_website_certificate_authorities;

pub(crate) mod shape_restore_domain_access;

pub(crate) mod shape_revoke_domain_access;

pub(crate) mod shape_sign_out_user;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_audit_stream_configuration;

pub(crate) mod shape_update_company_network_configuration;

pub(crate) mod shape_update_device_policy_configuration;

pub(crate) mod shape_update_domain_metadata;

pub(crate) mod shape_update_fleet_metadata;

pub(crate) mod shape_update_identity_provider_configuration;

pub(crate) mod shape_associate_domain_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_associate_website_authorization_provider_input;

pub(crate) mod shape_associate_website_certificate_authority_input;

pub(crate) mod shape_create_fleet_input;

pub(crate) mod shape_delete_fleet_input;

pub(crate) mod shape_describe_audit_stream_configuration_input;

pub(crate) mod shape_describe_company_network_configuration_input;

pub(crate) mod shape_describe_device_input;

pub(crate) mod shape_describe_device_policy_configuration_input;

pub(crate) mod shape_describe_domain_input;

pub(crate) mod shape_describe_fleet_metadata_input;

pub(crate) mod shape_describe_identity_provider_configuration_input;

pub(crate) mod shape_describe_website_certificate_authority_input;

pub(crate) mod shape_disassociate_domain_input;

pub(crate) mod shape_disassociate_website_authorization_provider_input;

pub(crate) mod shape_disassociate_website_certificate_authority_input;

pub(crate) mod shape_internal_server_error_exception;

pub(crate) mod shape_invalid_request_exception;

pub(crate) mod shape_list_devices_input;

pub(crate) mod shape_list_domains_input;

pub(crate) mod shape_list_fleets_input;

pub(crate) mod shape_list_website_authorization_providers_input;

pub(crate) mod shape_list_website_certificate_authorities_input;

pub(crate) mod shape_resource_already_exists_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_restore_domain_access_input;

pub(crate) mod shape_revoke_domain_access_input;

pub(crate) mod shape_sign_out_user_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_unauthorized_exception;

pub(crate) mod shape_update_audit_stream_configuration_input;

pub(crate) mod shape_update_company_network_configuration_input;

pub(crate) mod shape_update_device_policy_configuration_input;

pub(crate) mod shape_update_domain_metadata_input;

pub(crate) mod shape_update_fleet_metadata_input;

pub(crate) mod shape_update_identity_provider_configuration_input;

pub(crate) mod shape_device_summary_list;

pub(crate) mod shape_domain_summary_list;

pub(crate) mod shape_fleet_summary_list;

pub(crate) mod shape_security_group_ids;

pub(crate) mod shape_subnet_ids;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_website_authorization_providers_summary_list;

pub(crate) mod shape_website_ca_summary_list;

pub(crate) mod shape_device_summary;

pub(crate) mod shape_domain_summary;

pub(crate) mod shape_fleet_summary;

pub(crate) mod shape_website_authorization_provider_summary;

pub(crate) mod shape_website_ca_summary;

