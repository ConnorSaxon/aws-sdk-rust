// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_signaling_channel;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_stream;

pub(crate) mod shape_delete_signaling_channel;

pub(crate) mod shape_delete_stream;

pub(crate) mod shape_describe_edge_configuration;

pub(crate) mod shape_describe_image_generation_configuration;

pub(crate) mod shape_describe_mapped_resource_configuration;

pub(crate) mod shape_describe_media_storage_configuration;

pub(crate) mod shape_describe_notification_configuration;

pub(crate) mod shape_describe_signaling_channel;

pub(crate) mod shape_describe_stream;

pub(crate) mod shape_get_data_endpoint;

pub(crate) mod shape_get_signaling_channel_endpoint;

pub(crate) mod shape_list_signaling_channels;

pub(crate) mod shape_list_streams;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_tags_for_stream;

pub(crate) mod shape_start_edge_configuration_update;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_tag_stream;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_untag_stream;

pub(crate) mod shape_update_data_retention;

pub(crate) mod shape_update_image_generation_configuration;

pub(crate) mod shape_update_media_storage_configuration;

pub(crate) mod shape_update_notification_configuration;

pub(crate) mod shape_update_signaling_channel;

pub(crate) mod shape_update_stream;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_account_channel_limit_exceeded_exception;

pub(crate) mod shape_account_stream_limit_exceeded_exception;

pub(crate) mod shape_client_limit_exceeded_exception;

pub(crate) mod shape_create_signaling_channel_input;

pub(crate) mod shape_create_stream_input;

pub(crate) mod shape_delete_signaling_channel_input;

pub(crate) mod shape_delete_stream_input;

pub(crate) mod shape_describe_edge_configuration_input;

pub(crate) mod shape_describe_image_generation_configuration_input;

pub(crate) mod shape_describe_mapped_resource_configuration_input;

pub(crate) mod shape_describe_media_storage_configuration_input;

pub(crate) mod shape_describe_notification_configuration_input;

pub(crate) mod shape_describe_signaling_channel_input;

pub(crate) mod shape_describe_stream_input;

pub(crate) mod shape_device_stream_limit_exceeded_exception;

pub(crate) mod shape_get_data_endpoint_input;

pub(crate) mod shape_get_signaling_channel_endpoint_input;

pub(crate) mod shape_invalid_argument_exception;

pub(crate) mod shape_invalid_device_exception;

pub(crate) mod shape_invalid_resource_format_exception;

pub(crate) mod shape_list_signaling_channels_input;

pub(crate) mod shape_list_streams_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_tags_for_stream_input;

pub(crate) mod shape_no_data_retention_exception;

pub(crate) mod shape_not_authorized_exception;

pub(crate) mod shape_resource_in_use_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_edge_configuration_update_input;

pub(crate) mod shape_stream_edge_configuration_not_found_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_tag_stream_input;

pub(crate) mod shape_tags_per_resource_exceeded_limit_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_untag_stream_input;

pub(crate) mod shape_update_data_retention_input;

pub(crate) mod shape_update_image_generation_configuration_input;

pub(crate) mod shape_update_media_storage_configuration_input;

pub(crate) mod shape_update_notification_configuration_input;

pub(crate) mod shape_update_signaling_channel_input;

pub(crate) mod shape_update_stream_input;

pub(crate) mod shape_version_mismatch_exception;

pub(crate) mod shape_channel_info;

pub(crate) mod shape_channel_info_list;

pub(crate) mod shape_channel_name_condition;

pub(crate) mod shape_edge_config;

pub(crate) mod shape_image_generation_configuration;

pub(crate) mod shape_mapped_resource_configuration_list;

pub(crate) mod shape_media_storage_configuration;

pub(crate) mod shape_notification_configuration;

pub(crate) mod shape_resource_endpoint_list;

pub(crate) mod shape_resource_tags;

pub(crate) mod shape_single_master_channel_endpoint_configuration;

pub(crate) mod shape_single_master_configuration;

pub(crate) mod shape_stream_info;

pub(crate) mod shape_stream_info_list;

pub(crate) mod shape_stream_name_condition;

pub(crate) mod shape_tag;

pub(crate) mod shape_deletion_config;

pub(crate) mod shape_format_config;

pub(crate) mod shape_image_generation_destination_config;

pub(crate) mod shape_mapped_resource_configuration_list_item;

pub(crate) mod shape_notification_destination_config;

pub(crate) mod shape_recorder_config;

pub(crate) mod shape_resource_endpoint_list_item;

pub(crate) mod shape_uploader_config;

pub(crate) mod shape_local_size_config;

pub(crate) mod shape_media_source_config;

pub(crate) mod shape_schedule_config;

