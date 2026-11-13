# penjelasan

* pada dasarnya rust diciptakan untuk mengatasi permasalahan dari bahasa C dan C++
yang mana bahasa tersebut merupakan bahasa tingkat rendah dengan kecepatan yang tinggi
tetapi tidak memiliki keamanan yang cukup memadai dalam hal --- saya lanjutkan setelah saya paham

## cara instalasi dari awal linux
* curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
* cek terminal dengan cara rustc --version, karena versi saya versi terbaru
maka output yang dikeluarkan versi rustc 1.96.0 , rustc merupakan compiler yang digunakan untuk menerjemahkan sebuah kode rust agar bisa dianalisa oleh komputer menjadi bahasa mesin.

### cara membuat sebuah file pekerjaan pertama
* lewat terminal
mkdir test
cd test
mkdir test1.rs

* cara compile 
rustc test1.rs
./test1