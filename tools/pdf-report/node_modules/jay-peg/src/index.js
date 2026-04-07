import * as r from "restructure";

import DACMarker from "./markers/dac.js";
import DefineHuffmanTableMarker from "./markers/dht.js";
import DQTMarker from "./markers/dqt.js";
import DRIMarker from "./markers/dri.js";
import EndOfImageMarker from "./markers/eoi.js";
import EXIFMarker from "./markers/exif.js";
import JFIFMarker from "./markers/jfif.js";
import SOSMarker from "./markers/sos.js";
import StartOfFrameMarker from "./markers/sof.js";
import StartOfImageMarker from "./markers/soi.js";

const UnknownMarker = {
  length: r.uint16be,
  buf: new r.Buffer((parent) => parent.length - 2),
};

const unknownMarkers = Array(63)
  .fill(0)
  .reduce((acc, v, i) => ({ ...acc, [i + 0xffc0]: UnknownMarker }), {});

const Marker = new r.VersionedStruct(r.uint16be, {
  ...unknownMarkers,
  0xffc0: StartOfFrameMarker,
  0xffc1: StartOfFrameMarker,
  0xffc2: StartOfFrameMarker,
  0xffc3: StartOfFrameMarker,
  0xffc4: DefineHuffmanTableMarker,
  0xffc5: StartOfFrameMarker,
  0xffc6: StartOfFrameMarker,
  0xffc7: StartOfFrameMarker,
  0xffc9: StartOfFrameMarker,
  0xffca: StartOfFrameMarker,
  0xffcb: StartOfFrameMarker,
  0xffcc: DACMarker,
  0xffcd: StartOfFrameMarker,
  0xffce: StartOfFrameMarker,
  0xffcf: StartOfFrameMarker,
  0xffd8: StartOfImageMarker,
  0xffd9: EndOfImageMarker,
  0xffda: SOSMarker,
  0xffdb: DQTMarker,
  0xffdd: DRIMarker,
  0xffe0: JFIFMarker,
  0xffe1: EXIFMarker,
});

const JPEG = new r.Array(Marker);

const decode = (buffer) => {
  const markers = JPEG.fromBuffer(buffer);
  return markers.map(({ version, ...rest }) => ({ type: version, ...rest }));
};

export default { decode };
