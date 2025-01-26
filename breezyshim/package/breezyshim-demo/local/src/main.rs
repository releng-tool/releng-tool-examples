// SPDX-License-Identifier: 0BSD

fn main() {
    use breezyshim::branch::open as open_branch;
    breezyshim::plugin::load_plugins();
    let b = open_branch(&"https://code.launchpad.net/brz".parse().unwrap()).unwrap();
    println!("Last revision: {:?}", b.last_revision());
}
