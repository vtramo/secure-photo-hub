use std::env;
use anyhow::Context;
use yaml_rust2::Yaml;

const AWS_ACCESS_KEY_ID_ENV_VAR: &str = "AWS_ACCESS_KEY_ID";
const AWS_SECRET_ACCESS_KEY_ENV_VAR: &str = "AWS_SECRET_ACCESS_KEY";
const AWS_REGION_ENV_VAR: &str = "AWS_REGION";

const AWS_FIELD: &str = "aws";
const AWS_ACCESS_KEY_ID_FIELD: &str = "access-key-id";
const AWS_SECRET_ACCESS_KEY_FIELD: &str = "secret-access-key";
const AWS_REGION_FIELD: &str = "region";

pub async fn setup_aws_config(secrets: &Yaml) -> anyhow::Result<aws_config::SdkConfig> {
    let aws_secrets = &secrets[AWS_FIELD];
    let aws_access_key_id = extract_aws_access_key_id(aws_secrets)?;
    let aws_secret_access_key = extract_aws_secret_access_key(aws_secrets)?;
    let aws_region = extract_aws_region(aws_secrets)?;
    set_aws_environment_variables(&aws_access_key_id, &aws_secret_access_key, &aws_region)?;
    Ok(aws_config::load_from_env().await)
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
        .context(format!("Environment variable '{}' is not set or is empty", AWS_ACCESS_KEY_ID_ENV_VAR))
        .or_else(|_| {
            aws_secrets[AWS_ACCESS_KEY_ID_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_ACCESS_KEY_ID_FIELD))
                .map(str::to_string)
        }).map(|id| id.to_string())
}

fn extract_aws_secret_access_key(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_SECRET_ACCESS_KEY_ENV_VAR)
        .context(format!("Environment variable '{}' is not set or is empty", AWS_SECRET_ACCESS_KEY_ENV_VAR))
        .or_else(|_| {
            aws_secrets[AWS_SECRET_ACCESS_KEY_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_SECRET_ACCESS_KEY_FIELD))
                .map(str::to_string)
        }).map(|key| key.to_string())
}

fn extract_aws_region(aws_secrets: &Yaml) -> anyhow::Result<String> {
    env::var(AWS_REGION_ENV_VAR)
        .context(format!("Environment variable '{}' is not set or is empty", AWS_REGION_ENV_VAR))
        .or_else(|_| {
            aws_secrets[AWS_REGION_FIELD]
                .as_str()
                .context(missing_aws_secret(AWS_REGION_FIELD))
                .map(str::to_string)
        }).map(|region| region.to_string())
}

fn missing_aws_secret(secret_name: &str) -> String {
    format!("Missing or invalid {} in secrets", secret_name)
}