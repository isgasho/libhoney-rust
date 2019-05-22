use libhoney;
use libhoney::FieldHolder;

fn main() {
    let mut client = libhoney::init(libhoney::Config {
        options: libhoney::ClientOptions {
            api_key: std::env::var("HONEYCOMB_API_KEY").unwrap(),
            dataset: std::env::var("HONEYCOMB_DATASET").unwrap(),
            ..Default::default()
        },
        transmission_options: libhoney::TransmissionOptions::default(),
    });
    let mut event = client.new_event();
    event.add_field("extra", libhoney::Value::String("wheeee".to_string()));
    event.add_field("extra_ham", libhoney::Value::String("cheese".to_string()));
    event.send(&mut client);
    let response = client.responses().iter().next();
    println!("sent event, got back responses: {:#?}", response);
}
