import * as r from "restructure";

class ImageData {
  decode(stream) {
    const buffer = stream.buffer.slice(stream.pos);

    let length = 0;
    let i = buffer.indexOf(0xff);

    while (i !== -1) {
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

const SOSComponentSpecification = new r.Struct({
  scanComponentSelector: r.uint8,
  entropyCodingTable: new r.Buffer(1),
});

const SOSMarker = {
  name: () => "SOS",
  length: r.uint16be,
  numberOfImageComponents: r.uint8,
  componentSpecifications: new r.Array(
    SOSComponentSpecification,
    (parent) => parent.numberOfImageComponents,
  ),
  startOfSpectral: r.uint8,
  endOfSpectral: r.uint8,
  successiveApproximationBit: new r.Buffer(1),
  data: new ImageData(),
};

export default SOSMarker;
