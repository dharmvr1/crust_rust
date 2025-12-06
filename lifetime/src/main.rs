#[derive(Debug)]
pub struct CubeSat {
    id: u64,
}
#[derive(Debug)]
struct MailBox {
    message: Vec<Message>,
}

enum StatusMessage {
    Ok,
}
impl Copy for StatusMessage {}
impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

#[derive(Debug)]
struct Message {
    id: u64,
    content: String,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {
            id: self.id,
            content: self.content.clone(),
        }
    }
}

impl MailBox {
    fn post(&mut self, msg: Message) {
        self.message.push(msg);
    }

    fn deliver(&mut self, sat: &CubeSat) -> Option<Message> {
        for i in 0..self.message.len() {
            if self.message[i].id == sat.id {
                let msg = self.message.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = GroundStation {};

    let mut mail_box = MailBox { message: vec![] };

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message {
            id: sat.id,
            content: format!("hello to {}", sat_id),
        };
        base.send(&mut mail_box, msg);
    }
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail_box);
        println!("{:?},{:?}", sat, msg);
    }
}
