var $gXNCa$restructure = require("restructure");


function $parcel$defineInteropFlag(a) {
  Object.defineProperty(a, '__esModule', {value: true, configurable: true});
}

function $parcel$export(e, n, v, s) {
  Object.defineProperty(e, n, {get: v, set: s, enumerable: true, configurable: true});
}

$parcel$defineInteropFlag(module.exports);

$parcel$export(module.exports, "default", () => $4fa36e821943b400$export$2e2bcd8739ae039);


const $aa341883af0ae85e$var$DACTable = new $gXNCa$restructure.Struct({
    identifier: new $gXNCa$restructure.Buffer(1),
    value: new $gXNCa$restructure.Buffer(1)
});
const $aa341883af0ae85e$var$DACMarker = {
    name: ()=>"DAC",
    length: $gXNCa$restructure.uint16be,
    tables: new $gXNCa$restructure.Array($aa341883af0ae85e$var$DACTable, (parent)=>parent.length / 2)
};
var $aa341883af0ae85e$export$2e2bcd8739ae039 = $aa341883af0ae85e$var$DACMarker;



const $bd10db122e664297$export$a91b4a1eede2f230 = (array, offset)=>{
    return array[offset];
};
const $bd10db122e664297$export$8b37dabc2dc78838 = (array, offset)=>{
    return array[offset] << 8 | array[offset + 1];
};
const $bd10db122e664297$export$a549f6d34e76d37a = (array, offset)=>{
    return array[offset] | array[offset + 1] << 8;
};
const $bd10db122e664297$export$cfaef8ed399428b0 = (array, offset)=>{
    return $bd10db122e664297$export$89308d455f98d9(array, offset) >>> 0;
};
const $bd10db122e664297$export$d3485ee83d66d723 = (array, offset)=>{
    return $bd10db122e664297$export$41469b9fa4aaddde(array, offset) >>> 0;
};
const $bd10db122e664297$export$d4d5806f7c45a793 = (uint8Array)=>{
    return Array.from(uint8Array, (byte)=>byte.toString(16).padStart(2, "0")).join("");
};
const $bd10db122e664297$var$decoder = new TextDecoder("utf-8");
const $bd10db122e664297$export$f3924c82a04770ee = (uint8Array)=>{
    return $bd10db122e664297$var$decoder.decode(uint8Array);
};
const $bd10db122e664297$export$deffdc216bdfb4c = (arrays)=>{
    const totalLength = arrays.reduce((length, arr)=>length + arr.length, 0);
    const concatenatedArray = new Uint8Array(totalLength);
    let offset = 0;
    arrays.forEach((arr)=>{
        concatenatedArray.set(arr, offset);
        offset += arr.length;
    });
    return concatenatedArray;
};
const $bd10db122e664297$export$89308d455f98d9 = (array, offset)=>{
    return array[offset] << 24 | array[offset + 1] << 16 | array[offset + 2] << 8 | array[offset + 3];
};
const $bd10db122e664297$export$41469b9fa4aaddde = (array, offset)=>{
    return array[offset] | array[offset + 1] << 8 | array[offset + 2] << 16 | array[offset + 3] << 24;
};


class $876ae4d13cf7aa4e$var$HuffmanTableElements {
    decode(stream, parent) {
        const tables = {};
        let buffer = stream.buffer.slice(stream.pos, stream.pos + parent.length - 2);
        while(buffer.length > 0){
            let offset = 1;
            const elements = [];
            const identifier = (0, $bd10db122e664297$export$a91b4a1eede2f230)(buffer, 0);
            const lengths = buffer.slice(offset, offset + 16);
            offset += 16;
            for (const length of lengths){
                elements.push(buffer.slice(offset, offset + length));
                offset += length;
            }
            buffer = buffer.slice(offset);
            tables[identifier] = (0, $bd10db122e664297$export$deffdc216bdfb4c)(elements);
        }
        stream.pos += parent.length - 2;
        return tables;
    }
}
const $876ae4d13cf7aa4e$var$DefineHuffmanTableMarker = {
    name: ()=>"DHT",
    length: $gXNCa$restructure.uint16be,
    tables: new $876ae4d13cf7aa4e$var$HuffmanTableElements()
};
var $876ae4d13cf7aa4e$export$2e2bcd8739ae039 = $876ae4d13cf7aa4e$var$DefineHuffmanTableMarker;



const $a90ba18fc4ccf97a$var$DQTMarker = {
    name: ()=>"DQT",
    length: $gXNCa$restructure.uint16be,
    tables: new $gXNCa$restructure.Array(new $gXNCa$restructure.Struct({
        identifier: new $gXNCa$restructure.Buffer(1),
        data: new $gXNCa$restructure.Buffer(64)
    }), (parent)=>(parent.length - 2) / 65)
};
var $a90ba18fc4ccf97a$export$2e2bcd8739ae039 = $a90ba18fc4ccf97a$var$DQTMarker;



const $38e4486858db78cb$var$DRIMarker = {
    name: ()=>"DRI",
    length: $gXNCa$restructure.uint16be,
    restartInterval: $gXNCa$restructure.uint16be
};
var $38e4486858db78cb$export$2e2bcd8739ae039 = $38e4486858db78cb$var$DRIMarker;



const $c2766d9e4fc88dcb$var$EndOfImageMarker = {
    name: ()=>"EOI",
    afterEOI: new $gXNCa$restructure.Reserved($gXNCa$restructure.uint8, Infinity)
};
var $c2766d9e4fc88dcb$export$2e2bcd8739ae039 = $c2766d9e4fc88dcb$var$EndOfImageMarker;




const $0b53e5b7c7fe341c$var$tags = {
    ifd: {
        "010e": "imageDescription",
        "010f": "make",
        "011a": "xResolution",
        "011b": "yResolution",
        "011c": "planarConfiguration",
        "012d": "transferFunction",
        "013b": "artist",
        "013e": "whitePoint",
        "013f": "primaryChromaticities",
        "0100": "imageWidth",
        "0101": "imageHeight",
        "0102": "bitsPerSample",
        "0103": "compression",
        "0106": "photometricInterpretation",
        "0110": "model",
        "0111": "stripOffsets",
        "0112": "orientation",
        "0115": "samplesPerPixel",
        "0116": "rowsPerStrip",
        "0117": "stripByteCounts",
        "0128": "resolutionUnit",
        "0131": "software",
        "0132": "dateTime",
        "0201": "jpegInterchangeFormat",
        "0202": "jpegInterchangeFormatLength",
        "0211": "ycbCrCoefficients",
        "0212": "ycbCrSubSampling",
        "0213": "ycbCrPositioning",
        "0214": "referenceBlackWhite",
        "829a": "exposureTime",
        "829d": "fNumber",
        "920a": "focalLength",
        "927c": "makerNote",
        8298: "copyright",
        8769: "exifIFDPointer",
        8822: "exposureProgram",
        8824: "spectralSensitivity",
        8825: "gpsInfoIFDPointer",
        8827: "photographicSensitivity",
        8828: "oecf",
        8830: "sensitivityType",
        8831: "standardOutputSensitivity",
        8832: "recommendedExposureIndex",
        8833: "isoSpeed",
        8834: "isoSpeedLatitudeyyy",
        8835: "isoSpeedLatitudezzz",
        9000: "exifVersion",
        9003: "dateTimeOriginal",
        9004: "dateTimeDigitized",
        9101: "componentsConfiguration",
        9102: "compressedBitsPerPixel",
        9201: "shutterSpeedValue",
        9202: "apertureValue",
        9203: "brightnessValue",
        9204: "exposureBiasValue",
        9205: "maxApertureValue",
        9206: "subjectDistance",
        9207: "meteringMode",
        9208: "lightSource",
        9209: "flash",
        9214: "subjectArea",
        9286: "userComment",
        9290: "subSecTime",
        9291: "subSecTimeOriginal",
        9292: "subSecTimeDigitized",
        a000: "flashpixVersion",
        a001: "colorSpace",
        a002: "pixelXDimension",
        a003: "pixelYDimension",
        a004: "relatedSoundFile",
        a005: "interoperabilityIFDPointer",
        a20b: "flashEnergy",
        a20c: "spatialFrequencyResponse",
        a20e: "focalPlaneXResolution",
        a20f: "focalPlaneYResolution",
        a40a: "sharpness",
        a40b: "deviceSettingDescription",
        a40c: "subjectDistanceRange",
        a210: "focalPlaneResolutionUnit",
        a214: "subjectLocation",
        a215: "exposureIndex",
        a217: "sensingMethod",
        a300: "fileSource",
        a301: "sceneType",
        a302: "cfaPattern",
        a401: "customRendered",
        a402: "exposureMode",
        a403: "whiteBalance",
        a404: "digitalZoomRatio",
        a405: "focalLengthIn35mmFilm",
        a406: "sceneCaptureType",
        a407: "gainControl",
        a408: "contrast",
        a409: "saturation",
        a420: "imageUniqueID",
        a430: "cameraOwnerName",
        a431: "bodySerialNumber",
        a432: "lensSpecification",
        a433: "lensMake",
        a434: "lensModel",
        a435: "lensSerialNumber",
        a500: "gamma"
    },
    gps: {
        "0000": "gpsVersionID",
        "0001": "gpsLatitudeRef",
        "0002": "gpsLatitude",
        "0003": "gpsLongitudeRef",
        "0004": "gpsLongitude",
        "0005": "gpsAltitudeRef",
        "0006": "gpsAltitude",
        "0007": "gpsTimeStamp",
        "0008": "gpsSatellites",
        "0009": "gpsStatus",
        "000a": "gpsMeasureMode",
        "000b": "gpsDOP",
        "000c": "gpsSpeedRef",
        "000d": "gpsSpeed",
        "000e": "gpsTrackRef",
        "000f": "gpsTrack",
        "0010": "gpsImgDirectionRef",
        "0011": "gpsImgDirection",
        "0012": "gpsMapDatum",
        "0013": "gpsDestLatitudeRef",
        "0014": "gpsDestLatitude",
        "0015": "gpsDestLongitudeRef",
        "0016": "gpsDestLongitude",
        "0017": "gpsDestBearingRef",
        "0018": "gpsDestBearing",
        "0019": "gpsDestDistanceRef",
        "001a": "gpsDestDistance",
        "001b": "gpsProcessingMethod",
        "001c": "gpsAreaInformation",
        "001d": "gpsDateStamp",
        "001e": "gpsDifferential",
        "001f": "gpsHPositioningError"
    }
};
class $0b53e5b7c7fe341c$var$IDFEntries {
    constructor(bigEndian){
        this.bigEndian = bigEndian;
        this.bytes = [
            0,
            1,
            1,
            2,
            4,
            8,
            1,
            1,
            2,
            4,
            8,
            4,
            8
        ];
    }
    _getTagValue(dataValue, dataFormat, componentsNumber) {
        switch(dataFormat){
            case 2:
                return dataValue.toString("ascii").replace(/\0+$/, "");
            case 129:
                return dataValue.toString("utf8").replace(/\0+$/, "");
            case 7:
                return "0x" + dataValue.toString("hex");
            default:
                return this._getTagValueForNumericalData(dataValue, dataFormat, componentsNumber);
        }
    }
    _getTagValueForNumericalData(dataValue, dataFormat, componentsNumber) {
        const tagValue = [];
        const componentsBytes = this.bytes[dataFormat];
        for(let i = 0; i < componentsNumber; i += 1)tagValue.push(this._getSingleTagValueForNumericalData(dataValue, dataFormat, i * componentsBytes));
        return tagValue.length === 1 ? tagValue[0] : tagValue;
    }
    _getSingleTagValueForNumericalData(dataValue, dataFormat, pos) {
        const uint16 = (pos)=>this.bigEndian ? (0, $bd10db122e664297$export$8b37dabc2dc78838)(dataValue, pos) : (0, $bd10db122e664297$export$a549f6d34e76d37a)(dataValue, pos);
        const uint32 = (pos)=>this.bigEndian ? (0, $bd10db122e664297$export$cfaef8ed399428b0)(dataValue, pos) : (0, $bd10db122e664297$export$d3485ee83d66d723)(dataValue, pos);
        const int32 = (pos)=>this.bigEndian ? (0, $bd10db122e664297$export$89308d455f98d9)(dataValue, pos) : (0, $bd10db122e664297$export$41469b9fa4aaddde)(dataValue, pos);
        switch(dataFormat){
            case 1:
                return (0, $bd10db122e664297$export$a91b4a1eede2f230)(dataValue, pos);
            case 3:
                return uint16(pos);
            case 4:
                return uint32(pos);
            case 5:
                return uint32(pos) / uint32(pos + 4);
            case 9:
                return int32(pos);
            case 10:
                return int32(pos) / int32(pos + 4);
        }
    }
    _decodeIDFEntries(buffer, tags, offset, log = false) {
        let pos = 2 + offset;
        const entries = {};
        const uint16 = (pos)=>this.bigEndian ? (0, $bd10db122e664297$export$8b37dabc2dc78838)(buffer, pos) : (0, $bd10db122e664297$export$a549f6d34e76d37a)(buffer, pos);
        const uint32 = (pos)=>this.bigEndian ? (0, $bd10db122e664297$export$cfaef8ed399428b0)(buffer, pos) : (0, $bd10db122e664297$export$d3485ee83d66d723)(buffer, pos);
        const numberOfEntries = uint16(offset);
        for(let i = 0; i < numberOfEntries; i++){
            const tagAddress = buffer.slice(pos, pos + 2);
            const dataFormat = uint16(pos + 2);
            const componentsNumber = uint32(pos + 4);
            const componentsBytes = this.bytes[dataFormat];
            const dataLength = componentsNumber * componentsBytes;
            let dataValue = buffer.slice(pos + 8, pos + 12);
            if (dataLength > 4) {
                const dataOffset = this.bigEndian ? (0, $bd10db122e664297$export$cfaef8ed399428b0)(dataValue, 0) : (0, $bd10db122e664297$export$d3485ee83d66d723)(dataValue, 0);
                dataValue = buffer.slice(dataOffset, dataOffset + dataLength);
            }
            const tagValue = this._getTagValue(dataValue, dataFormat, componentsNumber);
            const tagNumber = this.bigEndian ? (0, $bd10db122e664297$export$d4d5806f7c45a793)(tagAddress) : (0, $bd10db122e664297$export$d4d5806f7c45a793)(tagAddress.reverse());
            const tagName = tags[tagNumber];
            entries[tagName] = tagValue;
            pos += 12;
        }
        return entries;
    }
    decode(stream, parent) {
        const buffer = stream.buffer.slice(stream.pos - 8);
        const offsetToFirstIFD = parent.offsetToFirstIFD;
        if (offsetToFirstIFD > buffer.length) {
            stream.pos += parent.parent.length - 16;
            return {};
        }
        const entries = this._decodeIDFEntries(buffer, $0b53e5b7c7fe341c$var$tags.ifd, offsetToFirstIFD);
        const { exifIFDPointer: exifIFDPointer, gpsInfoIFDPointer: gpsInfoIFDPointer } = entries;
        if (exifIFDPointer) entries.subExif = this._decodeIDFEntries(buffer, $0b53e5b7c7fe341c$var$tags.ifd, exifIFDPointer);
        if (gpsInfoIFDPointer) {
            const gps = gpsInfoIFDPointer;
            entries.gpsInfo = this._decodeIDFEntries(buffer, $0b53e5b7c7fe341c$var$tags.gps, gps, true);
        }
        stream.pos += parent.parent.length - 16;
        return entries;
    }
}
const $0b53e5b7c7fe341c$var$IFDData = (bigEndian)=>{
    const uint16 = bigEndian ? $gXNCa$restructure.uint16be : $gXNCa$restructure.uint16le;
    const uint32 = bigEndian ? $gXNCa$restructure.uint32be : $gXNCa$restructure.uint32le;
    return new $gXNCa$restructure.Struct({
        fortyTwo: uint16,
        offsetToFirstIFD: uint32,
        entries: new $0b53e5b7c7fe341c$var$IDFEntries(bigEndian)
    });
};
class $0b53e5b7c7fe341c$var$TIFFHeader {
    decode(stream, parent) {
        const byteOrder = (0, $bd10db122e664297$export$f3924c82a04770ee)(stream.buffer.slice(stream.pos, stream.pos + 2));
        const bigEndian = byteOrder === "MM";
        stream.pos += 2;
        const data = $0b53e5b7c7fe341c$var$IFDData(bigEndian).decode(stream, parent);
        return data.entries;
    }
}
const $0b53e5b7c7fe341c$var$EXIFMarker = {
    name: ()=>"EXIF",
    length: $gXNCa$restructure.uint16be,
    identifier: new $gXNCa$restructure.String(6),
    entries: new $0b53e5b7c7fe341c$var$TIFFHeader()
};
var $0b53e5b7c7fe341c$export$2e2bcd8739ae039 = $0b53e5b7c7fe341c$var$EXIFMarker;



const $04bcf19210df7b64$var$JFIFMarker = {
    name: ()=>"JFIF",
    length: $gXNCa$restructure.uint16be,
    identifier: new $gXNCa$restructure.String(5),
    version: $gXNCa$restructure.uint16be,
    units: $gXNCa$restructure.uint8,
    xDensity: $gXNCa$restructure.uint16be,
    yDensity: $gXNCa$restructure.uint16be,
    thumbnailWidth: $gXNCa$restructure.uint8,
    thumbnailHeight: $gXNCa$restructure.uint8
};
var $04bcf19210df7b64$export$2e2bcd8739ae039 = $04bcf19210df7b64$var$JFIFMarker;



class $2e255b533ff4fcbb$var$ImageData {
    decode(stream) {
        const buffer = stream.buffer.slice(stream.pos);
        let length = 0;
        let i = buffer.indexOf(0xff);
        while(i !== -1){
            length = i;
            const nextByte = buffer[length + 1];
            const comesRestart = nextByte >= 0xd0 && nextByte <= 0xd7;
            if (nextByte !== 0x00 && !comesRestart) break;
            i = buffer.indexOf(0xff, i + 1);
        }
        stream.pos += length;
        return buffer.slice(0, length);
    }
}
const $2e255b533ff4fcbb$var$SOSComponentSpecification = new $gXNCa$restructure.Struct({
    scanComponentSelector: $gXNCa$restructure.uint8,
    entropyCodingTable: new $gXNCa$restructure.Buffer(1)
});
const $2e255b533ff4fcbb$var$SOSMarker = {
    name: ()=>"SOS",
    length: $gXNCa$restructure.uint16be,
    numberOfImageComponents: $gXNCa$restructure.uint8,
    componentSpecifications: new $gXNCa$restructure.Array($2e255b533ff4fcbb$var$SOSComponentSpecification, (parent)=>parent.numberOfImageComponents),
    startOfSpectral: $gXNCa$restructure.uint8,
    endOfSpectral: $gXNCa$restructure.uint8,
    successiveApproximationBit: new $gXNCa$restructure.Buffer(1),
    data: new $2e255b533ff4fcbb$var$ImageData()
};
var $2e255b533ff4fcbb$export$2e2bcd8739ae039 = $2e255b533ff4fcbb$var$SOSMarker;



const $0765cace26fd2550$var$FrameColorComponent = new $gXNCa$restructure.Struct({
    id: $gXNCa$restructure.uint8,
    samplingFactors: $gXNCa$restructure.uint8,
    quantizationTableId: $gXNCa$restructure.uint8
});
const $0765cace26fd2550$var$StartOfFrameMarker = {
    name: ()=>"SOF",
    length: $gXNCa$restructure.uint16be,
    precision: $gXNCa$restructure.uint8,
    height: $gXNCa$restructure.uint16be,
    width: $gXNCa$restructure.uint16be,
    numberOfComponents: $gXNCa$restructure.uint8,
    components: new $gXNCa$restructure.Array($0765cace26fd2550$var$FrameColorComponent, (parent)=>parent.numberOfComponents)
};
var $0765cace26fd2550$export$2e2bcd8739ae039 = $0765cace26fd2550$var$StartOfFrameMarker;


const $5cb7f9bcb7bda090$var$StartOfImageMarker = {
    name: ()=>"SOI"
};
var $5cb7f9bcb7bda090$export$2e2bcd8739ae039 = $5cb7f9bcb7bda090$var$StartOfImageMarker;


const $4fa36e821943b400$var$UnknownMarker = {
    length: $gXNCa$restructure.uint16be,
    buf: new $gXNCa$restructure.Buffer((parent)=>parent.length - 2)
};
const $4fa36e821943b400$var$unknownMarkers = Array(63).fill(0).reduce((acc, v, i)=>({
        ...acc,
        [i + 0xffc0]: $4fa36e821943b400$var$UnknownMarker
    }), {});
const $4fa36e821943b400$var$Marker = new $gXNCa$restructure.VersionedStruct($gXNCa$restructure.uint16be, {
    ...$4fa36e821943b400$var$unknownMarkers,
    0xffc0: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc1: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc2: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc3: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc4: (0, $876ae4d13cf7aa4e$export$2e2bcd8739ae039),
    0xffc5: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc6: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc7: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffc9: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffca: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffcb: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffcc: (0, $aa341883af0ae85e$export$2e2bcd8739ae039),
    0xffcd: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffce: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffcf: (0, $0765cace26fd2550$export$2e2bcd8739ae039),
    0xffd8: (0, $5cb7f9bcb7bda090$export$2e2bcd8739ae039),
    0xffd9: (0, $c2766d9e4fc88dcb$export$2e2bcd8739ae039),
    0xffda: (0, $2e255b533ff4fcbb$export$2e2bcd8739ae039),
    0xffdb: (0, $a90ba18fc4ccf97a$export$2e2bcd8739ae039),
    0xffdd: (0, $38e4486858db78cb$export$2e2bcd8739ae039),
    0xffe0: (0, $04bcf19210df7b64$export$2e2bcd8739ae039),
    0xffe1: (0, $0b53e5b7c7fe341c$export$2e2bcd8739ae039)
});
const $4fa36e821943b400$var$JPEG = new $gXNCa$restructure.Array($4fa36e821943b400$var$Marker);
const $4fa36e821943b400$var$decode = (buffer)=>{
    const markers = $4fa36e821943b400$var$JPEG.fromBuffer(buffer);
    return markers.map(({ version: version, ...rest })=>({
            type: version,
            ...rest
        }));
};
var $4fa36e821943b400$export$2e2bcd8739ae039 = {
    decode: $4fa36e821943b400$var$decode
};


//# sourceMappingURL=index.cjs.map
