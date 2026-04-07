# jay-peg

<a href="https://www.npmjs.com/package/jay-peg">
  <img src="https://img.shields.io/npm/v/jay-peg.svg" />
</a>
<a href="https://github.com/diegomura/jay-peg/blob/master/LICENSE">
  <img src="https://img.shields.io/github/license/diegomura/jay-peg.svg" />
</a>
<a href="https://github.com/prettier/prettier">
  <img src="https://img.shields.io/badge/styled_with-prettier-ff69b4.svg" />
</a>

## Overview

A blazing-fast and compact JavaScript library dedicated to efficiently decoding JPEG images.

## Installation

Using npm:

```bash
npm install jay-peg
```

Using yarn:

```bash
yarn add jay-peg
```

## Usage

Use the `decoder` providing a JPEG data buffer as input.

```javascript
import JPEG from 'jay-peg';

const jpegBuffer = /* your JPEG buffer here */;
const imageMarkers = JPEG.decoder(jpegBuffer);

console.log(imageMarkers);
```

## Example Output

The output consists of a structured array of image markers:

```javascript
[
  {
    type: 65496,
    name: "SOI",
  },
  {
    type: 65505,
    name: "EXIF",
    length: 16382,
    identifier: "Exif\x00\x00",
    entries: [Object],
  },
  {
    type: 65499,
    name: "DAC",
    length: 132,
    tables: [[Object], [Object]],
  },
  // ... and so forth
  {
    type: 65497,
    name: "EOI",
  },
];
```

## API

### `decoder(buffer: Buffer | Uint8Array): Array<ImageMarker>`

The `decoder` function accepts a JPEG buffer as its sole argument and returns an array of image markers.

#### Parameters

- `buffer`: A Buffer or Uint8Array containing the JPEG image data.

#### Returns

An array of objects representing various markers found in the JPEG image.

### `ImageMarker`

Each `ImageMarker` object in the output array adheres to the following structure:

- `type` (Number): The marker type.
- `name` (String): The marker name.
- `length` (Number): The length of the marker data.
- Additional properties specific to certain marker types.

## Performance

Performance is a key focus of `jay-peg`. 4 sizes of images were benchmarked:

- `small`: 300 × 150, 8KB image
- `medium`: 800 × 600, 70KB image
- `large`: 1920 × 1080, 332KB image
- `huge`: 2448×3264, 2.2MB image

For each of these, the decoding speed was measured as follows:

```
Benchmarked: small:  x 13,393 ops/sec ±4.77% (96 runs sampled)
Benchmarked: medium:  x 12,894 ops/sec ±0.10% (99 runs sampled)
Benchmarked: large:  x 9,241 ops/sec ±0.25% (99 runs sampled)
Benchmarked: huge:  x 2,672 ops/sec ±0.12% (100 runs sampled)
```

_Measures were taken in an MacBook Air 2024, Apple M3 w/16GB of RAM._

## License

`jay-peg` is released under the [MIT License](LICENSE)
