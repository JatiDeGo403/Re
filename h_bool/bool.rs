fn main(){
    let a: bool = true;
    let b: bool = false;

    print!("hidup jokowi: {}\n", a);
    print!("hidup prabowo: {}\n", b);

    if a && b {
        print!("saya menyukai seorang wanita\n");
    }else{
        print!("saya menyukai kamu \n");
    }

    if a == !(b){
        print!("mau bagaimana lagi \n");
    }else if a == b {
        print!("yo ndak tahu \n");
    }
}