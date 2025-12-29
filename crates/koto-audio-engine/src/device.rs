//! Audio device management

use cpal::traits::{DeviceTrait, HostTrait};
use koto_core::{ChannelCount, KotoError, KotoResult, SampleRate};
use thiserror::Error;

/// Audio device error
#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("No audio host available")]
    NoHost,
    #[error("No output device available")]
    NoOutputDevice,
    #[error("No input device available")]
    NoInputDevice,
    #[error("Failed to get device config: {0}")]
    ConfigError(String),
    #[error("Stream error: {0}")]
    StreamError(String),
}

impl From<DeviceError> for KotoError {
    fn from(err: DeviceError) -> Self {
        KotoError::AudioDevice(err.to_string())
    }
}

/// Information about an audio device
#[derive(Debug, Clone)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub sample_rate: SampleRate,
    pub channels: ChannelCount,
    pub is_input: bool,
    pub is_output: bool,
}

/// Audio device manager
pub struct AudioDeviceManager {
    host: cpal::Host,
}

impl AudioDeviceManager {
    /// Create a new device manager
    pub fn new() -> KotoResult<Self> {
        let host = cpal::default_host();
        Ok(Self { host })
    }

    /// Get the default output device
    pub fn default_output_device(&self) -> Option<cpal::Device> {
        self.host.default_output_device()
    }

    /// Get the default input device
    pub fn default_input_device(&self) -> Option<cpal::Device> {
        self.host.default_input_device()
    }

    /// List all available output devices
    pub fn output_devices(&self) -> Vec<AudioDeviceInfo> {
        self.host
            .output_devices()
            .map(|devices| {
                devices
                    .filter_map(|device| self.device_info(&device, false, true).ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// List all available input devices
    pub fn input_devices(&self) -> Vec<AudioDeviceInfo> {
        self.host
            .input_devices()
            .map(|devices| {
                devices
                    .filter_map(|device| self.device_info(&device, true, false).ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get device info
    fn device_info(
        &self,
        device: &cpal::Device,
        is_input: bool,
        is_output: bool,
    ) -> Result<AudioDeviceInfo, DeviceError> {
        let name = device
            .name()
            .unwrap_or_else(|_| "Unknown Device".to_string());

        let config = if is_output {
            device.default_output_config()
        } else {
            device.default_input_config()
        }
        .map_err(|e| DeviceError::ConfigError(e.to_string()))?;

        Ok(AudioDeviceInfo {
            name,
            sample_rate: SampleRate(config.sample_rate().0),
            channels: ChannelCount(config.channels()),
            is_input,
            is_output,
        })
    }

    /// Get supported output configurations for a device
    pub fn supported_output_configs(
        &self,
        device: &cpal::Device,
    ) -> Result<Vec<cpal::SupportedStreamConfigRange>, DeviceError> {
        device
            .supported_output_configs()
            .map(|configs| configs.collect())
            .map_err(|e| DeviceError::ConfigError(e.to_string()))
    }

    /// Get supported input configurations for a device
    pub fn supported_input_configs(
        &self,
        device: &cpal::Device,
    ) -> Result<Vec<cpal::SupportedStreamConfigRange>, DeviceError> {
        device
            .supported_input_configs()
            .map(|configs| configs.collect())
            .map_err(|e| DeviceError::ConfigError(e.to_string()))
    }
}

impl Default for AudioDeviceManager {
    fn default() -> Self {
        Self::new().expect("Failed to create audio device manager")
    }
}
