//! AWS client configurations and utilities

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_kms::Client as KmsClient;
use aws_sdk_lambda::Client as LambdaClient;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_secretsmanager::Client as SecretsManagerClient;
use aws_sdk_sqs::Client as SqsClient;

use crate::errors::Result;

/// AWS client configuration
pub struct AwsClients {
    pub s3: S3Client,
    pub dynamodb: DynamoDbClient,
    pub sqs: SqsClient,
    pub lambda: LambdaClient,
    pub kms: KmsClient,
    pub secrets_manager: SecretsManagerClient,
}

impl AwsClients {
    /// Create AWS clients with default configuration
    pub async fn new() -> Result<Self> {
        let region_provider = RegionProviderChain::default_provider().or_else("sa-east-1"); // Default to São Paulo region for AR/UY

        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        Ok(Self {
            s3: S3Client::new(&config),
            dynamodb: DynamoDbClient::new(&config),
            sqs: SqsClient::new(&config),
            lambda: LambdaClient::new(&config),
            kms: KmsClient::new(&config),
            secrets_manager: SecretsManagerClient::new(&config),
        })
    }

    /// Create AWS clients with custom configuration
    pub async fn with_config(config: &aws_config::SdkConfig) -> Self {
        Self {
            s3: S3Client::new(config),
            dynamodb: DynamoDbClient::new(config),
            sqs: SqsClient::new(config),
            lambda: LambdaClient::new(config),
            kms: KmsClient::new(config),
            secrets_manager: SecretsManagerClient::new(config),
        }
    }
}
