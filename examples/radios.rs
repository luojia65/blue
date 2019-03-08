fn main() {
    for radio in blue::radios() {
        println!("{:?}", radio);
        for device in radio.devices() {
            println!("{:?}", device);
        }
    }
}
