import * as primitives from '@react-pdf/primitives';
export * from '@react-pdf/primitives';
import fs from 'fs';
import { Buffer } from 'buffer';
import FontStore from '@react-pdf/font';
import renderPDF from '@react-pdf/render';
import PDFDocument from '@react-pdf/pdfkit';
import layoutDocument from '@react-pdf/layout';
import { upperFirst } from '@react-pdf/fns';
import Reconciler from '@react-pdf/reconciler';

const omitNils = object => Object.fromEntries(Object.entries(object).filter(_ref => {
  let [, value] = _ref;
  return value !== undefined;
}));

const createInstance = (type, _ref) => {
  let {
    style,
    children,
    ...props
  } = _ref;
  return {
    type,
    box: {},
    style: style || {},
    props: props || {},
    children: []
  };
};
const createTextInstance = text => ({
  type: 'TEXT_INSTANCE',
  value: text
});
const appendChild = (parent, child) => {
  const isParentText = parent.type === 'TEXT' || parent.type === 'LINK' || parent.type === 'TSPAN' || parent.type === 'NOTE';
  const isChildTextInstance = child.type === 'TEXT_INSTANCE';
  const isOrphanTextInstance = isChildTextInstance && !isParentText;

  // Ignore orphan text instances.
  // Caused by cases such as <>{name && <Text>{name}</Text>}</>
  if (isOrphanTextInstance) {
    console.warn(`Invalid '${child.value}' string child outside <Text> component`);
    return;
  }
  parent.children.push(child);
};
const appendChildToContainer = (parentInstance, child) => {
  if (parentInstance.type === 'ROOT') {
    parentInstance.document = child;
  } else {
    appendChild(parentInstance, child);
  }
};
const insertBefore = (parentInstance, child, beforeChild) => {
  var _parentInstance$child;
  const index = (_parentInstance$child = parentInstance.children) === null || _parentInstance$child === void 0 ? void 0 : _parentInstance$child.indexOf(beforeChild);
  if (index === undefined) return;
  if (index !== -1 && child) parentInstance.children.splice(index, 0, child);
};
const removeChild = (parentInstance, child) => {
  var _parentInstance$child2;
  const index = (_parentInstance$child2 = parentInstance.children) === null || _parentInstance$child2 === void 0 ? void 0 : _parentInstance$child2.indexOf(child);
  if (index === undefined) return;
  if (index !== -1) parentInstance.children.splice(index, 1);
};
const removeChildFromContainer = (parentInstance, child) => {
  var _parentInstance$child3;
  const index = (_parentInstance$child3 = parentInstance.children) === null || _parentInstance$child3 === void 0 ? void 0 : _parentInstance$child3.indexOf(child);
  if (index === undefined) return;
  if (index !== -1) parentInstance.children.splice(index, 1);
};
const commitTextUpdate = (textInstance, oldText, newText) => {
  textInstance.value = newText;
};
const commitUpdate = (instance, updatePayload, type, oldProps, newProps) => {
  const {
    style,
    ...props
  } = newProps;
  instance.props = props;
  instance.style = style;
};
const createRenderer = _ref2 => {
  let {
    onChange = () => {}
  } = _ref2;
  return Reconciler({
    appendChild,
    appendChildToContainer,
    commitTextUpdate,
    commitUpdate,
    createInstance,
    createTextInstance,
    insertBefore,
    removeChild,
    removeChildFromContainer,
    resetAfterCommit: onChange
  });
};

var version$1 = "4.4.0";
var packageJson = {
	version: version$1};

const {
  version
} = packageJson;
const fontStore = new FontStore();

// We must keep a single renderer instance, otherwise React will complain
let renderer;

// The pdf instance acts as an event emitter for DOM usage.
// We only want to trigger an update when PDF content changes
const events = {};
const pdf = initialValue => {
  const onChange = () => {
    var _events$change;
    const listeners = ((_events$change = events.change) === null || _events$change === void 0 ? void 0 : _events$change.slice()) || [];
    for (let i = 0; i < listeners.length; i += 1) listeners[i]();
  };
  const container = {
    type: 'ROOT',
    document: null
  };
  renderer = renderer || createRenderer({
    onChange
  });
  const mountNode = renderer.createContainer(container);
  const updateContainer = (doc, callback) => {
    renderer.updateContainer(doc, mountNode, null, callback);
  };
  if (initialValue) updateContainer(initialValue);
  const render = async function (compress) {
    if (compress === void 0) {
      compress = true;
    }
    const props = container.document.props || {};
    const {
      pdfVersion,
      language,
      pageLayout,
      pageMode,
      title,
      author,
      subject,
      keywords,
      creator = 'react-pdf',
      producer = 'react-pdf',
      creationDate = new Date(),
      modificationDate,
      ownerPassword,
      userPassword,
      permissions
    } = props;
    const ctx = new PDFDocument({
      compress,
      pdfVersion,
      lang: language,
      displayTitle: true,
      autoFirstPage: false,
      ownerPassword,
      userPassword,
      permissions,
      info: omitNils({
        Title: title,
        Author: author,
        Subject: subject,
        Keywords: keywords,
        Creator: creator,
        Producer: producer,
        CreationDate: creationDate,
        ModificationDate: modificationDate
      })
    });
    if (pageLayout) {
      ctx._root.data.PageLayout = upperFirst(pageLayout);
    }
    if (pageMode) {
      ctx._root.data.PageMode = upperFirst(pageMode);
    }
    const layout = await layoutDocument(container.document, fontStore);
    const fileStream = renderPDF(ctx, layout);
    return {
      layout,
      fileStream
    };
  };
  const callOnRender = function (params) {
    if (params === void 0) {
      params = {};
    }
    if (container.document.props.onRender) {
      container.document.props.onRender(params);
    }
  };
  const toBlob = async () => {
    const chunks = [];
    const {
      layout: _INTERNAL__LAYOUT__DATA_,
      fileStream: instance
    } = await render();
    return new Promise((resolve, reject) => {
      instance.on('data', chunk => {
        chunks.push(chunk instanceof Uint8Array ? chunk : new Uint8Array(chunk));
      });
      instance.on('end', () => {
        try {
          const blob = new Blob(chunks, {
            type: 'application/pdf'
          });
          callOnRender({
            blob,
            _INTERNAL__LAYOUT__DATA_
          });
          resolve(blob);
        } catch (error) {
          reject(error);
        }
      });
    });
  };

  // TODO: rename this method to `toStream` in next major release, because it return stream not a buffer
  const toBuffer = async () => {
    const {
      layout: _INTERNAL__LAYOUT__DATA_,
      fileStream
    } = await render();
    callOnRender({
      _INTERNAL__LAYOUT__DATA_
    });
    return fileStream;
  };

  /*
   * TODO: remove this method in next major release. it is buggy
   * see
   * - https://github.com/diegomura/react-pdf/issues/2112
   * - https://github.com/diegomura/react-pdf/issues/2095
   */
  const toString = async () => {
    if (process.env.NODE_ENV === 'development') {
      console.warn('`toString` is deprecated and will be removed in next major release');
    }
    let result = '';
    const {
      fileStream: instance
    } = await render(false); // For some reason, when rendering to string if compress=true the document is blank

    return new Promise((resolve, reject) => {
      try {
        instance.on('data', buffer => {
          result += buffer;
        });
        instance.on('end', () => {
          callOnRender();
          resolve(result);
        });
      } catch (error) {
        reject(error);
      }
    });
  };
  const on = (event, listener) => {
    if (!events[event]) events[event] = [];
    events[event].push(listener);
  };
  const removeListener = (event, listener) => {
    if (!events[event]) return;
    const idx = events[event].indexOf(listener);
    if (idx > -1) events[event].splice(idx, 1);
  };
  return {
    on,
    container,
    toBlob,
    toBuffer,
    toString,
    removeListener,
    updateContainer
  };
};
const Font = fontStore;
const StyleSheet = {
  create: s => s
};

/**
 * @param {React.ReactElement} element
 * @returns {Promise<NodeJS.ReadableStream>}
 */
const renderToStream = async element => {
  const instance = pdf(element);
  const stream = await instance.toBuffer();
  return stream;
};

/**
 * @param {React.ReactElement} element
 * @param {string} filePath
 * @param {Function} [callback]
 */
const renderToFile = async (element, filePath, callback) => {
  const output = await renderToStream(element);
  const stream = fs.createWriteStream(filePath);
  output.pipe(stream);
  return new Promise((resolve, reject) => {
    stream.on('finish', () => {
      if (callback) callback(output, filePath);
      resolve(output);
    });
    stream.on('error', reject);
  });
};

/**
 * @param {React.ReactElement} element
 * @returns {Promise<Buffer>}
 */
const renderToBuffer = element => renderToStream(element).then(stream => new Promise((resolve, reject) => {
  const chunks = [];
  stream.on('data', chunk => chunks.push(chunk));
  stream.on('end', () => resolve(Buffer.concat(chunks)));
  stream.on('error', error => reject(error));
}));
const renderToString = element => {
  if (process.env.NODE_ENV === 'development') {
    console.warn('`renderToString` is deprecated and will be removed in next major release, use `renderToBuffer` instead');
  }
  return renderToBuffer(element).then(buffer => buffer.toString());
};

const throwEnvironmentError = name => {
  throw new Error(`${name} is a web specific API. You're either using this component on Node, or your bundler is not loading react-pdf from the appropriate web build.`);
};
const usePDF = () => {
  throwEnvironmentError('usePDF');
};
const PDFViewer = () => {
  throwEnvironmentError('PDFViewer');
};
const PDFDownloadLink = () => {
  throwEnvironmentError('PDFDownloadLink');
};
const BlobProvider = () => {
  throwEnvironmentError('BlobProvider');
};
const render = renderToFile;

// TODO: remove this default export in next major release because it breaks tree-shacking
var index = {
  pdf,
  Font,
  version,
  StyleSheet,
  usePDF,
  PDFViewer,
  BlobProvider,
  PDFDownloadLink,
  renderToStream,
  renderToString,
  renderToFile,
  render,
  ...primitives
};

export { BlobProvider, Font, PDFDownloadLink, PDFViewer, StyleSheet, createRenderer, index as default, pdf, render, renderToBuffer, renderToFile, renderToStream, renderToString, usePDF, version };
//# sourceMappingURL=react-pdf.js.map
