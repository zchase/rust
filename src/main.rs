#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world from Distelli!"
        }
    });

    server.listen("127.0.0.1:6767");
}