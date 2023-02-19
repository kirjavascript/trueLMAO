enum Video {
    PAL,
    NTSC
}

enum Region {
    US,
    EU,
    JP,
}

struct HWRevision {
    region: Region,
    video: Video,
}


impl HWRevision {

}

// from ROM
