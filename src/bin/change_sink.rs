use anyhow::Result;
use clap::{ArgEnum, Parser};

use serde::Deserialize;

/// Representation of a Pipewire device
#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: usize,
    pub info: Option<Info>,
    #[serde(rename = "type")]
    pub device_type: String,
    pub version: usize,
    pub permissions: Vec<String>,
}

/// Detailed info about a device
#[derive(Debug, Deserialize)]
pub struct Info {
    pub props: Option<Props>,
    pub error: Option<String>,
    #[serde(rename = "max-input-ports")]
    pub max_input_ports: Option<usize>,
    #[serde(rename = "max-output-ports")]
    pub max_output_ports: Option<usize>,
    /// Looks like this:
    /// [ "input-ports", "output-ports", "state", "props", "params" ]
    #[serde(rename = "change-mask")]
    pub change_mask: Vec<String>,
    #[serde(rename = "n-input-ports")]
    pub n_input_ports: Option<usize>,
    #[serde(rename = "n-output-ports")]
    pub n_output_ports: Option<usize>,
    //pub state: "suspended",
}

#[derive(Debug, Deserialize)]
pub struct Props {
    #[serde(rename = "media.class")]
    pub media_class: Option<String>,
    #[serde(rename = "device.id")]
    pub device_id: Option<usize>,
    #[serde(rename = "node.description")]
    pub node_description: Option<String>,
    #[serde(rename = "object.serial")]
    pub object_serial: usize,
    #[serde(rename = "api.alsa.path")]
    pub api_alsa_path: Option<String>,
    #[serde(rename = "api.alsa.card")]
    pub api_alsa_card: Option<usize>,
    #[serde(rename = "api.alsa.card.name")]
    pub api_alsa_card_name: Option<String>,
    #[serde(rename = "api.alsa.card.longname")]
    pub api_alsa_card_longname: Option<String>,
    //#[serde(rename="object.path")]
    //object_path: "v4l2:/dev/video2",
    //#[serde(rename="device.api")]
    //device_api: "v4l2",
    //#[serde(rename="node.name")]
    //node_name: "v4l2_input_pci-0000_00_14_0-usb-0_8_1_0",
    //#[serde(rename="factory.name")]
    //factory_name: "api_v4l2_source",
    //#[serde(rename="node.pause-on-idle")]
    //node_pause-on-idle: false,
    //#[serde(rename="client.id")]
    //client_id: 32,
    //#[serde(rename="media.role")]
    //media_role: "Camera",
    //#[serde(rename="node.driver")]
    //node_driver: true,
    //#[serde(rename="object.id")]
    //object_id: 49,
}

#[derive(Parser, Debug)]
#[clap(
    name = "change_sink",
    about = "Change the current sink to the specified device",
    author = "Arne Beer <contact@arne.beer>"
)]
struct CliArguments {
    /// The audio sink that should be switched to.
    #[clap(arg_enum)]
    pub target: Target,
}

#[derive(Parser, ArgEnum, Copy, Clone, Debug)]
enum Target {
    Hdmi,
    BuiltIn,
    Xonar,
}

fn main() -> Result<()> {
    // Parse commandline options.
    let args = CliArguments::parse();

    // Get current pipewire state.
    let capture = Cmd::new("pw-dump").run_success()?;
    let devices: Vec<Device> = serde_json::from_str(&capture.stdout_str())?;

    // Run through all devices and find the one we desire.
    for device in devices {
        let info = unwrap_or_continue!(device.info);
        let props = unwrap_or_continue!(info.props);
        let device_id = props.object_serial;
        // We are only interested in Audio/Sink type devices.
        match props.media_class {
            None => continue,
            Some(class) => {
                if class != "Audio/Sink" {
                    continue;
                }
            }
        }

        let description = unwrap_or_continue!(props.node_description);
        //println!("Device {device_id}: {description}");

        // Check if we find a device for the given name.
        let device_found = match args.target {
            Target::Hdmi => description.contains("HDMI"),
            Target::BuiltIn => description.starts_with("Built-in"),
            Target::Xonar => description.contains("Xonar"),
        };

        if !device_found {
            continue;
        }

        // Set the default sink.
        Cmd::new(format!("pactl set-default-sink {device_id}")).run_success()?;

        // Get all currently active sink inputs.
        // Output format looks like this:
        //
        // 188 56 187 PipeWire float32le 2ch 48000Hz
        //
        // We're interested in the first number.
        let capture = Cmd::new("pactl list short sink-inputs").run_success()?;

        let input_ids: Vec<String> = capture
            .stdout_str()
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| line.split('\t').next().map(|id| id.to_string()))
            .collect();

        //println!("{input_ids:?}");

        for id in input_ids {
            Cmd::new(format!("pactl move-sink-input {id} {device_id}")).run_success()?;
        }

        return Ok(());
    }

    println!("Couldn't find specified target sink.");

    Ok(())
}
