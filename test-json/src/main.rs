use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct BootNotificationRequest {
    #[serde(rename = "chargePointVender")]
    charge_point_vender: String,

    iccid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Payload {
    BootNotification(BootNotificationRequest),
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    action: String,
    payload: Value,
}

fn main() {
    let data = r#"{"action":"boot","payload":{"chargePointVender":"vendor","iccid":"4343444"}}"#;
    let result: Message = serde_json::from_str(data).unwrap();

    let boot_req: BootNotificationRequest = serde_json::from_value(result.payload).unwrap();
    println!("{:?}", boot_req);

    // let boot = BootNotification {
    //     charge_point_vender: "vendor".into(),
    //     iccid: Some("4343444".into()),
    // };
    // let message = Message {
    //     action: "boot".into(),
    //     payload: boot,
    // };
    // let result = serde_json::to_string(&message).unwrap();
    // println!("result {}", result);
}
