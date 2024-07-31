#!/bin/bash

# 检查参数数量
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 directory"
    exit 1
fi

# 输入和输出文件夹路径从命令行参数获取
dir="$1"

# 找到文件夹中的图片文件（支持jpg, png, jpeg格式）
image_file=$(find "$dir" -type f \( -iname \*.jpg -o -iname \*.png -o -iname \*.jpeg \))

# 遍历文件夹中的所有MP3文件
for audio_file in "$dir"/*.mp3
do
    # 获取音频文件的基础名（不包括路径和扩展名）
    base_name=$(basename "$audio_file" .mp3)

    # 获取图片的宽度和高度
    img_size=$(identify -format "%wx%h" "$image_file")

    # 确保宽度和高度都是2的倍数
    img_width=$(echo $img_size | cut -d'x' -f1)
    img_height=$(echo $img_size | cut -d'x' -f2)

    if (( img_width % 2 == 1 )); then
        img_width=$((img_width-1))
    fi

    if (( img_height % 2 == 1 )); then
        img_height=$((img_height-1))
    fi

    # 指定输出文件路径
    output_file="$dir/$base_name.mp4"

    # 使用ffmpeg将图片和音频合并成视频，并放入后台运行
    ffmpeg -loop 1 -i "$image_file" -i "$audio_file" -c:v libx264 -preset superfast -crf 30 -vf "scale=$img_width:$img_height" -tune stillimage -c:a copy -shortest "$output_file" &
done

# 等待所有后台任务完成
wait
