mod opt;
mod spawn;
mod move_vec;

use multithreading::lib;
use opt::opt;
use spawn::spawn;
use move_vec::move_vec;
fn main() {
    opt();
    lib();
    spawn();
    move_vec();
}
