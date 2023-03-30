// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_add_notification_channel;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_delete_insight;

pub(crate) mod shape_describe_account_health;

pub(crate) mod shape_describe_account_overview;

pub(crate) mod shape_describe_anomaly;

pub(crate) mod shape_describe_event_sources_config;

pub(crate) mod shape_describe_feedback;

pub(crate) mod shape_describe_insight;

pub(crate) mod shape_describe_organization_health;

pub(crate) mod shape_describe_organization_overview;

pub(crate) mod shape_describe_organization_resource_collection_health;

pub(crate) mod shape_describe_resource_collection_health;

pub(crate) mod shape_describe_service_integration;

pub(crate) mod shape_get_cost_estimation;

pub(crate) mod shape_get_resource_collection;

pub(crate) mod shape_list_anomalies_for_insight;

pub(crate) mod shape_list_anomalous_log_groups;

pub(crate) mod shape_list_events;

pub(crate) mod shape_list_insights;

pub(crate) mod shape_list_monitored_resources;

pub(crate) mod shape_list_notification_channels;

pub(crate) mod shape_list_organization_insights;

pub(crate) mod shape_list_recommendations;

pub(crate) mod shape_put_feedback;

pub(crate) mod shape_remove_notification_channel;

pub(crate) mod shape_search_insights;

pub(crate) mod shape_search_organization_insights;

pub(crate) mod shape_start_cost_estimation;

pub(crate) mod shape_update_event_sources_config;

pub(crate) mod shape_update_resource_collection;

pub(crate) mod shape_update_service_integration;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_add_notification_channel_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_describe_account_overview_input;

pub(crate) mod shape_describe_feedback_input;

pub(crate) mod shape_describe_organization_health_input;

pub(crate) mod shape_describe_organization_overview_input;

pub(crate) mod shape_describe_organization_resource_collection_health_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_anomalies_for_insight_input;

pub(crate) mod shape_list_anomalous_log_groups_input;

pub(crate) mod shape_list_events_input;

pub(crate) mod shape_list_insights_input;

pub(crate) mod shape_list_monitored_resources_input;

pub(crate) mod shape_list_notification_channels_input;

pub(crate) mod shape_list_organization_insights_input;

pub(crate) mod shape_list_recommendations_input;

pub(crate) mod shape_put_feedback_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_search_insights_input;

pub(crate) mod shape_search_organization_insights_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_cost_estimation_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_event_sources_config_input;

pub(crate) mod shape_update_resource_collection_input;

pub(crate) mod shape_update_service_integration_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_account_healths;

pub(crate) mod shape_anomalous_log_groups;

pub(crate) mod shape_channels;

pub(crate) mod shape_cloud_formation_healths;

pub(crate) mod shape_cost_estimation_resource_collection_filter;

pub(crate) mod shape_cost_estimation_time_range;

pub(crate) mod shape_event_sources_config;

pub(crate) mod shape_events;

pub(crate) mod shape_insight_feedback;

pub(crate) mod shape_list_events_filters;

pub(crate) mod shape_list_insights_status_filter;

pub(crate) mod shape_list_monitored_resources_filters;

pub(crate) mod shape_monitored_resource_identifiers;

pub(crate) mod shape_notification_channel_config;

pub(crate) mod shape_proactive_anomalies;

pub(crate) mod shape_proactive_anomaly;

pub(crate) mod shape_proactive_insight;

pub(crate) mod shape_proactive_insights;

pub(crate) mod shape_proactive_organization_insights;

pub(crate) mod shape_reactive_anomalies;

pub(crate) mod shape_reactive_anomaly;

pub(crate) mod shape_reactive_insight;

pub(crate) mod shape_reactive_insights;

pub(crate) mod shape_reactive_organization_insights;

pub(crate) mod shape_recommendations;

pub(crate) mod shape_resource_collection_filter;

pub(crate) mod shape_search_insights_filters;

pub(crate) mod shape_search_organization_insights_filters;

pub(crate) mod shape_service_healths;

pub(crate) mod shape_service_integration_config;

pub(crate) mod shape_service_resource_costs;

pub(crate) mod shape_start_time_range;

pub(crate) mod shape_tag_healths;

pub(crate) mod shape_update_resource_collection_filter;

pub(crate) mod shape_update_service_integration_config;

pub(crate) mod shape_validation_exception_fields;

pub(crate) mod shape_account_health;

pub(crate) mod shape_amazon_code_guru_profiler_integration;

pub(crate) mod shape_anomalous_log_group;

pub(crate) mod shape_anomaly_reported_time_range;

pub(crate) mod shape_anomaly_resources;

pub(crate) mod shape_anomaly_source_details;

pub(crate) mod shape_anomaly_source_metadata;

pub(crate) mod shape_anomaly_time_range;

pub(crate) mod shape_cloud_formation_collection_filter;

pub(crate) mod shape_cloud_formation_cost_estimation_resource_collection_filter;

pub(crate) mod shape_cloud_formation_health;

pub(crate) mod shape_event;

pub(crate) mod shape_event_time_range;

pub(crate) mod shape_insight_time_range;

pub(crate) mod shape_list_insights_any_status_filter;

pub(crate) mod shape_list_insights_closed_status_filter;

pub(crate) mod shape_list_insights_ongoing_status_filter;

pub(crate) mod shape_logs_anomaly_detection_integration;

pub(crate) mod shape_logs_anomaly_detection_integration_config;

pub(crate) mod shape_monitored_resource_identifier;

pub(crate) mod shape_notification_channel;

pub(crate) mod shape_notification_filter_config;

pub(crate) mod shape_ops_center_integration;

pub(crate) mod shape_ops_center_integration_config;

pub(crate) mod shape_prediction_time_range;

pub(crate) mod shape_proactive_anomaly_summary;

pub(crate) mod shape_proactive_insight_summary;

pub(crate) mod shape_proactive_organization_insight_summary;

pub(crate) mod shape_reactive_anomaly_summary;

pub(crate) mod shape_reactive_insight_summary;

pub(crate) mod shape_reactive_organization_insight_summary;

pub(crate) mod shape_recommendation;

pub(crate) mod shape_resource_collection;

pub(crate) mod shape_service_collection;

pub(crate) mod shape_service_health;

pub(crate) mod shape_service_resource_cost;

pub(crate) mod shape_sns_channel_config;

pub(crate) mod shape_tag_collection_filters;

pub(crate) mod shape_tag_cost_estimation_resource_collection_filter;

pub(crate) mod shape_tag_cost_estimation_resource_collection_filters;

pub(crate) mod shape_tag_health;

pub(crate) mod shape_update_cloud_formation_collection_filter;

pub(crate) mod shape_update_tag_collection_filter;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_account_insight_health;

pub(crate) mod shape_anomaly_resource;

pub(crate) mod shape_associated_resource_arns;

pub(crate) mod shape_cloud_formation_collection;

pub(crate) mod shape_cloud_watch_metrics_details;

pub(crate) mod shape_cost_estimation_stack_names;

pub(crate) mod shape_end_time_range;

pub(crate) mod shape_event_resources;

pub(crate) mod shape_insight_health;

pub(crate) mod shape_log_anomaly_showcases;

pub(crate) mod shape_performance_insights_metrics_details;

pub(crate) mod shape_recommendation_related_anomalies;

pub(crate) mod shape_recommendation_related_events;

pub(crate) mod shape_service_insight_health;

pub(crate) mod shape_stack_names;

pub(crate) mod shape_tag_collection;

pub(crate) mod shape_tag_collection_filter;

pub(crate) mod shape_tag_collections;

pub(crate) mod shape_cloud_watch_metrics_detail;

pub(crate) mod shape_cost_estimation_tag_values;

pub(crate) mod shape_event_resource;

pub(crate) mod shape_log_anomaly_showcase;

pub(crate) mod shape_performance_insights_metrics_detail;

pub(crate) mod shape_recommendation_related_anomaly;

pub(crate) mod shape_recommendation_related_event;

pub(crate) mod shape_service_names;

pub(crate) mod shape_tag_values;

pub(crate) mod shape_cloud_watch_metrics_data_summary;

pub(crate) mod shape_cloud_watch_metrics_dimensions;

pub(crate) mod shape_insight_severities;

pub(crate) mod shape_log_anomaly_classes;

pub(crate) mod shape_notification_message_types;

pub(crate) mod shape_performance_insights_metric_query;

pub(crate) mod shape_performance_insights_reference_data_list;

pub(crate) mod shape_performance_insights_stats;

pub(crate) mod shape_recommendation_related_anomaly_resources;

pub(crate) mod shape_recommendation_related_event_resources;

pub(crate) mod shape_related_anomaly_source_details;

pub(crate) mod shape_cloud_watch_metrics_dimension;

pub(crate) mod shape_log_anomaly_class;

pub(crate) mod shape_performance_insights_metric_dimension_group;

pub(crate) mod shape_performance_insights_metric_filter_map;

pub(crate) mod shape_performance_insights_reference_data;

pub(crate) mod shape_performance_insights_stat;

pub(crate) mod shape_recommendation_related_anomaly_resource;

pub(crate) mod shape_recommendation_related_anomaly_source_detail;

pub(crate) mod shape_recommendation_related_event_resource;

pub(crate) mod shape_timestamp_metric_value_pair_list;

pub(crate) mod shape_performance_insights_metric_dimensions;

pub(crate) mod shape_performance_insights_reference_comparison_values;

pub(crate) mod shape_recommendation_related_cloud_watch_metrics_source_details;

pub(crate) mod shape_timestamp_metric_value_pair;

pub(crate) mod shape_performance_insights_reference_metric;

pub(crate) mod shape_performance_insights_reference_scalar;

pub(crate) mod shape_recommendation_related_cloud_watch_metrics_source_detail;

