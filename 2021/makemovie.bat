ffmpeg -framerate 100 -i day09-%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day09-part2-100fps.mp4


ffmpeg -i day05-10fps.mp4 -vf "fps=10,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day05-10fps.gif

ffmpeg -i day09-part2-200fps.mp4 -vf "fps=200,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day09-part2-200fps.gif