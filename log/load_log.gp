# png
set terminal pngcairo size 4950,1962 enhanced font 'Verdana,20'
set output 'log.png'

set border linewidth 2.0

set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 5 \
    pointtype 7 pointsize 1.5

unset key

set yrange [ARG3:ARG4]
set xrange [ARG1:ARG2]

plot 'log1.dat' with linespoints linestyle 1
