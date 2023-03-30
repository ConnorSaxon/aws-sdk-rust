// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_batch_create_rum_metric_definitions;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_batch_delete_rum_metric_definitions;

pub(crate) mod shape_batch_get_rum_metric_definitions;

pub(crate) mod shape_create_app_monitor;

pub(crate) mod shape_delete_app_monitor;

pub(crate) mod shape_delete_rum_metrics_destination;

pub(crate) mod shape_get_app_monitor;

pub(crate) mod shape_get_app_monitor_data;

pub(crate) mod shape_list_app_monitors;

pub(crate) mod shape_list_rum_metrics_destinations;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_rum_events;

pub(crate) mod shape_put_rum_metrics_destination;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_app_monitor;

pub(crate) mod shape_update_rum_metric_definition;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_batch_create_rum_metric_definitions_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_app_monitor_input;

pub(crate) mod shape_get_app_monitor_data_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_put_rum_events_input;

pub(crate) mod shape_put_rum_metrics_destination_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_app_monitor_input;

pub(crate) mod shape_update_rum_metric_definition_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_app_monitor;

pub(crate) mod shape_app_monitor_configuration;

pub(crate) mod shape_app_monitor_details;

pub(crate) mod shape_app_monitor_summary_list;

pub(crate) mod shape_batch_create_rum_metric_definitions_errors;

pub(crate) mod shape_batch_delete_rum_metric_definitions_errors;

pub(crate) mod shape_custom_events;

pub(crate) mod shape_event_data_list;

pub(crate) mod shape_metric_definition_ids;

pub(crate) mod shape_metric_definition_request;

pub(crate) mod shape_metric_definitions;

pub(crate) mod shape_metric_destination_summary_list;

pub(crate) mod shape_query_filter;

pub(crate) mod shape_rum_event;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_time_range;

pub(crate) mod shape_user_details;

pub(crate) mod shape_app_monitor_summary;

pub(crate) mod shape_batch_create_rum_metric_definitions_error;

pub(crate) mod shape_batch_delete_rum_metric_definitions_error;

pub(crate) mod shape_data_storage;

pub(crate) mod shape_metric_definition;

pub(crate) mod shape_metric_destination_summary;

pub(crate) mod shape_cw_log;

pub(crate) mod shape_dimension_keys_map;

pub(crate) mod shape_favorite_pages;

pub(crate) mod shape_pages;

pub(crate) mod shape_telemetries;

