use std::thread;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;
use mqtt_async_client::{
    client::{
        Client,
        Publish,
        QoS,
    },
    Result,
};
use tokio::{
    self,
    time::{
        Duration,
    },
};

#[derive(Serialize, Deserialize, Debug)]
struct SubMessage {
    name: String,
    cmd: String,
    randnum: f64,
}

fn plain_client() -> Result<Client> {
    Client::builder()
        .set_host("localhost".to_owned())
        .set_port(1883)
        .set_connect_retry_delay(Duration::from_secs(1))
        .build()
}

fn main() -> Result<()> {
    let mut rng = thread_rng();
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut client = plain_client()?;
        client.connect().await?;
        //let topic: String = String::from("DATATOPIC");
        for _ in 1..10 {
            let temp: f64 = rng.gen_range(20.0, 30.0);
            let msg = SubMessage {
                name: "MQTT_DEV".to_string(),
                cmd: "randnum".to_string(),
                randnum: temp,
            };
            let x = serde_json::to_string(&msg).unwrap();
            let mut p = Publish::new("DataTopic".to_owned(), x.as_bytes().to_vec());
            p.set_qos(QoS::AtMostOnce);
            client.publish(&p).await?;
            //println!("{}", serde_json::to_string(&msg).unwrap());
            thread::sleep(Duration::from_millis(1000));
        }
        client.disconnect().await?;
        Ok(())
    })
}

