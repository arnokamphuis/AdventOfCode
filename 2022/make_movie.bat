ffmpeg -framerate 30 -i visible_%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day08-part1-30fps.mp4
ffmpeg -framerate 30 -i place_%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day08-part2-30fps.mp4
ffmpeg -framerate 5 -i crt_%05d.png -s:v 1600x240 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day10-part2-5fps.mp4

ffmpeg -framerate 120 -i search-part1-%05d.png -s:v 1344x328 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-part1-120fps.mp4
ffmpeg -framerate 120 -i search-part2-%05d.png -s:v 1344x328 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-part2-120fps.mp4

ffmpeg -framerate 240 -i search_%05d.png -s:v 1344x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-240fps.mp4

ffmpeg -framerate 1200 -i rope_%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day09-1200fps.mp4
ffmpeg -i day09-30fps.mp4 -vf "fps=30,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day09-30fps.gif
