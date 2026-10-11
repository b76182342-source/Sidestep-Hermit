#!/usr/bin/env python3
"""
生成演示音频：一段简单的原创旋律
使用 Python 标准库 wave 和 math
"""

import wave
import math
import struct

# 参数
SAMPLE_RATE = 44100  # 采样率
DURATION = 15  # 总时长（秒）
AMPLITUDE = 0.3  # 振幅（0-1）

# 音符频率（A4 = 440Hz 的十二平均律）
NOTES = {
    'C4': 261.63,
    'D4': 293.66,
    'E4': 329.63,
    'F4': 349.23,
    'G4': 392.00,
    'A4': 440.00,
    'B4': 493.88,
    'C5': 523.25,
}

# 旋律：(音符, 时长)
MELODY = [
    ('C4', 0.5),
    ('E4', 0.5),
    ('G4', 0.5),
    ('C5', 0.5),
    ('G4', 0.5),
    ('E4', 0.5),
    ('C4', 1.0),
    ('D4', 0.5),
    ('F4', 0.5),
    ('A4', 0.5),
    ('F4', 0.5),
    ('D4', 1.0),
    ('C4', 0.5),
    ('E4', 0.5),
    ('G4', 1.0),
    ('C5', 2.0),
]


def generate_tone(frequency, duration, sample_rate=SAMPLE_RATE, amplitude=AMPLITUDE):
    """生成单个音符的波形"""
    num_samples = int(sample_rate * duration)
    samples = []

    for i in range(num_samples):
        # 生成正弦波
        t = i / sample_rate
        sample = amplitude * math.sin(2 * math.pi * frequency * t)

        # 添加包络（淡入淡出）
        envelope = 1.0
        fade_samples = int(sample_rate * 0.01)  # 10ms 淡入淡出
        if i < fade_samples:
            envelope = i / fade_samples
        elif i > num_samples - fade_samples:
            envelope = (num_samples - i) / fade_samples

        sample *= envelope

        # 转换为 16 位整数
        sample_int = int(sample * 32767)
        samples.append(sample_int)

    return samples


def verify_frequency(samples, expected_freq, sample_rate=SAMPLE_RATE):
    """使用简单的过零检测验证主频"""
    if len(samples) < sample_rate // 10:  # 至少 0.1 秒
        return None, None

    zero_crossings = 0
    check_samples = min(len(samples), sample_rate)  # 检查最多 1 秒

    for i in range(1, check_samples):
        if (samples[i-1] < 0 and samples[i] >= 0) or (samples[i-1] >= 0 and samples[i] < 0):
            zero_crossings += 1

    # 每个周期两个过零点
    duration_checked = check_samples / sample_rate
    detected_freq = zero_crossings / 2 / duration_checked
    error = abs(detected_freq - expected_freq) / expected_freq * 100

    return detected_freq, error


def main():
    print("生成演示音频...")

    all_samples = []

    for i, (note, duration) in enumerate(MELODY):
        freq = NOTES[note]
        print(f"  {i+1}/{len(MELODY)}: {note} ({freq:.2f}Hz) - {duration}s")

        samples = generate_tone(freq, duration)

        # 验证频率（仅对足够长的音符）
        detected, error = verify_frequency(samples, freq)
        if detected is not None:
            if error > 5:
                print(f"    警告: 频率偏差 {error:.1f}% (检测: {detected:.1f}Hz)")
            else:
                print(f"    验证通过: {detected:.1f}Hz (误差 {error:.1f}%)")

        all_samples.extend(samples)

    # 写入 WAV 文件
    output_file = "demo-melody.wav"
    with wave.open(output_file, 'wb') as wav_file:
        wav_file.setnchannels(1)  # 单声道
        wav_file.setsampwidth(2)  # 16 位
        wav_file.setframerate(SAMPLE_RATE)

        # 打包样本数据
        packed_samples = struct.pack('h' * len(all_samples), *all_samples)
        wav_file.writeframes(packed_samples)

    duration_actual = len(all_samples) / SAMPLE_RATE
    print(f"\n✓ 生成完成: {output_file}")
    print(f"  时长: {duration_actual:.2f}s")
    print(f"  大小: {len(all_samples) * 2 / 1024:.1f} KB")

    # 生成对应的 LRC 歌词
    lrc_file = "demo-melody.lrc"
    with open(lrc_file, 'w', encoding='utf-8') as f:
        f.write("[ti:Demo Melody]\n")
        f.write("[ar:Sidestep Hermit Project]\n")
        f.write("[al:Demo]\n")
        f.write("[by:Generated]\n")
        f.write("[00:00.00]原创演示旋律\n")
        f.write("[00:03.00]用于应用截图\n")
        f.write("[00:06.00]Sidestep Hermit\n")
        f.write("[00:09.00]背着壳\n")
        f.write("[00:12.00]横着走\n")

    print(f"✓ 生成歌词: {lrc_file}")
    print("\n使用方法:")
    print(f"  mv {output_file} ~/Music/demo/")
    print(f"  mv {lrc_file} ~/Music/demo/")


if __name__ == "__main__":
    main()
