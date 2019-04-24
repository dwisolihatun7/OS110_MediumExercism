# OS110_MediumExercism
Medium Exercusm Rust

Saya akan menuliskan esay tentang pemecahan masalah scrabble-score untuk memenuhi tugas mata kuliah Sitem Operasi semester 110

Scrabble Score
Diberikan sebuah kata, kemudian diminta untuk menghitung jumlah score dari kata tersebut berdasarkan nilai dari
setiap huruf yang sebelumnya telah ditentukan.

Solve the problem
Yang pertama adalah, kita harus perhatikan nilai dari setiap huruf yang telah ditentukan. 
Dengan ketentuan sebagai berikut:

a,e,i,o,u,l,n,r,s,t => 1,
d,g => 2,
b,c,m,p => 3,
f, h, v, w, y => 4,
k => 5,
j, x => 8,
q, z => 10,


Jika di dalam kata tersebut terdat pengulangan huruf, maka tetap dihitung sesuai ketentuan.
Jika huruf tersebut muncul 2 kali (double), maka dihitung 2 kali,
jika huruf itu muncul 3 kali (triple), maka dihitung 3 kali.
Dalam scrabble-score.rs , kita diminta membuktikan scrabble score suatu kata, salah satunya STRASSE = 7
Tentu benar karena
S = 1 (triple), T = 1, R=1, A=1, E=1
1*3 (karena muncul 3 kali) +1+1+1+1+1 =7
Dalam pemecahannya, kata yang akan dihitung scrabble score nya, setiap hurufnya di ubah ke huruf kecil(lowecase) untuk memudahkan dibaca oleh sistem.
Sehingga scorenya untuk huruf kapital dan huruf kecilnya huruf tersebut sama, serta memuat code yang dibuild juga tidak terlalu panjang.
