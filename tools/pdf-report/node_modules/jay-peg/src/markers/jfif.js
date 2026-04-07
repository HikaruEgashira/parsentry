import * as r from "restructure";

const JFIFMarker = {
  name: () => "JFIF",
  length: r.uint16be,
  identifier: new r.String(5),
  version: r.uint16be,
  units: r.uint8,
  xDensity: r.uint16be,
  yDensity: r.uint16be,
  thumbnailWidth: r.uint8,
  thumbnailHeight: r.uint8,
};

export default JFIFMarker;
