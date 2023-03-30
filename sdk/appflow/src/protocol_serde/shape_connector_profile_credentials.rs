// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_connector_profile_credentials(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConnectorProfileCredentials) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.amplitude {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Amplitude").start_object();
        crate::protocol_serde::shape_amplitude_connector_profile_credentials::ser_amplitude_connector_profile_credentials(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.datadog {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Datadog").start_object();
        crate::protocol_serde::shape_datadog_connector_profile_credentials::ser_datadog_connector_profile_credentials(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.dynatrace {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Dynatrace").start_object();
        crate::protocol_serde::shape_dynatrace_connector_profile_credentials::ser_dynatrace_connector_profile_credentials(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.google_analytics {
        #[allow(unused_mut)]
        let mut object_8 = object.key("GoogleAnalytics").start_object();
        crate::protocol_serde::shape_google_analytics_connector_profile_credentials::ser_google_analytics_connector_profile_credentials(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.honeycode {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Honeycode").start_object();
        crate::protocol_serde::shape_honeycode_connector_profile_credentials::ser_honeycode_connector_profile_credentials(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.infor_nexus {
        #[allow(unused_mut)]
        let mut object_12 = object.key("InforNexus").start_object();
        crate::protocol_serde::shape_infor_nexus_connector_profile_credentials::ser_infor_nexus_connector_profile_credentials(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.marketo {
        #[allow(unused_mut)]
        let mut object_14 = object.key("Marketo").start_object();
        crate::protocol_serde::shape_marketo_connector_profile_credentials::ser_marketo_connector_profile_credentials(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.redshift {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Redshift").start_object();
        crate::protocol_serde::shape_redshift_connector_profile_credentials::ser_redshift_connector_profile_credentials(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.salesforce {
        #[allow(unused_mut)]
        let mut object_18 = object.key("Salesforce").start_object();
        crate::protocol_serde::shape_salesforce_connector_profile_credentials::ser_salesforce_connector_profile_credentials(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.service_now {
        #[allow(unused_mut)]
        let mut object_20 = object.key("ServiceNow").start_object();
        crate::protocol_serde::shape_service_now_connector_profile_credentials::ser_service_now_connector_profile_credentials(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.singular {
        #[allow(unused_mut)]
        let mut object_22 = object.key("Singular").start_object();
        crate::protocol_serde::shape_singular_connector_profile_credentials::ser_singular_connector_profile_credentials(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.slack {
        #[allow(unused_mut)]
        let mut object_24 = object.key("Slack").start_object();
        crate::protocol_serde::shape_slack_connector_profile_credentials::ser_slack_connector_profile_credentials(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.snowflake {
        #[allow(unused_mut)]
        let mut object_26 = object.key("Snowflake").start_object();
        crate::protocol_serde::shape_snowflake_connector_profile_credentials::ser_snowflake_connector_profile_credentials(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.trendmicro {
        #[allow(unused_mut)]
        let mut object_28 = object.key("Trendmicro").start_object();
        crate::protocol_serde::shape_trendmicro_connector_profile_credentials::ser_trendmicro_connector_profile_credentials(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.veeva {
        #[allow(unused_mut)]
        let mut object_30 = object.key("Veeva").start_object();
        crate::protocol_serde::shape_veeva_connector_profile_credentials::ser_veeva_connector_profile_credentials(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.zendesk {
        #[allow(unused_mut)]
        let mut object_32 = object.key("Zendesk").start_object();
        crate::protocol_serde::shape_zendesk_connector_profile_credentials::ser_zendesk_connector_profile_credentials(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.sapo_data {
        #[allow(unused_mut)]
        let mut object_34 = object.key("SAPOData").start_object();
        crate::protocol_serde::shape_sapo_data_connector_profile_credentials::ser_sapo_data_connector_profile_credentials(&mut object_34, var_33)?;
        object_34.finish();
    }
    if let Some(var_35) = &input.custom_connector {
        #[allow(unused_mut)]
        let mut object_36 = object.key("CustomConnector").start_object();
        crate::protocol_serde::shape_custom_connector_profile_credentials::ser_custom_connector_profile_credentials(&mut object_36, var_35)?;
        object_36.finish();
    }
    if let Some(var_37) = &input.pardot {
        #[allow(unused_mut)]
        let mut object_38 = object.key("Pardot").start_object();
        crate::protocol_serde::shape_pardot_connector_profile_credentials::ser_pardot_connector_profile_credentials(&mut object_38, var_37)?;
        object_38.finish();
    }
    Ok(())
}

