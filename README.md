## Epub file to Speech

This tool can help you to convert text file to speech.

### Usage


1. Download the release from [release page](https://github.com/smark-d/epub-to-speech/releases)
2. Convert the file to mp3 file

```bash

unzip epub-to-speech.zip

./epub-to-speech demo.txt
```
You can also convert the folder, it will convert all the text file in the folder.

```bash
./epub-to-speech ./
```

## Convert mp3 to mp4

This tool allows you to convert MP3 files into MP4 videos by adding a static image to each audio file.

### Usage

1. Download the script from source code [ffmpeg_cov.sh](./bin/ffmpeg_cov.sh).

2. Make sure the script has execute permissions:

```bash
chmod +x ./ffmpeg_cov.sh
```

3. Run the script on a directory containing the MP3 and image files:

```bash
./ffmpeg_cov.sh /path/to/directory
```
Replace `/path/to/directory` with the path to the directory that contains your MP3 and image files. The script will find all MP3 files and an image in the specified directory, then merge each MP3 file with the image into an MP4 file and save them in the same directory.

Please ensure that there is only one image file in the directory. The image file should be in jpg, png, or jpeg format.

> Note: This script uses ffmpeg for conversion, so make sure it's installed on your system before running the script.

Example:

I have a directory called `audio` that contains the following files:
- test1.mp3
- test2.mp3
- test3.mp3
- image.jpg

Run the script on the `audio` directory:
```bash
./ffmpeg_cov.sh /path/to/audio
```

The script will create the following files:
- test1.mp4
- test2.mp4
- test3.mp4


## Features

- [x] Support Chinese and English
- [x] Support Checkpoint
- [x] Support text file
- [x] Support folder convert
- [x] Support multi-thread
- [x] Convert mp3 to mp4 by adding a static image
- [ ] Support epub file
