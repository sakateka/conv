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

conv (master) λ md5sum -c <<<"$(md5sum ../iconv.result|sed 's/iconv/conv/')"
../conv.result: OK

conv (master) λ /usr/bin/time -v iconv -c -f cp1251 -t utf8 ../1g.txt -o /dev/null
	...
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:06.09
	Maximum resident set size (kbytes): 1046644
	...
conv (master) λ /usr/bin/time -v target/release/conv -f cp1251 -t utf8 ../1g.txt -o /dev/null 
	...
	Elapsed (wall clock) time (h:mm:ss or m:ss): 0:32.92
	Maximum resident set size (kbytes): 10808
	...
```
![2018-02-04_00 08 53](https://user-images.githubusercontent.com/2256154/35769624-3b0bd84e-0940-11e8-8c41-88aa94c3fecc.png)
