export const readUInt8 = (array, offset) => {
  return array[offset];
};

export const readUInt16BE = (array, offset) => {
  return (array[offset] << 8) | array[offset + 1];
};

export const readUInt16LE = (array, offset) => {
  return array[offset] | (array[offset + 1] << 8);
};

export const readUInt32BE = (array, offset) => {
  return readInt32BE(array, offset) >>> 0;
};

export const readUInt32LE = (array, offset) => {
  return readInt32LE(array, offset) >>> 0;
};

export const uint8ArrayToHexString = (uint8Array) => {
  return Array.from(uint8Array, (byte) =>
    byte.toString(16).padStart(2, "0"),
  ).join("");
};

const decoder = new TextDecoder("utf-8");

export const uint8ArrayToString = (uint8Array) => {
  return decoder.decode(uint8Array);
};

export const concatenateUint8Arrays = (arrays) => {
  const totalLength = arrays.reduce((length, arr) => length + arr.length, 0);
  const concatenatedArray = new Uint8Array(totalLength);

  let offset = 0;

  arrays.forEach((arr) => {
    concatenatedArray.set(arr, offset);
    offset += arr.length;
  });

  return concatenatedArray;
};

export const readInt32BE = (array, offset) => {
  return (
    (array[offset] << 24) |
    (array[offset + 1] << 16) |
    (array[offset + 2] << 8) |
    array[offset + 3]
  );
};

export const readInt32LE = (array, offset) => {
  return (
    array[offset] |
    (array[offset + 1] << 8) |
    (array[offset + 2] << 16) |
    (array[offset + 3] << 24)
  );
};
