import * as r from "restructure";

const FrameColorComponent = new r.Struct({
  id: r.uint8,
  samplingFactors: r.uint8,
  quantizationTableId: r.uint8,
});

const StartOfFrameMarker = {
  name: () => "SOF",
  length: r.uint16be,
  precision: r.uint8,
  height: r.uint16be,
  width: r.uint16be,
  numberOfComponents: r.uint8,
  components: new r.Array(
    FrameColorComponent,
    (parent) => parent.numberOfComponents,
  ),
};

export default StartOfFrameMarker;
