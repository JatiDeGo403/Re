fn main(){
    let mut a: i32 = 4;

    while a <= 7{
        print!("aku sayang kamu\n");

        a += 1;
    }

    let mut b: i32 = 1;
    while b <= 4{
        print!("{} \t", b);
        b += 1;
    }
    println!();

    let mut b: i16 = 1;
    while b <= 4{
        if b == 2 {
            b += 1;
            print!("\t");
            continue;
        }

        if b == 3 {
            b += 1;
            print!("\t");
            continue;
        }

        print!("{}\t", b);
        b += 1;
    }
    println!();

    let mut b: i16 = 1;
    while b <= 4{
        if b == 2 {
            b += 1;
            print!("\t");
            continue;
        }

        if b == 3 {
            b += 1;
            print!("\t");
            continue;
        }

        print!("{}\t", b);
        b += 1;
    }
    println!();

    let mut b: i32 = 1;
    while b <= 4{
        print!("{} \t", b);
        b += 1;
    }
    println!();

    for i in 1..=6{
        print!("{}\t", i);
    }
    println!();

    for i in 1..=5{
        for j in 1..=(5-i){
            print!("{}\t", j);
        }
        println!();
    }

    for i in 1..=5{
        for j in 1..=(5 - i){
            if j == 2 || j == 3 || j == 4{
                print!("{}\t", j);
            }else{
                print!("\t");
            }
        }
        println!();
    }
}