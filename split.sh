#!/bin/bash

# 检查是否有足够的参数传入
if [ "$#" -ne 2 ]; then
    echo "使用方法: $0 <视频文件> <分割份数>"
    exit 1
fi

# 输入的视频文件
input_video="$1"

# 要分割成的份数
split_count="$2"

# 检查分割份数是否为大于0的整数
if ! [[ "$split_count" =~ ^[1-9][0-9]*$ ]]; then
    echo "错误：分割份数必须是一个大于0的整数。"
    exit 1
fi

# 获取不带扩展名的原文件名
base_name=$(basename "$input_video" .mp4)

# 获取视频总时长（秒）
total_duration=$(ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 "$input_video")
total_duration=${total_duration%.*} # 转换为整数

# 计算每份视频的时长
split_duration=$((total_duration / split_count))

# 分割视频
for (( i=0; i<split_count; i++ )); do
    # 计算当前分割片段的开始时间
    start_time=$((i * split_duration))

    # 如果是最后一份，确保到视频结束
    if [ $((i + 1)) -eq $split_count ]; then
        split_duration=$((total_duration - start_time))
    fi

    # 输出文件名，包含原文件名和分割序号
    output_file="${base_name}_part$((i + 1)).mp4"

    # 使用ffmpeg分割视频
    ffmpeg -i "$input_video" -ss "$start_time" -t "$split_duration" -c copy "$output_file"

    echo "分割完成: $output_file"
done
