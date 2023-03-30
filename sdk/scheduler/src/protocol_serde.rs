// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_schedule;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_schedule_group;

pub(crate) mod shape_delete_schedule;

pub(crate) mod shape_delete_schedule_group;

pub(crate) mod shape_get_schedule;

pub(crate) mod shape_get_schedule_group;

pub(crate) mod shape_list_schedule_groups;

pub(crate) mod shape_list_schedules;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_schedule;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_schedule_group_input;

pub(crate) mod shape_create_schedule_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_schedule_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_flexible_time_window;

pub(crate) mod shape_schedule_group_list;

pub(crate) mod shape_schedule_list;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_target;

pub(crate) mod shape_dead_letter_config;

pub(crate) mod shape_ecs_parameters;

pub(crate) mod shape_event_bridge_parameters;

pub(crate) mod shape_kinesis_parameters;

pub(crate) mod shape_retry_policy;

pub(crate) mod shape_sage_maker_pipeline_parameters;

pub(crate) mod shape_schedule_group_summary;

pub(crate) mod shape_schedule_summary;

pub(crate) mod shape_sqs_parameters;

pub(crate) mod shape_capacity_provider_strategy;

pub(crate) mod shape_capacity_provider_strategy_item;

pub(crate) mod shape_network_configuration;

pub(crate) mod shape_placement_constraint;

pub(crate) mod shape_placement_constraints;

pub(crate) mod shape_placement_strategies;

pub(crate) mod shape_placement_strategy;

pub(crate) mod shape_sage_maker_pipeline_parameter;

pub(crate) mod shape_sage_maker_pipeline_parameter_list;

pub(crate) mod shape_tags;

pub(crate) mod shape_target_summary;

pub(crate) mod shape_aws_vpc_configuration;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_security_groups;

pub(crate) mod shape_subnets;

