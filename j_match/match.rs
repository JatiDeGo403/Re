fn main(){
    let a: i32 = 4;

    let b = match a {
        1 => "apalah",
        2 => " hmmm",
        3 => "ya sudahlah",
        4 => "aku sendirian",
        _ => "aku tidak tahu",
    };

    print!("{}\n", b);
}