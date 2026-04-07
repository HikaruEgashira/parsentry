import { upperFirst, capitalize, parseFloat as parseFloat$1, without, pick, compose, evolve, mapValues, matchPercent, castArray, isNil, omit, asyncCompose } from '@react-pdf/fns';
import * as P from '@react-pdf/primitives';
import resolveStyle, { transformColor, flatten } from '@react-pdf/stylesheet';
import layoutEngine, { fontSubstitution, wordHyphenation, scriptItemizer, textDecoration, justification, linebreaker, bidi, fromFragments } from '@react-pdf/textkit';
import * as Yoga from 'yoga-layout/load';
import { loadYoga as loadYoga$1 } from 'yoga-layout/load';
import emojiRegex from 'emoji-regex-xs';
import resolveImage from '@react-pdf/image';

/**
 * Apply transformation to text string
 *
 * @param {string} text
 * @param {string} transformation type
 * @returns {string} transformed text
 */
const transformText = (text, transformation) => {
    switch (transformation) {
        case 'uppercase':
            return text.toUpperCase();
        case 'lowercase':
            return text.toLowerCase();
        case 'capitalize':
            return capitalize(text);
        case 'upperfirst':
            return upperFirst(text);
        default:
            return text;
    }
};

const isTspan = (node) => node.type === P.Tspan;
const isTextInstance$4 = (node) => node.type === P.TextInstance;
const engines$1 = {
    bidi,
    linebreaker,
    justification,
    textDecoration,
    scriptItemizer,
    wordHyphenation,
    fontSubstitution,
};
const engine$1 = layoutEngine(engines$1);
const getFragments$1 = (fontStore, instance) => {
    if (!instance)
        return [{ string: '' }];
    const fragments = [];
    const { fill = 'black', fontFamily = 'Helvetica', fontWeight, fontStyle, fontSize = 18, textDecorationColor, textDecorationStyle, textTransform, opacity, } = instance.props;
    const _textDecoration = instance.props.textDecoration;
    const fontFamilies = typeof fontFamily === 'string' ? [fontFamily] : [...(fontFamily || [])];
    // Fallback font
    fontFamilies.push('Helvetica');
    const font = fontFamilies.map((fontFamilyName) => {
        const opts = { fontFamily: fontFamilyName, fontWeight, fontStyle };
        const obj = fontStore.getFont(opts);
        return obj?.data;
    });
    const attributes = {
        font,
        opacity,
        fontSize,
        color: fill,
        underlineStyle: textDecorationStyle,
        underline: _textDecoration === 'underline' ||
            _textDecoration === 'underline line-through' ||
            _textDecoration === 'line-through underline',
        underlineColor: textDecorationColor || fill,
        strike: _textDecoration === 'line-through' ||
            _textDecoration === 'underline line-through' ||
            _textDecoration === 'line-through underline',
        strikeStyle: textDecorationStyle,
        strikeColor: textDecorationColor || fill,
    };
    for (let i = 0; i < instance.children.length; i += 1) {
        const child = instance.children[i];
        if (isTextInstance$4(child)) {
            fragments.push({
                string: transformText(child.value, textTransform),
                attributes,
            });
        }
        else if (child) {
            fragments.push(...getFragments$1(fontStore, child));
        }
    }
    return fragments;
};
const getAttributedString$1 = (fontStore, instance) => fromFragments(getFragments$1(fontStore, instance));
const AlmostInfinity = 999999999999;
const shrinkWhitespaceFactor = { before: -0.5, after: -0.5 };
const layoutTspan = (fontStore) => (node, xOffset) => {
    const attributedString = getAttributedString$1(fontStore, node);
    const x = node.props.x === undefined ? xOffset : node.props.x;
    const y = node.props?.y || 0;
    const container = { x, y, width: AlmostInfinity, height: AlmostInfinity };
    const hyphenationCallback = node.props.hyphenationCallback ||
        fontStore?.getHyphenationCallback() ||
        null;
    const layoutOptions = { hyphenationCallback, shrinkWhitespaceFactor };
    const lines = engine$1(attributedString, container, layoutOptions).flat();
    return Object.assign({}, node, { lines });
};
// Consecutive TSpan elements should be joined with a space
const joinTSpanLines = (node) => {
    const children = node.children.map((child, index) => {
        if (!isTspan(child))
            return child;
        const textInstance = child.children[0];
        if (child.props.x === undefined &&
            index < node.children.length - 1 &&
            textInstance?.value) {
            return Object.assign({}, child, {
                children: [{ ...textInstance, value: `${textInstance.value} ` }],
            });
        }
        return child;
    }, []);
    return Object.assign({}, node, { children });
};
const layoutText$1 = (fontStore, node) => {
    if (!node.children)
        return node;
    let currentXOffset = node.props?.x || 0;
    const layoutFn = layoutTspan(fontStore);
    const joinedNode = joinTSpanLines(node);
    const children = joinedNode.children.map((child) => {
        const childWithLayout = layoutFn(child, currentXOffset);
        currentXOffset += childWithLayout.lines[0].xAdvance;
        return childWithLayout;
    });
    return Object.assign({}, node, { children });
};

const isDefs = (node) => node.type === P.Defs;
const getDefs = (node) => {
    const children = node.children || [];
    const defs = children.find(isDefs);
    const values = defs?.children || [];
    return values.reduce((acc, value) => {
        const id = value.props?.id;
        if (id)
            acc[id] = value;
        return acc;
    }, {});
};

const isNotDefs = (node) => node.type !== P.Defs;
const detachDefs = (node) => {
    if (!node.children)
        return node;
    const children = node.children.filter(isNotDefs);
    return Object.assign({}, node, { children });
};
const URL_REGEX = /url\(['"]?#([^'"]+)['"]?\)/;
const replaceDef = (defs, value) => {
    if (!value)
        return undefined;
    if (!URL_REGEX.test(value))
        return value;
    const match = value.match(URL_REGEX);
    return defs[match[1]];
};
const parseNodeDefs = (defs) => (node) => {
    const props = node.props;
    const fill = `fill` in props ? replaceDef(defs, props?.fill) : undefined;
    const clipPath = `clipPath` in props
        ? replaceDef(defs, props?.clipPath)
        : undefined;
    const newProps = Object.assign({}, node.props, { fill, clipPath });
    const children = node.children
        ? node.children.map(parseNodeDefs(defs))
        : undefined;
    return Object.assign({}, node, { props: newProps, children });
};
const parseDefs = (root) => {
    if (!root.children)
        return root;
    const defs = getDefs(root);
    const children = root.children.map(parseNodeDefs(defs));
    return Object.assign({}, root, { children });
};
const replaceDefs = (node) => {
    return detachDefs(parseDefs(node));
};

const parseViewbox = (value) => {
    if (!value)
        return null;
    if (typeof value !== 'string')
        return value;
    const values = value.split(/[,\s]+/).map(parseFloat$1);
    if (values.length !== 4)
        return null;
    return { minX: values[0], minY: values[1], maxX: values[2], maxY: values[3] };
};

const getContainer$1 = (node) => {
    const viewbox = parseViewbox(node.props.viewBox);
    if (viewbox) {
        return { width: viewbox.maxX, height: viewbox.maxY };
    }
    if (node.props.width && node.props.height) {
        return {
            width: parseFloat$1(node.props.width),
            height: parseFloat$1(node.props.height),
        };
    }
    return { width: 0, height: 0 };
};

const BASE_SVG_INHERITED_PROPS = [
    'x',
    'y',
    'clipPath',
    'clipRule',
    'opacity',
    'fill',
    'fillOpacity',
    'fillRule',
    'stroke',
    'strokeLinecap',
    'strokeLinejoin',
    'strokeOpacity',
    'strokeWidth',
    'textAnchor',
    'dominantBaseline',
    'color',
    'fontFamily',
    'fontSize',
    'fontStyle',
    'fontWeight',
    'letterSpacing',
    'opacity',
    'textDecoration',
    'lineHeight',
    'textAlign',
    'visibility',
    'wordSpacing',
];
// Do not inherit "x" for <tspan> elements from <text> parent
const TEXT_SVG_INHERITED_PROPS = without(['x'], BASE_SVG_INHERITED_PROPS);
const SVG_INHERITED_PROPS = {
    [P.Text]: TEXT_SVG_INHERITED_PROPS,
};
const getInheritProps = (node) => {
    const props = node.props || {};
    const svgInheritedProps = SVG_INHERITED_PROPS[node.type] ?? BASE_SVG_INHERITED_PROPS;
    return pick(svgInheritedProps, props);
};
const inheritProps = (node) => {
    if (!node.children)
        return node;
    const inheritedProps = getInheritProps(node);
    const children = node.children.map((child) => {
        const props = Object.assign({}, inheritedProps, child.props || {});
        const newChild = Object.assign({}, child, { props });
        return inheritProps(newChild);
    });
    return Object.assign({}, node, { children });
};

const parseAspectRatio = (value) => {
    if (typeof value !== 'string')
        return value;
    const match = value
        .replace(/[\s\r\t\n]+/gm, ' ')
        .replace(/^defer\s/, '')
        .split(' ');
    const align = (match[0] || 'xMidYMid');
    const meetOrSlice = (match[1] ||
        'meet');
    return { align, meetOrSlice };
};

const STYLE_PROPS = [
    'width',
    'height',
    'color',
    'stroke',
    'strokeWidth',
    'opacity',
    'fillOpacity',
    'strokeOpacity',
    'fill',
    'fillRule',
    'clipPath',
    'offset',
    'transform',
    'strokeLinejoin',
    'strokeLinecap',
    'strokeDasharray',
    'gradientUnits',
    'gradientTransform',
];
const VERTICAL_PROPS = ['y', 'y1', 'y2', 'height', 'cy', 'ry'];
const HORIZONTAL_PROPS = ['x', 'x1', 'x2', 'width', 'cx', 'rx'];
const isSvg$3 = (node) => node.type === P.Svg;
const isText$5 = (node) => node.type === P.Text;
const isTextInstance$3 = (node) => node.type === P.TextInstance;
const transformPercent = (container) => (props) => mapValues(props, (value, key) => {
    const match = matchPercent(value);
    if (match && VERTICAL_PROPS.includes(key)) {
        return match.percent * container.height;
    }
    if (match && HORIZONTAL_PROPS.includes(key)) {
        return match.percent * container.width;
    }
    return value;
});
const parsePercent = (value) => {
    const match = matchPercent(value);
    return match ? match.percent : parseFloat$1(value);
};
const parseTransform = (container) => (value) => {
    return resolveStyle(container, { transform: value }).transform;
};
const parseProps = (container) => (node) => {
    let props = transformPercent(container)(node.props);
    props = evolve({
        x: parseFloat$1,
        x1: parseFloat$1,
        x2: parseFloat$1,
        y: parseFloat$1,
        y1: parseFloat$1,
        y2: parseFloat$1,
        r: parseFloat$1,
        rx: parseFloat$1,
        ry: parseFloat$1,
        cx: parseFloat$1,
        cy: parseFloat$1,
        width: parseFloat$1,
        height: parseFloat$1,
        offset: parsePercent,
        fill: transformColor,
        opacity: parsePercent,
        stroke: transformColor,
        stopOpacity: parsePercent,
        stopColor: transformColor,
        transform: parseTransform(container),
        gradientTransform: parseTransform(container),
    }, props);
    return Object.assign({}, node, { props });
};
const mergeStyles$1 = (node) => {
    const style = node.style || {};
    const props = Object.assign({}, style, node.props);
    return Object.assign({}, node, { props });
};
const removeNoneValues = (node) => {
    const removeNone = (value) => (value === 'none' ? null : value);
    const props = mapValues(node.props, removeNone);
    return Object.assign({}, node, { props });
};
const pickStyleProps = (node) => {
    const props = node.props || {};
    const styleProps = pick(STYLE_PROPS, props);
    const style = Object.assign({}, styleProps, node.style || {});
    return Object.assign({}, node, { style });
};
const parseSvgProps = (node) => {
    const props = evolve({
        width: parseFloat$1,
        height: parseFloat$1,
        viewBox: parseViewbox,
        preserveAspectRatio: parseAspectRatio,
    }, node.props);
    return Object.assign({}, node, { props });
};
const wrapBetweenTspan = (node) => ({
    type: P.Tspan,
    props: {},
    style: {},
    children: [node],
});
const addMissingTspan = (node) => {
    if (!isText$5(node))
        return node;
    if (!node.children)
        return node;
    const resolveChild = (child) => isTextInstance$3(child) ? wrapBetweenTspan(child) : child;
    const children = node.children.map(resolveChild);
    return Object.assign({}, node, { children });
};
const parseText = (fontStore) => (node) => {
    if (isText$5(node))
        return layoutText$1(fontStore, node);
    if (!node.children)
        return node;
    const children = node.children.map(parseText(fontStore));
    return Object.assign({}, node, { children });
};
const resolveSvgNode = (container) => compose(parseProps(container), addMissingTspan, removeNoneValues, mergeStyles$1);
const resolveChildren = (container) => (node) => {
    if (!node.children)
        return node;
    const resolveChild = compose(resolveChildren(container), resolveSvgNode(container));
    const children = node.children.map(resolveChild);
    return Object.assign({}, node, { children });
};
const buildXLinksIndex = (node) => {
    const idIndex = {};
    const listToExplore = node.children?.slice(0) || [];
    while (listToExplore.length > 0) {
        const child = listToExplore.shift();
        if (child.props && 'id' in child.props) {
            idIndex[child.props.id] = child;
        }
        if (child.children)
            listToExplore.push(...child.children);
    }
    return idIndex;
};
const replaceXLinks = (node, idIndex) => {
    if (node.props && 'xlinkHref' in node.props) {
        const linkedNode = idIndex[node.props.xlinkHref.replace(/^#/, '')];
        // No node to extend from
        if (!linkedNode)
            return node;
        const newProps = Object.assign({}, linkedNode.props, node.props);
        delete newProps.xlinkHref;
        return Object.assign({}, linkedNode, { props: newProps });
    }
    const children = node.children?.map((child) => replaceXLinks(child, idIndex));
    return Object.assign({}, node, { children });
};
const resolveXLinks = (node) => {
    const idIndex = buildXLinksIndex(node);
    return replaceXLinks(node, idIndex);
};
const resolveSvgRoot = (node, fontStore) => {
    const container = getContainer$1(node);
    return compose(replaceDefs, parseText(fontStore), parseSvgProps, pickStyleProps, inheritProps, resolveChildren(container), resolveXLinks)(node);
};
/**
 * Pre-process SVG nodes so they can be rendered in the next steps
 *
 * @param node - Root node
 * @param fontStore - Font store
 * @returns Root node
 */
const resolveSvg = (node, fontStore) => {
    if (!('children' in node))
        return node;
    const resolveChild = (child) => resolveSvg(child, fontStore);
    const root = isSvg$3(node) ? resolveSvgRoot(node, fontStore) : node;
    const children = root.children?.map(resolveChild);
    return Object.assign({}, root, { children });
};

let instancePromise;
const loadYoga = async () => {
    // Yoga WASM binaries must be asynchronously compiled and loaded
    // to prevent Event emitter memory leak warnings, Yoga must be loaded only once
    const instance = await (instancePromise ??= loadYoga$1());
    const config = instance.Config.create();
    config.setPointScaleFactor(0);
    const node = { create: () => instance.Node.createWithConfig(config) };
    return { node };
};

const resolveYoga = async (root) => {
    const yoga = await loadYoga();
    return Object.assign({}, root, { yoga });
};

const getZIndex = (node) => node.style.zIndex;
const shouldSort = (node) => node.type !== P.Document && node.type !== P.Svg;
const sortZIndex = (a, b) => {
    const za = getZIndex(a);
    const zb = getZIndex(b);
    if (!za && !zb)
        return 0;
    if (!za)
        return 1;
    if (!zb)
        return -1;
    return zb - za;
};
/**
 * Sort children by zIndex value
 *
 * @param node
 * @returns Node
 */
const resolveNodeZIndex = (node) => {
    if (!node.children)
        return node;
    const sortedChildren = shouldSort(node)
        ? node.children.sort(sortZIndex)
        : node.children;
    const children = sortedChildren.map(resolveNodeZIndex);
    return Object.assign({}, node, { children });
};
/**
 * Sort children by zIndex value
 *
 * @param node
 * @returns Node
 */
const resolveZIndex = (root) => resolveNodeZIndex(root);

/* eslint-disable no-console */
// Caches emoji images data
const emojis = {};
const regex = emojiRegex();
/**
 * When an emoji as no variations, it might still have 2 parts,
 * the canonical emoji and an empty string.
 * ex.
 *   (no color) Array.from('❤️') => ["❤", "️"]
 *   (w/ color) Array.from('👍🏿') => ["👍", "🏿"]
 *
 * The empty string needs to be removed otherwise the generated
 * url will be incorect.
 */
const removeVariationSelectors = (x) => x !== '️';
const getCodePoints = (string, withVariationSelectors = false) => Array.from(string)
    .filter(withVariationSelectors ? () => true : removeVariationSelectors)
    .map((char) => char.codePointAt(0).toString(16))
    .join('-');
const buildEmojiUrl = (emoji, source) => {
    if ('builder' in source) {
        return source.builder(getCodePoints(emoji, source.withVariationSelectors));
    }
    const { url, format = 'png', withVariationSelectors } = source;
    return `${url}${getCodePoints(emoji, withVariationSelectors)}.${format}`;
};
const fetchEmojis = (string, source) => {
    if (!source)
        return [];
    const promises = [];
    Array.from(string.matchAll(regex)).forEach((match) => {
        const emoji = match[0];
        if (!emojis[emoji] || emojis[emoji].loading) {
            const emojiUrl = buildEmojiUrl(emoji, source);
            emojis[emoji] = { loading: true };
            promises.push(resolveImage({ uri: emojiUrl })
                .then((image) => {
                emojis[emoji].loading = false;
                emojis[emoji].data = image.data;
            })
                .catch((e) => {
                console.warn(e, 'Failed to load emoji image');
                emojis[emoji].loading = false;
            }));
        }
    });
    return promises;
};
const embedEmojis = (fragments) => {
    const result = [];
    for (let i = 0; i < fragments.length; i += 1) {
        const fragment = fragments[i];
        let lastIndex = 0;
        Array.from(fragment.string.matchAll(regex)).forEach((match) => {
            const { index } = match;
            const emoji = match[0];
            const emojiSize = fragment.attributes.fontSize;
            const chunk = fragment.string.slice(lastIndex, index + match[0].length);
            // If emoji image was found, we create a new fragment with the
            // correct attachment and object substitution character;
            if (emojis[emoji] && emojis[emoji].data) {
                result.push({
                    string: chunk.replace(match[0], String.fromCharCode(0xfffc)),
                    attributes: {
                        ...fragment.attributes,
                        attachment: {
                            width: emojiSize,
                            height: emojiSize,
                            yOffset: Math.floor(emojiSize * 0.1),
                            image: emojis[emoji].data,
                        },
                    },
                });
            }
            else {
                // If no emoji data, we try to use emojis in the font
                result.push({
                    string: chunk,
                    attributes: fragment.attributes,
                });
            }
            lastIndex = index + emoji.length;
        });
        if (lastIndex < fragment.string.length) {
            result.push({
                string: fragment.string.slice(lastIndex),
                attributes: fragment.attributes,
            });
        }
    }
    return result;
};

/**
 * Parses an HTML-compatible srcSet string into structured entries.
 * Only width descriptors (e.g. "300w") are supported.
 */
const parseSrcSet = (srcSet) => {
    return srcSet
        .split(',')
        .map((entry) => entry.trim())
        .filter(Boolean)
        .reduce((acc, entry) => {
        const parts = entry.split(/\s+/);
        const uri = parts[0];
        const descriptor = parts[1];
        if (descriptor && descriptor.endsWith('w')) {
            const width = parseFloat(descriptor);
            if (!Number.isNaN(width))
                acc.push({ uri, width });
        }
        return acc;
    }, []);
};
/**
 * Selects the most appropriate image source from a parsed srcSet
 * based on the target display width.
 *
 * Picks the smallest source that is >= targetWidth,
 * or the largest available if none are large enough.
 */
const selectSource = (entries, targetWidth) => {
    if (entries.length === 0)
        return '';
    const sorted = [...entries].sort((a, b) => a.width - b.width);
    const match = sorted.find((e) => e.width >= targetWidth);
    return match ? match.uri : sorted[sorted.length - 1].uri;
};
// TODO: Unify with page/getSize.ts parseValue/transformUnit into a shared unit parser
/**
 * Parses a CSS length value into points.
 * Supports: px, pt (default), in, mm, cm, vw, %.
 * vw and % are resolved relative to containerWidth.
 */
const parseLength = (value, containerWidth) => {
    const match = /^(-?\d*\.?\d+)\s*(px|pt|in|mm|cm|vw|%)?$/i.exec(value.trim());
    if (!match)
        return undefined;
    const num = parseFloat(match[1]);
    if (Number.isNaN(num))
        return undefined;
    const unit = match[2]?.toLowerCase();
    switch (unit) {
        case 'in':
            return num * 72;
        case 'mm':
            return num * (72 / 25.4);
        case 'cm':
            return num * (72 / 2.54);
        case 'px':
            return num;
        case 'vw':
            return (num / 100) * containerWidth;
        case '%':
            return (num / 100) * containerWidth;
        case 'pt':
        default:
            return num;
    }
};
/**
 * Evaluates a media condition like "(min-width: 600px)" or "(max-width: 400px)"
 * against the container width.
 */
const matchesCondition = (condition, containerWidth) => {
    const match = /\(\s*(min-width|max-width)\s*:\s*(-?\d*\.?\d+)\s*(px|pt|in|mm|cm|vw|%)?\s*\)/i.exec(condition);
    if (!match)
        return false;
    const feature = match[1].toLowerCase();
    const threshold = parseLength(`${match[2]}${match[3] || ''}`, containerWidth);
    if (threshold == null)
        return false;
    if (feature === 'min-width')
        return containerWidth >= threshold;
    if (feature === 'max-width')
        return containerWidth <= threshold;
    return false;
};
/**
 * Parses sizes attribute to get the target display width.
 *
 * Accepts a number (used directly as points) or a string.
 * For strings, evaluates media conditions against containerWidth
 * and returns the first matching entry's length value.
 *
 * Format: "(media-condition) length, (media-condition) length, default-length"
 */
const parseSizes = (sizes, containerWidth) => {
    if (sizes == null)
        return undefined;
    if (typeof sizes === 'number')
        return sizes;
    const entries = sizes.split(',').map((s) => s.trim());
    for (const entry of entries) {
        const conditionMatch = /^\(([^)]+)\)\s+(.+)$/.exec(entry);
        if (conditionMatch) {
            const condition = `(${conditionMatch[1]})`;
            const lengthStr = conditionMatch[2];
            if (matchesCondition(condition, containerWidth)) {
                return parseLength(lengthStr, containerWidth);
            }
        }
        else {
            return parseLength(entry, containerWidth);
        }
    }
    return undefined;
};
/**
 * Get image source, resolving srcSet if present.
 *
 * @param node - Image node
 * @param pageWidth - Page width for srcSet resolution
 * @returns Image src
 */
const getSource = (node, pageWidth) => {
    const { srcSet, sizes } = node.props;
    if (srcSet) {
        const entries = parseSrcSet(srcSet);
        if (entries.length > 0) {
            const targetWidth = parseSizes(sizes, pageWidth) || pageWidth;
            const uri = selectSource(entries, targetWidth);
            if (uri)
                return uri;
        }
    }
    if (node.props.src)
        return node.props.src;
    if (node.props.source)
        return node.props.source;
};

/**
 * Resolves `src` to `@react-pdf/image` interface.
 *
 * Also it handles factories and async sources.
 *
 * @param src
 * @returns Resolved src
 */
const resolveSource = async (src) => {
    const source = typeof src === 'function' ? await src() : await src;
    return typeof source === 'string' ? { uri: source } : source;
};

/**
 * Fetches image and appends data to node.
 * Ideally this fn should be immutable.
 *
 * @param node - Image or ImageBackground node
 * @param pageWidth - Page width for srcSet resolution
 */
const fetchImage = async (node, pageWidth) => {
    const { cache } = node.props;
    const src = getSource(node, pageWidth);
    if (!src) {
        console.warn(false, 'Image should receive either a "src" or "source" prop');
        return;
    }
    try {
        const source = await resolveSource(src);
        if (!source) {
            throw new Error(`Image's "src" or "source" prop returned ${source}`);
        }
        node.image = await resolveImage(source, { cache });
        if (Buffer.isBuffer(source) || source instanceof Blob)
            return;
        node.image.key = 'data' in source ? source.data.toString() : source.uri;
    }
    catch (e) {
        console.warn(e.message);
    }
};

const isImage$2 = (node) => node.type === P.Image;
const isImageBackground = (node) => node.type === P.ImageBackground;
/**
 * Get all asset promises that need to be resolved
 *
 * @param fontStore - Font store
 * @param node - Root node
 * @returns Asset promises
 */
const fetchAssets = (fontStore, node, pageWidth) => {
    const promises = [];
    const listToExplore = [node];
    const emojiSource = fontStore ? fontStore.getEmojiSource() : null;
    while (listToExplore.length > 0) {
        const n = listToExplore.shift();
        if (isImage$2(n) || isImageBackground(n)) {
            promises.push(fetchImage(n, pageWidth));
        }
        if (fontStore && n.style?.fontFamily) {
            const fontFamilies = castArray(n.style.fontFamily);
            promises.push(...fontFamilies.map((fontFamily) => fontStore.load({
                fontFamily,
                fontStyle: n.style.fontStyle,
                fontWeight: n.style.fontWeight,
            })));
        }
        if (typeof n === 'string') {
            promises.push(...fetchEmojis(n, emojiSource));
        }
        if ('value' in n && typeof n.value === 'string') {
            promises.push(...fetchEmojis(n.value, emojiSource));
        }
        if (n.children) {
            n.children.forEach((childNode) => {
                listToExplore.push(childNode);
            });
        }
    }
    return promises;
};
/**
 * Fetch assets for a page
 *
 * @param fontStore - Font store
 * @param page - Page node
 * @returns Asset promises
 */
const fetchPageAssets = (fontStore, page) => {
    const pageWidth = page.style?.width;
    return page.children
        ?.map((child) => fetchAssets(fontStore, child, pageWidth))
        .flat();
};
/**
 * Fetch image, font and emoji assets in parallel.
 * Layout process will not be resumed until promise resolves.
 *
 * @param node root node
 * @param fontStore font store
 * @returns Root node
 */
const resolveAssets = async (node, fontStore) => {
    const promises = node.children
        .map((page) => fetchPageAssets(fontStore, page))
        .flat();
    await Promise.all(promises);
    return node;
};

const isLink$1 = (node) => node.type === P.Link;
const DEFAULT_LINK_STYLES = {
    color: 'blue',
    textDecoration: 'underline',
};
/**
 * Computes styles using stylesheet
 *
 * @param container
 * @param node - Document node
 * @returns Computed styles
 */
const computeStyle = (container, node) => {
    let baseStyle = [node.style];
    if (isLink$1(node)) {
        baseStyle = Array.isArray(node.style)
            ? [DEFAULT_LINK_STYLES, ...node.style]
            : [DEFAULT_LINK_STYLES, node.style];
    }
    return resolveStyle(container, baseStyle);
};
/**
 * Resolves node styles
 *
 * @param container
 * @returns Resolve node styles
 */
const resolveNodeStyles = (container) => (node) => {
    const style = computeStyle(container, node);
    if (!node.children)
        return Object.assign({}, node, { style });
    const children = node.children.map(resolveNodeStyles(container));
    return Object.assign({}, node, { style, children });
};
/**
 * Resolves page styles
 *
 * @param page Document page
 * @returns Document page with resolved styles
 */
const resolvePageStyles = (page) => {
    const dpi = page.props?.dpi || 72;
    const style = page.style;
    const width = page.box?.width || style.width;
    const height = page.box?.height || style.height;
    const orientation = page.props?.orientation || 'portrait';
    const remBase = style?.fontSize || 18;
    const container = { width, height, orientation, dpi, remBase };
    return resolveNodeStyles(container)(page);
};
/**
 * Resolves document styles
 *
 * @param root - Document root
 * @returns Document root with resolved styles
 */
const resolveStyles = (root) => {
    if (!root.children)
        return root;
    const children = root.children.map(resolvePageStyles);
    return Object.assign({}, root, { children });
};

const getTransformStyle = (s) => (node) => isNil(node.style?.[s]) ? '50%' : node.style?.[s] ?? null;
/**
 * Get node origin
 *
 * @param node
 * @returns {{ left?: number, top?: number }} node origin
 */
const getOrigin = (node) => {
    if (!node.box)
        return null;
    const { left, top, width, height } = node.box;
    const transformOriginX = getTransformStyle('transformOriginX')(node);
    const transformOriginY = getTransformStyle('transformOriginY')(node);
    const percentX = matchPercent(transformOriginX);
    const percentY = matchPercent(transformOriginY);
    const offsetX = percentX ? width * percentX.percent : transformOriginX;
    const offsetY = percentY ? height * percentY.percent : transformOriginY;
    if (isNil(offsetX) || typeof offsetX === 'string')
        throw new Error(`Invalid origin offsetX: ${offsetX}`);
    if (isNil(offsetY) || typeof offsetY === 'string')
        throw new Error(`Invalid origin offsetY: ${offsetY}`);
    return { left: left + offsetX, top: top + offsetY };
};

/**
 * Resolve node origin
 *
 * @param node
 * @returns Node with origin attribute
 */
const resolveNodeOrigin = (node) => {
    const origin = getOrigin(node);
    const newNode = Object.assign({}, node, { origin });
    if (!node.children)
        return newNode;
    const children = node.children.map(resolveNodeOrigin);
    return Object.assign({}, newNode, { children });
};
/**
 * Resolve document origins
 *
 * @param root - Document root
 * @returns Document root
 */
const resolveOrigin = (root) => {
    if (!root.children)
        return root;
    const children = root.children.map(resolveNodeOrigin);
    return Object.assign({}, root, { children });
};

/**
 * Normalize bookmark value, expanding a plain string title into a full
 * bookmark object with default fit and expanded values
 *
 * @param bookmark - Bookmark value
 * @returns Normalized bookmark object
 */
const getBookmarkValue = (bookmark) => {
    return typeof bookmark === 'string'
        ? { title: bookmark, fit: false, expanded: false }
        : bookmark;
};
/**
 * Traverse document tree and resolve bookmark hierarchy, assigning each
 * bookmark a ref index and a reference to its nearest bookmark ancestor
 *
 * @param node - Document node
 * @returns Document node with resolved bookmarks
 */
const resolveBookmarks = (node) => {
    let refs = 0;
    const listToExplore = (node.children || []).map((value) => ({
        value,
        parent: null,
    }));
    while (listToExplore.length > 0) {
        const element = listToExplore.shift();
        if (!element)
            break;
        const child = element.value;
        let parent = element.parent;
        if (child.props && 'bookmark' in child.props && child.props.bookmark) {
            const bookmark = getBookmarkValue(child.props.bookmark);
            const ref = refs++;
            const newHierarchy = { ref, parent: parent?.ref, ...bookmark };
            child.props.bookmark = newHierarchy;
            parent = newHierarchy;
        }
        if (child.children) {
            child.children.forEach((childNode) => {
                listToExplore.push({ value: childNode, parent });
            });
        }
    }
    return node;
};

const VALID_ORIENTATIONS = ['portrait', 'landscape'];
/**
 * Get page orientation. Defaults to portrait
 *
 * @param page - Page object
 * @returns Page orientation
 */
const getOrientation = (page) => {
    const value = page.props?.orientation || 'portrait';
    return VALID_ORIENTATIONS.includes(value) ? value : 'portrait';
};

/**
 * Return true if page is landscape
 *
 * @param page - Page instance
 * @returns Is page landscape
 */
const isLandscape = (page) => getOrientation(page) === 'landscape';

// Page sizes for 72dpi. 72dpi is used internally by pdfkit.
const PAGE_SIZES = {
    '4A0': [4767.87, 6740.79],
    '2A0': [3370.39, 4767.87],
    A0: [2383.94, 3370.39],
    A1: [1683.78, 2383.94],
    A2: [1190.55, 1683.78],
    A3: [841.89, 1190.55],
    A4: [595.28, 841.89],
    A5: [419.53, 595.28],
    A6: [297.64, 419.53],
    A7: [209.76, 297.64],
    A8: [147.4, 209.76],
    A9: [104.88, 147.4],
    A10: [73.7, 104.88],
    B0: [2834.65, 4008.19],
    B1: [2004.09, 2834.65],
    B2: [1417.32, 2004.09],
    B3: [1000.63, 1417.32],
    B4: [708.66, 1000.63],
    B5: [498.9, 708.66],
    B6: [354.33, 498.9],
    B7: [249.45, 354.33],
    B8: [175.75, 249.45],
    B9: [124.72, 175.75],
    B10: [87.87, 124.72],
    C0: [2599.37, 3676.54],
    C1: [1836.85, 2599.37],
    C2: [1298.27, 1836.85],
    C3: [918.43, 1298.27],
    C4: [649.13, 918.43],
    C5: [459.21, 649.13],
    C6: [323.15, 459.21],
    C7: [229.61, 323.15],
    C8: [161.57, 229.61],
    C9: [113.39, 161.57],
    C10: [79.37, 113.39],
    RA0: [2437.8, 3458.27],
    RA1: [1729.13, 2437.8],
    RA2: [1218.9, 1729.13],
    RA3: [864.57, 1218.9],
    RA4: [609.45, 864.57],
    SRA0: [2551.18, 3628.35],
    SRA1: [1814.17, 2551.18],
    SRA2: [1275.59, 1814.17],
    SRA3: [907.09, 1275.59],
    SRA4: [637.8, 907.09],
    EXECUTIVE: [521.86, 756.0],
    FOLIO: [612.0, 936.0],
    LEGAL: [612.0, 1008.0],
    LETTER: [612.0, 792.0],
    TABLOID: [792.0, 1224.0],
    ID1: [153, 243],
};
/**
 * Parses scalar value in value and unit pairs
 *
 * @param value - Scalar value
 * @returns Parsed value
 */
const parseValue = (value) => {
    if (typeof value === 'number')
        return { value, unit: undefined };
    const match = /^(-?\d*\.?\d+)(in|mm|cm|pt|px)?$/g.exec(value);
    return match
        ? { value: parseFloat(match[1]), unit: match[2] || 'pt' }
        : { value, unit: undefined };
};
/**
 * Transform given scalar value to 72dpi equivalent of size
 *
 * @param value - Styles value
 * @param inputDpi - User defined dpi
 * @returns Transformed value
 */
const transformUnit = (value, inputDpi) => {
    if (!value)
        return 0;
    const scalar = parseValue(value);
    const outputDpi = 72;
    const mmFactor = (1 / 25.4) * outputDpi;
    const cmFactor = (1 / 2.54) * outputDpi;
    if (typeof scalar.value === 'string')
        throw new Error(`Invalid page size: ${value}`);
    switch (scalar.unit) {
        case 'in':
            return scalar.value * outputDpi;
        case 'mm':
            return scalar.value * mmFactor;
        case 'cm':
            return scalar.value * cmFactor;
        case 'px':
            return Math.round(scalar.value * (outputDpi / inputDpi));
        default:
            return scalar.value;
    }
};
const transformUnits = ({ width, height }, dpi) => ({
    width: transformUnit(width, dpi),
    height: transformUnit(height, dpi),
});
/**
 * Transforms array into size object
 *
 * @param v - Values array
 * @returns Size object with width and height
 */
const toSizeObject = (v) => ({
    width: v[0],
    height: v[1],
});
/**
 * Flip size object
 *
 * @param v - Size object
 * @returns Flipped size object
 */
const flipSizeObject = (v) => ({
    width: v.height,
    height: v.width,
});
/**
 * Returns size object from a given string
 *
 * @param v - Page size string
 * @returns Size object with width and height
 */
const getStringSize = (v) => {
    return toSizeObject(PAGE_SIZES[v.toUpperCase()]);
};
/**
 * Returns size object from a single number
 *
 * @param n - Page size number
 * @returns Size object with width and height
 */
const getNumberSize = (n) => toSizeObject([n, n]);
/**
 * Return page size in an object { width, height }
 *
 * @param page - Page node
 * @returns Size object with width and height
 */
const getSize = (page) => {
    const value = page.props?.size || 'A4';
    const dpi = page.props?.dpi || 72;
    let size;
    if (typeof value === 'string') {
        size = getStringSize(value);
    }
    else if (Array.isArray(value)) {
        size = transformUnits(toSizeObject(value), dpi);
    }
    else if (typeof value === 'number') {
        size = transformUnits(getNumberSize(value), dpi);
    }
    else {
        size = transformUnits(value, dpi);
    }
    return isLandscape(page) ? flipSizeObject(size) : size;
};

/**
 * Resolves page size
 *
 * @param page
 * @returns Page with resolved size in style attribute
 */
const resolvePageSize = (page) => {
    const size = getSize(page);
    const style = flatten(page.style || {});
    return { ...page, style: { ...style, ...size } };
};
/**
 * Resolves page sizes
 *
 * @param root  -Document root
 * @returns Document root with resolved page sizes
 */
const resolvePageSizes = (root) => {
    if (!root.children)
        return root;
    const children = root.children.map(resolvePageSize);
    return Object.assign({}, root, { children });
};

const isFixed = (node) => {
    if (!node.props)
        return false;
    return 'fixed' in node.props ? node.props.fixed === true : false;
};

/**
 * Get line index at given height
 *
 * @param node
 * @param height
 */
const lineIndexAtHeight = (node, height) => {
    let y = 0;
    if (!node.lines)
        return 0;
    for (let i = 0; i < node.lines.length; i += 1) {
        const line = node.lines[i];
        if (y + line.box.height > height)
            return i;
        y += line.box.height;
    }
    return node.lines.length;
};

/**
 * Get height for given text line index
 *
 * @param node
 * @param index
 */
const heightAtLineIndex = (node, index) => {
    let counter = 0;
    if (!node.lines)
        return counter;
    for (let i = 0; i < index; i += 1) {
        const line = node.lines[i];
        if (!line)
            break;
        counter += line.box.height;
    }
    return counter;
};

const getLineBreak = (node, height) => {
    const top = node.box?.top || 0;
    const widows = node.props.widows || 2;
    const orphans = node.props.orphans || 2;
    const linesQuantity = node.lines.length;
    const slicedLine = lineIndexAtHeight(node, height - top);
    if (slicedLine === 0) {
        return 0;
    }
    if (linesQuantity < orphans) {
        return linesQuantity;
    }
    if (slicedLine < orphans || linesQuantity < orphans + widows) {
        return 0;
    }
    if (linesQuantity === orphans + widows) {
        return orphans;
    }
    if (linesQuantity - slicedLine < widows) {
        return linesQuantity - widows;
    }
    return slicedLine;
};
// Also receives contentArea in case it's needed
const splitText = (node, height) => {
    const slicedLineIndex = getLineBreak(node, height);
    const currentHeight = heightAtLineIndex(node, slicedLineIndex);
    const nextHeight = node.box.height - currentHeight;
    const current = Object.assign({}, node, {
        box: {
            ...node.box,
            height: currentHeight,
            borderBottomWidth: 0,
        },
        style: {
            ...node.style,
            marginBottom: 0,
            paddingBottom: 0,
            borderBottomWidth: 0,
            borderBottomLeftRadius: 0,
            borderBottomRightRadius: 0,
        },
        lines: node.lines.slice(0, slicedLineIndex),
    });
    const next = Object.assign({}, node, {
        box: {
            ...node.box,
            top: 0,
            height: nextHeight,
            borderTopWidth: 0,
        },
        style: {
            ...node.style,
            marginTop: 0,
            paddingTop: 0,
            borderTopWidth: 0,
            borderTopLeftRadius: 0,
            borderTopRightRadius: 0,
        },
        lines: node.lines.slice(slicedLineIndex),
    });
    return [current, next];
};

const getTop$1 = (node) => node.box?.top || 0;
const hasFixedHeight = (node) => !isNil(node.style?.height);
const splitNode = (node, height) => {
    if (!node)
        return [null, null];
    const nodeTop = getTop$1(node);
    const current = Object.assign({}, node, {
        box: {
            ...node.box,
            borderBottomWidth: 0,
        },
        style: {
            ...node.style,
            marginBottom: 0,
            paddingBottom: 0,
            borderBottomWidth: 0,
            borderBottomLeftRadius: 0,
            borderBottomRightRadius: 0,
        },
    });
    current.style.height = height - nodeTop;
    const nextHeight = hasFixedHeight(node)
        ? node.box.height - (height - nodeTop)
        : null;
    const next = Object.assign({}, node, {
        box: {
            ...node.box,
            top: 0,
            borderTopWidth: 0,
        },
        style: {
            ...node.style,
            marginTop: 0,
            paddingTop: 0,
            borderTopWidth: 0,
            borderTopLeftRadius: 0,
            borderTopRightRadius: 0,
        },
        props: {
            ...node.props,
            bookmark: null,
        },
    });
    if (nextHeight) {
        next.style.height = nextHeight;
    }
    return [current, next];
};

const NON_WRAP_TYPES = [P.Svg, P.Note, P.Image, P.Canvas];
const getWrap = (node) => {
    if (NON_WRAP_TYPES.includes(node.type))
        return false;
    if (!node.props)
        return true;
    return 'wrap' in node.props ? node.props.wrap : true;
};

const getComputedPadding = (node, edge) => {
    const { yogaNode } = node;
    return yogaNode ? yogaNode.getComputedPadding(edge) : null;
};
/**
 * Get Yoga computed paddings. Zero otherwise
 *
 * @param  node
 * @returns paddings
 */
const getPadding = (node) => {
    const { style, box } = node;
    const paddingTop = getComputedPadding(node, Yoga.Edge.Top) ||
        box?.paddingTop ||
        style?.paddingTop ||
        0;
    const paddingRight = getComputedPadding(node, Yoga.Edge.Right) ||
        box?.paddingRight ||
        style?.paddingRight ||
        0;
    const paddingBottom = getComputedPadding(node, Yoga.Edge.Bottom) ||
        box?.paddingBottom ||
        style?.paddingBottom ||
        0;
    const paddingLeft = getComputedPadding(node, Yoga.Edge.Left) ||
        box?.paddingLeft ||
        style?.paddingLeft ||
        0;
    return { paddingTop, paddingRight, paddingBottom, paddingLeft };
};

const getWrapArea = (page) => {
    const height = page.style?.height;
    const { paddingBottom } = getPadding(page);
    return height - paddingBottom;
};

const getContentArea = (page) => {
    const height = page.style?.height;
    const { paddingTop, paddingBottom } = getPadding(page);
    return height - paddingBottom - paddingTop;
};

const isString = (value) => typeof value === 'string';
const isNumber = (value) => typeof value === 'number';
const isBoolean = (value) => typeof value === 'boolean';
const isFragment = (value) => value && value.type === Symbol.for('react.fragment');
/**
 * Transforms a react element instance to internal element format.
 *
 * Can return multiple instances in the case of arrays or fragments.
 *
 * @param element - React element
 * @returns Parsed React elements
 */
const createInstances = (element) => {
    if (!element)
        return [];
    if (Array.isArray(element)) {
        return element.reduce((acc, el) => acc.concat(createInstances(el)), []);
    }
    if (isBoolean(element)) {
        return [];
    }
    if (isString(element) || isNumber(element)) {
        return [{ type: P.TextInstance, value: `${element}` }];
    }
    if (isFragment(element)) {
        // @ts-expect-error figure out why this is complains
        return createInstances(element.props.children);
    }
    if (!isString(element.type)) {
        // @ts-expect-error figure out why this is complains
        return createInstances(element.type(element.props));
    }
    const { type, props: { style = {}, children, ...props }, } = element;
    const nextChildren = castArray(children).reduce((acc, child) => acc.concat(createInstances(child)), []);
    return [
        {
            type,
            style,
            props,
            children: nextChildren,
        },
    ];
};

const getBreak = (node) => 'break' in node.props ? node.props.break : false;
const getMinPresenceAhead = (node) => 'minPresenceAhead' in node.props ? node.props.minPresenceAhead : 0;
const getFurthestEnd = (elements) => {
    if (elements.length === 0)
        return null;
    return Math.max(...elements.map((node) => node.box.top + node.box.height));
};
const getEndOfMinPresenceAhead = (child) => {
    return (child.box.top +
        child.box.height +
        child.box.marginBottom +
        getMinPresenceAhead(child));
};
const getEndOfPresence = (child, futureElements) => {
    const afterMinPresenceAhead = getEndOfMinPresenceAhead(child);
    const nonFixedFuture = futureElements.filter((node) => !('fixed' in node.props));
    const endOfFurthestFutureElement = getFurthestEnd(nonFixedFuture);
    // When there are no future non-fixed siblings, use only minPresenceAhead
    if (endOfFurthestFutureElement === null)
        return afterMinPresenceAhead;
    return Math.min(afterMinPresenceAhead, endOfFurthestFutureElement);
};
const shouldBreak = (child, futureElements, height, previousElements) => {
    if ('fixed' in child.props)
        return false;
    const shouldSplit = height < child.box.top + child.box.height;
    const canWrap = getWrap(child);
    // Calculate the y coordinate where the desired presence of the child ends
    const endOfPresence = getEndOfPresence(child, futureElements);
    // If the child is already at the top of the page, breaking won't improve its presence
    // (as long as react-pdf does not support breaking into differently sized containers)
    const breakingImprovesPresence = previousElements.filter((node) => !isFixed(node)).length > 0;
    return (getBreak(child) ||
        (shouldSplit && !canWrap) ||
        (!shouldSplit && endOfPresence > height && breakingImprovesPresence));
};

const IGNORABLE_CODEPOINTS = [
    8232, // LINE_SEPARATOR
    8233, // PARAGRAPH_SEPARATOR
    8288, // WORD_JOINER
];
const buildSubsetForFont = (font) => IGNORABLE_CODEPOINTS.reduce((acc, codePoint) => {
    if (font &&
        font.hasGlyphForCodePoint &&
        font.hasGlyphForCodePoint(codePoint)) {
        return acc;
    }
    return [...acc, String.fromCharCode(codePoint)];
}, []);
const ignoreChars = (fragments) => fragments.map((fragment) => {
    const charSubset = buildSubsetForFont(fragment.attributes.font[0]);
    const subsetRegex = new RegExp(charSubset.join('|'));
    return {
        string: fragment.string.replace(subsetRegex, ''),
        attributes: fragment.attributes,
    };
});

const PREPROCESSORS = [ignoreChars, embedEmojis];
const isImage$1 = (node) => node.type === P.Image;
const isTextInstance$2 = (node) => node.type === P.TextInstance;
/**
 * Get textkit fragments of given node object
 *
 * @param fontStore - Font store
 * @param instance - Node
 * @param parentLink - Parent link
 * @param level - Fragment level
 * @returns Text fragments
 */
const getFragments = (fontStore, instance, parentLink = null, level = 0) => {
    if (!instance)
        return [{ string: '' }];
    let fragments = [];
    const { color = 'black', direction = 'ltr', fontFamily = 'Helvetica', fontWeight, fontStyle, fontSize = 18, textAlign, lineHeight, textDecoration, textDecorationColor, textDecorationStyle, textTransform, letterSpacing, textIndent, opacity, verticalAlign, } = instance.style;
    const fontFamilies = typeof fontFamily === 'string' ? [fontFamily] : [...(fontFamily || [])];
    // Fallback font
    fontFamilies.push('Helvetica');
    const font = fontFamilies.map((fontFamilyName) => {
        const opts = { fontFamily: fontFamilyName, fontWeight, fontStyle };
        const obj = fontStore.getFont(opts);
        return obj?.data;
    });
    // Don't pass main background color to textkit. Will be rendered by the render package instead
    const backgroundColor = level === 0 ? null : instance.style.backgroundColor;
    const attributes = {
        font,
        color,
        opacity,
        fontSize,
        lineHeight,
        direction,
        verticalAlign,
        backgroundColor,
        indent: textIndent,
        characterSpacing: letterSpacing,
        strikeStyle: textDecorationStyle,
        underlineStyle: textDecorationStyle,
        underline: textDecoration === 'underline' ||
            textDecoration === 'underline line-through' ||
            textDecoration === 'line-through underline',
        strike: textDecoration === 'line-through' ||
            textDecoration === 'underline line-through' ||
            textDecoration === 'line-through underline',
        strikeColor: textDecorationColor || color,
        underlineColor: textDecorationColor || color,
        // @ts-expect-error allow this props access
        link: parentLink || instance.props?.src || instance.props?.href,
        align: textAlign || (direction === 'rtl' ? 'right' : 'left'),
    };
    for (let i = 0; i < instance.children.length; i += 1) {
        const child = instance.children[i];
        if (isImage$1(child)) {
            fragments.push({
                string: String.fromCharCode(0xfffc),
                attributes: {
                    ...attributes,
                    attachment: {
                        width: (child.style.width || fontSize),
                        height: (child.style.height || fontSize),
                        image: child.image.data,
                    },
                },
            });
        }
        else if (isTextInstance$2(child)) {
            fragments.push({
                string: transformText(child.value, textTransform),
                attributes,
            });
        }
        else if (child) {
            fragments.push(...getFragments(fontStore, child, attributes.link, level + 1));
        }
    }
    for (let i = 0; i < PREPROCESSORS.length; i += 1) {
        const preprocessor = PREPROCESSORS[i];
        fragments = preprocessor(fragments);
    }
    return fragments;
};
/**
 * Get textkit attributed string from text node
 *
 * @param fontStore - Font store
 * @param instance Node
 * @returns Attributed string
 */
const getAttributedString = (fontStore, instance) => {
    const fragments = getFragments(fontStore, instance);
    return fromFragments(fragments);
};

const engines = {
    bidi,
    linebreaker,
    justification,
    textDecoration,
    scriptItemizer,
    wordHyphenation,
    fontSubstitution,
};
const engine = layoutEngine(engines);
const getMaxLines = (node) => node.style?.maxLines;
const getTextOverflow = (node) => node.style?.textOverflow;
/**
 * Get layout container for specific text node
 *
 * @param {number} width
 * @param {number} height
 * @param {Object} node
 * @returns {Object} layout container
 */
const getContainer = (width, height, node) => {
    const maxLines = getMaxLines(node);
    const textOverflow = getTextOverflow(node);
    return {
        x: 0,
        y: 0,
        width,
        maxLines,
        height: height || Infinity,
        truncateMode: textOverflow,
    };
};
/**
 * Get text layout options for specific text node
 *
 * @param {Object} node instance
 * @returns {Object} layout options
 */
const getLayoutOptions = (fontStore, node) => ({
    hyphenationPenalty: node.props.hyphenationPenalty,
    shrinkWhitespaceFactor: { before: -0.5, after: -0.5 },
    hyphenationCallback: node.props.hyphenationCallback ||
        fontStore?.getHyphenationCallback() ||
        null,
});
/**
 * Get text lines for given node
 *
 * @param node - Node
 * @param width - Container width
 * @param height - Container height
 * @param fontStore - Font store
 * @returns Layout lines
 */
const layoutText = (node, width, height, fontStore) => {
    const attributedString = getAttributedString(fontStore, node);
    const container = getContainer(width, height, node);
    const options = getLayoutOptions(fontStore, node);
    const lines = engine(attributedString, container, options);
    return lines.reduce((acc, line) => [...acc, ...line], []);
};

const isSvg$2 = (node) => node.type === P.Svg;
const isText$4 = (node) => node.type === P.Text;
const shouldIterate = (node) => !isSvg$2(node) && !isText$4(node);
const shouldLayoutText = (node) => isText$4(node) && !node.lines;
/**
 * Performs text layout on text node if wasn't calculated before.
 * Text layout is usually performed on Yoga's layout process (via setMeasureFunc),
 * but we need to layout those nodes with fixed width and height.
 *
 * @param node
 * @returns Layout node
 */
const resolveTextLayout = (node, fontStore) => {
    if (shouldLayoutText(node)) {
        const width = node.box.width - (node.box.paddingRight + node.box.paddingLeft);
        const height = node.box.height - (node.box.paddingTop + node.box.paddingBottom);
        node.lines = layoutText(node, width, height, fontStore);
    }
    if (shouldIterate(node)) {
        if (!node.children)
            return node;
        const mapChild = (child) => resolveTextLayout(child, fontStore);
        const children = node.children.map(mapChild);
        return Object.assign({}, node, { children });
    }
    return node;
};

const BASE_INHERITABLE_PROPERTIES = [
    'color',
    'fontFamily',
    'fontSize',
    'fontStyle',
    'fontWeight',
    'letterSpacing',
    'opacity',
    'textDecoration',
    'textTransform',
    'lineHeight',
    'textAlign',
    'visibility',
    'wordSpacing',
];
const TEXT_INHERITABLE_PROPERTIES = [
    ...BASE_INHERITABLE_PROPERTIES,
    'backgroundColor',
];
const isType$2 = (type) => (node) => node.type === type;
const isSvg$1 = isType$2(P.Svg);
const isText$3 = isType$2(P.Text);
// Merge style values
const mergeValues = (styleName, value, inheritedValue) => {
    switch (styleName) {
        case 'textDecoration': {
            // merge not none and not false textDecoration values to one rule
            return [inheritedValue, value].filter((v) => v && v !== 'none').join(' ');
        }
        default:
            return value;
    }
};
// Merge inherited and node styles
const merge = (inheritedStyles, style) => {
    const mergedStyles = { ...inheritedStyles };
    Object.entries(style).forEach(([styleName, value]) => {
        mergedStyles[styleName] = mergeValues(styleName, value, inheritedStyles[styleName]);
    });
    return mergedStyles;
};
/**
 * Merges styles with node
 *
 * @param inheritedStyles - Style object
 * @returns Merge styles function
 */
const mergeStyles = (inheritedStyles) => (node) => {
    const style = merge(inheritedStyles, node.style || {});
    return Object.assign({}, node, { style });
};
/**
 * Inherit style values from the root to the leafs
 *
 * @param node - Document root
 * @returns Document root with inheritance
 *
 */
const resolveInheritance = (node) => {
    if (isSvg$1(node))
        return node;
    if (!('children' in node))
        return node;
    const inheritableProperties = isText$3(node)
        ? TEXT_INHERITABLE_PROPERTIES
        : BASE_INHERITABLE_PROPERTIES;
    const inheritStyles = pick(inheritableProperties, node.style || {});
    const resolveChild = compose(resolveInheritance, mergeStyles(inheritStyles));
    const children = node.children.map(resolveChild);
    return Object.assign({}, node, { children });
};

const getComputedMargin = (node, edge) => {
    const { yogaNode } = node;
    return yogaNode ? yogaNode.getComputedMargin(edge) : null;
};
/**
 * Get Yoga computed magins. Zero otherwise
 *
 * @param node
 * @returns Margins
 */
const getMargin = (node) => {
    const { style, box } = node;
    const marginTop = getComputedMargin(node, Yoga.Edge.Top) ||
        box?.marginTop ||
        style?.marginTop ||
        0;
    const marginRight = getComputedMargin(node, Yoga.Edge.Right) ||
        box?.marginRight ||
        style?.marginRight ||
        0;
    const marginBottom = getComputedMargin(node, Yoga.Edge.Bottom) ||
        box?.marginBottom ||
        style?.marginBottom ||
        0;
    const marginLeft = getComputedMargin(node, Yoga.Edge.Left) ||
        box?.marginLeft ||
        style?.marginLeft ||
        0;
    return { marginTop, marginRight, marginBottom, marginLeft };
};

/**
 * Get Yoga computed position. Zero otherwise
 *
 * @param node
 * @returns Position
 */
const getPosition = (node) => {
    const { yogaNode } = node;
    return {
        top: yogaNode?.getComputedTop() || 0,
        right: yogaNode?.getComputedRight() || 0,
        bottom: yogaNode?.getComputedBottom() || 0,
        left: yogaNode?.getComputedLeft() || 0,
    };
};

const DEFAULT_DIMENSION = {
    width: 0,
    height: 0,
};
/**
 * Get Yoga computed dimensions. Zero otherwise
 *
 * @param node
 * @returns Dimensions
 */
const getDimension = (node) => {
    const { yogaNode } = node;
    if (!yogaNode)
        return DEFAULT_DIMENSION;
    return {
        width: yogaNode.getComputedWidth(),
        height: yogaNode.getComputedHeight(),
    };
};

const getComputedBorder = (yogaNode, edge) => (yogaNode ? yogaNode.getComputedBorder(edge) : 0);
/**
 * Get Yoga computed border width. Zero otherwise
 *
 * @param node
 * @returns Border widths
 */
const getBorderWidth = (node) => {
    const { yogaNode } = node;
    return {
        borderTopWidth: getComputedBorder(yogaNode, Yoga.Edge.Top),
        borderRightWidth: getComputedBorder(yogaNode, Yoga.Edge.Right),
        borderBottomWidth: getComputedBorder(yogaNode, Yoga.Edge.Bottom),
        borderLeftWidth: getComputedBorder(yogaNode, Yoga.Edge.Left),
    };
};

/**
 * Set display attribute to node's Yoga instance
 *
 * @param value - Display
 * @returns Node instance wrapper
 */
const setDisplay = (value) => (node) => {
    const { yogaNode } = node;
    if (yogaNode) {
        yogaNode.setDisplay(value === 'none' ? Yoga.Display.None : Yoga.Display.Flex);
    }
    return node;
};

const OVERFLOW = {
    hidden: Yoga.Overflow.Hidden,
    scroll: Yoga.Overflow.Scroll,
};
/**
 * Set overflow attribute to node's Yoga instance
 *
 * @param value - Overflow value
 * @returns Node instance wrapper
 */
const setOverflow = (value) => (node) => {
    const { yogaNode } = node;
    if (!isNil(value) && yogaNode) {
        const overflow = OVERFLOW[value] || Yoga.Overflow.Visible;
        yogaNode.setOverflow(overflow);
    }
    return node;
};

const FLEX_WRAP = {
    wrap: Yoga.Wrap.Wrap,
    'wrap-reverse': Yoga.Wrap.WrapReverse,
};
/**
 * Set flex wrap attribute to node's Yoga instance
 *
 * @param value - Flex wrap value
 * @returns Node instance wrapper
 */
const setFlexWrap = (value) => (node) => {
    const { yogaNode } = node;
    if (yogaNode) {
        const flexWrap = FLEX_WRAP[value] || Yoga.Wrap.NoWrap;
        yogaNode.setFlexWrap(flexWrap);
    }
    return node;
};

/**
 * Set generic yoga attribute to node's Yoga instance, handing `auto`, edges and percentage cases
 *
 * @param attr - Property
 * @param edge - Edge
 * @returns Node instance wrapper
 */
const setYogaValue = (attr, edge) => (value) => (node) => {
    const { yogaNode } = node;
    if (!isNil(value) && yogaNode) {
        const hasEdge = !isNil(edge);
        const fixedMethod = `set${upperFirst(attr)}`;
        const autoMethod = `${fixedMethod}Auto`;
        const percentMethod = `${fixedMethod}Percent`;
        const percent = matchPercent(value);
        if (percent && !yogaNode[percentMethod]) {
            throw new Error(`You can't pass percentage values to ${attr} property`);
        }
        if (percent) {
            if (hasEdge) {
                yogaNode[percentMethod]?.(edge, percent.value);
            }
            else {
                yogaNode[percentMethod]?.(percent.value);
            }
        }
        else if (value === 'auto') {
            if (hasEdge) {
                yogaNode[autoMethod]?.(edge);
            }
            else {
                yogaNode[autoMethod]?.();
            }
        }
        else if (hasEdge) {
            yogaNode[fixedMethod]?.(edge, value);
        }
        else {
            yogaNode[fixedMethod]?.(value);
        }
    }
    return node;
};

/**
 * Set flex grow attribute to node's Yoga instance
 *
 * @param  value - Flex grow value
 * @returns Node instance wrapper
 */
const setFlexGrow = (value) => (node) => {
    return setYogaValue('flexGrow')(value || 0)(node);
};

/**
 * Set flex basis attribute to node's Yoga instance
 *
 * @param flex - Basis value
 * @param node - Node instance
 * @returns Node instance
 */
const setFlexBasis = setYogaValue('flexBasis');

const ALIGN = {
    'flex-start': Yoga.Align.FlexStart,
    center: Yoga.Align.Center,
    'flex-end': Yoga.Align.FlexEnd,
    stretch: Yoga.Align.Stretch,
    baseline: Yoga.Align.Baseline,
    'space-between': Yoga.Align.SpaceBetween,
    'space-around': Yoga.Align.SpaceAround,
    'space-evenly': Yoga.Align.SpaceEvenly,
};
/**
 * Set generic align attribute to node's Yoga instance
 *
 * @param attr - Specific align property
 * @param value - Specific align value
 * @param node - Node
 * @returns Node
 */
const setAlign = (attr) => (value) => (node) => {
    const { yogaNode } = node;
    const defaultValue = attr === 'items' ? Yoga.Align.Stretch : Yoga.Align.Auto;
    if (yogaNode) {
        const align = ALIGN[value] || defaultValue;
        yogaNode[`setAlign${upperFirst(attr)}`](align);
    }
    return node;
};

/**
 * Set align self attribute to node's Yoga instance
 *
 * @param align - Value
 * @param node - Node instance
 * @returns Node instance
 */
const setAlignSelf = setAlign('self');

/**
 * Set align items attribute to node's Yoga instance
 *
 * @param align - Value
 * @param node - Node instance
 * @returns Node instance
 */
const setAlignItems = setAlign('items');

/**
 * Set flex shrink attribute to node's Yoga instance
 *
 * @param value - Flex shrink value
 * @returns Node instance wrapper
 */
const setFlexShrink = (value) => (node) => {
    return setYogaValue('flexShrink')(value || 1)(node);
};

/**
 * Set aspect ratio attribute to node's Yoga instance
 *
 * @param value - Ratio
 * @returns Node instance
 */
const setAspectRatio = (value) => (node) => {
    const { yogaNode } = node;
    if (!isNil(value) && yogaNode) {
        yogaNode.setAspectRatio(value);
    }
    return node;
};

/**
 * Set align content attribute to node's Yoga instance
 *
 * @param align - Value
 * @param node - Instance
 * @returns Node instance
 */
const setAlignContent = setAlign('content');

const POSITION = {
    absolute: Yoga.PositionType.Absolute,
    relative: Yoga.PositionType.Relative,
    static: Yoga.PositionType.Static,
};
/**
 * Set position type attribute to node's Yoga instance
 *
 * @param value - Position position type
 * @returns Node instance
 */
const setPositionType = (value) => (node) => {
    const { yogaNode } = node;
    if (!isNil(value) && yogaNode) {
        yogaNode.setPositionType(POSITION[value]);
    }
    return node;
};

const FLEX_DIRECTIONS = {
    row: Yoga.FlexDirection.Row,
    'row-reverse': Yoga.FlexDirection.RowReverse,
    'column-reverse': Yoga.FlexDirection.ColumnReverse,
};
/**
 * Set flex direction attribute to node's Yoga instance
 *
 * @param value - Flex direction value
 * @returns Node instance wrapper
 */
const setFlexDirection = (value) => (node) => {
    const { yogaNode } = node;
    if (yogaNode) {
        const flexDirection = FLEX_DIRECTIONS[value] || Yoga.FlexDirection.Column;
        yogaNode.setFlexDirection(flexDirection);
    }
    return node;
};

const JUSTIFY_CONTENT = {
    center: Yoga.Justify.Center,
    'flex-end': Yoga.Justify.FlexEnd,
    'space-between': Yoga.Justify.SpaceBetween,
    'space-around': Yoga.Justify.SpaceAround,
    'space-evenly': Yoga.Justify.SpaceEvenly,
};
/**
 * Set justify content attribute to node's Yoga instance
 *
 * @param value - Justify content value
 * @returns Node instance wrapper
 */
const setJustifyContent = (value) => (node) => {
    const { yogaNode } = node;
    if (!isNil(value) && yogaNode) {
        const justifyContent = JUSTIFY_CONTENT[value] || Yoga.Justify.FlexStart;
        yogaNode.setJustifyContent(justifyContent);
    }
    return node;
};

/**
 * Set margin top attribute to node's Yoga instance
 *
 * @param margin - Margin top
 * @param node - Node instance
 * @returns Node instance
 */
const setMarginTop = setYogaValue('margin', Yoga.Edge.Top);
/**
 * Set margin right attribute to node's Yoga instance
 *
 * @param margin - Margin right
 * @param node - Node instance
 * @returns Node instance
 */
const setMarginRight = setYogaValue('margin', Yoga.Edge.Right);
/**
 * Set margin bottom attribute to node's Yoga instance
 *
 * @param margin - Margin bottom
 * @param node - Node instance
 * @returns Node instance
 */
const setMarginBottom = setYogaValue('margin', Yoga.Edge.Bottom);
/**
 * Set margin left attribute to node's Yoga instance
 *
 * @param margin - Margin left
 * @param node - Node instance
 * @returns Node instance
 */
const setMarginLeft = setYogaValue('margin', Yoga.Edge.Left);

/**
 * Set padding top attribute to node's Yoga instance
 *
 * @param padding - Padding top
 * @param node - Node instance
 * @returns Node instance
 */
const setPaddingTop = setYogaValue('padding', Yoga.Edge.Top);
/**
 * Set padding right attribute to node's Yoga instance
 *
 * @param padding - Padding right
 * @param node - Node instance
 * @returns Node instance
 */
const setPaddingRight = setYogaValue('padding', Yoga.Edge.Right);
/**
 * Set padding bottom attribute to node's Yoga instance
 *
 * @param padding - Padding bottom
 * @param node Node instance
 * @returns Node instance
 */
const setPaddingBottom = setYogaValue('padding', Yoga.Edge.Bottom);
/**
 * Set padding left attribute to node's Yoga instance
 *
 * @param padding - Padding left
 * @param node - Node instance
 * @returns Node instance
 */
const setPaddingLeft = setYogaValue('padding', Yoga.Edge.Left);

/**
 * Set border top attribute to node's Yoga instance
 *
 * @param border - Border top width
 * @param node - Node instance
 * @returns Node instance
 */
const setBorderTop = setYogaValue('border', Yoga.Edge.Top);
/**
 * Set border right attribute to node's Yoga instance
 *
 * @param border - Border right width
 * @param node - Node instance
 * @returns Node instance
 */
const setBorderRight = setYogaValue('border', Yoga.Edge.Right);
/**
 * Set border bottom attribute to node's Yoga instance
 *
 * @param border - Border bottom width
 * @param node - Node instance
 * @returns Node instance
 */
const setBorderBottom = setYogaValue('border', Yoga.Edge.Bottom);
/**
 * Set border left attribute to node's Yoga instance
 *
 * @param border - Border left width
 * @param node - Node instance
 * @returns Node instance
 */
const setBorderLeft = setYogaValue('border', Yoga.Edge.Left);

/**
 * Set position top attribute to node's Yoga instance
 *
 * @param position - Position top
 * @param node - Node instance
 * @returns Node instance
 */
const setPositionTop = setYogaValue('position', Yoga.Edge.Top);
/**
 * Set position right attribute to node's Yoga instance
 *
 * @param position - Position right
 * @param node - Node instance
 * @returns Node instance
 */
const setPositionRight = setYogaValue('position', Yoga.Edge.Right);
/**
 * Set position bottom attribute to node's Yoga instance
 *
 * @param position - Position bottom
 * @param node - Node instance
 * @returns Node instance
 */
const setPositionBottom = setYogaValue('position', Yoga.Edge.Bottom);
/**
 * Set position left attribute to node's Yoga instance
 *
 * @param position - Position left
 * @param node - Node instance
 * @returns Node instance
 */
const setPositionLeft = setYogaValue('position', Yoga.Edge.Left);

/**
 * Set width to node's Yoga instance
 *
 * @param width - Width
 * @param node - Node instance
 * @returns Node instance
 */
const setWidth = setYogaValue('width');
/**
 * Set min width to node's Yoga instance
 *
 * @param min - Width
 * @param node - Node instance
 * @returns Node instance
 */
const setMinWidth = setYogaValue('minWidth');
/**
 * Set max width to node's Yoga instance
 *
 * @param max - Width
 * @param node - Node instance
 * @returns Node instance
 */
const setMaxWidth = setYogaValue('maxWidth');
/**
 * Set height to node's Yoga instance
 *
 * @param height - Height
 * @param node - Node instance
 * @returns Node instance
 */
const setHeight = setYogaValue('height');
/**
 * Set min height to node's Yoga instance
 *
 * @param min - Height
 * @param node - Node instance
 * @returns Node instance
 */
const setMinHeight = setYogaValue('minHeight');
/**
 * Set max height to node's Yoga instance
 *
 * @param max - Height
 * @param node - Node instance
 * @returns Node instance
 */
const setMaxHeight = setYogaValue('maxHeight');

/**
 * Set rowGap value to node's Yoga instance
 *
 * @param value - Gap value
 * @returns Node instance wrapper
 */
const setRowGap = setYogaValue('gap', Yoga.Gutter.Row);
/**
 * Set columnGap value to node's Yoga instance
 *
 * @param value - Gap value
 * @returns Node instance wrapper
 */
const setColumnGap = setYogaValue('gap', Yoga.Gutter.Column);

const getAspectRatio = (viewbox) => {
    if (!viewbox)
        return null;
    if (typeof viewbox === 'string')
        return null;
    return (viewbox.maxX - viewbox.minX) / (viewbox.maxY - viewbox.minY);
};
/**
 * Yoga svg measure function
 *
 * @param page
 * @param node
 * @returns Measure svg
 */
const measureCanvas$1 = (page, node) => (width, widthMode, height, heightMode) => {
    const aspectRatio = getAspectRatio(node.props.viewBox) || 1;
    if (widthMode === Yoga.MeasureMode.Exactly ||
        widthMode === Yoga.MeasureMode.AtMost) {
        return { width, height: width / aspectRatio };
    }
    if (heightMode === Yoga.MeasureMode.Exactly) {
        return { width: height * aspectRatio };
    }
    return {};
};

/**
 * Get lines width (if any)
 *
 * @param node
 * @returns Lines width
 */
const linesWidth = (node) => {
    if (!node.lines)
        return 0;
    return Math.max(0, ...node.lines.map((line) => line.xAdvance));
};

/**
 * Get lines height (if any)
 *
 * @param node
 * @returns Lines height
 */
const linesHeight = (node) => {
    if (!node.lines)
        return -1;
    return node.lines.reduce((acc, line) => acc + line.box.height, 0);
};

const ALIGNMENT_FACTORS = { center: 0.5, right: 1 };
/**
 * Yoga text measure function
 *
 * @param page
 * @param node
 * @param fontStore
 * @returns {MeasureText} measure text function
 */
const measureText = (page, node, fontStore) => (width, widthMode, height) => {
    if (widthMode === Yoga.MeasureMode.Exactly) {
        if (!node.lines)
            node.lines = layoutText(node, width, height, fontStore);
        return { height: linesHeight(node), width };
    }
    if (widthMode === Yoga.MeasureMode.AtMost) {
        const alignFactor = ALIGNMENT_FACTORS[node.style?.textAlign] || 0;
        if (!node.lines) {
            node.lines = layoutText(node, width, height, fontStore);
            node.alignOffset = (width - linesWidth(node)) * alignFactor; // Compensate align in variable width containers
        }
        return {
            height: linesHeight(node),
            width: Math.min(width, linesWidth(node)),
        };
    }
    return {};
};

/**
 * Get image ratio
 *
 * @param node - Image node
 * @returns Image ratio
 */
const getRatio = (node) => {
    return node.image?.data ? node.image.width / node.image.height : 1;
};

/**
 * Checks if page has auto height
 *
 * @param page
 * @returns Is page height auto
 */
const isHeightAuto = (page) => isNil(page.box?.height);

const SAFETY_HEIGHT$1 = 10;
/**
 * Yoga image measure function
 *
 * @param page - Page
 * @param node - Node
 * @returns Measure image
 */
const measureImage = (page, node) => (width, widthMode, height, heightMode) => {
    const imageRatio = getRatio(node);
    const imageMargin = getMargin(node);
    const pagePadding = getPadding(page);
    // TODO: Check image percentage margins
    const pageArea = isHeightAuto(page)
        ? Infinity
        : (page.box?.height || 0) -
            pagePadding.paddingTop -
            pagePadding.paddingBottom -
            imageMargin.marginTop -
            imageMargin.marginBottom -
            SAFETY_HEIGHT$1;
    // Skip measure if image data not present yet
    if (!node.image)
        return { width: 0, height: 0 };
    if (widthMode === Yoga.MeasureMode.Exactly &&
        heightMode === Yoga.MeasureMode.Undefined) {
        const scaledHeight = width / imageRatio;
        return { height: Math.min(pageArea, scaledHeight) };
    }
    if (heightMode === Yoga.MeasureMode.Exactly &&
        (widthMode === Yoga.MeasureMode.AtMost ||
            widthMode === Yoga.MeasureMode.Undefined)) {
        return { width: Math.min(height * imageRatio, width) };
    }
    if (widthMode === Yoga.MeasureMode.Exactly &&
        heightMode === Yoga.MeasureMode.AtMost) {
        const scaledHeight = width / imageRatio;
        return { height: Math.min(height, pageArea, scaledHeight) };
    }
    if (widthMode === Yoga.MeasureMode.AtMost &&
        heightMode === Yoga.MeasureMode.AtMost) {
        if (imageRatio > 1) {
            return {
                width,
                height: Math.min(width / imageRatio, height),
            };
        }
        return {
            height,
            width: Math.min(height * imageRatio, width),
        };
    }
    return { height, width };
};

const SAFETY_HEIGHT = 10;
const getMax = (values) => Math.max(-Infinity, ...values);
/**
 * Helper object to predict canvas size
 * TODO: Implement remaining functions (as close as possible);
 */
const measureCtx = () => {
    const ctx = {};
    const points = [];
    const nil = () => ctx;
    const addPoint = (x, y) => points.push([x, y]);
    const moveTo = (x, y) => {
        addPoint(x, y);
        return ctx;
    };
    const rect = (x, y, w, h) => {
        addPoint(x, y);
        addPoint(x + w, y);
        addPoint(x, y + h);
        addPoint(x + w, y + h);
        return ctx;
    };
    const ellipse = (x, y, rx, ry) => {
        ry = ry || rx;
        addPoint(x - rx, y - ry);
        addPoint(x + rx, y - ry);
        addPoint(x + rx, y + ry);
        addPoint(x - rx, y + ry);
        return ctx;
    };
    const polygon = (...pts) => {
        points.push(...pts);
        return ctx;
    };
    // Change dimensions
    ctx.rect = rect;
    ctx.moveTo = moveTo;
    ctx.lineTo = moveTo;
    ctx.circle = ellipse;
    ctx.polygon = polygon;
    ctx.ellipse = ellipse;
    ctx.roundedRect = rect;
    // To be implemented
    ctx.text = nil;
    ctx.path = nil;
    ctx.lineWidth = nil;
    ctx.bezierCurveTo = nil;
    ctx.quadraticCurveTo = nil;
    ctx.scale = nil;
    ctx.rotate = nil;
    ctx.translate = nil;
    // These don't change dimensions
    ctx.dash = nil;
    ctx.clip = nil;
    ctx.save = nil;
    ctx.fill = nil;
    ctx.font = nil;
    ctx.stroke = nil;
    ctx.lineCap = nil;
    ctx.opacity = nil;
    ctx.restore = nil;
    ctx.lineJoin = nil;
    ctx.fontSize = nil;
    ctx.fillColor = nil;
    ctx.miterLimit = nil;
    ctx.strokeColor = nil;
    ctx.fillOpacity = nil;
    ctx.strokeOpacity = nil;
    ctx.linearGradient = nil;
    ctx.radialGradient = nil;
    ctx.getWidth = () => getMax(points.map((p) => p[0]));
    ctx.getHeight = () => getMax(points.map((p) => p[1]));
    return ctx;
};
/**
 * @typedef {Function} MeasureCanvas
 * @returns {{ width: number, height: number }} canvas width and height
 */
/**
 * Yoga canvas measure function
 *
 * @param {Object} page
 * @param {Object} node
 * @returns {MeasureCanvas} measure canvas
 */
const measureCanvas = (page, node) => () => {
    const imageMargin = getMargin(node);
    const pagePadding = getPadding(page);
    // TODO: Check image percentage margins
    const pageArea = isHeightAuto(page)
        ? Infinity
        : (page.box?.height || 0) -
            pagePadding.paddingTop -
            pagePadding.paddingBottom -
            imageMargin.marginTop -
            imageMargin.marginBottom -
            SAFETY_HEIGHT;
    const ctx = measureCtx();
    node.props.paint(ctx);
    const width = ctx.getWidth();
    const height = Math.min(pageArea, ctx.getHeight());
    return { width, height };
};

const isType$1 = (type) => (node) => node.type === type;
const isSvg = isType$1(P.Svg);
const isText$2 = isType$1(P.Text);
const isNote = isType$1(P.Note);
const isPage = isType$1(P.Page);
const isImage = isType$1(P.Image);
const isCanvas = isType$1(P.Canvas);
const isTextInstance$1 = isType$1(P.TextInstance);
const setNodeHeight = (node) => {
    const value = isPage(node) ? node.box?.height : node.style?.height;
    return setHeight(value);
};
/**
 * Set styles valeus into yoga node before layout calculation
 *
 * @param node
 */
const setYogaValues = (node) => {
    compose(setNodeHeight(node), setWidth(node.style.width), setMinWidth(node.style.minWidth), setMaxWidth(node.style.maxWidth), setMinHeight(node.style.minHeight), setMaxHeight(node.style.maxHeight), setMarginTop(node.style.marginTop), setMarginRight(node.style.marginRight), setMarginBottom(node.style.marginBottom), setMarginLeft(node.style.marginLeft), setPaddingTop(node.style.paddingTop), setPaddingRight(node.style.paddingRight), setPaddingBottom(node.style.paddingBottom), setPaddingLeft(node.style.paddingLeft), setPositionType(node.style.position), setPositionTop(node.style.top), setPositionRight(node.style.right), setPositionBottom(node.style.bottom), setPositionLeft(node.style.left), setBorderTop(node.style.borderTopWidth), setBorderRight(node.style.borderRightWidth), setBorderBottom(node.style.borderBottomWidth), setBorderLeft(node.style.borderLeftWidth), setDisplay(node.style.display), setFlexDirection(node.style.flexDirection), setAlignSelf(node.style.alignSelf), setAlignContent(node.style.alignContent), setAlignItems(node.style.alignItems), setJustifyContent(node.style.justifyContent), setFlexWrap(node.style.flexWrap), setOverflow(node.style.overflow), setAspectRatio(node.style.aspectRatio), setFlexBasis(node.style.flexBasis), setFlexGrow(node.style.flexGrow), setFlexShrink(node.style.flexShrink), setRowGap(node.style.rowGap), setColumnGap(node.style.columnGap))(node);
};
/**
 * Inserts child into parent' yoga node
 *
 * @param parent parent
 * @returns Insert yoga nodes
 */
const insertYogaNodes = (parent) => (child) => {
    parent.insertChild(child.yogaNode, parent.getChildCount());
    return child;
};
const setMeasureFunc = (node, page, fontStore) => {
    const { yogaNode } = node;
    if (isText$2(node)) {
        yogaNode.setMeasureFunc(measureText(page, node, fontStore));
    }
    if (isImage(node)) {
        yogaNode.setMeasureFunc(measureImage(page, node));
    }
    if (isCanvas(node)) {
        yogaNode.setMeasureFunc(measureCanvas(page, node));
    }
    if (isSvg(node)) {
        yogaNode.setMeasureFunc(measureCanvas$1(page, node));
    }
    return node;
};
const isLayoutElement = (node) => !isText$2(node) && !isNote(node) && !isSvg(node);
/**
 * @typedef {Function} CreateYogaNodes
 * @param {Object} node
 * @returns {Object} node with appended yoga node
 */
/**
 * Creates and add yoga node to document tree
 * Handles measure function for text and image nodes
 *
 * @returns Create yoga nodes
 */
const createYogaNodes = (page, fontStore, yoga) => (node) => {
    const yogaNode = yoga.node.create();
    const result = Object.assign({}, node, { yogaNode });
    setYogaValues(result);
    if (isLayoutElement(node) && node.children) {
        const resolveChild = compose(insertYogaNodes(yogaNode), createYogaNodes(page, fontStore, yoga));
        result.children = node.children.map(resolveChild);
    }
    setMeasureFunc(result, page, fontStore);
    return result;
};
/**
 * Performs yoga calculation
 *
 * @param page - Page node
 * @returns Page node
 */
const calculateLayout = (page) => {
    page.yogaNode.calculateLayout();
    return page;
};
/**
 * Saves Yoga layout result into 'box' attribute of node
 *
 * @param node
 * @returns Node with box data
 */
const persistDimensions = (node) => {
    if (isTextInstance$1(node))
        return node;
    const box = Object.assign(getPadding(node), getMargin(node), getBorderWidth(node), getPosition(node), getDimension(node));
    const newNode = Object.assign({}, node, { box });
    if (!node.children)
        return newNode;
    const children = node.children.map(persistDimensions);
    return Object.assign({}, newNode, { children });
};
/**
 * Removes yoga node from document tree
 *
 * @param node
 * @returns Node without yoga node
 */
const destroyYogaNodes = (node) => {
    const newNode = Object.assign({}, node);
    delete newNode.yogaNode;
    if (!node.children)
        return newNode;
    const children = node.children.map(destroyYogaNodes);
    return Object.assign({}, newNode, { children });
};
/**
 * Free yoga node from document tree
 *
 * @param node
 * @returns Node without yoga node
 */
const freeYogaNodes = (node) => {
    if (node.yogaNode)
        node.yogaNode.freeRecursive();
    return node;
};
/**
 * Calculates page object layout using Yoga.
 * Takes node values from 'box' and 'style' attributes, and persist them back into 'box'
 * Destroy yoga values at the end.
 *
 * @param page - Object
 * @returns Page object with correct 'box' layout attributes
 */
const resolvePageDimensions = (page, fontStore, yoga) => {
    if (isNil(page))
        return null;
    return compose(destroyYogaNodes, freeYogaNodes, persistDimensions, calculateLayout, createYogaNodes(page, fontStore, yoga))(page);
};
/**
 * Calculates root object layout using Yoga.
 *
 * @param node - Root object
 * @param fontStore - Font store
 * @returns Root object with correct 'box' layout attributes
 */
const resolveDimensions = (node, fontStore) => {
    if (!node.children)
        return node;
    const resolveChild = (child) => resolvePageDimensions(child, fontStore, node.yoga);
    const children = node.children.map(resolveChild);
    return Object.assign({}, node, { children });
};

const isText$1 = (node) => node.type === P.Text;
// Prevent splitting elements by low decimal numbers
const SAFETY_THRESHOLD = 0.001;
const assingChildren = (children, node) => Object.assign({}, node, { children });
const getTop = (node) => node.box?.top || 0;
const allFixed = (nodes) => nodes.every(isFixed);
const isDynamic = (node) => node.props && 'render' in node.props;
const relayoutPage = compose(resolveTextLayout, resolvePageDimensions, resolveInheritance, resolvePageStyles);
const warnUnavailableSpace = (node) => {
    console.warn(`Node of type ${node.type} can't wrap between pages and it's bigger than available page height`);
};
const splitNodes = (height, contentArea, nodes) => {
    const currentChildren = [];
    const nextChildren = [];
    for (let i = 0; i < nodes.length; i += 1) {
        const child = nodes[i];
        const futureNodes = nodes.slice(i + 1);
        const futureFixedNodes = futureNodes.filter(isFixed);
        const nodeTop = getTop(child);
        const nodeHeight = child.box.height;
        const isOutside = height <= nodeTop;
        const shouldBreak$1 = shouldBreak(child, futureNodes, height, currentChildren);
        const shouldSplit = height + SAFETY_THRESHOLD < nodeTop + nodeHeight;
        const canWrap = getWrap(child);
        const fitsInsidePage = nodeHeight <= contentArea;
        if (isFixed(child)) {
            nextChildren.push(child);
            currentChildren.push(child);
            continue;
        }
        if (isOutside) {
            const box = Object.assign({}, child.box, { top: child.box.top - height });
            const next = Object.assign({}, child, { box });
            nextChildren.push(next);
            continue;
        }
        if (!fitsInsidePage && !canWrap) {
            currentChildren.push(child);
            nextChildren.push(...futureNodes);
            warnUnavailableSpace(child);
            break;
        }
        if (shouldBreak$1) {
            const box = Object.assign({}, child.box, { top: child.box.top - height });
            const props = Object.assign({}, child.props, {
                wrap: true,
                break: false,
            });
            const next = Object.assign({}, child, { box, props });
            currentChildren.push(...futureFixedNodes);
            nextChildren.push(next, ...futureNodes);
            break;
        }
        if (shouldSplit) {
            const [currentChild, nextChild] = split(child, height, contentArea);
            // All children are moved to the next page, it doesn't make sense to show the parent on the current page
            if (child.children.length > 0 && currentChild.children.length === 0) {
                // But if the current page is empty then we can just include the parent on the current page
                if (currentChildren.length === 0) {
                    currentChildren.push(child, ...futureFixedNodes);
                    nextChildren.push(...futureNodes);
                }
                else {
                    const box = Object.assign({}, child.box, {
                        top: child.box.top - height,
                    });
                    const next = Object.assign({}, child, { box });
                    currentChildren.push(...futureFixedNodes);
                    nextChildren.push(next, ...futureNodes);
                }
                break;
            }
            if (currentChild)
                currentChildren.push(currentChild);
            if (nextChild)
                nextChildren.push(nextChild);
            continue;
        }
        currentChildren.push(child);
    }
    return [currentChildren, nextChildren];
};
const splitChildren = (height, contentArea, node) => {
    const children = node.children || [];
    const availableHeight = height - getTop(node);
    return splitNodes(availableHeight, contentArea, children);
};
const splitView = (node, height, contentArea) => {
    const [currentNode, nextNode] = splitNode(node, height);
    const [currentChilds, nextChildren] = splitChildren(height, contentArea, node);
    return [
        assingChildren(currentChilds, currentNode),
        assingChildren(nextChildren, nextNode),
    ];
};
const split = (node, height, contentArea) => isText$1(node) ? splitText(node, height) : splitView(node, height, contentArea);
const shouldResolveDynamicNodes = (node) => {
    const children = node.children || [];
    return isDynamic(node) || children.some(shouldResolveDynamicNodes);
};
const resolveDynamicNodes = (props, node) => {
    const isNodeDynamic = isDynamic(node);
    // Call render prop on dynamic nodes and append result to children
    const resolveChildren = (children = []) => {
        if (isNodeDynamic) {
            const res = node.props.render(props);
            return (createInstances(res)
                .filter(Boolean)
                // @ts-expect-error rework dynamic nodes. conflicting types
                .map((n) => resolveDynamicNodes(props, n)));
        }
        return children.map((c) => resolveDynamicNodes(props, c));
    };
    // We reset dynamic text box so it can be computed again later on
    const resetHeight = isNodeDynamic && isText$1(node);
    const box = resetHeight ? { ...node.box, height: 0 } : node.box;
    const children = resolveChildren(node.children);
    // @ts-expect-error handle text here specifically
    const lines = isNodeDynamic ? null : node.lines;
    return Object.assign({}, node, { box, lines, children });
};
const resolveDynamicPage = (props, page, fontStore, yoga) => {
    if (shouldResolveDynamicNodes(page)) {
        const resolvedPage = resolveDynamicNodes(props, page);
        return relayoutPage(resolvedPage, fontStore, yoga);
    }
    return page;
};
const splitPage = (page, pageNumber, fontStore, yoga) => {
    const wrapArea = getWrapArea(page);
    const contentArea = getContentArea(page);
    const dynamicPage = resolveDynamicPage({ pageNumber }, page, fontStore, yoga);
    const height = page.style.height;
    const [currentChilds, nextChilds] = splitNodes(wrapArea, contentArea, dynamicPage.children);
    const relayout = (node) => 
    // @ts-expect-error rework pagination
    relayoutPage(node, fontStore, yoga);
    const currentBox = { ...page.box, height };
    const currentPage = relayout(Object.assign({}, page, { box: currentBox, children: currentChilds }));
    if (nextChilds.length === 0 || allFixed(nextChilds))
        return [currentPage, null];
    const nextBox = omit('height', page.box);
    const nextProps = omit('bookmark', page.props);
    const nextPage = relayout(Object.assign({}, page, {
        props: nextProps,
        box: nextBox,
        children: nextChilds,
    }));
    return [currentPage, nextPage];
};
const resolvePageIndices = (fontStore, yoga, page, pageNumber, pages) => {
    const totalPages = pages.length;
    const props = {
        totalPages,
        pageNumber: pageNumber + 1,
        subPageNumber: page.subPageNumber + 1,
        subPageTotalPages: page.subPageTotalPages,
    };
    return resolveDynamicPage(props, page, fontStore, yoga);
};
const assocSubPageData = (subpages) => {
    return subpages.map((page, i) => ({
        ...page,
        subPageNumber: i,
        subPageTotalPages: subpages.length,
    }));
};
const dissocSubPageData = (page) => {
    return omit(['subPageNumber', 'subPageTotalPages'], page);
};
const paginate = (page, pageNumber, fontStore, yoga) => {
    if (!page)
        return [];
    if (page.props?.wrap === false)
        return [page];
    let splittedPage = splitPage(page, pageNumber, fontStore, yoga);
    const pages = [splittedPage[0]];
    let nextPage = splittedPage[1];
    while (nextPage !== null) {
        splittedPage = splitPage(nextPage, pageNumber + pages.length, fontStore, yoga);
        pages.push(splittedPage[0]);
        nextPage = splittedPage[1];
    }
    return pages;
};
/**
 * Performs pagination. This is the step responsible of breaking the whole document
 * into pages following pagiation rules, such as `fixed`, `break` and dynamic nodes.
 *
 * @param root - Document node
 * @param fontStore - Font store
 * @returns Layout node
 */
const resolvePagination = (root, fontStore) => {
    let pages = [];
    let pageNumber = 1;
    for (let i = 0; i < root.children.length; i += 1) {
        const page = root.children[i];
        let subpages = paginate(page, pageNumber, fontStore, root.yoga);
        subpages = assocSubPageData(subpages);
        pageNumber += subpages.length;
        pages = pages.concat(subpages);
    }
    pages = pages.map((...args) => dissocSubPageData(resolvePageIndices(fontStore, root.yoga, ...args)));
    return assingChildren(pages, root);
};

/**
 * Translates page percentage horizontal paddings in fixed ones
 *
 * @param container - Page container
 * @returns Resolve page horizontal padding
 */
const resolvePageHorizontalPadding = (container) => (value) => {
    const match = matchPercent(value);
    const width = container.width;
    return match ? match.percent * width : value;
};
/**
 * Translates page percentage vertical paddings in fixed ones
 *
 * @param container - Page container
 * @returns Resolve page vertical padding
 */
const resolvePageVerticalPadding = (container) => (value) => {
    const match = matchPercent(value);
    const height = container.height;
    return match ? match.percent * height : value;
};
/**
 * Translates page percentage paddings in fixed ones
 *
 * @param page
 * @returns Page with fixed paddings
 */
const resolvePagePaddings = (page) => {
    const container = page.style;
    const style = evolve({
        paddingTop: resolvePageVerticalPadding(container),
        paddingLeft: resolvePageHorizontalPadding(container),
        paddingRight: resolvePageHorizontalPadding(container),
        paddingBottom: resolvePageVerticalPadding(container),
    }, page.style);
    return Object.assign({}, page, { style });
};
/**
 * Translates all pages percentage paddings in fixed ones
 * This has to be computed from pages calculated size and not by Yoga
 * because at this point we didn't performed pagination yet.
 *
 * @param root - Document root
 * @returns Document root with translated page paddings
 */
const resolvePagesPaddings = (root) => {
    if (!root.children)
        return root;
    const children = root.children.map(resolvePagePaddings);
    return Object.assign({}, root, { children });
};

const resolveRadius = (box) => (value) => {
    if (!value)
        return undefined;
    const match = matchPercent(value);
    return match ? match.percent * Math.min(box.width, box.height) : value;
};
/**
 * Transforms percent border radius into fixed values
 *
 * @param node
 * @returns Node
 */
const resolvePercentRadius = (node) => {
    const style = evolve({
        borderTopLeftRadius: resolveRadius(node.box),
        borderTopRightRadius: resolveRadius(node.box),
        borderBottomRightRadius: resolveRadius(node.box),
        borderBottomLeftRadius: resolveRadius(node.box),
    }, node.style || {});
    const newNode = Object.assign({}, node, { style });
    if (!node.children)
        return newNode;
    const children = node.children.map(resolvePercentRadius);
    return Object.assign({}, newNode, { children });
};

/**
 * Transform percent height into fixed
 *
 * @param height
 * @returns Height
 */
const transformHeight = (pageArea, height) => {
    const match = matchPercent(height);
    return match ? match.percent * pageArea : height;
};
/**
 * Get page area (height minus paddings)
 *
 * @param page
 * @returns Page area
 */
const getPageArea = (page) => {
    const pageHeight = page.style.height;
    const pagePaddingTop = (page.style?.paddingTop || 0);
    const pagePaddingBottom = (page.style?.paddingBottom || 0);
    return pageHeight - pagePaddingTop - pagePaddingBottom;
};
/**
 * Transform node percent height to fixed
 *
 * @param page
 * @param node
 * @returns Transformed node
 */
const resolveNodePercentHeight = (page, node) => {
    if (isNil(page.style?.height))
        return node;
    if (isNil(node.style?.height))
        return node;
    const pageArea = getPageArea(page);
    const height = transformHeight(pageArea, node.style.height);
    const style = Object.assign({}, node.style, { height });
    return Object.assign({}, node, { style });
};
/**
 * Transform page immediate children with percent height to fixed
 *
 * @param page
 * @returns Transformed page
 */
const resolvePagePercentHeight = (page) => {
    if (!page.children)
        return page;
    const resolveChild = (child) => resolveNodePercentHeight(page, child);
    const children = page.children.map(resolveChild);
    return Object.assign({}, page, { children });
};
/**
 * Transform all page immediate children with percent height to fixed.
 * This is needed for computing correct dimensions on pre-pagination layout.
 *
 * @param root - Document root
 * @returns Transformed document root
 */
const resolvePercentHeight = (root) => {
    if (!root.children)
        return root;
    const children = root.children.map(resolvePagePercentHeight);
    return Object.assign({}, root, { children });
};

const isType = (type) => (node) => node.type === type;
const isLink = isType(P.Link);
const isText = isType(P.Text);
const isTextInstance = isType(P.TextInstance);
/**
 * Checks if node has render prop
 *
 * @param node
 * @returns Has render prop?
 */
const hasRenderProp = (node) => 'render' in node.props;
/**
 * Checks if node is text type (Text or TextInstance)
 *
 * @param node
 * @returns Are all children text instances?
 */
const isTextType = (node) => isText(node) || isTextInstance(node);
/**
 * Checks if is tet link that needs to be wrapped in Text
 *
 * @param node
 * @returns Are all children text instances?
 */
const isTextLink = (node) => {
    const children = node.children || [];
    // Text string inside a Link
    if (children.every(isTextInstance))
        return true;
    // Text node inside a Link
    if (children.every(isText))
        return false;
    return children.every(isTextType);
};
/**
 * Wraps node children inside Text node
 *
 * @param node
 * @returns Node with intermediate Text child
 */
const wrapText = (node) => {
    const textElement = {
        type: P.Text,
        props: {},
        style: {},
        box: {},
        children: node.children,
    };
    return Object.assign({}, node, { children: [textElement] });
};
const transformLink = (node) => {
    if (!isLink(node))
        return node;
    // If has render prop substitute the instance by a Text, that will
    // ultimately render the inline Link via the textkit PDF renderer.
    if (hasRenderProp(node))
        return Object.assign({}, node, { type: P.Text });
    // If is a text link (either contains Text or TextInstance), wrap it
    // inside a Text element so styles are applied correctly
    if (isTextLink(node))
        return wrapText(node);
    return node;
};
/**
 * Transforms Link layout to correctly render text and dynamic rendered links
 *
 * @param node
 * @returns Node with link substitution
 */
const resolveLinkSubstitution = (node) => {
    if (!node.children)
        return node;
    const resolveChild = compose(transformLink, resolveLinkSubstitution);
    const children = node.children.map(resolveChild);
    return Object.assign({}, node, { children });
};

const layout = asyncCompose(resolveZIndex, resolveOrigin, resolveAssets, resolvePagination, resolveTextLayout, resolvePercentRadius, resolveDimensions, resolveSvg, resolveAssets, resolveInheritance, resolvePercentHeight, resolvePagesPaddings, resolveStyles, resolveLinkSubstitution, resolveBookmarks, resolvePageSizes, resolveYoga);

export { layout as default };
