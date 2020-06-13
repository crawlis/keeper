pub struct NatsSubscriber {
    conn: nats::Connection,
    sub: nats::Subscription,
}

impl NatsSubscriber {
    pub fn new(uri: &str, subject: &str) -> Result<NatsSubscriber, std::io::Error> {
        let conn = nats::connect(uri)?;
        let sub = format!("{}.*", subject);
        let sub = conn.queue_subscribe(&sub, "keeper")?;
        Ok(NatsSubscriber { conn, sub })
    }
    pub fn get_next_message(&self) -> Option<nats::Message> {
        self.sub.try_next()
    }
    pub fn close(self) -> Result<(), std::io::Error> {
        self.conn.close()
    }
}
