//! WhatsApp message formatting

use shared::types::MarkerSet;
use shared::errors::Result;
use tracing::info;

/// Formats messages for WhatsApp delivery
pub struct MessageFormatter {
    // TODO: Add formatting configuration
}

impl MessageFormatter {
    pub fn new() -> Self {
        Self {}
    }

    /// Formats a brief report for WhatsApp
    pub fn format_brief_report(&self, markers: &MarkerSet) -> Result<String> {
        info!("Formatting brief report for WhatsApp");
        
        // TODO: Implement WhatsApp-specific formatting
        // - Use appropriate emojis
        // - Keep within WhatsApp message limits
        // - Include clear disclaimers
        // - Add call-to-action buttons if needed
        
        Ok(format!(
            "📊 *Análisis Completado*\n\n\
            🗣️ Velocidad: {:.1} sílabas/seg\n\
            ⏱️ Fonación: {:.0}%\n\
            ⏸️ Pausas: {:.1}s promedio\n\
            ✅ Confianza: {:.0}%\n\n\
            _Análisis informativo únicamente._\n\
            _No constituye diagnóstico médico._",
            markers.speech_rate.unwrap_or(0.0),
            markers.phonation_time.unwrap_or(0.0) * 100.0,
            markers.average_pause_length.unwrap_or(0.0),
            markers.confidence_score * 100.0
        ))
    }

    /// Formats consent request message
    pub fn format_consent_request(&self) -> Result<String> {
        info!("Formatting consent request message");
        
        Ok(
            "🔒 *Consentimiento para Análisis de Voz*\n\n\
            Para analizar tu voz y proporcionar información sobre marcadores neurocognitivos, necesitamos tu consentimiento explícito.\n\n\
            📋 *¿Qué procesamos?*\n\
            • Audio de voz para análisis técnico\n\
            • Marcadores de velocidad, prosodia y fluidez\n\
            • Retención por 30 días (configurable)\n\n\
            ✅ *Opciones de consentimiento:*\n\
            • Análisis básico de voz\n\
            • Análisis con transcripción (opcional)\n\
            • Modo investigación (opcional)\n\n\
            Responde *SÍ* para continuar o *NO* para cancelar.".to_string()
        )
    }

    /// Formats error message
    pub fn format_error_message(&self, error: &str) -> Result<String> {
        Ok(format!(
            "❌ *Error en el Procesamiento*\n\n\
            Lo sentimos, hubo un problema al procesar tu audio.\n\n\
            Error: {}\n\n\
            Por favor, intenta enviar otro mensaje de voz o contacta soporte si el problema persiste.",
            error
        ))
    }
}
