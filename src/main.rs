extern crate notify_rust;

fn main() {
    let mut server = notify_rust::server::NotificationServer::new();
    server.start(|n| {
         if ! n.summary.is_empty() {
            print!("{}", n.summary);
         }

         if ! n.body.is_empty() {
            if ! n.summary.is_empty() {
                // Maybe this is an impossible case?
                // notify-send won't allow it, at least
                print!(" ");
            }
            print!("{}", n.body);
         }
         print!("\n");
    });
}
