fn main(){
    /* pada dasarnya tipe data dirust sangat banyak ukuranya 
    baik itu desimal maupun bilangan utuh atau minus, tetapi saya akan
    memberikan beberapa saja */

    let a = "empat"; // tipe data variabel a merupakan String,
    //  bisa dilihat dari penggunaan petik 2 dan saat di space value tetap tidak berubah
    let b = 'd'; // tipe data variabel b merupakan char,
    // bisa dilihat dari penggunaat petik 1 dan saat di space value berubah, char hanya bisa menyimpan 1 huruf saja tanpa space
    let c = 11.11; // saya kurang tahu pembeda double dan float jika menggunakan let, 
    // jika sudah tahu maka akan saya tambahkan penjelasan di md
    let d = 1; // variabel d memiliki tipe data int, int di rust dapat dipanggil dengan berbagai cara
    // int8, int16, int32, int64, int128 semakin besar jumlah bits dibelakang int maka nilai maximum yang dipanggil
    // dapat menampung hingga quadriliun
    let e = true; // variabel e memiliki tipe data bool (boolean) yang hanya bernilai true atau false saja

    print!("{} {} \n{}\n{}\napakah aku dewa: {}\n", a,b,c,d,e);
}