# WhatsApp-Tell: Audio Neurocognitive Markers Service

A Rust-based service that processes user audio via WhatsApp to compute speech and voice markers related to neurocognitive health, providing accessible screening-style insights.

## 🎯 Overview

WhatsApp-Tell is designed to provide non-diagnostic neurocognitive health insights through WhatsApp voice messages. The service analyzes speech patterns, prosody, and fluency markers to generate brief, interpretable reports while maintaining strict privacy and compliance standards for Argentina and Uruguay.

### Key Features

- **WhatsApp Integration**: Seamless voice message processing via WhatsApp Business API
- **Neurocognitive Analysis**: Speech rate, prosody, fluency, and lexical markers
- **Privacy-First**: GDPR-compliant with AR/UY-specific privacy controls
- **Non-Diagnostic**: Clear disclaimers and informational-only results
- **Multi-Modal**: Voice notes, file uploads, and templated messaging
- **Audit Trail**: Complete compliance tracking and data subject rights

## 🏗️ Architecture

```
[WhatsApp Client] 
    ⇅ Meta WhatsApp Business Platform
        → [Webhook Gateway] (Rust Lambda)
            → [Ingestion Queue] (SQS FIFO)
                → [Media Fetcher] (Rust Lambda)
                    ↳ Download media → S3 (encrypted)
                    ↳ Metadata → DynamoDB
                → [Audio Processor] (Rust Lambda)
                    ↳ Orchestrate existing audio Lambdas
                    ↳ VAD → segmentation → features → aggregation
                → [Report Composer] (Rust Lambda)
                    ↳ Format WhatsApp reply
                    ↳ Optional PDF generation
            ← [Outbound Reply] (WhatsApp Send API)
```

## 📁 Project Structure

This is a Rust workspace containing four main modules:

- **`gateway/`** - WhatsApp webhook ingress, signature validation, and message handling
- **`processor/`** - Audio processing orchestration and Lambda invocation
- **`reporting/`** - Report generation and WhatsApp message formatting  
- **`shared/`** - Common types, AWS clients, error handling, and utilities

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- AWS CLI configured
- WhatsApp Business API access
- Terraform or AWS CDK for infrastructure

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/Tell-toolkit/whatsapp-tell.git
   cd whatsapp-tell
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Set up environment variables**
   ```bash
   export AWS_REGION=sa-east-1
   export WHATSAPP_ACCESS_TOKEN=your_token
   export WHATSAPP_WEBHOOK_VERIFY_TOKEN=your_verify_token
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

### Infrastructure Deployment

The service requires AWS infrastructure including:
- API Gateway for webhook endpoints
- SQS FIFO queues for message processing
- S3 buckets for media storage
- DynamoDB tables for conversation state
- Lambda functions for processing
- KMS keys for encryption

See the `infra/` directory for Terraform/CDK configurations.

## 🔧 Development

### Adding New Features

1. **Update shared types** in `shared/src/types.rs` for new data structures
2. **Implement business logic** in the appropriate module
3. **Add tests** with comprehensive coverage
4. **Update documentation** and README as needed

### Code Standards

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maintain 80%+ test coverage
- Follow Rust naming conventions
- Document all public APIs

### Testing

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo test --features test-coverage

# Run integration tests
cargo test --test integration
```

## 🔒 Privacy & Compliance

### Data Protection

- **Encryption**: All data encrypted at rest and in transit
- **Minimal Retention**: Configurable data retention policies
- **User Rights**: Full data subject rights implementation
- **Audit Trail**: Complete processing audit logs

### Regional Compliance

- **Argentina**: Ley 25.326 compliance with AAIP requirements
- **Uruguay**: Ley 18.331 compliance with URCDP requirements
- **Cross-Border**: Adequate protection for international transfers

### Consent Management

Users must explicitly consent to:
- Audio processing for neurocognitive markers
- Optional ASR/semantic analysis
- Data retention periods
- Research mode participation

## 📊 Monitoring & Observability

- **CloudWatch**: Metrics, logs, and alarms
- **X-Ray**: Distributed tracing
- **OpenTelemetry**: Structured logging and metrics
- **SLOs**: Latency, success rate, and consent capture metrics

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [Wiki](https://github.com/Tell-toolkit/whatsapp-tell/wiki)
- **Issues**: [GitHub Issues](https://github.com/Tell-toolkit/whatsapp-tell/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Tell-toolkit/whatsapp-tell/discussions)

## ⚠️ Disclaimer

This service provides **informational insights only** and does not constitute medical diagnosis or treatment. Users should consult healthcare professionals for medical concerns. The service is designed for research and awareness purposes only.

---

**Built with ❤️ by the Tell-toolkit team**
