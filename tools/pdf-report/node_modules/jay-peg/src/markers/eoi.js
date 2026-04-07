import * as r from "restructure";

const EndOfImageMarker = {
  name: () => "EOI",
  afterEOI: new r.Reserved(r.uint8, Infinity),
};

export default EndOfImageMarker;
