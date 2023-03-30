// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_associate_attribute_group;

pub(crate) mod shape_associate_resource;

pub(crate) mod shape_create_application;

pub(crate) mod shape_create_attribute_group;

pub(crate) mod shape_delete_application;

pub(crate) mod shape_delete_attribute_group;

pub(crate) mod shape_disassociate_attribute_group;

pub(crate) mod shape_disassociate_resource;

pub(crate) mod shape_get_application;

pub(crate) mod shape_get_associated_resource;

pub(crate) mod shape_get_attribute_group;

pub(crate) mod shape_get_configuration;

pub(crate) mod shape_list_applications;

pub(crate) mod shape_list_associated_attribute_groups;

pub(crate) mod shape_list_associated_resources;

pub(crate) mod shape_list_attribute_groups;

pub(crate) mod shape_list_attribute_groups_for_application;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_configuration;

pub(crate) mod shape_sync_resource;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_application;

pub(crate) mod shape_update_attribute_group;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_application_input;

pub(crate) mod shape_create_attribute_group_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_put_configuration_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_update_application_input;

pub(crate) mod shape_update_attribute_group_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_app_registry_configuration;

pub(crate) mod shape_application;

pub(crate) mod shape_application_summaries;

pub(crate) mod shape_application_summary;

pub(crate) mod shape_attribute_group;

pub(crate) mod shape_attribute_group_details_list;

pub(crate) mod shape_attribute_group_ids;

pub(crate) mod shape_attribute_group_summaries;

pub(crate) mod shape_attribute_group_summary;

pub(crate) mod shape_integrations;

pub(crate) mod shape_resource;

pub(crate) mod shape_resources;

pub(crate) mod shape_tags;

pub(crate) mod shape_attribute_group_details;

pub(crate) mod shape_resource_group;

pub(crate) mod shape_resource_info;

pub(crate) mod shape_resource_integrations;

pub(crate) mod shape_tag_query_configuration;

pub(crate) mod shape_resource_details;

