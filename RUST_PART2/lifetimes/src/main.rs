mod opt;
mod kirat;
mod struct1;
mod struct2;
mod struct3;

use lifetimes::lib;
use opt::opt;
use kirat::kirat;
use struct1::struct1;
use struct2::struct2;
use struct3::struct3;
fn main() {
    opt();
    lib();
    kirat();
    struct1();
    struct2();
    struct3();
}
