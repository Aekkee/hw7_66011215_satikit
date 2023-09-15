fn main() {
    //1.1
    let args: Vec<String> = std::env::args().collect();
    if args.get(1) == None {
    } else {
        let mut args: Vec<f32> = args[1..].iter().map(|x| x.parse().unwrap_or(0.)).collect();

        //ascending order
        args.sort_by(|x, y| x.partial_cmp(&y).unwrap());
        println!("{args:?}");

        //descending order
        args.sort_by(|x, y| y.partial_cmp(&x).unwrap());
        println!("{args:?}")
    }
}
