// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_lifecycle_policy;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_delete_lifecycle_policy;

pub(crate) mod shape_get_lifecycle_policies;

pub(crate) mod shape_get_lifecycle_policy;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_lifecycle_policy;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_create_lifecycle_policy_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_invalid_request_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_update_lifecycle_policy_input;

pub(crate) mod shape_lifecycle_policy;

pub(crate) mod shape_lifecycle_policy_summary_list;

pub(crate) mod shape_parameter_list;

pub(crate) mod shape_policy_details;

pub(crate) mod shape_policy_id_list;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_action;

pub(crate) mod shape_event_source;

pub(crate) mod shape_lifecycle_policy_summary;

pub(crate) mod shape_parameters;

pub(crate) mod shape_schedule;

pub(crate) mod shape_tag;

pub(crate) mod shape_action_list;

pub(crate) mod shape_archive_rule;

pub(crate) mod shape_create_rule;

pub(crate) mod shape_cross_region_copy_action;

pub(crate) mod shape_cross_region_copy_rule;

pub(crate) mod shape_deprecate_rule;

pub(crate) mod shape_event_parameters;

pub(crate) mod shape_fast_restore_rule;

pub(crate) mod shape_resource_location_list;

pub(crate) mod shape_resource_type_values_list;

pub(crate) mod shape_retain_rule;

pub(crate) mod shape_schedule_list;

pub(crate) mod shape_share_rule;

pub(crate) mod shape_target_tag_list;

pub(crate) mod shape_archive_retain_rule;

pub(crate) mod shape_cross_region_copy_deprecate_rule;

pub(crate) mod shape_cross_region_copy_retain_rule;

pub(crate) mod shape_encryption_configuration;

pub(crate) mod shape_exclude_data_volume_tag_list;

pub(crate) mod shape_cross_region_copy_action_list;

pub(crate) mod shape_cross_region_copy_rules;

pub(crate) mod shape_retention_archive_tier;

pub(crate) mod shape_share_rules;

pub(crate) mod shape_snapshot_owner_list;

pub(crate) mod shape_tags_to_add_list;

pub(crate) mod shape_variable_tags_list;

pub(crate) mod shape_availability_zone_list;

pub(crate) mod shape_times_list;

pub(crate) mod shape_share_target_account_list;

