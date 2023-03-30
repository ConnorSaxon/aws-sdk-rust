// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_batch_get_metric_data;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_configuration_set;

pub(crate) mod shape_create_configuration_set_event_destination;

pub(crate) mod shape_create_contact;

pub(crate) mod shape_create_contact_list;

pub(crate) mod shape_create_custom_verification_email_template;

pub(crate) mod shape_create_dedicated_ip_pool;

pub(crate) mod shape_create_deliverability_test_report;

pub(crate) mod shape_create_email_identity;

pub(crate) mod shape_create_email_identity_policy;

pub(crate) mod shape_create_email_template;

pub(crate) mod shape_create_import_job;

pub(crate) mod shape_delete_configuration_set;

pub(crate) mod shape_delete_configuration_set_event_destination;

pub(crate) mod shape_delete_contact;

pub(crate) mod shape_delete_contact_list;

pub(crate) mod shape_delete_custom_verification_email_template;

pub(crate) mod shape_delete_dedicated_ip_pool;

pub(crate) mod shape_delete_email_identity;

pub(crate) mod shape_delete_email_identity_policy;

pub(crate) mod shape_delete_email_template;

pub(crate) mod shape_delete_suppressed_destination;

pub(crate) mod shape_get_account;

pub(crate) mod shape_get_blacklist_reports;

pub(crate) mod shape_get_configuration_set;

pub(crate) mod shape_get_configuration_set_event_destinations;

pub(crate) mod shape_get_contact;

pub(crate) mod shape_get_contact_list;

pub(crate) mod shape_get_custom_verification_email_template;

pub(crate) mod shape_get_dedicated_ip;

pub(crate) mod shape_get_dedicated_ip_pool;

pub(crate) mod shape_get_dedicated_ips;

pub(crate) mod shape_get_deliverability_dashboard_options;

pub(crate) mod shape_get_deliverability_test_report;

pub(crate) mod shape_get_domain_deliverability_campaign;

pub(crate) mod shape_get_domain_statistics_report;

pub(crate) mod shape_get_email_identity;

pub(crate) mod shape_get_email_identity_policies;

pub(crate) mod shape_get_email_template;

pub(crate) mod shape_get_import_job;

pub(crate) mod shape_get_suppressed_destination;

pub(crate) mod shape_list_configuration_sets;

pub(crate) mod shape_list_contact_lists;

pub(crate) mod shape_list_contacts;

pub(crate) mod shape_list_custom_verification_email_templates;

pub(crate) mod shape_list_dedicated_ip_pools;

pub(crate) mod shape_list_deliverability_test_reports;

pub(crate) mod shape_list_domain_deliverability_campaigns;

pub(crate) mod shape_list_email_identities;

pub(crate) mod shape_list_email_templates;

pub(crate) mod shape_list_import_jobs;

pub(crate) mod shape_list_recommendations;

pub(crate) mod shape_list_suppressed_destinations;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_put_account_dedicated_ip_warmup_attributes;

pub(crate) mod shape_put_account_details;

pub(crate) mod shape_put_account_sending_attributes;

pub(crate) mod shape_put_account_suppression_attributes;

pub(crate) mod shape_put_account_vdm_attributes;

pub(crate) mod shape_put_configuration_set_delivery_options;

pub(crate) mod shape_put_configuration_set_reputation_options;

pub(crate) mod shape_put_configuration_set_sending_options;

pub(crate) mod shape_put_configuration_set_suppression_options;

pub(crate) mod shape_put_configuration_set_tracking_options;

pub(crate) mod shape_put_configuration_set_vdm_options;

pub(crate) mod shape_put_dedicated_ip_in_pool;

pub(crate) mod shape_put_dedicated_ip_warmup_attributes;

pub(crate) mod shape_put_deliverability_dashboard_option;

pub(crate) mod shape_put_email_identity_configuration_set_attributes;

pub(crate) mod shape_put_email_identity_dkim_attributes;

pub(crate) mod shape_put_email_identity_dkim_signing_attributes;

pub(crate) mod shape_put_email_identity_feedback_attributes;

pub(crate) mod shape_put_email_identity_mail_from_attributes;

pub(crate) mod shape_put_suppressed_destination;

pub(crate) mod shape_send_bulk_email;

pub(crate) mod shape_send_custom_verification_email;

pub(crate) mod shape_send_email;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_test_render_email_template;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_configuration_set_event_destination;

pub(crate) mod shape_update_contact;

pub(crate) mod shape_update_contact_list;

pub(crate) mod shape_update_custom_verification_email_template;

pub(crate) mod shape_update_email_identity_policy;

pub(crate) mod shape_update_email_template;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_account_suspended_exception;

pub(crate) mod shape_already_exists_exception;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_batch_get_metric_data_input;

pub(crate) mod shape_concurrent_modification_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_configuration_set_event_destination_input;

pub(crate) mod shape_create_configuration_set_input;

pub(crate) mod shape_create_contact_input;

pub(crate) mod shape_create_contact_list_input;

pub(crate) mod shape_create_custom_verification_email_template_input;

pub(crate) mod shape_create_dedicated_ip_pool_input;

pub(crate) mod shape_create_deliverability_test_report_input;

pub(crate) mod shape_create_email_identity_input;

pub(crate) mod shape_create_email_identity_policy_input;

pub(crate) mod shape_create_email_template_input;

pub(crate) mod shape_create_import_job_input;

pub(crate) mod shape_internal_service_error_exception;

pub(crate) mod shape_invalid_next_token_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_contacts_input;

pub(crate) mod shape_list_import_jobs_input;

pub(crate) mod shape_list_recommendations_input;

pub(crate) mod shape_mail_from_domain_not_verified_exception;

pub(crate) mod shape_message_rejected;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_put_account_dedicated_ip_warmup_attributes_input;

pub(crate) mod shape_put_account_details_input;

pub(crate) mod shape_put_account_sending_attributes_input;

pub(crate) mod shape_put_account_suppression_attributes_input;

pub(crate) mod shape_put_account_vdm_attributes_input;

pub(crate) mod shape_put_configuration_set_delivery_options_input;

pub(crate) mod shape_put_configuration_set_reputation_options_input;

pub(crate) mod shape_put_configuration_set_sending_options_input;

pub(crate) mod shape_put_configuration_set_suppression_options_input;

pub(crate) mod shape_put_configuration_set_tracking_options_input;

pub(crate) mod shape_put_configuration_set_vdm_options_input;

pub(crate) mod shape_put_dedicated_ip_in_pool_input;

pub(crate) mod shape_put_dedicated_ip_warmup_attributes_input;

pub(crate) mod shape_put_deliverability_dashboard_option_input;

pub(crate) mod shape_put_email_identity_configuration_set_attributes_input;

pub(crate) mod shape_put_email_identity_dkim_attributes_input;

pub(crate) mod shape_put_email_identity_dkim_signing_attributes_input;

pub(crate) mod shape_put_email_identity_feedback_attributes_input;

pub(crate) mod shape_put_email_identity_mail_from_attributes_input;

pub(crate) mod shape_put_suppressed_destination_input;

pub(crate) mod shape_send_bulk_email_input;

pub(crate) mod shape_send_custom_verification_email_input;

pub(crate) mod shape_send_email_input;

pub(crate) mod shape_sending_paused_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_test_render_email_template_input;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_update_configuration_set_event_destination_input;

pub(crate) mod shape_update_contact_input;

pub(crate) mod shape_update_contact_list_input;

pub(crate) mod shape_update_custom_verification_email_template_input;

pub(crate) mod shape_update_email_identity_policy_input;

pub(crate) mod shape_update_email_template_input;

pub(crate) mod shape_account_details;

pub(crate) mod shape_batch_get_metric_data_query;

pub(crate) mod shape_blacklist_report;

pub(crate) mod shape_bulk_email_content;

pub(crate) mod shape_bulk_email_entry;

pub(crate) mod shape_bulk_email_entry_result_list;

pub(crate) mod shape_configuration_set_name_list;

pub(crate) mod shape_custom_verification_email_templates_list;

pub(crate) mod shape_daily_volumes;

pub(crate) mod shape_dedicated_ip;

pub(crate) mod shape_dedicated_ip_list;

pub(crate) mod shape_dedicated_ip_pool;

pub(crate) mod shape_deliverability_test_report;

pub(crate) mod shape_deliverability_test_reports;

pub(crate) mod shape_delivery_options;

pub(crate) mod shape_destination;

pub(crate) mod shape_dkim_attributes;

pub(crate) mod shape_dkim_signing_attributes;

pub(crate) mod shape_dns_token_list;

pub(crate) mod shape_domain_deliverability_campaign;

pub(crate) mod shape_domain_deliverability_campaign_list;

pub(crate) mod shape_domain_deliverability_tracking_option;

pub(crate) mod shape_domain_deliverability_tracking_options;

pub(crate) mod shape_email_content;

pub(crate) mod shape_email_template_content;

pub(crate) mod shape_email_template_metadata_list;

pub(crate) mod shape_event_destination_definition;

pub(crate) mod shape_event_destinations;

pub(crate) mod shape_failure_info;

pub(crate) mod shape_identity_info_list;

pub(crate) mod shape_import_data_source;

pub(crate) mod shape_import_destination;

pub(crate) mod shape_import_job_summary_list;

pub(crate) mod shape_isp_placements;

pub(crate) mod shape_list_contacts_filter;

pub(crate) mod shape_list_management_options;

pub(crate) mod shape_list_of_contact_lists;

pub(crate) mod shape_list_of_contacts;

pub(crate) mod shape_list_of_dedicated_ip_pools;

pub(crate) mod shape_mail_from_attributes;

pub(crate) mod shape_message_tag;

pub(crate) mod shape_metric_data_error_list;

pub(crate) mod shape_metric_data_result_list;

pub(crate) mod shape_overall_volume;

pub(crate) mod shape_placement_statistics;

pub(crate) mod shape_policy_map;

pub(crate) mod shape_recommendations_list;

pub(crate) mod shape_reputation_options;

pub(crate) mod shape_send_quota;

pub(crate) mod shape_sending_options;

pub(crate) mod shape_suppressed_destination;

pub(crate) mod shape_suppressed_destination_summaries;

pub(crate) mod shape_suppression_attributes;

pub(crate) mod shape_suppression_options;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_topic;

pub(crate) mod shape_topic_preference;

pub(crate) mod shape_topic_preference_list;

pub(crate) mod shape_topics;

pub(crate) mod shape_tracking_options;

pub(crate) mod shape_vdm_attributes;

pub(crate) mod shape_vdm_options;

pub(crate) mod shape_additional_contact_email_addresses;

pub(crate) mod shape_blacklist_entries;

pub(crate) mod shape_bulk_email_entry_result;

pub(crate) mod shape_cloud_watch_destination;

pub(crate) mod shape_contact;

pub(crate) mod shape_contact_list;

pub(crate) mod shape_contact_list_destination;

pub(crate) mod shape_custom_verification_email_template_metadata;

pub(crate) mod shape_daily_volume;

pub(crate) mod shape_dashboard_attributes;

pub(crate) mod shape_dashboard_options;

pub(crate) mod shape_domain_isp_placements;

pub(crate) mod shape_email_template_metadata;

pub(crate) mod shape_esps;

pub(crate) mod shape_event_destination;

pub(crate) mod shape_guardian_attributes;

pub(crate) mod shape_guardian_options;

pub(crate) mod shape_identity_info;

pub(crate) mod shape_import_job_summary;

pub(crate) mod shape_inbox_placement_tracking_option;

pub(crate) mod shape_ip_list;

pub(crate) mod shape_isp_placement;

pub(crate) mod shape_kinesis_firehose_destination;

pub(crate) mod shape_message;

pub(crate) mod shape_metric_data_error;

pub(crate) mod shape_metric_data_result;

pub(crate) mod shape_pinpoint_destination;

pub(crate) mod shape_raw_message;

pub(crate) mod shape_recommendation;

pub(crate) mod shape_replacement_email_content;

pub(crate) mod shape_review_details;

pub(crate) mod shape_sns_destination;

pub(crate) mod shape_suppressed_destination_attributes;

pub(crate) mod shape_suppressed_destination_summary;

pub(crate) mod shape_suppression_list_destination;

pub(crate) mod shape_suppression_list_reasons;

pub(crate) mod shape_template;

pub(crate) mod shape_topic_filter;

pub(crate) mod shape_volume_statistics;

pub(crate) mod shape_blacklist_entry;

pub(crate) mod shape_body;

pub(crate) mod shape_cloud_watch_dimension_configuration;

pub(crate) mod shape_content;

pub(crate) mod shape_domain_isp_placement;

pub(crate) mod shape_event_types;

pub(crate) mod shape_metric_value_list;

pub(crate) mod shape_replacement_template;

pub(crate) mod shape_timestamp_list;

pub(crate) mod shape_cloud_watch_dimension_configurations;

pub(crate) mod shape_isp_name_list;

