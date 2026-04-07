import * as r from "restructure";

const DACTable = new r.Struct({
  identifier: new r.Buffer(1),
  value: new r.Buffer(1),
});

const DACMarker = {
  name: () => "DAC",
  length: r.uint16be,
  tables: new r.Array(DACTable, (parent) => parent.length / 2),
};

export default DACMarker;
