fn main() {
    //2.2
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
        let mut vec_num = result;
        for _ in 0..vec_num.len() - 1 {
            for i in 0..vec_num.len() - 1 {
                if vec_num[i].0 > vec_num[i + 1].0 {
                    vec_num.swap(i, i + 1);
                }
            }
        }
        println!("{:?}", vec_num);

        //descending due to x
        vec_num.reverse();
        println!("{:?}", vec_num);

        //ascending due to y
        for _ in 0..vec_num.len() - 1 {
            for i in 0..vec_num.len() - 1 {
                if vec_num[i].1 > vec_num[i + 1].1 {
                    vec_num.swap(i, i + 1);
                }
            }
        }
        println!("{:?}", vec_num);

        //descending due to y
        vec_num.reverse();
        println!("{:?}", vec_num);
    }
}
