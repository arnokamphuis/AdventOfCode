set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 1 \
    pointtype 7 pointsize 0

set style line 2 \
    linecolor rgb '#ad6000' \
    linetype 1 linewidth 1 \
    pointtype 7 pointsize 0

plot 'line1.txt' with linespoints linestyle 1, 'line2.txt' with linespoints linestyle 1