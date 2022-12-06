//
// Configuration structure
//
use analog_channel::ChannelConfig;

pub struct Config {
    pub version: u16,
    pub channels: [ChannelConfig; 6],
    pub report_frequency_ms: u32,    // reporting frequency
    pub match_delay_count: u16,      // loop counter for output match
    pub analog_refresh_rate_hz: u16, // frequency in Hz for the ADC/DAC loop
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            channels: [
                ChannelConfig::default(),
                ChannelConfig::default(),
                ChannelConfig::default(),
                ChannelConfig::default(),
                ChannelConfig::default(),
                ChannelConfig::default(),
            ],
            report_frequency_ms: 100,
            match_delay_count: 20,
            analog_refresh_rate_hz: 100,
        }
    }
}
