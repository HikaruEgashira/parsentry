import * as r from "restructure";

const DQTMarker = {
  name: () => "DQT",
  length: r.uint16be,
  tables: new r.Array(
    new r.Struct({
      identifier: new r.Buffer(1),
      data: new r.Buffer(64),
    }),
    (parent) => (parent.length - 2) / 65,
  ),
};

export default DQTMarker;
