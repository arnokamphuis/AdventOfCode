ffmpeg -framerate 2 -i part2-%05d.png -s:v 1280x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day03-part2-2fps.mp4
