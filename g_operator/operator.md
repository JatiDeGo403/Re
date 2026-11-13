# operator md

* operator
- digunakan untuk melakukan operasi matematika sederhana berupa pengurangan
+ digunakan untuk melakukan operasi matematika sederhana berupa penambahan
/ digunakan untuk melakukan operasi matematika sederhana berupa pembagian 
* digunakan untuk melakukan operasi matematika sederhana berupa perkalian

## operator assigment
== operator tersebut digunakan untuk memberikan sebuah nilai bool (true atau false) jika dilakukan sebuah perbandingan, operator ini bisa digunakan pada tipe data apapun, selama nilainya sama dengan yang di compare hasilnya tetap true. kecuali di !()

!= operator tersebut masih sama seperti dengan == tetapi jika hasilnya true maka akan dibalik ke false begitu pula sebaliknya.

> dan < digunakan pada saat ingin membandingkan 2 atau lebih variabel untuk menentukan apakah itu akan menghasilkan benar atau tidaknya dari perbandingan tersebut.

## logical operator
&& operator tersebut hanya bisa digunakan apabila tipe data dari variabel menggunakan bool, apabila tidak maka terjadi compile error
|| operator tersebut biasa digunakan saat salah satu variabel bernilai true tetapi ingin mengeluarkan output print pertama, analoginya sebagai berikut:
1 || 0 = true
0 || 1 = true
1 || 1 = true
0 || 0 = false

! not operator tersebut digunakan saat ingin membuat sebuah value yang seharusnya bertipe true menjadi false, contoh penerapan:
let a = true;
print!("{}\n", a);
outputnya: true
jika digunakan !
print!("{}\n", !(a));
outputnya: false