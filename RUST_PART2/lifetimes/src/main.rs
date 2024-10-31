mod opt;
mod kirat;

use lifetimes::lib;
use opt::opt;
use kirat::kirat;
fn main() {
    opt();
    lib();
    kirat();
}
