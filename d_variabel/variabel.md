# variabel penjelasan:

* let pada dasarnya merupakan sebuah kata kunci (keyword) yang mengunci sebuah variabel tergantung dari nilai yang diberikan kepada variabel tersebut, sebagai contoh:
let x = "empat";
let x = 4;
hal tersebut akan dapat menghasilkan output empat dan 4 jika diprint, tetapi pada kenyataannya akan terjadi runtime error, berupa unused x, hal tersebut dapat terjadi karena x pertama yang dideklarasikan tidak dapat dipanggil karena sudah di timpa menggunakan x yang bernilai 4, untuk mengatasi hal tersebut saya menggunakan print! yang saya buat 2 dibawah x pertama.

* {} (bracket), bracket tersebut gunanya adalah tempat dimana nilai tersebut akan dicetak, pada {} memang terserah untuk penempatannya boleh dimana saja, akan tetapi pada pemanggilan variabel akan mempengaruhi output dari bracket tersebut, sebagai contoh:
let a = "saya";
let b = "fara";

print!("{} sayang {}\n", a, b);
outputnya = saya sayang fara

akan tetapi jika
print!("{} sayang {}\n", b, a);
outputnya = fara sayang saya

oleh karena itu pemanggilan variabel yang menyimpan sebuah value sangat diperhatikan agar output yang dikeluarkan tidak ambigu seperti dicontoh.