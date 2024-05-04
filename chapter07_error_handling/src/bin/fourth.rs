/**
 * error: `?` couldn't convert the error to `std::io::Error`
 *
 * numbers.push(line.parse()?);     // parsing integers can fail
 *                          ^
 *           the trait `std::convert::From<std::num::ParseIntError>`
 *           is not implemented for `std::io::Error`
 *
 * note: the question mark operation (`?`) implicitly performs a conversion
 * on the error value using the `From` trait
 */


fn main() {

}