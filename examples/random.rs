use serialstudio::{
    data::{DataGroup, TelemetryFrame, DataSet},
    SerialStudioSource,
};
use serde_json::json;

fn main() {
    // Create server
    let mut server = SerialStudioSource::new();

    // Start
    server.start("localhost:8019".to_string());

    // Data loop
    loop {
        // Get a random-ish value (using only std)
        let random = Box::into_raw(Box::new(123)) as usize;

        // Send a frame
        server.publish(TelemetryFrame {
            title: "Random number generator".to_string(),
            groups: vec![DataGroup {
                title: "Main group".to_string(),
                widget_type: None,
                datasets: vec![
                    DataSet {
                        title: "Random Number".to_string(),
                        value: json!(random),
                        graph:None,
                        unit: None,
                        w_type: None
                    }
                ]
            }],
        })
    }
}
