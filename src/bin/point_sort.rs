fn main() {
    //2.1
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<f32> = args[1..].iter().map(|x| x.parse().unwrap_or(0.)).collect();
    if args.is_empty() {
    } else {

        let mut result: Vec<(f32 , f32)> = Vec::new();
        for i in 0..args.len() {
            if i % 2 == 1 {
                result.push((
                    args[i - 1],
                    args[i],
                ));
            } else {
                continue;
            }
        }

        //ascending due to x
        result.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
        println!("{:?}", result);

        //descending due to x
        result.reverse();
        println!("{:?}", result);

        //ascending due to y
        result.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        println!("{:?}", result);

        //descending due to y
        result.reverse();
        println!("{:?}", result);
    }
}
