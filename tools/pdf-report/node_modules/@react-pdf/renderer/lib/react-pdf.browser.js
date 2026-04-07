import * as primitives from '@react-pdf/primitives';
export * from '@react-pdf/primitives';
import queue from 'queue';
import { useRef, useState, useEffect, useCallback, forwardRef } from 'react';
import FontStore from '@react-pdf/font';
import renderPDF from '@react-pdf/render';
import PDFDocument from '@react-pdf/pdfkit';
import layoutDocument from '@react-pdf/layout';
import { upperFirst } from '@react-pdf/fns';
import Reconciler from '@react-pdf/reconciler';
import { jsx } from 'react/jsx-runtime';

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
 * PDF hook
 *
 * @param {Object} [options] hook options
 * @returns {[Object, Function]} pdf state and update function
 */
const usePDF = function (_temp) {
  let {
    document
  } = _temp === void 0 ? {} : _temp;
  const pdfInstance = useRef(null);
  const [state, setState] = useState({
    url: null,
    blob: null,
    error: null,
    loading: !!document
  });

  // Setup rendering queue
  useEffect(() => {
    const renderQueue = queue({
      autostart: true,
      concurrency: 1
    });
    const queueDocumentRender = () => {
      setState(prev => ({
        ...prev,
        loading: true
      }));
      renderQueue.splice(0, renderQueue.length, () => state.error ? Promise.resolve() : pdfInstance.current.toBlob());
    };
    const onRenderFailed = error => {
      console.error(error);
      setState(prev => ({
        ...prev,
        loading: false,
        error
      }));
    };
    const onRenderSuccessful = blob => {
      setState({
        blob,
        error: null,
        loading: false,
        url: URL.createObjectURL(blob)
      });
    };
    pdfInstance.current = pdf();
    pdfInstance.current.on('change', queueDocumentRender);
    if (document) {
      pdfInstance.current.updateContainer(document);
    }
    renderQueue.on('error', onRenderFailed);
    renderQueue.on('success', onRenderSuccessful);
    return () => {
      renderQueue.end();
      pdfInstance.current.removeListener('change', queueDocumentRender);
    };
  }, []);

  // Revoke old unused url instances
  useEffect(() => {
    return () => {
      if (state.url) {
        URL.revokeObjectURL(state.url);
      }
    };
  }, [state.url]);
  const update = useCallback(newDoc => {
    pdfInstance.current.updateContainer(newDoc);
  }, []);
  return [state, update];
};

const PDFViewer = _ref => {
  let {
    title,
    style,
    className,
    children,
    innerRef,
    showToolbar = true,
    ...props
  } = _ref;
  const [instance, updateInstance] = usePDF();
  useEffect(() => updateInstance(children), [children]);
  const src = instance.url ? `${instance.url}#toolbar=${showToolbar ? 1 : 0}` : null;
  return /*#__PURE__*/jsx("iframe", {
    src: src,
    title: title,
    ref: innerRef,
    style: style,
    className: className,
    ...props
  });
};

const BlobProvider = _ref => {
  let {
    document: doc,
    children
  } = _ref;
  const [instance, updateInstance] = usePDF();
  useEffect(() => updateInstance(doc), [doc]);
  if (!doc) {
    console.warn('You should pass a valid document to BlobProvider');
    return null;
  }
  return children(instance);
};

const PDFDownloadLinkBase = (_ref, ref) => {
  let {
    fileName = 'document.pdf',
    document: doc,
    children,
    onClick,
    href,
    ...rest
  } = _ref;
  const [instance, updateInstance] = usePDF();
  useEffect(() => updateInstance(doc), [doc]);
  if (!doc) {
    console.warn('You should pass a valid document to PDFDownloadLink');
    return null;
  }
  const handleDownloadIE = () => {
    if (instance && window.navigator.msSaveBlob) {
      // IE
      window.navigator.msSaveBlob(instance.blob, fileName);
    }
  };
  const handleClick = event => {
    handleDownloadIE();
    if (typeof onClick === 'function') onClick(event, instance);
  };
  return /*#__PURE__*/jsx("a", {
    href: instance.url,
    download: fileName,
    onClick: handleClick,
    ref: ref,
    ...rest,
    children: typeof children === 'function' ? children(instance) : children
  });
};
const PDFDownloadLink = /*#__PURE__*/forwardRef(PDFDownloadLinkBase);

const throwEnvironmentError = name => {
  throw new Error(`${name} is a Node specific API. You're either using this method in a browser, or your bundler is not loading react-pdf from the appropriate web build.`);
};
const renderToStream = () => {
  throwEnvironmentError('renderToStream');
};
const renderToBuffer = () => {
  throwEnvironmentError('renderToBuffer');
};
const renderToString = () => {
  throwEnvironmentError('renderToString');
};
const renderToFile = () => {
  throwEnvironmentError('renderToFile');
};
const render = () => {
  throwEnvironmentError('render');
};

// TODO: remove this default export in next major release because it breaks tree-shacking
var index = {
  pdf,
  usePDF,
  Font,
  version,
  StyleSheet,
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
//# sourceMappingURL=react-pdf.browser.js.map
