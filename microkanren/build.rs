use std::io;

fn main () -> io::Result <()> {
    tangle::tangle_all_before_build ()
}
