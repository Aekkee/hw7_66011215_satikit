fn main() {

    //3.1 and 3.2 use same argument
    let args: Vec<String> = std::env::args().collect();

    //3.1
    
    if args.get(1) == None {
    } else {
        let str_p: i32 = args[1].parse().unwrap_or(0);
        let end_p: i32 = args[2].parse().unwrap_or(0);
        let d: i32 = args[3].parse().unwrap_or(0);
        println!("<style>\ntable, td, th {{\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}}\n</style>\n");
        println!("<table>");
        println!("\t<tr>\n\t\t<th>Fahr</th>\n\t\t<th>Celcius</th>\n\t</tr>");
        if str_p < end_p {
            for i in 0..=(str_p - end_p).abs() / d {
                println!(
                    "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{:.2}</td>\n\t</tr>",
                    str_p + (i) * d,
                    5.0 / 9.0 * ((str_p as f32 + i as f32 * d as f32) - 32.0)
                );
            }
        } else {
            for i in 0..=(str_p - end_p).abs() / d {
                println!(
                    "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{:.2}</td>\n\t</tr>",
                    str_p - (i) * d,
                    5.0 / 9.0 * ((str_p as f32 - i as f32 * d as f32) - 32.0)
                );
            }
        }
        println!("</table>");

    //3.2

        println!("<table>");
        println!("\t<tr>\n\t\t<th>x</th>\n\t\t<th>x^2</th>\n\t\t<th>x^3</th>\n\t</tr>");
        if str_p < end_p {
            for i in 0..=(str_p - end_p).abs() / d {
                let num = str_p + (i) * d;
                println!(
                    "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                    num,
                    num.pow(2),
                    num.pow(3)
                );
            }
        } else {
            for i in 0..=(str_p - end_p).abs() / d {
                let num = str_p - (i) * d;
                println!(
                    "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                    num,
                    num.pow(2),
                    num.pow(3)
                );
            }
        }
        println!("</table>");
    }
}
