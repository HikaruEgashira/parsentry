import { compose, castArray, parseFloat as parseFloat$1, matchPercent } from '@react-pdf/fns';
import matchMedia from 'media-engine';
import hlsToHex from 'hsl-to-hex';
import colorString from 'color-string';
import parse$1 from 'postcss-value-parser/lib/parse.js';
import parseUnit from 'postcss-value-parser/lib/unit.js';

/**
 * Remove nil values from array
 *
 * @param array - Style array
 * @returns Style array without nils
 */
const compact = (array) => array.filter((item) => item != null);
/**
 * Merges style objects array
 *
 * @param styles - Style array
 * @returns Merged style object
 */
const mergeStyles = (styles) => styles.reduce((acc, style) => {
    const s = Array.isArray(style) ? flatten(style) : style;
    if (!s)
        return acc;
    for (const key of Object.keys(s)) {
        if (s[key] != null) {
            acc[key] = s[key];
        }
    }
    return acc;
}, {});
/**
 * Flattens an array of style objects, into one aggregated style object.
 * Supports nested arrays of styles.
 *
 * @param styles - Style or style array (can be nested)
 * @returns Flattened style object
 */
const flatten = compose(mergeStyles, compact, (castArray));

/**
 * Resolves media queries in styles object
 *
 * @param container - Container for which styles are resolved
 * @param style - Style description
 * @returns Resolved style object
 */
const resolveMediaQueries = (container, style) => {
    return Object.entries(style).reduce((acc, [key, value]) => {
        if (key.startsWith('@media')) {
            return { ...acc, ...matchMedia({ [key]: value }, container) };
        }
        return { ...acc, [key]: value };
    }, {});
};

const isRgb = (value) => /^rgba?\(/i.test(value);
const isHsl = (value) => /^hsla?\(/i.test(value);
/**
 * Transform rgb color to hexa
 *
 * @param value - Styles value
 * @returns Transformed value
 */
const parseRgb = (value) => {
    const rgb = colorString.get.rgb(value);
    if (!rgb)
        return value;
    return colorString.to.hex(rgb);
};
/**
 * Transform Hsl color to hexa
 *
 * @param value - Styles value
 * @returns Transformed value
 */
const parseHsl = (value) => {
    const hsl = colorString.get.hsl(value);
    if (!hsl)
        return value;
    const [h, s, l, a] = hsl;
    const hex = hlsToHex(Math.round(h), Math.round(s), Math.round(l));
    // Append alpha channel if not fully opaque
    if (a !== undefined && a < 1) {
        const alphaHex = Math.round(a * 255)
            .toString(16)
            .toUpperCase()
            .padStart(2, '0');
        return hex.toUpperCase() + alphaHex;
    }
    return hex.toUpperCase();
};
/**
 * Transform given color to hexa
 *
 * @param value - Styles value
 * @returns Transformed value
 */
const transformColor = (value) => {
    const trimmed = value?.trim();
    if (isRgb(trimmed))
        return parseRgb(trimmed);
    if (isHsl(trimmed))
        return parseHsl(trimmed);
    return trimmed;
};

const VALUE_REGEX = /^(-?\d*\.?\d+)(in|mm|cm|pt|vh|vw|px|rem)?$/;
const DEFAULT_DPI = 72;
const DEFAULT_REM_BASE = 18;
const MM_PER_INCH = 25.4;
const CM_PER_INCH = 2.54;
/**
 * Parses scalar value in value and unit pairs
 *
 * @param value - Scalar value
 * @returns Parsed value
 */
const parseValue = (value) => {
    if (typeof value === 'number')
        return { value, unit: undefined };
    const match = VALUE_REGEX.exec(value);
    return match
        ? { value: parseFloat(match[1]), unit: (match[2] || 'pt') }
        : { value, unit: undefined };
};
/**
 * Transform given scalar value to points
 *
 * @param container - Container with dimensions and settings
 * @param value - Styles value
 * @returns Transformed value in points
 */
const transformUnit = (container, value) => {
    const scalar = parseValue(value);
    if (typeof scalar.value !== 'number')
        return scalar.value;
    const inputDpi = container.dpi || DEFAULT_DPI;
    switch (scalar.unit) {
        case 'rem':
            return scalar.value * (container.remBase || DEFAULT_REM_BASE);
        case 'in':
            return scalar.value * DEFAULT_DPI;
        case 'mm':
            return scalar.value * (DEFAULT_DPI / MM_PER_INCH);
        case 'cm':
            return scalar.value * (DEFAULT_DPI / CM_PER_INCH);
        case 'vh':
            return scalar.value * (container.height / 100);
        case 'vw':
            return scalar.value * (container.width / 100);
        case 'px':
            return Math.round(scalar.value * (DEFAULT_DPI / inputDpi));
        default:
            return scalar.value;
    }
};

const processNumberValue = (key, value) => ({
    [key]: parseFloat$1(value),
});
const processUnitValue = (key, value, container) => ({
    [key]: transformUnit(container, value),
});
const processColorValue = (key, value) => {
    const result = { [key]: transformColor(value) };
    return result;
};
const processNoopValue = (key, value) => ({
    [key]: value,
});

// Matches border shorthand: "width style color" (e.g., "1px solid red")
const BORDER_SHORTHAND_REGEX = /^(?<width>-?\d+(?:\.\d+)?(?:in|mm|cm|pt|vw|vh|px|rem)?)\s+(?<style>\S+)\s+(?<color>.+)$/;
const matchBorderShorthand = (value) => BORDER_SHORTHAND_REGEX.exec(value);
const resolveBorderShorthand = (key, value, container) => {
    const match = matchBorderShorthand(`${value}`);
    // Handle shorthand properties that don't match the full border pattern
    // (e.g., borderColor: 'red', borderWidth: 5, borderRadius: 10)
    if (!match) {
        if (key.match(/Color$/)) {
            const color = transformColor(`${value}`);
            return {
                borderTopColor: color,
                borderRightColor: color,
                borderBottomColor: color,
                borderLeftColor: color,
            };
        }
        if (key.match(/Style$/)) {
            const style = value;
            if (typeof style === 'number')
                throw new Error(`Invalid border style: ${style}`);
            return {
                borderTopStyle: style,
                borderRightStyle: style,
                borderBottomStyle: style,
                borderLeftStyle: style,
            };
        }
        if (key.match(/Width$/)) {
            const width = transformUnit(container, value);
            if (typeof width !== 'number')
                throw new Error(`Invalid border width: ${width}`);
            return {
                borderTopWidth: width,
                borderRightWidth: width,
                borderBottomWidth: width,
                borderLeftWidth: width,
            };
        }
        if (key.match(/Radius$/)) {
            const radius = transformUnit(container, value);
            if (typeof radius !== 'number')
                throw new Error(`Invalid border radius: ${radius}`);
            return {
                borderTopLeftRadius: radius,
                borderTopRightRadius: radius,
                borderBottomRightRadius: radius,
                borderBottomLeftRadius: radius,
            };
        }
        return { [key]: value };
    }
    // Full border shorthand matched: "width style color"
    const { width: widthMatch, style: styleMatch, color: colorMatch, } = match.groups;
    const style = styleMatch;
    const color = transformColor(colorMatch);
    const width = transformUnit(container, widthMatch);
    if (key.match(/(Top|Right|Bottom|Left)$/)) {
        return {
            [`${key}Color`]: color,
            [`${key}Style`]: style,
            [`${key}Width`]: width,
        };
    }
    if (typeof width !== 'number')
        throw new Error(`Invalid border width: ${width}`);
    if (typeof style === 'number')
        throw new Error(`Invalid border style: ${style}`);
    return {
        borderTopColor: color,
        borderTopStyle: style,
        borderTopWidth: width,
        borderRightColor: color,
        borderRightStyle: style,
        borderRightWidth: width,
        borderBottomColor: color,
        borderBottomStyle: style,
        borderBottomWidth: width,
        borderLeftColor: color,
        borderLeftStyle: style,
        borderLeftWidth: width,
    };
};
const handlers$b = {
    border: (resolveBorderShorthand),
    borderBottom: (resolveBorderShorthand),
    borderBottomColor: (processColorValue),
    borderBottomLeftRadius: (processUnitValue),
    borderBottomRightRadius: (processUnitValue),
    borderBottomStyle: (processNoopValue),
    borderBottomWidth: (processUnitValue),
    borderColor: (resolveBorderShorthand),
    borderLeft: (resolveBorderShorthand),
    borderLeftColor: (processColorValue),
    borderLeftStyle: (processNoopValue),
    borderLeftWidth: (processUnitValue),
    borderRadius: (resolveBorderShorthand),
    borderRight: (resolveBorderShorthand),
    borderRightColor: (processColorValue),
    borderRightStyle: (processNoopValue),
    borderRightWidth: (processUnitValue),
    borderStyle: (resolveBorderShorthand),
    borderTop: (resolveBorderShorthand),
    borderTopColor: (processColorValue),
    borderTopLeftRadius: (processUnitValue),
    borderTopRightRadius: (processUnitValue),
    borderTopStyle: (processNoopValue),
    borderTopWidth: (processUnitValue),
    borderWidth: (resolveBorderShorthand),
};

const handlers$a = {
    backgroundColor: (processColorValue),
    color: (processColorValue),
    opacity: (processNumberValue),
};

const handlers$9 = {
    height: (processUnitValue),
    maxHeight: (processUnitValue),
    maxWidth: (processUnitValue),
    minHeight: (processUnitValue),
    minWidth: (processUnitValue),
    width: (processUnitValue),
};

// https://developer.mozilla.org/en-US/docs/Web/CSS/flex#values
// TODO: change flex defaults to [0, 1, 'auto'] as in spec in next major release
const flexDefaults = [1, 1, 0];
const flexAuto = [1, 1, 'auto'];
const flexNone = [0, 0, 'auto'];
const flexInitial = [0, 1, 'auto'];
const processFlexShorthand = (key, value, container) => {
    let defaults = flexDefaults;
    let matches = [];
    if (value === 'auto') {
        defaults = flexAuto;
    }
    else if (value === 'none') {
        defaults = flexNone;
    }
    else if (value === 'initial') {
        defaults = flexInitial;
    }
    else {
        matches = `${value}`.split(' ');
    }
    const flexGrow = parseFloat$1(matches[0] || defaults[0]);
    const flexShrink = parseFloat$1(matches[1] || defaults[1]);
    const flexBasis = transformUnit(container, matches[2] || defaults[2]);
    return { flexGrow, flexShrink, flexBasis };
};
const handlers$8 = {
    alignContent: (processNoopValue),
    alignItems: (processNoopValue),
    alignSelf: (processNoopValue),
    flex: (processFlexShorthand),
    flexBasis: (processUnitValue),
    flexDirection: (processNoopValue),
    flexFlow: (processNoopValue),
    flexGrow: (processNumberValue),
    flexShrink: (processNumberValue),
    flexWrap: (processNoopValue),
    justifyContent: (processNoopValue),
    justifySelf: (processNoopValue),
};

const processGapShorthand = (_key, value, container) => {
    const parts = `${value}`.split(' ');
    const rowGap = transformUnit(container, parts[0]);
    const columnGap = transformUnit(container, parts[1] || parts[0]);
    return { rowGap, columnGap };
};
const handlers$7 = {
    gap: (processGapShorthand),
    columnGap: (processUnitValue),
    rowGap: (processUnitValue),
};

const handlers$6 = {
    aspectRatio: (processNumberValue),
    bottom: (processUnitValue),
    display: (processNoopValue),
    left: (processUnitValue),
    position: (processNoopValue),
    right: (processUnitValue),
    top: (processUnitValue),
    overflow: (processNoopValue),
    zIndex: (processNumberValue),
};

const BOX_MODEL_UNITS = new Set([
    'px',
    'in',
    'mm',
    'cm',
    'pt',
    '%',
    'vw',
    'vh',
    'rem',
    '',
]);
const logError = (style, value) => {
    const name = style.toString();
    // eslint-disable-next-line no-console
    console.error(`
    @react-pdf/stylesheet parsing error:
    ${name}: ${value},
    ${' '.repeat(name.length + 2)}^
    Unsupported ${name} value format
  `);
};
/**
 * @param options
 * @param [options.expandsTo]
 * @param [options.maxValues]
 * @param [options.autoSupported]
 */
const expandBoxModel = ({ expandsTo, maxValues = 1, autoSupported = false, } = {}) => (model, value, container) => {
    const nodes = parse$1(`${value}`);
    const parts = [];
    for (let i = 0; i < nodes.length; i++) {
        const node = nodes[i];
        // value contains `calc`, `url` or other css function
        // `,`, `/` or strings that unsupported by margin and padding
        if (node.type === 'function' ||
            node.type === 'string' ||
            node.type === 'div') {
            logError(model, value);
            return {};
        }
        if (node.type === 'word') {
            if (node.value === 'auto' && autoSupported) {
                parts.push(node.value);
            }
            else {
                const result = parseUnit(node.value);
                // when unit isn't specified this condition is true
                if (result && BOX_MODEL_UNITS.has(result.unit)) {
                    parts.push(node.value);
                }
                else {
                    logError(model, value);
                    return {};
                }
            }
        }
    }
    // checks that we have enough parsed values
    if (parts.length > maxValues) {
        logError(model, value);
        return {};
    }
    const first = transformUnit(container, parts[0]);
    if (expandsTo) {
        const second = transformUnit(container, parts[1] || parts[0]);
        const third = transformUnit(container, parts[2] || parts[0]);
        const fourth = transformUnit(container, parts[3] || parts[1] || parts[0]);
        return expandsTo({ first, second, third, fourth });
    }
    return {
        [model]: first,
    };
};

const processMargin = expandBoxModel({
    expandsTo: ({ first, second, third, fourth }) => ({
        marginTop: first,
        marginRight: second,
        marginBottom: third,
        marginLeft: fourth,
    }),
    maxValues: 4,
    autoSupported: true,
});
const processMarginVertical = expandBoxModel({
    expandsTo: ({ first, second }) => ({
        marginTop: first,
        marginBottom: second,
    }),
    maxValues: 2,
    autoSupported: true,
});
const processMarginHorizontal = expandBoxModel({
    expandsTo: ({ first, second }) => ({
        marginRight: first,
        marginLeft: second,
    }),
    maxValues: 2,
    autoSupported: true,
});
const processMarginSingle = expandBoxModel({
    autoSupported: true,
});
const handlers$5 = {
    margin: (processMargin),
    marginBottom: (processMarginSingle),
    marginHorizontal: (processMarginHorizontal),
    marginLeft: (processMarginSingle),
    marginRight: (processMarginSingle),
    marginTop: (processMarginSingle),
    marginVertical: (processMarginVertical),
};

const processPadding = expandBoxModel({
    expandsTo: ({ first, second, third, fourth }) => ({
        paddingTop: first,
        paddingRight: second,
        paddingBottom: third,
        paddingLeft: fourth,
    }),
    maxValues: 4,
});
const processPaddingVertical = expandBoxModel({
    expandsTo: ({ first, second }) => ({
        paddingTop: first,
        paddingBottom: second,
    }),
    maxValues: 2,
});
const processPaddingHorizontal = expandBoxModel({
    expandsTo: ({ first, second }) => ({
        paddingRight: first,
        paddingLeft: second,
    }),
    maxValues: 2,
});
const processPaddingSingle = expandBoxModel();
const handlers$4 = {
    padding: (processPadding),
    paddingBottom: (processPaddingSingle),
    paddingHorizontal: (processPaddingHorizontal),
    paddingLeft: (processPaddingSingle),
    paddingRight: (processPaddingSingle),
    paddingTop: (processPaddingSingle),
    paddingVertical: (processPaddingVertical),
};

const offsetKeyword = (value) => {
    switch (value) {
        case 'top':
        case 'left':
            return '0%';
        case 'right':
        case 'bottom':
            return '100%';
        case 'center':
            return '50%';
        default:
            return value;
    }
};

const isVerticalKeyword = (value) => value === 'top' || value === 'bottom';
const processObjectPosition = (key, value, container) => {
    const match = `${value}`.split(' ');
    let xValue;
    let yValue;
    if (match.length === 1) {
        // Per CSS spec, single vertical keywords (top/bottom) default X to 'center'
        // Single horizontal keywords (left/right/center) default Y to 'center'
        if (isVerticalKeyword(match[0])) {
            xValue = 'center';
            yValue = match[0];
        }
        else {
            xValue = match[0];
            yValue = 'center';
        }
    }
    else {
        xValue = match[0];
        yValue = match[1];
    }
    const objectPositionX = offsetKeyword(transformUnit(container, xValue));
    const objectPositionY = offsetKeyword(transformUnit(container, yValue));
    return { objectPositionX, objectPositionY };
};
const processObjectPositionValue = (key, value, container) => ({
    [key]: offsetKeyword(transformUnit(container, value)),
});
const handlers$3 = {
    objectPosition: (processObjectPosition),
    objectPositionX: (processObjectPositionValue),
    objectPositionY: (processObjectPositionValue),
    objectFit: (processNoopValue),
};

const castInt = (value) => {
    if (typeof value === 'number')
        return value;
    return parseInt(value, 10);
};

const DEFAULT_FONT_SIZE = 18;
const FONT_WEIGHTS = {
    thin: 100,
    hairline: 100,
    ultralight: 200,
    extralight: 200,
    light: 300,
    normal: 400,
    medium: 500,
    semibold: 600,
    demibold: 600,
    bold: 700,
    ultrabold: 800,
    extrabold: 800,
    heavy: 900,
    black: 900,
};
const transformFontWeight = (value) => {
    if (!value)
        return FONT_WEIGHTS.normal;
    if (typeof value === 'number')
        return value;
    const lv = value.toLowerCase();
    if (lv in FONT_WEIGHTS)
        return FONT_WEIGHTS[lv];
    return castInt(value);
};
const processFontWeight = (key, value) => {
    return { [key]: transformFontWeight(value) };
};
const transformLineHeight = (value, styles, container) => {
    if (value === '')
        return value;
    const fontSize = transformUnit(container, styles.fontSize || DEFAULT_FONT_SIZE);
    const lineHeight = transformUnit(container, value);
    // Percent values: use this number multiplied by the element's font size
    const { percent } = matchPercent(lineHeight) || {};
    if (percent)
        return percent * fontSize;
    // Unitless values: use this number multiplied by the element's font size
    return isNaN(Number(value)) ? lineHeight : lineHeight * fontSize;
};
const processLineHeight = (key, value, container, styles) => {
    return {
        [key]: transformLineHeight(value, styles, container),
    };
};
const handlers$2 = {
    direction: (processNoopValue),
    fontFamily: (processNoopValue),
    fontSize: (processUnitValue),
    fontStyle: (processNoopValue),
    fontWeight: (processFontWeight),
    letterSpacing: (processUnitValue),
    lineHeight: (processLineHeight),
    maxLines: (processNumberValue),
    textAlign: (processNoopValue),
    textDecoration: (processNoopValue),
    textDecorationColor: (processColorValue),
    textDecorationStyle: (processNoopValue),
    textIndent: (processNoopValue),
    textOverflow: (processNoopValue),
    textTransform: (processNoopValue),
    verticalAlign: (processNoopValue),
};

const matchNumber = (value) => /^-?(\d+\.?\d*|\d*\.\d+)$/.test(value);
const castFloat = (value) => {
    if (typeof value !== 'string')
        return value;
    if (matchNumber(value))
        return parseFloat(value);
    return value;
};

const parse = (transformString) => {
    const transforms = transformString.trim().split(/\)[ ,]|\)/);
    // Handle "initial", "inherit", "unset".
    if (transforms.length === 1) {
        return [[transforms[0], true]];
    }
    const parsed = [];
    for (let i = 0; i < transforms.length; i += 1) {
        const transform = transforms[i];
        if (transform) {
            const [name, rawValue] = transform.split('(');
            const splitChar = rawValue.indexOf(',') >= 0 ? ',' : ' ';
            const value = rawValue.split(splitChar).map((val) => val.trim());
            parsed.push({ operation: name.trim(), value });
        }
    }
    return parsed;
};
const parseAngle = (value) => {
    const unitsRegexp = /(-?\d*\.?\d*)(\w*)?/i;
    const match = unitsRegexp.exec(value);
    const angle = match?.[1] ?? '';
    const unit = match?.[2] ?? '';
    const number = Number.parseFloat(angle);
    return unit === 'rad' ? (number * 180) / Math.PI : number;
};
const normalizeTransformOperation = ({ operation, value, }) => {
    switch (operation) {
        case 'scale': {
            const [scaleX, scaleY = scaleX] = value.map((num) => Number.parseFloat(num));
            return { operation: 'scale', value: [scaleX, scaleY] };
        }
        case 'scaleX': {
            return { operation: 'scale', value: [Number.parseFloat(value[0]), 1] };
        }
        case 'scaleY': {
            return { operation: 'scale', value: [1, Number.parseFloat(value[0])] };
        }
        case 'rotate': {
            const angle = parseAngle(value[0]);
            const cx = value[1] ? Number.parseFloat(value[1]) : 0;
            const cy = value[2] ? Number.parseFloat(value[2]) : 0;
            return { operation: 'rotate', value: [angle, cx, cy] };
        }
        case 'translate': {
            const [x, y] = value.map((num) => Number.parseFloat(num));
            return { operation: 'translate', value: [x, y] };
        }
        case 'translateX': {
            return {
                operation: 'translate',
                value: [Number.parseFloat(value[0]), 0],
            };
        }
        case 'translateY': {
            return {
                operation: 'translate',
                value: [0, Number.parseFloat(value[0])],
            };
        }
        case 'skew': {
            return {
                operation: 'skew',
                value: value.map(parseAngle),
            };
        }
        case 'skewX': {
            return { operation: 'skew', value: [parseAngle(value[0]), 0] };
        }
        case 'skewY': {
            return { operation: 'skew', value: [0, parseAngle(value[0])] };
        }
        default: {
            // Handle matrix and other potential operations
            return {
                operation,
                value: value.map((num) => Number.parseFloat(num)),
            };
        }
    }
};
const normalize = (operations) => {
    return operations.map((operation) => normalizeTransformOperation(operation));
};
const processTransform = (key, value) => {
    if (typeof value !== 'string')
        return { [key]: value };
    return { [key]: normalize(parse(value)) };
};
const Y_AXIS_SHORTHANDS = { top: true, bottom: true };
const sortTransformOriginPair = (a, b) => {
    if (Y_AXIS_SHORTHANDS[a])
        return 1;
    if (Y_AXIS_SHORTHANDS[b])
        return -1;
    return 0;
};
const getTransformOriginPair = (values) => {
    if (!values || values.length === 0)
        return ['center', 'center'];
    const pair = values.length === 1 ? [values[0], 'center'] : [values[0], values[1]];
    return pair.sort(sortTransformOriginPair);
};
// Transforms shorthand transformOrigin values
const processTransformOriginShorthand = (key, value, container) => {
    const match = `${value}`.split(' ');
    const pair = getTransformOriginPair(match);
    const transformOriginX = transformUnit(container, pair[0]);
    const transformOriginY = transformUnit(container, pair[1]);
    return {
        transformOriginX: offsetKeyword(transformOriginX) || castFloat(transformOriginX),
        transformOriginY: offsetKeyword(transformOriginY) || castFloat(transformOriginY),
    };
};
const processTransformOriginValue = (key, value, container) => {
    const v = transformUnit(container, value);
    return { [key]: offsetKeyword(v) || castFloat(v) };
};
const handlers$1 = {
    transform: processTransform,
    gradientTransform: processTransform,
    transformOrigin: (processTransformOriginShorthand),
    transformOriginX: (processTransformOriginValue),
    transformOriginY: (processTransformOriginValue),
};

const handlers = {
    fill: (processColorValue),
    stroke: (processColorValue),
    strokeDasharray: (processNoopValue),
    strokeWidth: (processUnitValue),
    fillOpacity: (processNumberValue),
    strokeOpacity: (processNumberValue),
    fillRule: (processNoopValue),
    textAnchor: (processNoopValue),
    strokeLinecap: (processNoopValue),
    strokeLinejoin: (processNoopValue),
    visibility: (processNoopValue),
    clipPath: (processNoopValue),
    dominantBaseline: (processNoopValue),
};

const shorthands = {
    ...handlers$b,
    ...handlers$a,
    ...handlers$9,
    ...handlers$8,
    ...handlers$7,
    ...handlers$6,
    ...handlers$5,
    ...handlers$4,
    ...handlers$3,
    ...handlers$2,
    ...handlers$1,
    ...handlers,
};
/**
 * Expand shorthand properties and resolve units/values.
 *
 * @param container - Container dimensions for unit resolution
 * @returns Function that resolves a style object
 */
const resolve = (container) => (style) => {
    const propsArray = Object.keys(style);
    const resolvedStyle = {};
    for (let i = 0; i < propsArray.length; i += 1) {
        const key = propsArray[i];
        const value = style[key];
        if (!shorthands[key]) {
            resolvedStyle[key] = value;
            continue;
        }
        const resolved = shorthands[key](key, value, container, style);
        Object.assign(resolvedStyle, resolved);
    }
    return resolvedStyle;
};

/**
 * Resolves styles
 *
 * @param container
 * @param style - Style
 * @returns Resolved style
 */
const resolveStyles = (container, style) => {
    const computeMediaQueries = (value) => resolveMediaQueries(container, value);
    return compose(resolve(container), computeMediaQueries, flatten)(style);
};

export { resolveStyles as default, flatten, transformColor };
