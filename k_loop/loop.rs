fn main(){
    let mut a: i16 = 0;

    let b = loop {
        println!(" hidup jokowi");

        if a == 4{
            break a;
        }

        a += 1;
    };
    print!("stop pada angka: ");
    print!("{}\n", b);
}