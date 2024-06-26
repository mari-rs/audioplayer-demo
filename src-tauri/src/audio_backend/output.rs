use std::sync::mpsc::{Receiver, Sender};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


use super::process::Process;
use crate::event_handler::{GuiToProcessMsg, ProcessToGuiMsg};

pub fn spawn_cpal_stream(
    to_gui_tx: Sender<ProcessToGuiMsg>,
    from_gui_rx: Receiver<GuiToProcessMsg>
) -> cpal::Stream {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no output device available");

    let sample_rate = device.default_output_config().unwrap().sample_rate();

    let config = cpal::StreamConfig {
        channels: 2,
        sample_rate,
        buffer_size: cpal::BufferSize::Default,
    };

    let mut process = Process::new(to_gui_tx, from_gui_rx);
    
    let stream = device
        .build_output_stream(
            &config, 
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| process.process(data),
            move |err| {
                eprintln!("{}", err);
            },
            None
        )
        .unwrap();

    stream.play().unwrap();

    stream
}