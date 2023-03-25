use image_recog::buoy;
use image_recog::capture;
use image_recog::process;
use image_recog::server;
use image_recog::format;
use image_recog::log;

fn main() {
    let capture = capture::Capture::new();
    let image_cap = capture.get_color_frame();
    let processed = process::ImageProcess::new(image_cap);
    let server = server::WebServer::new();
    loop {
        log::time_log("");
        let buoys: buoy::AllBuoy = buoy::find_buoys(&processed, &capture);
        let heading = buoys.get_heading();
        let send_text: String = format::format(buoys, heading);
        log::log(&format!("{}", send_text.clone()));
        server.send_message(format!("{send_text}"));
    }
}
