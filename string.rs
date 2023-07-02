fn main(){

    let x = "2";

    let intx: u32 = x.parse().expect("failed to convert");

    println!("{}", intx + 4);
}
