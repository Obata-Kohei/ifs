# creatorで出力したフラクタル画像を一つの動画にするスクリプト．
# creator実行ファイルとこのpythonスクリプトは同じディレクトリにあることを想定している．

import os
import glob
from datetime import datetime
import subprocess
import cv2
import numpy as np

# 設定
n = 11
fps = 1 / 1
fractal_image_size = (1080, 1080)  # 作成されるフラクタルの画像サイズ(width, height)
final_video_size = (1080, 1920)    # 動画サイズ．(width, height)

def main():
    # 実行時ディレクトリの作成
    dir_name = f"{datetime.now():%Y%m%d%H%M%S}"

    base_dir = os.path.dirname(os.path.abspath(__file__))
    creator_path = os.path.join(base_dir, "creator.exe")

    # 画像生成 (creator.exeの呼び出し)
    print("IFS images generation")
    command = [
        creator_path,
        "--n", str(n),
        "--width", str(fractal_image_size[0]),
        "--height", str(fractal_image_size[1]),
        "--path", os.path.join(base_dir, dir_name),
    ]

    try:
        subprocess.run(command, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error in image generation: {e}")
        return

    #  動画ライターの初期化
    video_path = f"{base_dir}/{dir_name}/{dir_name}.mp4"
    # コーデックの設定 (MP4用)
    fourcc = cv2.VideoWriter_fourcc(*'mp4v')
    video_writer = cv2.VideoWriter(video_path, fourcc, fps, final_video_size)

    # 画像の加工と動画への書き込み
    print("Procedding images")
    image_files = sorted(glob.glob(os.path.join(base_dir, dir_name, "*.png")))

    if not image_files:
        print("No images found to process.")
        return

    for i, file_path in enumerate(image_files):
        img = cv2.imread(file_path)
        if img is None:
            continue

        # 黒い背景キャンバスを作成
        canvas = np.zeros((final_video_size[1], final_video_size[0], 3), dtype=np.uint8)

        # 中央に配置
        x_offset = (final_video_size[0] - fractal_image_size[0]) // 2
        y_offset = (final_video_size[1] - fractal_image_size[1]) // 2
        canvas[y_offset:y_offset+fractal_image_size[1], x_offset:x_offset+fractal_image_size[0]] = img

        # 通し番号を描画
        text = f"{i+1:03d}"
        cv2.putText(canvas, text, (50, 100), cv2.FONT_HERSHEY_SIMPLEX, 
                    1.5, (255, 255, 255), 2, cv2.LINE_AA)

        # 動画の1フレームとして書き込み
        video_writer.write(canvas)

    # 最後に必ず解放する
    video_writer.release()
    print(f"Video made at: {video_path}")


if __name__ == "__main__":
    main()