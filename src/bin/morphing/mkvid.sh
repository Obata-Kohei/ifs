#!/bin/sh

width=128
height=128
fps=30
duration_sec=5

presets=("gasket" "fern" "tree" "dragon" "fern2" "square" "vicsek" "pentagon" "spiral" "snowflake")
presets=($(printf "%s\n" "${presets[@]}" | shuf))  # シャッフル

len=${#presets[@]}

for (( i=0; i<${len}; i++ )); do
    current=${presets[${i}]}
    next=${presets[$(( (i + 1) % len ))]}

    echo "Making morphing ${current} and ${next}"
    ./morphing.exe --n $((fps * duration_sec)) --width ${width} --height ${height} --src-ifs ${current} --dst-ifs ${next} --path result
    ffmpeg -framerate ${fps} -i result/%05d.png -c:v libx264 -pix_fmt yuv420p ${current}2${next}.mp4
done


# 1. 結合するファイルリスト（input.txt）を作成
list_file="file_list.txt"
> "$list_file" # ファイルを新規作成（または空にする）

for (( i=0; i<${len}; i++ )); do
    current=${presets[${i}]}
    next=${presets[$(( (i + 1) % len ))]}
    
    # ffmpeg用の書式 'file ファイル名' で書き込む
    echo "file '${current}2${next}.mp4'" >> "$list_file"
done

# 2. リストを元に結合
echo "Concatenating all videos..."
ffmpeg -f concat -safe 0 -i "$list_file" -c copy final_output.mp4

# 3. 使い終わったリストファイルを削除
rm "$list_file"
rm *2*.mp4
rm -rf ./result

echo "Done! Output: final_output.mp4"
