use ws::*;

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("opened")
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let msg = match message.as_text() {
            Ok(v) => v,
            Err(e) => panic!("Problem message.as_text: {:?}", e),
        };
        if msg == "play" {
            return self.out.broadcast("play_music");
        }
        if msg == "ping" {
            return self.out.send("pong");
        }
        Ok(())
    }
}

fn main() {
    // 第二引数はラムダ
    listen("127.0.0.1:3012", |out| Client { out: out }).unwrap();
}
