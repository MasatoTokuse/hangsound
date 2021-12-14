use ws::*;

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("opened")
    }

    fn on_message(&mut self, _: Message) -> Result<()> {
        self.out.broadcast("play_music")
    }
}

fn main() {
    // 第二引数はラムダ
    listen("127.0.0.1:3012", |out| Client { out: out } ).unwrap();
}