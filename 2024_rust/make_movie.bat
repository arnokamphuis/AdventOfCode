ffmpeg -framerate 30 -i visible_%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day08-part1-30fps.mp4
ffmpeg -framerate 30 -i place_%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day08-part2-30fps.mp4
ffmpeg -framerate 5 -i crt_%05d.png -s:v 1600x240 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day10-part2-5fps.mp4

ffmpeg -framerate 120 -i search-part1-%05d.png -s:v 1344x328 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-part1-120fps.mp4
ffmpeg -framerate 120 -i search-part2-%05d.png -s:v 1344x328 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-part2-120fps.mp4

ffmpeg -framerate 240 -i search_%05d.png -s:v 1344x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-240fps.mp4


ffmpeg -framerate 240 -i field_part1_%05d.png -s:v 600x800 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day22-part1-240fps.mp4
ffmpeg -framerate 240 -i field_part2_%05d.png -s:v 600x800 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day22-part2-240fps.mp4

ffmpeg -i day09-30fps.mp4 -vf "fps=30,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day09-30fps.gif

ffmpeg -framerate 240 -i elfs_%05d.png -s:v 1256x1248 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day23-240fps.mp4


ffmpeg -framerate 1200 -i day06_24_%06d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day06-1200fps.mp4
ffmpeg -framerate 400 -i day06_24_%04d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day06-400fps.mp4


ffmpeg -framerate 30 -i day12_24_%06d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day12-30fps.mp4

ffmpeg -i day12-30fps.mp4 -vf "fps=30,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day12-30fps.gif


ffmpeg -framerate 200 -i day15_24_part_1_%06d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day15-part1-200fps.mp4
ffmpeg -framerate 200 -i day15_24_part_2_%06d.png -s:v 1440x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day15-part2-200fps.mp4
