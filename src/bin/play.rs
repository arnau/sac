extern crate url;

use url::Url;

fn main() {
    let base = Url::parse("http://dummy.org/records/").unwrap();
    let u = base.join("type=bar").unwrap();

    println!("{:?}", u);
    println!("{:?}", u.path());
    println!("{:?}", u.path_segments());
}
