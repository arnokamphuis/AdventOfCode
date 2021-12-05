ffmpeg -framerate 2 -i boards-%05d.png -s:v 1280x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day04-part2-2fps.mp4


ffmpeg -i day05-10fps.mp4 -vf "fps=10,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day05-10fps.gif