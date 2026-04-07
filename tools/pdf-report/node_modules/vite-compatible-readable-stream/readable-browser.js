const Registry = require('./lib/_registry.js');

require('./lib/_stream_readable.js');
require('./lib/_stream_writable.js');
require('./lib/_stream_duplex.js');
require('./lib/_stream_transform.js');
require('./lib/_stream_passthrough.js');

exports = module.exports = Registry.Readable;

exports.Stream = Registry.Readable;
exports.Readable = Registry.Readable;
exports.Writable = Registry.Writable;
exports.Duplex = Registry.Duplex;
exports.Transform = Registry.Transform;
exports.PassThrough = Registry.PassThrough;
exports.finished = require('./lib/internal/streams/end-of-stream.js');
exports.pipeline = require('./lib/internal/streams/pipeline.js');
