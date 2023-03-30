// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_associate_browser_settings;

pub(crate) mod shape_associate_network_settings;

pub(crate) mod shape_associate_trust_store;

pub(crate) mod shape_associate_user_access_logging_settings;

pub(crate) mod shape_associate_user_settings;

pub(crate) mod shape_create_browser_settings;

pub(crate) mod shape_create_identity_provider;

pub(crate) mod shape_create_network_settings;

pub(crate) mod shape_create_portal;

pub(crate) mod shape_create_trust_store;

pub(crate) mod shape_create_user_access_logging_settings;

pub(crate) mod shape_create_user_settings;

pub(crate) mod shape_delete_browser_settings;

pub(crate) mod shape_delete_identity_provider;

pub(crate) mod shape_delete_network_settings;

pub(crate) mod shape_delete_portal;

pub(crate) mod shape_delete_trust_store;

pub(crate) mod shape_delete_user_access_logging_settings;

pub(crate) mod shape_delete_user_settings;

pub(crate) mod shape_disassociate_browser_settings;

pub(crate) mod shape_disassociate_network_settings;

pub(crate) mod shape_disassociate_trust_store;

pub(crate) mod shape_disassociate_user_access_logging_settings;

pub(crate) mod shape_disassociate_user_settings;

pub(crate) mod shape_get_browser_settings;

pub(crate) mod shape_get_identity_provider;

pub(crate) mod shape_get_network_settings;

pub(crate) mod shape_get_portal;

pub(crate) mod shape_get_portal_service_provider_metadata;

pub(crate) mod shape_get_trust_store;

pub(crate) mod shape_get_trust_store_certificate;

pub(crate) mod shape_get_user_access_logging_settings;

pub(crate) mod shape_get_user_settings;

pub(crate) mod shape_list_browser_settings;

pub(crate) mod shape_list_identity_providers;

pub(crate) mod shape_list_network_settings;

pub(crate) mod shape_list_portals;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_trust_store_certificates;

pub(crate) mod shape_list_trust_stores;

pub(crate) mod shape_list_user_access_logging_settings;

pub(crate) mod shape_list_user_settings;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_browser_settings;

pub(crate) mod shape_update_identity_provider;

pub(crate) mod shape_update_network_settings;

pub(crate) mod shape_update_portal;

pub(crate) mod shape_update_trust_store;

pub(crate) mod shape_update_user_access_logging_settings;

pub(crate) mod shape_update_user_settings;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_browser_settings_input;

pub(crate) mod shape_create_identity_provider_input;

pub(crate) mod shape_create_network_settings_input;

pub(crate) mod shape_create_portal_input;

pub(crate) mod shape_create_trust_store_input;

pub(crate) mod shape_create_user_access_logging_settings_input;

pub(crate) mod shape_create_user_settings_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_update_browser_settings_input;

pub(crate) mod shape_update_identity_provider_input;

pub(crate) mod shape_update_network_settings_input;

pub(crate) mod shape_update_portal_input;

pub(crate) mod shape_update_trust_store_input;

pub(crate) mod shape_update_user_access_logging_settings_input;

pub(crate) mod shape_update_user_settings_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_browser_settings;

pub(crate) mod shape_browser_settings_list;

pub(crate) mod shape_certificate;

pub(crate) mod shape_certificate_summary_list;

pub(crate) mod shape_identity_provider;

pub(crate) mod shape_identity_provider_list;

pub(crate) mod shape_network_settings;

pub(crate) mod shape_network_settings_list;

pub(crate) mod shape_portal;

pub(crate) mod shape_portal_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_trust_store;

pub(crate) mod shape_trust_store_summary_list;

pub(crate) mod shape_user_access_logging_settings;

pub(crate) mod shape_user_access_logging_settings_list;

pub(crate) mod shape_user_settings;

pub(crate) mod shape_user_settings_list;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_arn_list;

pub(crate) mod shape_browser_settings_summary;

pub(crate) mod shape_certificate_summary;

pub(crate) mod shape_identity_provider_details;

pub(crate) mod shape_identity_provider_summary;

pub(crate) mod shape_network_settings_summary;

pub(crate) mod shape_portal_summary;

pub(crate) mod shape_security_group_id_list;

pub(crate) mod shape_subnet_id_list;

pub(crate) mod shape_trust_store_summary;

pub(crate) mod shape_user_access_logging_settings_summary;

pub(crate) mod shape_user_settings_summary;

pub(crate) mod shape_validation_exception_field;

