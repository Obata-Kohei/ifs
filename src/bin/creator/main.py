import os
import shutil
import glob
from datetime import datetime
import subprocess
import cv2
import numpy as np
import mkbgm

# CONFIG
N_IMAGES = 100
FPS = 60
TITLE_TOTAL_DURATION = 3.0  # タイトルの合計時間（この時間でN枚を流す）
MAIN_FRAME_DURATION = 1.0   # 本編1枚あたりの表示時間
VIDEO_LENGTH = TITLE_TOTAL_DURATION + N_IMAGES * MAIN_FRAME_DURATION
FRACTAL_SIZE = (1080, 1080)
VIDEO_SIZE = (1080, 1920)

def generate_fractal_images(exe_path, output_dir, n, size) -> bool:
    os.makedirs(output_dir, exist_ok=True)
    command = [exe_path, "--n", str(n), "--width", str(size[0]), "--height", str(size[1]), "--path", output_dir]
    try:
        subprocess.run(command, check=True)
        return True
    except Exception:
        return False

def process_and_save_images(source_dir, title_dir, main_dir, date_str, video_size, image_size):
    """画像を加工してタイトル用と本編用の連番画像を別フォルダに保存する"""
    os.makedirs(title_dir, exist_ok=True)
    os.makedirs(main_dir, exist_ok=True)
    
    src_files = sorted(glob.glob(os.path.join(source_dir, "*.png")))
    x_off, y_off = (video_size[0] - image_size[0]) // 2, (video_size[1] - image_size[1]) // 2

    for i, fp in enumerate(src_files):
        img = cv2.imread(fp)
        if img is None: continue

        # --- 基本のキャンバスと通し番号 (共通) ---
        canvas = np.zeros((video_size[1], video_size[0], 3), dtype=np.uint8)
        canvas[y_off:y_off+image_size[1], x_off:x_off+image_size[0]] = img
        cv2.putText(canvas, f"{i:03d}", (video_size[0]//2 - 100, video_size[1]//2),
                    cv2.FONT_HERSHEY_SIMPLEX, 1.5, (255, 255, 255), 2, cv2.LINE_AA)

        # --- 本編用保存 ---
        cv2.imwrite(os.path.join(main_dir, f"frame_{i:04d}.png"), canvas)

        # --- タイトル用加工 (日付追加) ---
        title_canvas = canvas.copy()
        cv2.putText(title_canvas, date_str, (50, y_off - 20),
                    cv2.FONT_HERSHEY_SIMPLEX, 2.0, (255, 255, 255), 2, cv2.LINE_AA)
        cv2.imwrite(os.path.join(title_dir, f"frame_{i:04d}.png"), title_canvas)

    return len(src_files)

def run_ffmpeg_images_to_video(input_pattern, output_path, fps, duration_per_frame=None):
    """FFmpegを使用して画像群から動画を作成する"""
    # duration_per_frameが指定されている場合は本編（低速）、ない場合はタイトル（高速）
    if duration_per_frame:
        # 1枚につき指定秒数表示 (-framerate 1/秒)
        cmd = [
            'ffmpeg', '-y', '-framerate', str(1/duration_per_frame),
            '-i', input_pattern,
            '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-r', str(fps), output_path
        ]
    else:
        # 指定した合計時間で全画像を流し切る計算
        title_fps = N_IMAGES / TITLE_TOTAL_DURATION
        cmd = [
            'ffmpeg', '-y', '-framerate', str(title_fps),
            '-i', input_pattern,
            '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-r', str(fps), output_path
        ]
    subprocess.run(cmd, check=True, capture_output=True)

def concatenate_videos_ffmpeg(video_list, output_path):
    """FFmpegのconcat機能で動画を無劣化結合する"""
    list_file = "list.txt"
    with open(list_file, 'w') as f:
        for v in video_list:
            # FFmpeg用にパスのスラッシュを修正
            f.write(f"file '{os.path.abspath(v).replace('\\', '/')}'\n")
    
    cmd = ['ffmpeg', '-y', '-f', 'concat', '-safe', '0', '-i', list_file, '-c', 'copy', output_path]
    subprocess.run(cmd, check=True, capture_output=True)
    os.remove(list_file)

def combine_video_bgm(input_mp4, input_wav, output_mp4):
    """動画とBGMを結合する（動画はそのまま，音声を差し替え）"""
    cmd = [
        'ffmpeg', '-y',
        '-i', input_mp4,
        '-i', input_wav,
        '-c:v', 'copy',
        '-map', '0:v:0',
        '-map', '1:a:0',
        '-shortest',
        output_mp4
    ]
    subprocess.run(cmd, check=True, capture_output=True)




def main():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    now = datetime.now()
    ts = now.strftime("%Y-%m-%d")  #"%Y%m%d%H%M%S")
    date_display = now.strftime("%Y-%m-%d") # 動画内表示タイトル用日付
    
    work_dir = os.path.join(base_dir, "video", ts)
    raw_img_dir = os.path.join(work_dir, "raw")
    title_img_dir = os.path.join(work_dir, "title_imgs")
    main_img_dir = os.path.join(work_dir, "main_imgs")

    title_mp4 = os.path.join(work_dir, "title.mp4")
    main_mp4 = os.path.join(work_dir, "main.mp4")
    mp4_no_audio = os.path.join(work_dir, f"{ts}_noaudio.mp4")
    final_wav = os.path.join(work_dir, f"{ts}.wav")
    final_mp4 = os.path.join(work_dir, f"{ts}.mp4")

    # 1. フラクタル生成
    print("=== Generating Fractal Images ===")
    if not generate_fractal_images(os.path.join(base_dir, "creator.exe"), raw_img_dir, N_IMAGES, FRACTAL_SIZE):
        return

    # 2. 画像加工 (日付・通し番号)
    print("=== Processing Images ===")
    process_and_save_images(raw_img_dir, title_img_dir, main_img_dir, date_display, VIDEO_SIZE, FRACTAL_SIZE)

    # 3. FFmpegで各パート作成
    print("=== Encoding Videos with FFmpeg ===")
    run_ffmpeg_images_to_video(os.path.join(title_img_dir, "frame_%04d.png"), title_mp4, FPS)
    run_ffmpeg_images_to_video(os.path.join(main_img_dir, "frame_%04d.png"), main_mp4, FPS, MAIN_FRAME_DURATION)

    # 4. 結合
    print("=== Concatenating Videos ===")
    concatenate_videos_ffmpeg([title_mp4, main_mp4], mp4_no_audio)

    # 5. BGMを作成
    print("=== Generating BGM ===")
    mkbgm.make_bgm(VIDEO_LENGTH, final_wav)

    # 6. BGMと動画を合成
    print("=== Combining video and bgm ===")
    combine_video_bgm(mp4_no_audio, final_wav, final_mp4)

    # jsonを移動させる
    json_src = os.path.join(raw_img_dir, f"{os.path.basename(raw_img_dir)}.json") # raw_img_dirの中にある'raw.json'
    json_dst = os.path.join(work_dir, f"{ts}.json")

    if os.path.exists(json_src):
        shutil.move(json_src, json_dst)
    else:
        print(f"Warning: JSON file found at {json_src}")

    # クリーンアップ
    print("Cleaning up")
    shutil.rmtree(raw_img_dir); shutil.rmtree(title_img_dir); shutil.rmtree(main_img_dir)
    os.remove(title_mp4); os.remove(main_mp4); os.remove(mp4_no_audio); os.remove(final_wav)

    print(f"=== ALL PROCESS DONE ===\nFinal video: {mp4_no_audio}")

if __name__ == "__main__":
    main()