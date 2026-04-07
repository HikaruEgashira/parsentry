import * as r from "restructure";
import {
  readUInt8,
  readUInt16BE,
  readUInt16LE,
  readUInt32BE,
  readUInt32LE,
  uint8ArrayToHexString,
  uint8ArrayToString,
  readInt32BE,
  readInt32LE,
} from "./utils.js";

const tags = {
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
    a500: "gamma",
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
    "001f": "gpsHPositioningError",
  },
};

class IDFEntries {
  constructor(bigEndian) {
    this.bigEndian = bigEndian;
    this.bytes = [0, 1, 1, 2, 4, 8, 1, 1, 2, 4, 8, 4, 8];
  }

  _getTagValue(dataValue, dataFormat, componentsNumber) {
    switch (dataFormat) {
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
    for (let i = 0; i < componentsNumber; i += 1) {
      tagValue.push(this._getSingleTagValueForNumericalData(dataValue, dataFormat, i * componentsBytes));
    }
    return tagValue.length === 1 ? tagValue[0] : tagValue;
  }

  _getSingleTagValueForNumericalData(dataValue, dataFormat, pos) {
    const uint16 = (pos) =>
      this.bigEndian
        ? readUInt16BE(dataValue, pos)
        : readUInt16LE(dataValue, pos);

    const uint32 = (pos) =>
      this.bigEndian
        ? readUInt32BE(dataValue, pos)
        : readUInt32LE(dataValue, pos);

    const int32 = (pos) =>
      this.bigEndian
        ? readInt32BE(dataValue, pos)
        : readInt32LE(dataValue, pos);

    switch (dataFormat) {
      case 1:
        return readUInt8(dataValue, pos)
      case 3:
        return uint16(pos);
      case 4:
        return uint32(pos);
      case 5:
        return uint32(pos) / uint32(pos + 4);
      case 9:
        return int32(pos);
      case 10: {
        return int32(pos) / int32(pos + 4);
      }
    }
  }

  _decodeIDFEntries(buffer, tags, offset, log = false) {
    let pos = 2 + offset;

    const entries = {};

    const uint16 = (pos) =>
      this.bigEndian ? readUInt16BE(buffer, pos) : readUInt16LE(buffer, pos);

    const uint32 = (pos) =>
      this.bigEndian ? readUInt32BE(buffer, pos) : readUInt32LE(buffer, pos);

    const numberOfEntries = uint16(offset);

    for (let i = 0; i < numberOfEntries; i++) {
      const tagAddress = buffer.slice(pos, pos + 2);
      const dataFormat = uint16(pos + 2);
      const componentsNumber = uint32(pos + 4);
      const componentsBytes = this.bytes[dataFormat];
      const dataLength = componentsNumber * componentsBytes;

      let dataValue = buffer.slice(pos + 8, pos + 12);

      if (dataLength > 4) {
        const dataOffset = this.bigEndian
          ? readUInt32BE(dataValue, 0)
          : readUInt32LE(dataValue, 0);

        dataValue = buffer.slice(dataOffset, dataOffset + dataLength);
      }

      const tagValue = this._getTagValue(dataValue, dataFormat, componentsNumber);

      const tagNumber = this.bigEndian
        ? uint8ArrayToHexString(tagAddress)
        : uint8ArrayToHexString(tagAddress.reverse());

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

    const entries = this._decodeIDFEntries(buffer, tags.ifd, offsetToFirstIFD);
    const { exifIFDPointer, gpsInfoIFDPointer } = entries;

    if (exifIFDPointer) {
      entries.subExif = this._decodeIDFEntries(
        buffer,
        tags.ifd,
        exifIFDPointer,
      );
    }

    if (gpsInfoIFDPointer) {
      const gps = gpsInfoIFDPointer;
      entries.gpsInfo = this._decodeIDFEntries(buffer, tags.gps, gps, true);
    }

    stream.pos += parent.parent.length - 16;

    return entries;
  }
}

const IFDData = (bigEndian) => {
  const uint16 = bigEndian ? r.uint16be : r.uint16le;
  const uint32 = bigEndian ? r.uint32be : r.uint32le;

  return new r.Struct({
    fortyTwo: uint16,
    offsetToFirstIFD: uint32,
    entries: new IDFEntries(bigEndian),
  });
};

class TIFFHeader {
  decode(stream, parent) {
    const byteOrder = uint8ArrayToString(
      stream.buffer.slice(stream.pos, stream.pos + 2),
    );

    const bigEndian = byteOrder === "MM";

    stream.pos += 2;

    const data = IFDData(bigEndian).decode(stream, parent);

    return data.entries;
  }
}

const EXIFMarker = {
  name: () => "EXIF",
  length: r.uint16be,
  identifier: new r.String(6),
  entries: new TIFFHeader(),
};

export default EXIFMarker;
