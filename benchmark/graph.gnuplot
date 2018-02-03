# vim: ft=gnuplot

set xzeroaxis lt -1
set yzeroaxis lt -1
set xtics axis
set ytics axis
#set xrange [0:2100]
set yrange [0:100]
set xlabel 'Size, kb'
set ylabel 'Time, ms'

set style line 1 lt 1 lw 2 lc rgb '#F44336' pt 7 ps 1
set style line 2 lt 1 lw 2 lc rgb '#009688' pt 7 ps 1
set style line 3 lt 1 lw 2 lc rgb '#2196F3' pt 7 ps 1
set style line 4 lt 1 lw 2 lc rgb '#F57F17' pt 7 ps 1
set grid xtics lc rgb '#555555' lw 1 lt 0
set grid ytics lc rgb '#555555' lw 1 lt 0
set key autotitle columnhead
set key top right
set grid
plot 'result.txt' using 1:2 with linespoints ls 1 ti 'iconv', \
     'result.txt' using 1:3 with linespoints ls 2 ti 'conv (-O1)', \
     'result.txt' using 1:4 with linespoints ls 3 ti 'conv (-O2)', \
     'result.txt' using 1:5 with linespoints ls 4 ti 'conv (-O3)'
