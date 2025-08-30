fn main() {
    // Fenster is header-only, no compilation needed
    // Just tell cargo to rerun if fenster.h changes
    println!("cargo:rerun-if-changed=fenster.h");
}