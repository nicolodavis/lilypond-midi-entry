use clap::Parser;

/// LilyPond MIDI note entry.
#[derive(Clone, Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The MIDI port to read events from.
    #[clap(short, long, value_parser)]
    pub port: String,

    /// The MIDI event that is used to enable duration 1.
    #[clap(long, value_parser, default_value_t = 84)]
    pub midi_duration_1: u8,

    /// The MIDI event that is used to toggle between sharp / flat modes.
    #[clap(long, value_parser, default_value_t = 85)]
    pub midi_flat_toggle: u8,

    /// The MIDI event that is used to enable duration 2.
    #[clap(long, value_parser, default_value_t = 86)]
    pub midi_duration_2: u8,

    /// The MIDI event that is used to enable a dotted duration.
    #[clap(long, value_parser, default_value_t = 87)]
    pub midi_duration_dot: u8,

    /// The MIDI event that is used to enable duration 4.
    #[clap(long, value_parser, default_value_t = 88)]
    pub midi_duration_4: u8,

    /// The MIDI event that is used to enable duration 8.
    #[clap(long, value_parser, default_value_t = 89)]
    pub midi_duration_8: u8,

    /// The MIDI event that is used to enable duration 16.
    #[clap(long, value_parser, default_value_t = 91)]
    pub midi_duration_16: u8,

    /// The MIDI event that can generate a custom key sequence.
    #[clap(long, value_parser, default_value_t = 90)]
    pub midi_custom_1: u8,
    #[clap(long, value_parser, default_value = "")]
    pub midi_custom_1_value: String,

    /// The MIDI event that can generate a custom key sequence.
    #[clap(long, value_parser, default_value_t = 92)]
    pub midi_custom_2: u8,
    #[clap(long, value_parser, default_value = "")]
    pub midi_custom_2_value: String,

    /// The MIDI event that can generate a custom key sequence.
    #[clap(long, value_parser, default_value_t = 94)]
    pub midi_custom_3: u8,
    #[clap(long, value_parser, default_value = "")]
    pub midi_custom_3_value: String,

    /// The MIDI event that is used to generate a backspace.
    #[clap(long, value_parser, default_value_t = 96)]
    pub midi_backspace: u8,
}
