// rfetch

// ASCII ART
const fedora = [
    "\x1b[0;94m",
    "       ,'''''.   ",
    "       |   ,.  | ",
    "       |  |  '_' ",
    "  ,....|  |..    ",
    ".'  ,_;|   ..'   ",
    "|  |   |  |      ",
    "|  ',_,'  |      ",
    " '.     ,'       ",
    "   '''''         "
];
const distro = fedora;

fn main() {
    println!("{}{}", distro[0], distro[1]);
    println!("{}{}", distro[0], distro[2]);
    println!("{}{}", distro[0], distro[3]);
    println!("{}{}", distro[0], distro[4]);
    println!("{}{}", distro[0], distro[5]);
    println!("{}{}", distro[0], distro[6]);
    println!("{}{}", distro[0], distro[7]);
    println!("{}{}", distro[0], distro[8]);
}
