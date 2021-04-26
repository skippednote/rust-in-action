#[derive(Debug)]
struct CubeSet {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSet, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSet {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSet {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
