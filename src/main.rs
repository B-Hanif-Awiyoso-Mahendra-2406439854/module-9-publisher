use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use std::{env, thread, time::Duration};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    let amqp_url =
        env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_owned());
    let mut p = loop {
        match CrosstownBus::new_queue_publisher(amqp_url.clone()) {
            Ok(publisher) => break publisher,
            Err(error) => {
                println!("Waiting for RabbitMQ at {amqp_url}: {error:?}");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };

    _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406439854-Amir".to_owned(),
        },
    );
    _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406439854-Budi".to_owned(),
        },
    );
    _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406439854-Cica".to_owned(),
        },
    );
    _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406439854-Dira".to_owned(),
        },
    );
    _ = p.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406439854-Emir".to_owned(),
        },
    );
}
