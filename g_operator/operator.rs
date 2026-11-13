fn main(){
    let a = 4;
    let b = 2;

    let c = a + b;
    print!("a + b = {}\n", c);
    let c = a - b;
    print!("a - b = {}\n", c);
    let c = a * b;
    print!("a * b = {}\n", c);
    let c = a / b;
    print!("a / b = {}\n", c);
    println!(" ");

    // operator assigment
    let mut c = 16;
    print!("{} nilai awal\n", c);

    c += 4;
    print!("{} jika menggunakan operator assigment (+=)\n", c);
    c -= 4;
    print!("{} jika menggunakan operator assigment (-=)\n", c);
    c *= 4;
    print!("{} jika menggunakan operator assigment (*=)\n", c);
    c /= 4;
    print!("{} jika menggunakan operator assigment (/=)\n", c);
    c %= 4;
    print!("{} jika menggunakan operator assigment (%=)\n", c);
    println!(" ");

    // comparasi operator
    let d = 19;
    let e = 20;
    print!("hasil dari != {}\n", d != e);
    print!("hasil dari == = {}\n", e == d);
    print!("hasil dari < = {}\n", d < e);
    print!("hasil dari > = {}\n", d > e);
    println!(" ");

    // logical operator
    let g = true;
    let h = false;
    if g && !(h) {
        print!("halah nyocot \n");
    }else{
        print!("itu benar\n");
    }

    if h || g {
        print!("who know \n");
    }else{
        print!("yes its Me\n");
    }
}