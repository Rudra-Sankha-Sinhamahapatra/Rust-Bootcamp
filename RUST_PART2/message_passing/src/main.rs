mod opt;
mod assignment1;
mod asn1_kirat;

use opt::opt;
use message_passing::lib;
use assignment1::assignment1;
use asn1_kirat::asn1_kirat;
fn main() {
  opt();
  lib();
  assignment1();
  asn1_kirat();
}
