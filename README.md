# Heatman

Create heat map image from given csv file.

## Usage

```sh
$ java -jar heatmapper.jar data.csv
$ java -jar heatmapper.jar -h
Usage: java -jar heatmapper.jar [OPTIONS] <DATA.CSV>
OPTIONS:
    -w, --width <WIDTH>:      specifies width of resultant image.
    -h, --height <HEIGHT>:    specifies height of resultant image.
    -p, --pixel <SIZE>:       specifies a pixel size of resultant image.
    -g, --gray:               output the grayscaled heatmap image.
    -o, --output <DEST.FILE>: destination image file. Default is 'heatmap.png.'
    -H, --help:               print this message.
DATA.CSV:
    csv formatted data file.
    The first column and the first row show the labels.
```

