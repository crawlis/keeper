pub struct NatsSubscriber {
    conn: nats::Connection,
    sub: nats::Subscription,
}

impl NatsSubscriber {
    pub fn new(uri: &str, subject: &str) -> std::io::Result<NatsSubscriber> {
        let conn = nats::connect(uri)?;
        let sub = format!("{}.*", subject);
        let sub = conn.queue_subscribe(&sub, "keeper")?;
        Ok(NatsSubscriber { conn, sub })
    }
    pub fn get_next_message(&self) -> Option<nats::Message> {
        self.sub.next()
    }
    pub fn close(self) {
        self.conn.close()
    }
}
