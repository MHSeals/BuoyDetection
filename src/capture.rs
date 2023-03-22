use realsense_rust::{
    config::Config,
    context::Context,
    device::Device,
    frame::{ColorFrame, CompositeFrame, DepthFrame},
    kind::{Rs2CameraInfo, Rs2Format, Rs2ProductLine, Rs2StreamKind},
    pipeline::{ActivePipeline, InactivePipeline},
};

use std::{
    collections::HashSet,
    ffi::CStr,
    sync::mpsc::{self, Receiver, SyncSender},
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

pub struct Capture {
    stream_thread: JoinHandle<()>,
    signal: SyncSender<FrameType>,
    death_signal: SyncSender<u8>,
    depth_receiver: Receiver<DepthFrame>,
    color_receiver: Receiver<ColorFrame>,
}

enum FrameType {
    DEPTH,
    COLOR,
}

impl Capture {
    pub fn new() -> Self {
        let (send_signal, receive_signal) = mpsc::sync_channel::<FrameType>(0);
        let (color_send, color_receive) = mpsc::sync_channel::<ColorFrame>(0);
        let (depth_send, depth_receive) = mpsc::sync_channel::<DepthFrame>(0);
        let (death_send, death_receive) = mpsc::sync_channel::<u8>(0);
        let stream_thread = thread::spawn(move || {
            let mut owned_devices: HashSet<Rs2ProductLine> = HashSet::new();
            owned_devices.insert(Rs2ProductLine::D400);
            let context: Context = Context::new().unwrap();
            let mut connected_devices: Vec<Device> = context.query_devices(owned_devices.clone());
            let usb_wait: Duration = Duration::from_millis(100);
            while connected_devices.is_empty() {
                connected_devices = context.query_devices(owned_devices.clone());
                sleep(usb_wait);
            }
            println!("Camera connected");
            let pipeline: InactivePipeline = InactivePipeline::try_from(&context).unwrap();
            let mut configuration: Config = Config::new();

            let usb_connection: &CStr = connected_devices[0]
                .info(Rs2CameraInfo::UsbTypeDescriptor)
                .unwrap();
            let usb_type: f32 = usb_connection.to_str().unwrap().parse::<f32>().unwrap();
            configuration
                .enable_device_from_serial(
                    connected_devices[0]
                        .info(Rs2CameraInfo::SerialNumber)
                        .unwrap(),
                )
                .unwrap()
                .disable_all_streams()
                .unwrap()
                .enable_stream(Rs2StreamKind::Depth, None, 640, 0, Rs2Format::Z16, 30)
                .unwrap()
                .enable_stream(Rs2StreamKind::Color, None, 640, 0, Rs2Format::Rgba8, 30)
                .unwrap();
            let mut pipeline: ActivePipeline = pipeline.start(Some(configuration)).unwrap();
            let timeout = Duration::from_millis(1000);
            let loop_sleep = Duration::from_millis(10);
            'mainloop: loop {
                let death_result = death_receive.try_recv();
                let _death = match death_result {
                    Ok(_life) => {
                        panic!("Main loop killed");
                    }
                    Err(death) => death,
                };
                let frame_type_result = receive_signal.try_recv();
                let frame_type = match frame_type_result {
                    Ok(ftype) => ftype,
                    Err(_error) => {
                        thread::sleep(loop_sleep);
                        continue 'mainloop;
                    }
                };
                let frames: CompositeFrame = pipeline.wait(Some(timeout)).unwrap();
                match frame_type {
                    FrameType::DEPTH => {
                        let mut depth_frames = frames.frames_of_type::<DepthFrame>();
                        if !depth_frames.is_empty() {
                            depth_send.send(depth_frames.pop().unwrap()).unwrap();
                        }
                    }
                    FrameType::COLOR => {
                        let mut color_frames = frames.frames_of_type::<ColorFrame>();
                        if !color_frames.is_empty() {
                            color_send.send(color_frames.pop().unwrap()).unwrap();
                        }
                    }
                }
            }
        });
        return Self {
            stream_thread,
            signal: send_signal,
            depth_receiver: depth_receive,
            color_receiver: color_receive,
            death_signal: death_send,
        };
    }
    pub fn get_depth_frame(&self) -> DepthFrame {
        let _result = self.signal.send(FrameType::DEPTH);
        return self.depth_receiver.recv().unwrap();
    }
    pub fn get_center_depth(&self) -> f32 {
        let mut distance: f32 = 0.0;
        let depth_frame: DepthFrame = self.get_depth_frame();
        let tmp_distance: f32 = depth_frame
            .distance(depth_frame.width() / 2, depth_frame.height() / 2)
            .unwrap();
        if tmp_distance != 0.0 {
            distance = tmp_distance;
        }
        return distance;
    }
    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        if x > 640 || y > 480 {
            return 0.0;
        }
        let mut distance: f32 = 0.0;
        let depth_frame: DepthFrame = self.get_depth_frame();
        let tmp_distance: f32 = depth_frame.distance(x, y).unwrap();
        if tmp_distance != 0.0 {
            distance = tmp_distance;
        }
        return distance;
    }
    pub fn get_color_frame(&self) -> ColorFrame {
        let _result = self.signal.send(FrameType::COLOR);
        return self.color_receiver.recv().unwrap();
    }
    pub fn end(&mut self) {
        self.death_signal.send(1).unwrap();
    }
}

pub fn stream() {
    let mut capture: Capture = Capture::new();
    println!("{}", capture.get_depth(100, 100));
    capture.end();
}
