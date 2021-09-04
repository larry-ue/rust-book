fn main() {
    println!("Hello, world!");
    let south_germany = "Grub Gott";
    let japan = "symbols";
    let regions = [south_germany, japan];

    for region in regions.iter(){
        println!("{}", &region);
    }
}
