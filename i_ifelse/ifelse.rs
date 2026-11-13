fn main(){
    let a: i8 = 16;
    let b: i8 = 4;

    // single option
    if (a + b ) == 20{
        print!(" aku sayang kamu \n");
    }else{
        print!("aku benci kamu\n");
    }

    //double or many option
    if a == 16{
        print!(" hmm \n",);
        if b == 4 {
            print!(" kira kira\n");
            if a <= 4{
                print!("oke gas\n");
            }else{
                print!(" hidup jokowi\n");
            }
        }else{
            print!(" apalah \n")
        }
    }else{
        print!(" ya saya tahu\n");
    }

    // nodiff
    let c = if a < 9{
        " apa salah"
    }else if a < 15{
        " apaa salah"
    }else if a < 0{
        " apaaa salahh"
    }else if a < 17{
        " apaaa benarrr"
    }else{
        "tidak ada opsi"
    };

    print!("{}\n", c);
}