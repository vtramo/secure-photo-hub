use std::env;

use anyhow::{anyhow, Context};
use aws_config::SdkConfig;
use aws_sdk_s3::config::Credentials;
use aws_sdk_sts::Client as AwsStsClient;
use yaml_rust2::Yaml;

const AWS_ACCESS_KEY_ID_ENV_VAR: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY_ENV_VAR: &str = "AWS_SECRET_ACCESS_KEY";
const AWS_REGION_ENV_VAR: &str = "AWS_REGION";
const AWS_BUCKET_NAME_ENV_VAR: &str = "AWS_BUCKET_NAME";
const AWS_ENDPOINT_URL_ENV_VAR: &'static str = "AWS_ENDPOINT_URL";

const AWS_S3_FIELD: &str = "aws-s3";
const AWS_ACCESS_KEY_ID_FIELD: &str = "access-key-id";
const AWS_SECRET_ACCESS_KEY_FIELD: &str = "secret-access-key";
const AWS_REGION_FIELD: &str = "region";
const AWS_ENDPOINT_URL_FIELD: &'static str = "endpoint-url";
const AWS_BUCKET_NAME_FIELD: &'static str = "bucket-name";

#[derive(Debug, Clone)]
pub struct AwsS3Config {
    pub bucket_name: String,
    pub sdk_config: aws_config::SdkConfig,
}

pub async fn setup_aws_s3_config(secrets: &Yaml) -> anyhow::Result<AwsS3Config> {
    let aws_secrets = &secrets[AWS_S3_FIELD];
    let aws_access_key_id = extract_aws_access_key_id(aws_secrets)?;
    let aws_secret_access_key = extract_aws_secret_access_key(aws_secrets)?;
    let aws_region = extract_aws_region(aws_secrets)?;
    let aws_endpoint_url = extract_aws_endpoint_url(aws_secrets)?;
    let bucket_name = extract_aws_s3_bucket_name(aws_secrets)?;
    set_aws_environment_variables(&aws_access_key_id, &aws_secret_access_key, &aws_region)?;
    let sdk_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(Credentials::new(&aws_access_key_id, &aws_secret_access_key, None, None, ""))
        .endpoint_url(aws_endpoint_url)
        .load().await;

    check_aws_credentials(&sdk_config).await?;

    Ok(AwsS3Config {
        bucket_name,
        sdk_config
    })
}

async fn check_aws_credentials(sdk_config: &SdkConfig) -> anyhow::Result<()> {
    let sts_client = AwsStsClient::new(&sdk_config);
    
    if let Err(_) = sts_client.get_caller_identity().send().await {
        return Err(anyhow!("Invalid AWS Credentials!"));
    }
    
    Ok(())
}

fn set_aws_environment_variables(
    aws_access_key_id: &str,
    aws_secret_access_key: &str,
    aws_region: &str,
) -> anyhow::Result<()> {
    unsafe {
        env::set_var(AWS_ACCESS_KEY_ID_ENV_VAR, aws_access_key_id);
        env::set_var(AWS_SECRET_ACCESS_KEY_ENV_VAR, aws_secret_access_key);
        env::set_var(AWS_REGION_ENV_VAR, aws_region);
    };

    Ok(())
}

fn extract_aws_access_key_id(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_ACCESS_KEY_ID_ENV_VAR)
        .context(format!(
            "Environment variable '{}' is not set or is empty",
            AWS_ACCESS_KEY_ID_ENV_VAR
        ))
        .or_else(|_| {
            aws_secrets[AWS_ACCESS_KEY_ID_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_ACCESS_KEY_ID_FIELD))
                .map(str::to_string)
        })
        .map(|id| id.to_string())
}

fn extract_aws_secret_access_key(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_SECRET_ACCESS_KEY_ENV_VAR)
        .context(format!(
            "Environment variable '{}' is not set or is empty",
            AWS_SECRET_ACCESS_KEY_ENV_VAR
        ))
        .or_else(|_| {
            aws_secrets[AWS_SECRET_ACCESS_KEY_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_SECRET_ACCESS_KEY_FIELD))
                .map(str::to_string)
        })
        .map(|key| key.to_string())
}

fn extract_aws_region(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_REGION_ENV_VAR)
        .context(format!(
            "Environment variable '{}' is not set or is empty",
            AWS_REGION_ENV_VAR
        ))
        .or_else(|_| {
            aws_secrets[AWS_REGION_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_REGION_FIELD))
                .map(str::to_string)
        })
        .map(|region| region.to_string())
}

fn extract_aws_endpoint_url(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_ENDPOINT_URL_ENV_VAR)
        .context(format!(
            "Environment variable '{}' is not set or is empty",
            "endpoint-url"
        ))
        .or_else(|_| {
            aws_secrets[AWS_ENDPOINT_URL_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_ENDPOINT_URL_FIELD))
                .map(str::to_string)
        })
        .map(|region| region.to_string())
}

fn extract_aws_s3_bucket_name(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_BUCKET_NAME_ENV_VAR)
        .context(format!(
            "Environment variable '{}' is not set or is empty",
            "bucket-name"
        ))
        .or_else(|_| {
            aws_secrets[AWS_BUCKET_NAME_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_BUCKET_NAME_FIELD))
                .map(str::to_string)
        })
        .map(|bucket_name| bucket_name.to_string())
}

fn missing_aws_secret(secret_name: &str) -> String {
    format!("Missing or invalid {} in secrets", secret_name)
}
