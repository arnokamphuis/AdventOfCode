ffmpeg -framerate 1200 -i rope_%05d.png -s:v 720x720 -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p day09-1200fps.mp4
ffmpeg -i day09-30fps.mp4 -vf "fps=30,scale=720:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" -loop 0 day09-30fps.gif
