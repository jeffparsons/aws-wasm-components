#[allow(warnings)]
mod bindings;

use bindings::jeffparsons::ec2::ec2;

fn main() {
    ec2::hello_world();
}
