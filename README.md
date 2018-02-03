# conv
toy char converter
```
conv (master) λ curl -o ../book1.txt http://vojnaimir.ru/files/book1.txt
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 1439k  100 1439k    0     0   719k      0  0:00:02  0:00:02 --:--:--  556k
conv (master) λ i=0; while ((i++ < 20)); do cat ../book1.txt >> ../big.txt; done; ls -lh ../big.txt
-rw-r--r-- 1 user user 29M Feb  3 18:08 ../big.txt
conv (master) λ time iconv -f cp1251 -t utf8 ../big.txt -o ../iconv.result

real	0m0.211s
user	0m0.159s
sys	0m0.053s
conv (master) λ time target/release/conv -f cp1251 -t utf8 ../big.txt -o ../conv.result

real	0m0.950s
user	0m0.917s
sys	0m0.032s
conv (master) λ md5sum ../*conv.result
2d085340c12689a5586ec26163ade7ba  ../conv.result
2d085340c12689a5586ec26163ade7ba  ../iconv.result
```
