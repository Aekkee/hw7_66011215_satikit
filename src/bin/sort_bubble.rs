fn main() {
    //1.2
    let args: Vec<String> = std::env::args().collect();
    if args.get(1) == None {
    } else {
        let mut args: Vec<f32> = args[1..].iter().map(|x| x.parse().unwrap_or(0.)).collect();

        //ascending order
        for _ in 0..args.len() - 1 {
            for i in 0..args.len() - 1 {
                if args[i] > args[i + 1] {
                    args.swap(i, i + 1);
                }
            }
        }
        println!("{args:?}");

        //descending order
        for _ in 0..args.len() - 1 {
            for i in 0..args.len() - 1 {
                if args[i] < args[i + 1] {
                    args.swap(i, i + 1);
                }
            }
        }
        println!("{args:?}")
    }
}
