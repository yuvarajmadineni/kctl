use rdkafka::admin::AdminOptions;

use crate::admin_kafka;

pub async fn delete_topic(topic_name: String) {
    let admin_client = admin_kafka::create_admin_client();
    let res = admin_client
        .delete_topics(&[&topic_name], &AdminOptions::default())
        .await;

    match res {
        Ok(inner_res) => match &inner_res[0] {
            Ok(data) => {
                println!("Topic {} deleted successfully", data);
            }

            Err(e) => {
                println!("Error while deleting topic {:?}", e)
            }
        },
        Err(e) => {
            println!("Error while deleting the topic {:?}", e)
        }
    }
}
