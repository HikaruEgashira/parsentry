import * as P from '@react-pdf/primitives';
import { isNil, matchPercent } from '@react-pdf/fns';
import absPath from 'abs-svg-path';
import parsePath from 'parse-svg-path';
import normalizePath from 'normalize-svg-path';
import colorString from 'color-string';

const renderPath = (ctx, node) => {
    const d = node.props?.d;
    if (d)
        ctx.path(node.props.d);
};

const KAPPA$3 = 4.0 * ((Math.sqrt(2) - 1.0) / 3.0);
const renderRect = (ctx, node) => {
    const x = node.props?.x || 0;
    const y = node.props?.y || 0;
    const rx = node.props?.rx || 0;
    const ry = node.props?.ry || 0;
    const width = node.props?.width || 0;
    const height = node.props?.height || 0;
    if (!width || !height)
        return;
    if (rx && ry) {
        const krx = rx * KAPPA$3;
        const kry = ry * KAPPA$3;
        ctx.moveTo(x + rx, y);
        ctx.lineTo(x - rx + width, y);
        ctx.bezierCurveTo(x - rx + width + krx, y, x + width, y + ry - kry, x + width, y + ry);
        ctx.lineTo(x + width, y + height - ry);
        ctx.bezierCurveTo(x + width, y + height - ry + kry, x - rx + width + krx, y + height, x - rx + width, y + height);
        ctx.lineTo(x + rx, y + height);
        ctx.bezierCurveTo(x + rx - krx, y + height, x, y + height - ry + kry, x, y + height - ry);
        ctx.lineTo(x, y + ry);
        ctx.bezierCurveTo(x, y + ry - kry, x + rx - krx, y, x + rx, y);
    }
    else {
        ctx.moveTo(x, y);
        ctx.lineTo(x + width, y);
        ctx.lineTo(x + width, y + height);
        ctx.lineTo(x, y + height);
    }
    ctx.closePath();
};

const renderLine$1 = (ctx, node) => {
    const { x1, x2, y1, y2 } = node.props || {};
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
};

const renderGroup = () => {
    // noop
};

const KAPPA$2 = 4.0 * ((Math.sqrt(2) - 1.0) / 3.0);
const drawEllipse = (ctx, rx, ry, cx = 0, cy = 0) => {
    const x = cx - rx;
    const y = cy - ry;
    const ox = rx * KAPPA$2;
    const oy = ry * KAPPA$2;
    const xe = x + rx * 2;
    const ye = y + ry * 2;
    const xm = x + rx;
    const ym = y + ry;
    ctx.moveTo(x, ym);
    ctx.bezierCurveTo(x, ym - oy, xm - ox, y, xm, y);
    ctx.bezierCurveTo(xm + ox, y, xe, ym - oy, xe, ym);
    ctx.bezierCurveTo(xe, ym + oy, xm + ox, ye, xm, ye);
    ctx.bezierCurveTo(xm - ox, ye, x, ym + oy, x, ym);
    ctx.closePath();
};
const renderEllipse = (ctx, node) => {
    const { cx, cy, rx, ry } = node.props || {};
    drawEllipse(ctx, rx, ry, cx, cy);
};

const renderCircle = (ctx, node) => {
    const cx = node.props?.cx;
    const cy = node.props?.cy;
    const r = node.props?.r;
    drawEllipse(ctx, r, r, cx, cy);
};

const encodeGlyphs = (font, glyphs) => {
    // Embedded font path (has font subset)
    if (font.subset) {
        const res = [];
        for (let i = 0; i < glyphs.length; i++) {
            const glyph = glyphs[i];
            const gid = font.subset.includeGlyph(glyph.id);
            res.push(`0000${gid.toString(16)}`.slice(-4));
            if (font.widths[gid] == null) {
                font.widths[gid] = glyph.advanceWidth * font.scale;
            }
            if (font.unicode[gid] == null) {
                font.unicode[gid] = glyph.codePoints;
            }
        }
        return res;
    }
    // Standard font path
    const res = [];
    for (const glyph of glyphs) {
        res.push(`00${glyph.id.toString(16)}`.slice(-2));
    }
    return res;
};

/* eslint-disable no-return-assign */
const number = (n) => {
    if (n > -1e21 && n < 1e21) {
        return Math.round(n * 1e6) / 1e6;
    }
    throw new Error(`unsupported number: ${n}`);
};
const _renderGlyphs = (ctx, encoded, positions, x, y) => {
    const commands = [];
    const scale = ctx._fontSize / 1000;
    let i;
    let last = 0;
    let hadOffset = false;
    ctx.save();
    // flip coordinate system
    ctx.transform(1, 0, 0, -1, 0, ctx.page.height);
    y = ctx.page.height - y;
    // add current font to page if necessary
    if (ctx.page.fonts[ctx._font.id] == null) {
        ctx.page.fonts[ctx._font.id] = ctx._font.ref();
    }
    // begin the text object
    ctx.addContent('BT');
    // text position
    ctx.addContent(`1 0 0 1 ${number(x)} ${number(y)} Tm`);
    // font and font size
    ctx.addContent(`/${ctx._font.id} ${number(ctx._fontSize)} Tf`);
    // Adds a segment of text to the TJ command buffer
    const addSegment = (cur) => {
        if (last < cur) {
            const hex = encoded.slice(last, cur).join('');
            const advance = positions[cur - 1].xAdvance - positions[cur - 1].advanceWidth;
            commands.push(`<${hex}> ${number(-advance)}`);
        }
        return (last = cur);
    };
    // Flushes the current TJ commands to the output stream
    const flush = (s) => {
        addSegment(s);
        if (commands.length > 0) {
            ctx.addContent(`[${commands.join(' ')}] TJ`);
            return (commands.length = 0);
        }
    };
    for (i = 0; i < positions.length; i += 1) {
        // If we have an x or y offset, we have to break out of the current TJ command
        // so we can move the text position.
        const pos = positions[i];
        if (pos.xOffset || pos.yOffset) {
            // Flush the current buffer
            flush(i);
            // Move the text position and flush just the current character
            ctx.addContent(`1 0 0 1 ${number(x + pos.xOffset * scale)} ${number(y + pos.yOffset * scale)} Tm`);
            flush(i + 1);
            hadOffset = true;
        }
        else {
            // If the last character had an offset, reset the text position
            if (hadOffset) {
                ctx.addContent(`1 0 0 1 ${number(x)} ${number(y)} Tm`);
                hadOffset = false;
            }
            // Group segments that don't have any advance adjustments
            if (pos.xAdvance - pos.advanceWidth !== 0) {
                addSegment(i + 1);
            }
        }
        x += pos.xAdvance * scale;
    }
    // Flush any remaining commands
    flush(i);
    // end the text object
    ctx.addContent('ET');
    // restore flipped coordinate system
    return ctx.restore();
};
const renderGlyphs = (ctx, glyphs, positions, x, y) => {
    const scale = 1000 / ctx._fontSize;
    const unitsPerEm = ctx._font.font.unitsPerEm || 1000;
    const advanceWidthScale = 1000 / unitsPerEm;
    // Glyph encoding and positioning
    const encodedGlyphs = encodeGlyphs(ctx._font, glyphs);
    const encodedPositions = positions.map((pos, i) => ({
        xAdvance: pos.xAdvance * scale,
        yAdvance: pos.yAdvance * scale,
        xOffset: pos.xOffset,
        yOffset: pos.yOffset,
        advanceWidth: glyphs[i].advanceWidth * advanceWidthScale,
    }));
    return _renderGlyphs(ctx, encodedGlyphs, encodedPositions, x, y);
};

const renderRun$1 = (ctx, run) => {
    if (!run.glyphs)
        return;
    if (!run.positions)
        return;
    const runAdvanceWidth = run.xAdvance;
    const font = run.attributes.font?.[0];
    const { fontSize, color, opacity } = run.attributes;
    if (color)
        ctx.fillColor(color);
    ctx.fillOpacity(opacity);
    if (font) {
        ctx.font(font.type === 'STANDARD' ? font.fullName : font, fontSize);
    }
    try {
        renderGlyphs(ctx, run.glyphs, run.positions, 0, 0);
    }
    catch (error) {
        console.log(error);
    }
    ctx.translate(runAdvanceWidth, 0);
};
const renderSpan = (ctx, line, textAnchor, dominantBaseline) => {
    ctx.save();
    const x = line.box?.x || 0;
    const y = line.box?.y || 0;
    const font = line.runs[0]?.attributes.font?.[0];
    const scale = line.runs[0]?.attributes?.scale || 1;
    const width = line.xAdvance;
    if (!font)
        return;
    const ascent = font.ascent * scale;
    const xHeight = font.xHeight * scale;
    const descent = font.descent * scale;
    const capHeight = font.capHeight * scale;
    let xTranslate = x;
    let yTranslate = y;
    switch (textAnchor) {
        case 'middle':
            xTranslate = x - width / 2;
            break;
        case 'end':
            xTranslate = x - width;
            break;
        default:
            xTranslate = x;
            break;
    }
    switch (dominantBaseline) {
        case 'middle':
        case 'central':
            yTranslate = y + capHeight / 2;
            break;
        case 'hanging':
            yTranslate = y + capHeight;
            break;
        case 'mathematical':
            yTranslate = y + xHeight;
            break;
        case 'text-after-edge':
            yTranslate = y + descent;
            break;
        case 'text-before-edge':
            yTranslate = y + ascent;
            break;
        default:
            yTranslate = y;
            break;
    }
    ctx.translate(xTranslate, yTranslate);
    line.runs.forEach((run) => renderRun$1(ctx, run));
    ctx.restore();
};
const renderSvgText = (ctx, node) => {
    const children = node.children;
    children.forEach((span) => renderSpan(ctx, span.lines[0], span.props.textAnchor, span.props.dominantBaseline));
};

const pairs = (values) => {
    const result = [];
    for (let i = 0; i < values.length; i += 2) {
        result.push([values[i], values[i + 1]]);
    }
    return result;
};
/**
 * Parse svg-like points into number arrays
 *
 * @param points string ex. "20,30 50,60"
 * @returns points array ex. [[20, 30], [50, 60]]
 */
const parsePoints = (points) => {
    let values = (points || '')
        .trim()
        .replace(/,/g, ' ')
        .replace(/(\d)-(\d)/g, '$1 -$2')
        .split(/\s+/);
    if (values.length % 2 !== 0) {
        values = values.slice(0, -1);
    }
    const mappedValues = values.map(parseFloat);
    return pairs(mappedValues);
};

const drawPolyline = (ctx, points) => {
    if (points.length > 0) {
        ctx.moveTo(points[0][0], points[0][1]);
        points.slice(1).forEach((p) => ctx.lineTo(p[0], p[1]));
    }
};
const renderPolyline = (ctx, node) => {
    const points = parsePoints(node.props.points || '');
    drawPolyline(ctx, points);
};

const renderPolygon = (ctx, node) => {
    const points = parsePoints(node.props.points || '');
    drawPolyline(ctx, points);
    ctx.closePath();
};

const renderImage$1 = (ctx, node) => {
    if (!node.box)
        return;
    if (!node.image?.data)
        return;
    const { x = 0, y = 0 } = node.props;
    const { width, height, opacity } = node.style;
    const paddingTop = node.box.paddingLeft || 0;
    const paddingLeft = node.box.paddingLeft || 0;
    if (width === 0 || height === 0) {
        console.warn(`Image with src '${node.props.href}' skipped due to invalid dimensions`);
        return;
    }
    if (typeof width === 'string' || typeof height === 'string') {
        console.warn(`Image with src '${node.props.href}' skipped due to percentage width or height`);
        return;
    }
    ctx.save();
    ctx
        .fillOpacity(opacity || 1)
        .image(node.image.data, x + paddingLeft, y + paddingTop, {
        width,
        height,
    });
    ctx.restore();
};

// This constant is used to approximate a symmetrical arc using a cubic
// Bezier curve.
const KAPPA$1 = 4.0 * ((Math.sqrt(2) - 1.0) / 3.0);
const clipNode = (ctx, node) => {
    if (!node.box)
        return;
    if (!node.style)
        return;
    const { top, left, width, height } = node.box;
    const { borderTopLeftRadius = 0, borderTopRightRadius = 0, borderBottomRightRadius = 0, borderBottomLeftRadius = 0, } = node.style;
    // Border top
    // @ts-expect-error this is always a number due to resolve border radius step
    const rtr = Math.min(borderTopRightRadius, 0.5 * width, 0.5 * height);
    const ctr = rtr * (1.0 - KAPPA$1);
    ctx.moveTo(left + rtr, top);
    ctx.lineTo(left + width - rtr, top);
    ctx.bezierCurveTo(left + width - ctr, top, left + width, top + ctr, left + width, top + rtr);
    // Border right
    // @ts-expect-error this is always a number due to resolve border radius step
    const rbr = Math.min(borderBottomRightRadius, 0.5 * width, 0.5 * height);
    const cbr = rbr * (1.0 - KAPPA$1);
    ctx.lineTo(left + width, top + height - rbr);
    ctx.bezierCurveTo(left + width, top + height - cbr, left + width - cbr, top + height, left + width - rbr, top + height);
    // Border bottom
    // @ts-expect-error this is always a number due to resolve border radius step
    const rbl = Math.min(borderBottomLeftRadius, 0.5 * width, 0.5 * height);
    const cbl = rbl * (1.0 - KAPPA$1);
    ctx.lineTo(left + rbl, top + height);
    ctx.bezierCurveTo(left + cbl, top + height, left, top + height - cbl, left, top + height - rbl);
    // Border left
    // @ts-expect-error this is always a number due to resolve border radius step
    const rtl = Math.min(borderTopLeftRadius, 0.5 * width, 0.5 * height);
    const ctl = rtl * (1.0 - KAPPA$1);
    ctx.lineTo(left, top + rtl);
    ctx.bezierCurveTo(left, top + ctl, left + ctl, top, left + rtl, top);
    ctx.closePath();
    ctx.clip();
};

const applySingleTransformation = (ctx, transform, origin) => {
    const { operation, value } = transform;
    switch (operation) {
        case 'scale': {
            const [scaleX, scaleY] = value;
            ctx.scale(scaleX, scaleY, { origin });
            break;
        }
        case 'rotate': {
            const [angle, cx, cy] = value;
            ctx.translate(cx, cy);
            ctx.rotate(angle, { origin });
            ctx.translate(-cx, -cy);
            break;
        }
        case 'translate': {
            const [x, y = 0] = value;
            ctx.translate(x, y, { origin });
            break;
        }
        case 'skew': {
            const [xAngle = 0, yAngle = 0] = value;
            const radx = (xAngle * Math.PI) / 180;
            const rady = (yAngle * Math.PI) / 180;
            const tanx = Math.tan(radx);
            const tany = Math.tan(rady);
            let x = 0;
            let y = 0;
            if (origin != null) {
                [x, y] = Array.from(origin);
                const x1 = x + tanx * y;
                const y1 = y + tany * x;
                x -= x1;
                y -= y1;
            }
            ctx.transform(1, tany, tanx, 1, x, y);
            break;
        }
        case 'matrix': {
            ctx.transform(...value);
            break;
        }
        default: {
            console.error(`Transform operation: '${operation}' doesn't supported`);
        }
    }
};
const applyTransformations = (ctx, node) => {
    if (!node.origin)
        return;
    const { props, style } = node;
    const origin = [node.origin.left, node.origin.top];
    const propsTransform = 'transform' in props ? props.transform : undefined;
    const operations = style?.transform || propsTransform || [];
    operations.forEach((operation) => {
        applySingleTransformation(ctx, operation, origin);
    });
};

// From https://github.com/dy/svg-path-bounds/blob/master/index.js
const getPathBoundingBox = (node) => {
    const path = normalizePath(absPath(parsePath(node.props?.d || '')));
    if (!path.length)
        return [0, 0, 0, 0];
    const bounds = [Infinity, Infinity, -Infinity, -Infinity];
    for (let i = 0, l = path.length; i < l; i += 1) {
        const points = path[i].slice(1);
        for (let j = 0; j < points.length; j += 2) {
            if (points[j + 0] < bounds[0])
                bounds[0] = points[j + 0];
            if (points[j + 1] < bounds[1])
                bounds[1] = points[j + 1];
            if (points[j + 0] > bounds[2])
                bounds[2] = points[j + 0];
            if (points[j + 1] > bounds[3])
                bounds[3] = points[j + 1];
        }
    }
    return bounds;
};
const getCircleBoundingBox = (node) => {
    const r = node.props?.r || 0;
    const cx = node.props?.cx || 0;
    const cy = node.props?.cy || 0;
    return [cx - r, cy - r, cx + r, cy + r];
};
const getEllipseBoundingBox = (node) => {
    const cx = node.props?.cx || 0;
    const cy = node.props?.cy || 0;
    const rx = node.props?.rx || 0;
    const ry = node.props?.ry || 0;
    return [cx - rx, cy - ry, cx + rx, cy + ry];
};
const getLineBoundingBox = (node) => {
    const x1 = node.props?.x1 || 0;
    const y1 = node.props?.y1 || 0;
    const x2 = node.props?.x2 || 0;
    const y2 = node.props?.y2 || 0;
    return [
        Math.min(x1, x2),
        Math.min(y1, y2),
        Math.max(x1, x2),
        Math.max(y1, y2),
    ];
};
const getRectBoundingBox = (node) => {
    const x = node.props?.x || 0;
    const y = node.props?.y || 0;
    const width = node.props?.width || 0;
    const height = node.props?.height || 0;
    return [x, y, x + width, y + height];
};
const max = (values) => Math.max(-Infinity, ...values);
const min = (values) => Math.min(Infinity, ...values);
const getPolylineBoundingBox = (node) => {
    const points = parsePoints(node.props?.points);
    const xValues = points.map((p) => p[0]);
    const yValues = points.map((p) => p[1]);
    return [min(xValues), min(yValues), max(xValues), max(yValues)];
};
const boundingBoxFns = {
    [P.Rect]: getRectBoundingBox,
    [P.Line]: getLineBoundingBox,
    [P.Path]: getPathBoundingBox,
    [P.Circle]: getCircleBoundingBox,
    [P.Ellipse]: getEllipseBoundingBox,
    [P.Polygon]: getPolylineBoundingBox,
    [P.Polyline]: getPolylineBoundingBox,
};
const getBoundingBox = (node) => {
    const boundingBoxFn = boundingBoxFns[node.type];
    return boundingBoxFn ? boundingBoxFn(node) : [0, 0, 0, 0];
};

const setStrokeWidth = (ctx, node) => {
    if (!node.props)
        return;
    if (!('strokeWidth' in node.props))
        return;
    const lineWidth = node.props.strokeWidth;
    if (lineWidth)
        ctx.lineWidth(lineWidth);
};
const setStrokeColor = (ctx, node) => {
    if (!node.props)
        return;
    if (!('stroke' in node.props))
        return;
    const strokeColor = node.props.stroke;
    if (strokeColor)
        ctx.strokeColor(strokeColor);
};
const setOpacity = (ctx, node) => {
    if (!node.props)
        return;
    if (!('opacity' in node.props))
        return;
    const opacity = node.props.opacity;
    if (!isNil(opacity))
        ctx.opacity(opacity);
};
const setFillOpacity = (ctx, node) => {
    if (!node.props)
        return;
    if (!('fillOpacity' in node.props))
        return;
    const fillOpacity = node.props.fillOpacity || null;
    if (!isNil(fillOpacity))
        ctx.fillOpacity(fillOpacity);
};
const setStrokeOpacity = (ctx, node) => {
    if (!node.props)
        return;
    if (!('strokeOpacity' in node.props))
        return;
    const strokeOpacity = node.props?.strokeOpacity;
    if (!isNil(strokeOpacity))
        ctx.strokeOpacity(strokeOpacity);
};
const setLineJoin = (ctx, node) => {
    if (!node.props)
        return;
    if (!('strokeLinejoin' in node.props))
        return;
    const lineJoin = node.props.strokeLinejoin;
    if (lineJoin)
        ctx.lineJoin(lineJoin);
};
const setLineCap = (ctx, node) => {
    if (!node.props)
        return;
    if (!('strokeLinecap' in node.props))
        return;
    const lineCap = node.props?.strokeLinecap;
    if (lineCap)
        ctx.lineCap(lineCap);
};
const setLineDash = (ctx, node) => {
    if (!node.props)
        return;
    if (!('strokeDasharray' in node.props))
        return;
    const value = node.props?.strokeDasharray || null;
    // @ts-expect-error check this works as expected
    if (value)
        ctx.dash(value.split(/[\s,]+/).map(Number));
};
const hasLinearGradientFill = (node) => {
    if (!node.props)
        return false;
    if (!('fill' in node.props))
        return false;
    if (typeof node.props.fill === 'string')
        return false;
    return node.props.fill?.type === P.LinearGradient;
};
const hasRadialGradientFill = (node) => {
    if (!node.props)
        return false;
    if (!('fill' in node.props))
        return false;
    if (typeof node.props.fill === 'string')
        return false;
    return node.props.fill?.type === P.RadialGradient;
};
function multiplyMatrices(m1, m2) {
    const a = m1[0] * m2[0] + m1[2] * m2[1];
    const b = m1[1] * m2[0] + m1[3] * m2[1];
    const c = m1[0] * m2[2] + m1[2] * m2[3];
    const d = m1[1] * m2[2] + m1[3] * m2[3];
    const e = m1[0] * m2[4] + m1[2] * m2[5] + m1[4];
    const f = m1[1] * m2[4] + m1[3] * m2[5] + m1[5];
    return [a, b, c, d, e, f];
}
const transformGradient = (grad, transforms, bbox, units) => {
    const matrices = transforms.map((transform) => {
        switch (transform.operation) {
            case 'scale': {
                const value = transform.value;
                return [value[0], 0, 0, value[1], 0, 0];
            }
            case 'translate': {
                const value = transform.value;
                let x = value[0] || 0;
                let y = value[1] || 0;
                if (units === 'objectBoundingBox') {
                    x = (bbox[2] - bbox[0]) * x;
                    y = (bbox[3] - bbox[1]) * y;
                }
                return [1, 0, 0, 1, x, y];
            }
            case 'rotate': {
                const value = transform.value;
                const cos = Math.cos(value[0]);
                const sin = Math.sin(value[0]);
                const cx = value[1] || 0;
                const cy = value[2] || 0;
                return [
                    cos,
                    sin,
                    -sin,
                    cos,
                    cx * (1 - cos) + cy * sin,
                    cy * (1 - cos) - cx * sin,
                ];
            }
            case 'skew': {
                const value = transform.value;
                return [1, Math.tan(value[0]), Math.tan(value[1]), 1, 0, 0];
            }
            case 'matrix': {
                const value = transform.value;
                let x = value[4] || 0;
                let y = value[5] || 0;
                if (units === 'objectBoundingBox') {
                    x = (bbox[2] - bbox[0]) * x;
                    y = (bbox[3] - bbox[1]) * y;
                }
                return [value[0], value[1], value[2], value[3], x, y];
            }
            default:
                return [1, 0, 0, 1, 0, 0];
        }
    });
    const matrix = matrices.reduce(multiplyMatrices, [1, 0, 0, 1, 0, 0]);
    grad.setTransform(...matrix);
};
// Math simplified from https://github.com/devongovett/svgkit/blob/master/src/elements/SVGGradient.js#L104
const setLinearGradientFill = (ctx, node) => {
    if (!node.props)
        return;
    if (!('fill' in node.props))
        return;
    const bbox = getBoundingBox(node);
    const gradient = node.props?.fill;
    if (!gradient)
        return;
    const units = gradient.props.gradientUnits || 'objectBoundingBox';
    const transforms = gradient.props.gradientTransform || [];
    let x1 = gradient.props.x1 || 0;
    let y1 = gradient.props.y1 || 0;
    let x2 = gradient.props.x2 || 1;
    let y2 = gradient.props.y2 || 0;
    if (units === 'objectBoundingBox') {
        const m0 = bbox[2] - bbox[0];
        const m3 = bbox[3] - bbox[1];
        const m4 = bbox[0];
        const m5 = bbox[1];
        x1 = m0 * x1 + m4;
        y1 = m3 * y1 + m5;
        x2 = m0 * x2 + m4;
        y2 = m3 * y2 + m5;
    }
    const grad = ctx.linearGradient(x1, y1, x2, y2);
    transformGradient(grad, transforms, bbox, units);
    gradient.children?.forEach((stop) => {
        grad.stop(stop.props.offset, stop.props.stopColor, stop.props.stopOpacity);
    });
    ctx.fill(grad);
};
// Math simplified from https://github.com/devongovett/svgkit/blob/master/src/elements/SVGGradient.js#L155
const setRadialGradientFill = (ctx, node) => {
    if (!node.props)
        return;
    if (!('fill' in node.props))
        return;
    const bbox = getBoundingBox(node);
    const gradient = node.props?.fill;
    if (!gradient)
        return;
    const units = gradient.props.gradientUnits || 'objectBoundingBox';
    const transforms = gradient.props.gradientTransform || [];
    let r = gradient.props.r || 0.5;
    let cx = gradient.props.cx || 0.5;
    let cy = gradient.props.cy || 0.5;
    let fx = gradient.props.fx || cx;
    let fy = gradient.props.fy || cy;
    if (units === 'objectBoundingBox') {
        const m0 = bbox[2] - bbox[0];
        const m3 = bbox[3] - bbox[1];
        const m4 = bbox[0];
        const m5 = bbox[1];
        r = r * m0;
        cx = m0 * cx + m4;
        cy = m3 * cy + m5;
        fx = m0 * fx + m4;
        fy = m3 * fy + m5;
    }
    const grad = ctx.radialGradient(cx, cy, 0, fx, fy, r);
    transformGradient(grad, transforms, bbox, units);
    gradient.children?.forEach((stop) => {
        grad.stop(stop.props.offset, stop.props.stopColor, stop.props.stopOpacity);
    });
    ctx.fill(grad);
};
const setFillColor = (ctx, node) => {
    if (!node.props)
        return;
    if (!('fill' in node.props))
        return;
    const fillColor = node.props?.fill;
    if (fillColor)
        ctx.fillColor(fillColor);
};
const setFill = (ctx, node) => {
    if (hasLinearGradientFill(node))
        return setLinearGradientFill(ctx, node);
    if (hasRadialGradientFill(node))
        return setRadialGradientFill(ctx, node);
    return setFillColor(ctx, node);
};
const draw = (ctx, node) => {
    const props = node.props || {};
    if ('fill' in props && 'stroke' in props && props.fill && props.stroke) {
        ctx.fillAndStroke(props.fillRule);
    }
    else if ('fill' in props && props.fill) {
        ctx.fill(props.fillRule);
    }
    else if ('stroke' in props && props.stroke) {
        ctx.stroke();
    }
    else {
        ctx.save();
        ctx.opacity(0);
        ctx.fill(null);
        ctx.restore();
    }
};
const noop = () => { };
const renderFns$1 = {
    [P.Tspan]: noop,
    [P.TextInstance]: noop,
    [P.Path]: renderPath,
    [P.Rect]: renderRect,
    [P.Line]: renderLine$1,
    [P.G]: renderGroup,
    [P.Text]: renderSvgText,
    [P.Circle]: renderCircle,
    [P.Image]: renderImage$1,
    [P.Ellipse]: renderEllipse,
    [P.Polygon]: renderPolygon,
    [P.Polyline]: renderPolyline,
};
const renderNode$1 = (ctx, node) => {
    const renderFn = renderFns$1[node.type];
    if (renderFn) {
        renderFn(ctx, node);
    }
    else {
        console.warn(`SVG node of type ${node.type} is not currently supported`);
    }
};
const drawNode = (ctx, node) => {
    setLineCap(ctx, node);
    setLineDash(ctx, node);
    setLineJoin(ctx, node);
    setStrokeWidth(ctx, node);
    setStrokeColor(ctx, node);
    setFill(ctx, node);
    setStrokeOpacity(ctx, node);
    setFillOpacity(ctx, node);
    setOpacity(ctx, node);
    applyTransformations(ctx, node);
    renderNode$1(ctx, node);
    draw(ctx, node);
};
const clipPath = (ctx, node) => {
    if (!node.props)
        return;
    if (!('clipPath' in node.props))
        return;
    const value = node.props.clipPath;
    if (value) {
        const children = value.children || [];
        children.forEach((child) => renderNode$1(ctx, child));
        ctx.clip();
    }
};
const drawChildren = (ctx, node) => {
    const children = node.children || [];
    children.forEach((child) => {
        ctx.save();
        clipPath(ctx, child);
        drawNode(ctx, child);
        drawChildren(ctx, child);
        ctx.restore();
    });
};
const resolveAspectRatio = (ctx, node) => {
    if (!node.box)
        return;
    const { width, height } = node.box;
    const { viewBox, preserveAspectRatio } = node.props;
    const { meetOrSlice = 'meet', align = 'xMidYMid' } = preserveAspectRatio || {};
    if (viewBox == null || width == null || height == null)
        return;
    const x = viewBox?.minX || 0;
    const y = viewBox?.minY || 0;
    const logicalWidth = viewBox?.maxX || width;
    const logicalHeight = viewBox?.maxY || height;
    const logicalRatio = logicalWidth / logicalHeight;
    const physicalRatio = width / height;
    const scaleX = width / logicalWidth;
    const scaleY = height / logicalHeight;
    if (align === 'none') {
        ctx.scale(scaleX, scaleY);
        ctx.translate(-x, -y);
        return;
    }
    if ((logicalRatio < physicalRatio && meetOrSlice === 'meet') ||
        (logicalRatio >= physicalRatio && meetOrSlice === 'slice')) {
        ctx.scale(scaleY, scaleY);
        switch (align) {
            case 'xMinYMin':
            case 'xMinYMid':
            case 'xMinYMax':
                ctx.translate(-x, -y);
                break;
            case 'xMidYMin':
            case 'xMidYMid':
            case 'xMidYMax':
                ctx.translate(-x - (logicalWidth - (width * logicalHeight) / height) / 2, -y);
                break;
            default:
                ctx.translate(-x - (logicalWidth - (width * logicalHeight) / height), -y);
        }
    }
    else {
        ctx.scale(scaleX, scaleX);
        switch (align) {
            case 'xMinYMin':
            case 'xMidYMin':
            case 'xMaxYMin':
                ctx.translate(-x, -y);
                break;
            case 'xMinYMid':
            case 'xMidYMid':
            case 'xMaxYMid':
                ctx.translate(-x, -y - (logicalHeight - (height * logicalWidth) / width) / 2);
                break;
            default:
                ctx.translate(-x, -y - (logicalHeight - (height * logicalWidth) / width));
        }
    }
};
const moveToOrigin = (ctx, node) => {
    if (!node.box)
        return;
    const { top, left } = node.box;
    const paddingLeft = node.box.paddingLeft || 0;
    const paddingTop = node.box.paddingTop || 0;
    ctx.translate(left + paddingLeft, top + paddingTop);
};
const renderSvg = (ctx, node) => {
    ctx.save();
    clipNode(ctx, node);
    moveToOrigin(ctx, node);
    resolveAspectRatio(ctx, node);
    drawChildren(ctx, node);
    ctx.restore();
};

const black = { value: '#000', opacity: 1 };
// TODO: parse to number[] in layout to avoid this step
const parseColor = (hex) => {
    if (!hex)
        return black;
    const parsed = colorString.get(hex);
    if (!parsed)
        return black;
    const value = colorString.to.hex(parsed.value.slice(0, 3));
    const opacity = parsed.value[3];
    return { value, opacity };
};

const DEST_REGEXP = /^#.+/;
const isSrcId$1 = (src) => src.match(DEST_REGEXP);
const renderAttachment = (ctx, attachment) => {
    const { xOffset = 0, yOffset = 0, width, height, image } = attachment;
    ctx.translate(-width + xOffset, -height + yOffset);
    ctx.image(image, 0, 0, {
        fit: [width, height],
        align: 'center',
        valign: 'bottom',
    });
};
const renderAttachments = (ctx, run, glyphs) => {
    if (!glyphs)
        return;
    if (!run.positions)
        return;
    const font = run.attributes.font?.[0];
    if (!font)
        return;
    ctx.save();
    const space = font.glyphForCodePoint(0x20);
    const objectReplacement = font.glyphForCodePoint(0xfffc);
    let attachmentAdvance = 0;
    for (let i = 0; i < glyphs.length; i += 1) {
        const position = run.positions[i];
        const glyph = glyphs[i];
        attachmentAdvance += position.xAdvance || 0;
        if (glyph.id === objectReplacement.id && run.attributes.attachment) {
            ctx.translate(attachmentAdvance, position.yOffset || 0);
            renderAttachment(ctx, run.attributes.attachment);
            glyphs[i] = space;
            attachmentAdvance = 0;
        }
    }
    ctx.restore();
};
const renderRun = (ctx, run) => {
    if (!run.glyphs)
        return;
    if (!run.positions)
        return;
    const font = run.attributes.font?.[0];
    if (!font)
        return;
    const { fontSize, link } = run.attributes;
    const color = parseColor(run.attributes.color);
    const opacity = isNil(run.attributes.opacity)
        ? color.opacity
        : run.attributes.opacity;
    const { height = 0, descent = 0, xAdvance = 0 } = run;
    ctx.fillColor(color.value);
    ctx.fillOpacity(opacity);
    if (link) {
        if (isSrcId$1(link)) {
            ctx.goTo(0, -height - descent, xAdvance, height, link.slice(1));
        }
        else {
            ctx.link(0, -height - descent, xAdvance, height, link);
        }
    }
    // Copy glyphs to avoid mutating the original array
    const glyphs = [...run.glyphs];
    renderAttachments(ctx, run, glyphs);
    ctx.font(font.type === 'STANDARD' ? font.fullName : font, fontSize);
    try {
        renderGlyphs(ctx, glyphs, run.positions, 0, 0);
    }
    catch (error) {
        console.log(error);
    }
    ctx.translate(xAdvance, 0);
};
const renderBackground$1 = (ctx, rect, backgroundColor) => {
    const color = parseColor(backgroundColor);
    ctx.save();
    ctx.fillOpacity(color.opacity);
    ctx.rect(rect.x, rect.y, rect.width, rect.height);
    ctx.fill(color.value);
    ctx.restore();
};
const renderDecorationLine = (ctx, decorationLine) => {
    const color = parseColor(decorationLine.color);
    const opacity = isNil(decorationLine.opacity)
        ? color.opacity
        : decorationLine.opacity;
    ctx.save();
    ctx.lineWidth(decorationLine.rect.height);
    ctx.strokeOpacity(opacity);
    if (/dashed/.test(decorationLine.style)) {
        ctx.dash(3 * decorationLine.rect.height, {});
    }
    else if (/dotted/.test(decorationLine.style)) {
        ctx.dash(decorationLine.rect.height, {});
    }
    if (/wavy/.test(decorationLine.style)) {
        const dist = Math.max(2, decorationLine.rect.height);
        let step = 1.1 * dist;
        const stepCount = Math.floor(decorationLine.rect.width / (2 * step));
        // Adjust step to fill entire width
        const remainingWidth = decorationLine.rect.width - stepCount * 2 * step;
        const adjustment = remainingWidth / stepCount / 2;
        step += adjustment;
        const cp1y = decorationLine.rect.y + dist;
        const cp2y = decorationLine.rect.y - dist;
        let { x } = decorationLine.rect;
        ctx.moveTo(decorationLine.rect.x, decorationLine.rect.y);
        for (let i = 0; i < stepCount; i += 1) {
            ctx.bezierCurveTo(x + step, cp1y, x + step, cp2y, x + 2 * step, decorationLine.rect.y);
            x += 2 * step;
        }
    }
    else {
        ctx.moveTo(decorationLine.rect.x, decorationLine.rect.y);
        ctx.lineTo(decorationLine.rect.x + decorationLine.rect.width, decorationLine.rect.y);
        if (/double/.test(decorationLine.style)) {
            ctx.moveTo(decorationLine.rect.x, decorationLine.rect.y + decorationLine.rect.height * 2);
            ctx.lineTo(decorationLine.rect.x + decorationLine.rect.width, decorationLine.rect.y + decorationLine.rect.height * 2);
        }
    }
    ctx.stroke(color.value);
    ctx.restore();
};
const renderLine = (ctx, line) => {
    if (!line.box)
        return;
    const lineAscent = line.ascent || 0;
    ctx.save();
    ctx.translate(line.box.x, line.box.y + lineAscent);
    for (let i = 0; i < line.runs.length; i += 1) {
        const run = line.runs[i];
        const isLastRun = i === line.runs.length - 1;
        if (run.attributes.backgroundColor) {
            const xAdvance = run.xAdvance ?? 0;
            const overflowRight = isLastRun ? line.overflowRight ?? 0 : 0;
            const backgroundRect = {
                x: 0,
                y: -lineAscent,
                height: line.box.height,
                width: xAdvance - overflowRight,
            };
            renderBackground$1(ctx, backgroundRect, run.attributes.backgroundColor);
        }
        renderRun(ctx, run);
    }
    ctx.restore();
    ctx.save();
    ctx.translate(line.box.x, line.box.y);
    if (line.decorationLines) {
        for (let i = 0; i < line.decorationLines.length; i += 1) {
            const decorationLine = line.decorationLines[i];
            renderDecorationLine(ctx, decorationLine);
        }
    }
    ctx.restore();
};
const renderBlock = (ctx, block) => {
    block.forEach((line) => {
        renderLine(ctx, line);
    });
};
const renderText = (ctx, node) => {
    if (!node.box)
        return;
    if (!node.lines)
        return;
    const { top, left } = node.box;
    const blocks = [node.lines];
    const paddingTop = node.box?.paddingTop || 0;
    const paddingLeft = node.box?.paddingLeft || 0;
    const initialY = node.lines[0] ? node.lines[0].box.y : 0;
    const offsetX = node.alignOffset || 0;
    ctx.save();
    ctx.translate(left + paddingLeft - offsetX, top + paddingTop - initialY);
    blocks.forEach((block) => {
        renderBlock(ctx, block);
    });
    ctx.restore();
};

const renderPage = (ctx, node) => {
    if (!node.box)
        return;
    const { width, height } = node.box;
    const dpi = node.props?.dpi || 72;
    const userUnit = dpi / 72;
    ctx.addPage({ size: [width, height], margin: 0, userUnit });
};

const renderNote = (ctx, node) => {
    if (!node.box)
        return;
    const { top, left } = node.box;
    const value = node?.children?.[0].value || '';
    const color = node.style?.backgroundColor;
    ctx.note(left, top, 0, 0, value, { color });
};

const isNumeric = (n) => {
    return !Number.isNaN(parseFloat(n)) && Number.isFinite(n);
};
const applyContainObjectFit = (cw, ch, iw, ih, px, py) => {
    const cr = cw / ch;
    const ir = iw / ih;
    const pxp = matchPercent(px ?? null);
    const pyp = matchPercent(py ?? null);
    const pxv = pxp ? pxp.percent : 0.5;
    const pyv = pyp ? pyp.percent : 0.5;
    if (cr > ir) {
        const height = ch;
        const width = height * ir;
        const yOffset = isNumeric(py) ? py : 0;
        const xOffset = isNumeric(px) ? px : (cw - width) * pxv;
        return { width, height, xOffset, yOffset };
    }
    const width = cw;
    const height = width / ir;
    const xOffset = isNumeric(px) ? px : 0;
    const yOffset = isNumeric(py) ? py : (ch - height) * pyv;
    return { width, height, yOffset, xOffset };
};
const applyNoneObjectFit = (cw, ch, iw, ih, px, py) => {
    const width = iw;
    const height = ih;
    const pxp = matchPercent(px ?? null);
    const pyp = matchPercent(py ?? null);
    const pxv = pxp ? pxp.percent : 0.5;
    const pyv = pyp ? pyp.percent : 0.5;
    const xOffset = isNumeric(px) ? px : (cw - width) * pxv;
    const yOffset = isNumeric(py) ? py : (ch - height) * pyv;
    return { width, height, xOffset, yOffset };
};
const applyCoverObjectFit = (cw, ch, iw, ih, px, py) => {
    const ir = iw / ih;
    const cr = cw / ch;
    const pxp = matchPercent(px ?? null);
    const pyp = matchPercent(py ?? null);
    const pxv = pxp ? pxp.percent : 0.5;
    const pyv = pyp ? pyp.percent : 0.5;
    if (cr > ir) {
        const width = cw;
        const height = width / ir;
        const xOffset = isNumeric(px) ? px : 0;
        const yOffset = isNumeric(py) ? py : (ch - height) * pyv;
        return { width, height, yOffset, xOffset };
    }
    const height = ch;
    const width = height * ir;
    const xOffset = isNumeric(px) ? px : (cw - width) * pxv;
    const yOffset = isNumeric(py) ? py : 0;
    return { width, height, xOffset, yOffset };
};
const applyScaleDownObjectFit = (cw, ch, iw, ih, px, py) => {
    const containDimension = applyContainObjectFit(cw, ch, iw, ih, px, py);
    const noneDimension = applyNoneObjectFit(cw, ch, iw, ih, px, py);
    return containDimension.width < noneDimension.width
        ? containDimension
        : noneDimension;
};
const applyFillObjectFit = (cw, ch, px, py) => {
    return {
        width: cw,
        height: ch,
        xOffset: matchPercent(px ?? null) ? 0 : px || 0,
        yOffset: matchPercent(py ?? null) ? 0 : py || 0,
    };
};
const resolveObjectFit = (type = 'fill', cw, ch, iw, ih, px, py) => {
    switch (type) {
        case 'contain':
            return applyContainObjectFit(cw, ch, iw, ih, px, py);
        case 'cover':
            return applyCoverObjectFit(cw, ch, iw, ih, px, py);
        case 'none':
            return applyNoneObjectFit(cw, ch, iw, ih, px, py);
        case 'scale-down':
            return applyScaleDownObjectFit(cw, ch, iw, ih, px, py);
        default:
            return applyFillObjectFit(cw, ch, px, py);
    }
};

const drawImage = (ctx, options) => {
    const { x, y, width: containerWidth, height: containerHeight, image, opacity, objectFit, objectPositionX, objectPositionY, imageCache = new Map(), } = options;
    const { width, height, xOffset, yOffset } = resolveObjectFit(objectFit, containerWidth, containerHeight, image.width, image.height, objectPositionX, objectPositionY);
    if (!image.data)
        return;
    if (width === 0 || height === 0)
        return;
    const cacheKey = image.key;
    let embeddedImage = imageCache.get(cacheKey);
    if (!embeddedImage) {
        const src = image.data;
        embeddedImage =
            (typeof src === 'string' && ctx._imageRegistry[src]) ||
                ctx.openImage(src);
        if (!embeddedImage.obj) {
            embeddedImage.embed(ctx);
        }
    }
    if (cacheKey)
        imageCache.set(cacheKey, embeddedImage);
    const imageOpacity = isNil(opacity) ? 1 : opacity;
    ctx
        .fillOpacity(imageOpacity)
        .image(embeddedImage, x + xOffset, y + yOffset, { width, height });
};

const renderImage = (ctx, node, options) => {
    if (!node.box)
        return;
    if (!node.image)
        return;
    const { left, top } = node.box;
    const paddingTop = node.box.paddingTop || 0;
    const paddingRight = node.box.paddingRight || 0;
    const paddingBottom = node.box.paddingBottom || 0;
    const paddingLeft = node.box.paddingLeft || 0;
    ctx.save();
    clipNode(ctx, node);
    drawImage(ctx, {
        x: left + paddingLeft,
        y: top + paddingTop,
        width: node.box.width - paddingLeft - paddingRight,
        height: node.box.height - paddingTop - paddingBottom,
        image: node.image,
        opacity: node.style?.opacity,
        objectFit: node.style?.objectFit,
        objectPositionX: node.style?.objectPositionX,
        objectPositionY: node.style?.objectPositionY,
        imageCache: options.imageCache,
    });
    ctx.restore();
};

const renderImageBackground = (ctx, node, options) => {
    if (!node.box)
        return;
    if (!node.image)
        return;
    const { left, top } = node.box;
    const paddingTop = node.box.paddingTop || 0;
    const paddingRight = node.box.paddingRight || 0;
    const paddingBottom = node.box.paddingBottom || 0;
    const paddingLeft = node.box.paddingLeft || 0;
    ctx.save();
    clipNode(ctx, node);
    drawImage(ctx, {
        x: left + paddingLeft,
        y: top + paddingTop,
        width: node.box.width - paddingLeft - paddingRight,
        height: node.box.height - paddingTop - paddingBottom,
        image: node.image,
        opacity: node.style?.opacity,
        objectFit: node.style?.objectFit,
        objectPositionX: node.style?.objectPositionX,
        objectPositionY: node.style?.objectPositionY,
        imageCache: options.imageCache,
    });
    ctx.restore();
};

const ZERO = { top: 0, bottom: 0, left: 0, right: 0 };
const resolveHitSlop = (hitSlop) => {
    if (!hitSlop)
        return ZERO;
    if (typeof hitSlop === 'number') {
        return { top: hitSlop, bottom: hitSlop, left: hitSlop, right: hitSlop };
    }
    return {
        top: hitSlop.top ?? 0,
        bottom: hitSlop.bottom ?? 0,
        left: hitSlop.left ?? 0,
        right: hitSlop.right ?? 0,
    };
};

const CONTENT_COLOR = '#a1c6e7';
const PADDING_COLOR = '#c4deb9';
const MARGIN_COLOR = '#f8cca1';
const HITSLOP_COLOR = '#d8b4fe';
// TODO: Draw debug boxes using clipping to enhance quality
const debugContent = (ctx, node) => {
    if (!node.box)
        return;
    const { left, top, width, height, paddingLeft = 0, paddingTop = 0, paddingRight = 0, paddingBottom = 0, borderLeftWidth = 0, borderTopWidth = 0, borderRightWidth = 0, borderBottomWidth = 0, } = node.box;
    ctx
        .fillColor(CONTENT_COLOR)
        .opacity(0.5)
        .rect(left + paddingLeft + borderLeftWidth, top + paddingTop + borderTopWidth, width - paddingLeft - paddingRight - borderRightWidth - borderLeftWidth, height - paddingTop - paddingBottom - borderTopWidth - borderBottomWidth)
        .fill();
};
const debugPadding = (ctx, node) => {
    if (!node.box)
        return;
    const { left, top, width, height, paddingLeft = 0, paddingTop = 0, paddingRight = 0, paddingBottom = 0, borderLeftWidth = 0, borderTopWidth = 0, borderRightWidth = 0, borderBottomWidth = 0, } = node.box;
    ctx.fillColor(PADDING_COLOR).opacity(0.5);
    // Padding top
    ctx
        .rect(left + paddingLeft + borderLeftWidth, top + borderTopWidth, width - paddingRight - paddingLeft - borderLeftWidth - borderRightWidth, paddingTop)
        .fill();
    // Padding left
    ctx
        .rect(left + borderLeftWidth, top + borderTopWidth, paddingLeft, height - borderTopWidth - borderBottomWidth)
        .fill();
    // Padding right
    ctx
        .rect(left + width - paddingRight - borderRightWidth, top + borderTopWidth, paddingRight, height - borderTopWidth - borderBottomWidth)
        .fill();
    // Padding bottom
    ctx
        .rect(left + paddingLeft + borderLeftWidth, top + height - paddingBottom - borderBottomWidth, width - paddingRight - paddingLeft - borderLeftWidth - borderRightWidth, paddingBottom)
        .fill();
};
const debugMargin = (ctx, node) => {
    if (!node.box)
        return;
    const { left, top, width, height } = node.box;
    const { marginLeft = 0, marginTop = 0, marginRight = 0, marginBottom = 0, } = node.box;
    ctx.fillColor(MARGIN_COLOR).opacity(0.5);
    // Margin top
    ctx.rect(left, top - marginTop, width, marginTop).fill();
    // Margin left
    ctx
        .rect(left - marginLeft, top - marginTop, marginLeft, height + marginTop + marginBottom)
        .fill();
    // Margin right
    ctx
        .rect(left + width, top - marginTop, marginRight, height + marginTop + marginBottom)
        .fill();
    // Margin bottom
    ctx.rect(left, top + height, width, marginBottom).fill();
};
const debugText = (ctx, node) => {
    if (!node.box)
        return;
    const { left, top, width, height } = node.box;
    const { marginLeft = 0, marginTop = 0, marginRight = 0, marginBottom = 0, } = node.box;
    const roundedWidth = Math.round(width + marginLeft + marginRight);
    const roundedHeight = Math.round(height + marginTop + marginBottom);
    ctx
        .fontSize(6)
        .opacity(1)
        .fillColor('black')
        .text(`${roundedWidth} x ${roundedHeight}`, left - marginLeft, Math.max(top - marginTop - 4, 1), { width: Infinity });
};
const debugOrigin = (ctx, node) => {
    if (node.origin) {
        ctx
            .circle(node.origin.left, node.origin.top, 3)
            .fill('red')
            .circle(node.origin.left, node.origin.top, 5)
            .stroke('red');
    }
};
const debugHitSlop = (ctx, node) => {
    if (!node.box || !node.props)
        return;
    if (!('hitSlop' in node.props))
        return;
    const slop = resolveHitSlop(node.props.hitSlop);
    if (!slop.top && !slop.bottom && !slop.left && !slop.right)
        return;
    const { left, top, width, height } = node.box;
    ctx.fillColor(HITSLOP_COLOR).opacity(0.35);
    // Top
    if (slop.top) {
        ctx
            .rect(left - slop.left, top - slop.top, width + slop.left + slop.right, slop.top)
            .fill();
    }
    // Bottom
    if (slop.bottom) {
        ctx
            .rect(left - slop.left, top + height, width + slop.left + slop.right, slop.bottom)
            .fill();
    }
    // Left
    if (slop.left) {
        ctx.rect(left - slop.left, top, slop.left, height).fill();
    }
    // Right
    if (slop.right) {
        ctx.rect(left + width, top, slop.right, height).fill();
    }
};
const renderDebug = (ctx, node) => {
    if (!node.props)
        return;
    if (!('debug' in node.props) || !node.props.debug)
        return;
    ctx.save();
    debugContent(ctx, node);
    debugPadding(ctx, node);
    debugMargin(ctx, node);
    debugHitSlop(ctx, node);
    debugText(ctx, node);
    debugOrigin(ctx, node);
    ctx.restore();
};

const availableMethods = [
    'dash',
    'clip',
    'save',
    'path',
    'fill',
    'font',
    'text',
    'rect',
    'scale',
    'moveTo',
    'lineTo',
    'stroke',
    'rotate',
    'circle',
    'lineCap',
    'opacity',
    'ellipse',
    'polygon',
    'restore',
    'lineJoin',
    'fontSize',
    'fillColor',
    'lineWidth',
    'translate',
    'miterLimit',
    'strokeColor',
    'fillOpacity',
    'roundedRect',
    'fillAndStroke',
    'strokeOpacity',
    'bezierCurveTo',
    'quadraticCurveTo',
    'linearGradient',
    'radialGradient',
];
const painter = (ctx) => {
    const p = availableMethods.reduce((acc, prop) => ({
        ...acc,
        [prop]: (...args) => {
            // @ts-expect-error ctx[prop] is a function
            ctx[prop](...args);
            return p;
        },
    }), {});
    return p;
};
const renderCanvas = (ctx, node) => {
    if (!node.box)
        return;
    const { top, left, width, height } = node.box;
    const paddingTop = node.box.paddingTop || 0;
    const paddingLeft = node.box.paddingLeft || 0;
    const paddingRight = node.box.paddingRight || 0;
    const paddingBottom = node.box.paddingBottom || 0;
    const availableWidth = width - paddingLeft - paddingRight;
    const availableHeight = height - paddingTop - paddingBottom;
    if (!availableWidth || !availableHeight) {
        console.warn('Canvas element has null width or height. Please provide valid values via the `style` prop in order to correctly render it.');
    }
    ctx.save().translate(left + paddingLeft, top + paddingTop);
    if (node.props.paint) {
        node.props.paint(painter(ctx), availableWidth, availableHeight);
    }
    ctx.restore();
};

// Ref: https://www.w3.org/TR/css-backgrounds-3/#borders
// This constant is used to approximate a symmetrical arc using a cubic Bezier curve.
const KAPPA = 4.0 * ((Math.sqrt(2) - 1.0) / 3.0);
const clipBorderTop = (ctx, layout, style, rtr, rtl) => {
    const { top, left, width, height } = layout;
    const { borderTopWidth, borderRightWidth, borderLeftWidth } = style;
    // Clip outer top border edge
    ctx.moveTo(left + rtl, top);
    ctx.lineTo(left + width - rtr, top);
    // Ellipse coefficients outer top right cap
    const c0 = rtr * (1.0 - KAPPA);
    // Clip outer top right cap
    ctx.bezierCurveTo(left + width - c0, top, left + width, top + c0, left + width, top + rtr);
    // Move down in case the margin exceedes the radius
    const topRightYCoord = top + Math.max(borderTopWidth, rtr);
    ctx.lineTo(left + width, topRightYCoord);
    // Clip inner top right cap
    ctx.lineTo(left + width - borderRightWidth, topRightYCoord);
    // Ellipse coefficients inner top right cap
    const innerTopRightRadiusX = Math.max(rtr - borderRightWidth, 0);
    const innerTopRightRadiusY = Math.max(rtr - borderTopWidth, 0);
    const c1 = innerTopRightRadiusX * (1.0 - KAPPA);
    const c2 = innerTopRightRadiusY * (1.0 - KAPPA);
    // Clip inner top right cap
    ctx.bezierCurveTo(left + width - borderRightWidth, top + borderTopWidth + c2, left + width - borderRightWidth - c1, top + borderTopWidth, left + width - borderRightWidth - innerTopRightRadiusX, top + borderTopWidth);
    // Clip inner top border edge
    ctx.lineTo(left + Math.max(rtl, borderLeftWidth), top + borderTopWidth);
    // Ellipse coefficients inner top left cap
    const innerTopLeftRadiusX = Math.max(rtl - borderLeftWidth, 0);
    const innerTopLeftRadiusY = Math.max(rtl - borderTopWidth, 0);
    const c3 = innerTopLeftRadiusX * (1.0 - KAPPA);
    const c4 = innerTopLeftRadiusY * (1.0 - KAPPA);
    const topLeftYCoord = top + Math.max(borderTopWidth, rtl);
    // Clip inner top left cap
    ctx.bezierCurveTo(left + borderLeftWidth + c3, top + borderTopWidth, left + borderLeftWidth, top + borderTopWidth + c4, left + borderLeftWidth, topLeftYCoord);
    ctx.lineTo(left, topLeftYCoord);
    // Move down in case the margin exceedes the radius
    ctx.lineTo(left, top + rtl);
    // Ellipse coefficients outer top left cap
    const c5 = rtl * (1.0 - KAPPA);
    // Clip outer top left cap
    ctx.bezierCurveTo(left, top + c5, left + c5, top, left + rtl, top);
    ctx.closePath();
    ctx.clip();
    // Clip border top cap joins
    if (borderRightWidth) {
        const trSlope = -borderTopWidth / borderRightWidth;
        ctx.moveTo(left + width / 2, trSlope * (-width / 2) + top);
        ctx.lineTo(left + width, top);
        ctx.lineTo(left, top);
        ctx.lineTo(left, top + height);
        ctx.closePath();
        ctx.clip();
    }
    if (borderLeftWidth) {
        const trSlope = -borderTopWidth / borderLeftWidth;
        ctx.moveTo(left + width / 2, trSlope * (-width / 2) + top);
        ctx.lineTo(left, top);
        ctx.lineTo(left + width, top);
        ctx.lineTo(left + width, top + height);
        ctx.closePath();
        ctx.clip();
    }
};
const fillBorderTop = (ctx, layout, style, rtr, rtl) => {
    const { top, left, width } = layout;
    const { borderTopColor, borderTopWidth, borderTopStyle, borderRightWidth, borderLeftWidth, } = style;
    const c0 = rtl * (1.0 - KAPPA);
    const c1 = rtr * (1.0 - KAPPA);
    ctx.moveTo(left, top + Math.max(rtl, borderTopWidth));
    ctx.bezierCurveTo(left, top + c0, left + c0, top, left + rtl, top);
    ctx.lineTo(left + width - rtr, top);
    ctx.bezierCurveTo(left + width - c1, top, left + width, top + c1, left + width, top + rtr);
    ctx.strokeColor(borderTopColor);
    ctx.lineWidth(Math.max(borderRightWidth, borderTopWidth, borderLeftWidth) * 2);
    if (borderTopStyle === 'dashed') {
        ctx.dash(borderTopWidth * 2, { space: borderTopWidth * 1.2 });
    }
    else if (borderTopStyle === 'dotted') {
        ctx.dash(borderTopWidth, { space: borderTopWidth * 1.2 });
    }
    ctx.stroke();
    ctx.undash();
};
const clipBorderRight = (ctx, layout, style, rtr, rbr) => {
    const { top, left, width, height } = layout;
    const { borderTopWidth, borderRightWidth, borderBottomWidth } = style;
    // Clip outer right border edge
    ctx.moveTo(left + width, top + rtr);
    ctx.lineTo(left + width, top + height - rbr);
    // Ellipse coefficients outer bottom right cap
    const c0 = rbr * (1.0 - KAPPA);
    // Clip outer top right cap
    ctx.bezierCurveTo(left + width, top + height - c0, left + width - c0, top + height, left + width - rbr, top + height);
    // Move left in case the margin exceedes the radius
    const topBottomXCoord = left + width - Math.max(borderRightWidth, rbr);
    ctx.lineTo(topBottomXCoord, top + height);
    // Clip inner bottom right cap
    ctx.lineTo(topBottomXCoord, top + height - borderBottomWidth);
    // Ellipse coefficients inner bottom right cap
    const innerBottomRightRadiusX = Math.max(rbr - borderRightWidth, 0);
    const innerBottomRightRadiusY = Math.max(rbr - borderBottomWidth, 0);
    const c1 = innerBottomRightRadiusX * (1.0 - KAPPA);
    const c2 = innerBottomRightRadiusY * (1.0 - KAPPA);
    // Clip inner top right cap
    ctx.bezierCurveTo(left + width - borderRightWidth - c1, top + height - borderBottomWidth, left + width - borderRightWidth, top + height - borderBottomWidth - c2, left + width - borderRightWidth, top + height - Math.max(rbr, borderBottomWidth));
    // Clip inner right border edge
    ctx.lineTo(left + width - borderRightWidth, top + Math.max(rtr, borderTopWidth));
    // Ellipse coefficients inner top right cap
    const innerTopRightRadiusX = Math.max(rtr - borderRightWidth, 0);
    const innerTopRightRadiusY = Math.max(rtr - borderTopWidth, 0);
    const c3 = innerTopRightRadiusX * (1.0 - KAPPA);
    const c4 = innerTopRightRadiusY * (1.0 - KAPPA);
    const topRightXCoord = left + width - Math.max(rtr, borderRightWidth);
    // Clip inner top left cap
    ctx.bezierCurveTo(left + width - borderRightWidth, top + borderTopWidth + c4, left + width - borderRightWidth - c3, top + borderTopWidth, topRightXCoord, top + borderTopWidth);
    ctx.lineTo(topRightXCoord, top);
    // Move right in case the margin exceedes the radius
    ctx.lineTo(left + width - rtr, top);
    // Ellipse coefficients outer top right cap
    const c5 = rtr * (1.0 - KAPPA);
    // Clip outer top right cap
    ctx.bezierCurveTo(left + width - c5, top, left + width, top + c5, left + width, top + rtr);
    ctx.closePath();
    ctx.clip();
    // Clip border right cap joins
    if (borderTopWidth) {
        const trSlope = -borderTopWidth / borderRightWidth;
        ctx.moveTo(left + width / 2, trSlope * (-width / 2) + top);
        ctx.lineTo(left + width, top);
        ctx.lineTo(left + width, top + height);
        ctx.lineTo(left, top + height);
        ctx.closePath();
        ctx.clip();
    }
    if (borderBottomWidth) {
        const brSlope = borderBottomWidth / borderRightWidth;
        ctx.moveTo(left + width / 2, brSlope * (-width / 2) + top + height);
        ctx.lineTo(left + width, top + height);
        ctx.lineTo(left + width, top);
        ctx.lineTo(left, top);
        ctx.closePath();
        ctx.clip();
    }
};
const fillBorderRight = (ctx, layout, style, rtr, rbr) => {
    const { top, left, width, height } = layout;
    const { borderRightColor, borderRightStyle, borderRightWidth, borderTopWidth, borderBottomWidth, } = style;
    const c0 = rbr * (1.0 - KAPPA);
    const c1 = rtr * (1.0 - KAPPA);
    ctx.moveTo(left + width - rtr, top);
    ctx.bezierCurveTo(left + width - c1, top, left + width, top + c1, left + width, top + rtr);
    ctx.lineTo(left + width, top + height - rbr);
    ctx.bezierCurveTo(left + width, top + height - c0, left + width - c0, top + height, left + width - rbr, top + height);
    ctx.strokeColor(borderRightColor);
    ctx.lineWidth(Math.max(borderRightWidth, borderTopWidth, borderBottomWidth) * 2);
    if (borderRightStyle === 'dashed') {
        ctx.dash(borderRightWidth * 2, { space: borderRightWidth * 1.2 });
    }
    else if (borderRightStyle === 'dotted') {
        ctx.dash(borderRightWidth, { space: borderRightWidth * 1.2 });
    }
    ctx.stroke();
    ctx.undash();
};
const clipBorderBottom = (ctx, layout, style, rbl, rbr) => {
    const { top, left, width, height } = layout;
    const { borderBottomWidth, borderRightWidth, borderLeftWidth } = style;
    // Clip outer top border edge
    ctx.moveTo(left + width - rbr, top + height);
    ctx.lineTo(left + rbl, top + height);
    // Ellipse coefficients outer top right cap
    const c0 = rbl * (1.0 - KAPPA);
    // Clip outer top right cap
    ctx.bezierCurveTo(left + c0, top + height, left, top + height - c0, left, top + height - rbl);
    // Move up in case the margin exceedes the radius
    const bottomLeftYCoord = top + height - Math.max(borderBottomWidth, rbl);
    ctx.lineTo(left, bottomLeftYCoord);
    // Clip inner bottom left cap
    ctx.lineTo(left + borderLeftWidth, bottomLeftYCoord);
    // Ellipse coefficients inner top right cap
    const innerBottomLeftRadiusX = Math.max(rbl - borderLeftWidth, 0);
    const innerBottomLeftRadiusY = Math.max(rbl - borderBottomWidth, 0);
    const c1 = innerBottomLeftRadiusX * (1.0 - KAPPA);
    const c2 = innerBottomLeftRadiusY * (1.0 - KAPPA);
    // Clip inner bottom left cap
    ctx.bezierCurveTo(left + borderLeftWidth, top + height - borderBottomWidth - c2, left + borderLeftWidth + c1, top + height - borderBottomWidth, left + borderLeftWidth + innerBottomLeftRadiusX, top + height - borderBottomWidth);
    // Clip inner bottom border edge
    ctx.lineTo(left + width - Math.max(rbr, borderRightWidth), top + height - borderBottomWidth);
    // Ellipse coefficients inner top left cap
    const innerBottomRightRadiusX = Math.max(rbr - borderRightWidth, 0);
    const innerBottomRightRadiusY = Math.max(rbr - borderBottomWidth, 0);
    const c3 = innerBottomRightRadiusX * (1.0 - KAPPA);
    const c4 = innerBottomRightRadiusY * (1.0 - KAPPA);
    const bottomRightYCoord = top + height - Math.max(borderBottomWidth, rbr);
    // Clip inner top left cap
    ctx.bezierCurveTo(left + width - borderRightWidth - c3, top + height - borderBottomWidth, left + width - borderRightWidth, top + height - borderBottomWidth - c4, left + width - borderRightWidth, bottomRightYCoord);
    ctx.lineTo(left + width, bottomRightYCoord);
    // Move down in case the margin exceedes the radius
    ctx.lineTo(left + width, top + height - rbr);
    // Ellipse coefficients outer top left cap
    const c5 = rbr * (1.0 - KAPPA);
    // Clip outer top left cap
    ctx.bezierCurveTo(left + width, top + height - c5, left + width - c5, top + height, left + width - rbr, top + height);
    ctx.closePath();
    ctx.clip();
    // Clip border bottom cap joins
    if (borderRightWidth) {
        const brSlope = borderBottomWidth / borderRightWidth;
        ctx.moveTo(left + width / 2, brSlope * (-width / 2) + top + height);
        ctx.lineTo(left + width, top + height);
        ctx.lineTo(left, top + height);
        ctx.lineTo(left, top);
        ctx.closePath();
        ctx.clip();
    }
    if (borderLeftWidth) {
        const trSlope = -borderBottomWidth / borderLeftWidth;
        ctx.moveTo(left + width / 2, trSlope * (width / 2) + top + height);
        ctx.lineTo(left, top + height);
        ctx.lineTo(left + width, top + height);
        ctx.lineTo(left + width, top);
        ctx.closePath();
        ctx.clip();
    }
};
const fillBorderBottom = (ctx, layout, style, rbl, rbr) => {
    const { top, left, width, height } = layout;
    const { borderBottomColor, borderBottomStyle, borderBottomWidth, borderRightWidth, borderLeftWidth, } = style;
    const c0 = rbl * (1.0 - KAPPA);
    const c1 = rbr * (1.0 - KAPPA);
    ctx.moveTo(left + width, top + height - rbr);
    ctx.bezierCurveTo(left + width, top + height - c1, left + width - c1, top + height, left + width - rbr, top + height);
    ctx.lineTo(left + rbl, top + height);
    ctx.bezierCurveTo(left + c0, top + height, left, top + height - c0, left, top + height - rbl);
    ctx.strokeColor(borderBottomColor);
    ctx.lineWidth(Math.max(borderBottomWidth, borderRightWidth, borderLeftWidth) * 2);
    if (borderBottomStyle === 'dashed') {
        ctx.dash(borderBottomWidth * 2, { space: borderBottomWidth * 1.2 });
    }
    else if (borderBottomStyle === 'dotted') {
        ctx.dash(borderBottomWidth, { space: borderBottomWidth * 1.2 });
    }
    ctx.stroke();
    ctx.undash();
};
const clipBorderLeft = (ctx, layout, style, rbl, rtl) => {
    const { top, left, width, height } = layout;
    const { borderTopWidth, borderLeftWidth, borderBottomWidth } = style;
    // Clip outer left border edge
    ctx.moveTo(left, top + height - rbl);
    ctx.lineTo(left, top + rtl);
    // Ellipse coefficients outer top left cap
    const c0 = rtl * (1.0 - KAPPA);
    // Clip outer top left cap
    ctx.bezierCurveTo(left, top + c0, left + c0, top, left + rtl, top);
    // Move right in case the margin exceedes the radius
    const topLeftCoordX = left + Math.max(borderLeftWidth, rtl);
    ctx.lineTo(topLeftCoordX, top);
    // Clip inner top left cap
    ctx.lineTo(topLeftCoordX, top + borderTopWidth);
    // Ellipse coefficients inner top left cap
    const innerTopLeftRadiusX = Math.max(rtl - borderLeftWidth, 0);
    const innerTopLeftRadiusY = Math.max(rtl - borderTopWidth, 0);
    const c1 = innerTopLeftRadiusX * (1.0 - KAPPA);
    const c2 = innerTopLeftRadiusY * (1.0 - KAPPA);
    // Clip inner top right cap
    ctx.bezierCurveTo(left + borderLeftWidth + c1, top + borderTopWidth, left + borderLeftWidth, top + borderTopWidth + c2, left + borderLeftWidth, top + Math.max(rtl, borderTopWidth));
    // Clip inner left border edge
    ctx.lineTo(left + borderLeftWidth, top + height - Math.max(rbl, borderBottomWidth));
    // Ellipse coefficients inner bottom left cap
    const innerBottomLeftRadiusX = Math.max(rbl - borderLeftWidth, 0);
    const innerBottomLeftRadiusY = Math.max(rbl - borderBottomWidth, 0);
    const c3 = innerBottomLeftRadiusX * (1.0 - KAPPA);
    const c4 = innerBottomLeftRadiusY * (1.0 - KAPPA);
    const bottomLeftXCoord = left + Math.max(rbl, borderLeftWidth);
    // Clip inner top left cap
    ctx.bezierCurveTo(left + borderLeftWidth, top + height - borderBottomWidth - c4, left + borderLeftWidth + c3, top + height - borderBottomWidth, bottomLeftXCoord, top + height - borderBottomWidth);
    ctx.lineTo(bottomLeftXCoord, top + height);
    // Move left in case the margin exceedes the radius
    ctx.lineTo(left + rbl, top + height);
    // Ellipse coefficients outer top right cap
    const c5 = rbl * (1.0 - KAPPA);
    // Clip outer top right cap
    ctx.bezierCurveTo(left + c5, top + height, left, top + height - c5, left, top + height - rbl);
    ctx.closePath();
    ctx.clip();
    // Clip border right cap joins
    if (borderBottomWidth) {
        const trSlope = -borderBottomWidth / borderLeftWidth;
        ctx.moveTo(left + width / 2, trSlope * (width / 2) + top + height);
        ctx.lineTo(left, top + height);
        ctx.lineTo(left, top);
        ctx.lineTo(left + width, top);
        ctx.closePath();
        ctx.clip();
    }
    if (borderBottomWidth) {
        const trSlope = -borderTopWidth / borderLeftWidth;
        ctx.moveTo(left + width / 2, trSlope * (-width / 2) + top);
        ctx.lineTo(left, top);
        ctx.lineTo(left, top + height);
        ctx.lineTo(left + width, top + height);
        ctx.closePath();
        ctx.clip();
    }
};
const fillBorderLeft = (ctx, layout, style, rbl, rtl) => {
    const { top, left, height } = layout;
    const { borderLeftColor, borderLeftStyle, borderLeftWidth, borderTopWidth, borderBottomWidth, } = style;
    const c0 = rbl * (1.0 - KAPPA);
    const c1 = rtl * (1.0 - KAPPA);
    ctx.moveTo(left + rbl, top + height);
    ctx.bezierCurveTo(left + c0, top + height, left, top + height - c0, left, top + height - rbl);
    ctx.lineTo(left, top + rtl);
    ctx.bezierCurveTo(left, top + c1, left + c1, top, left + rtl, top);
    ctx.strokeColor(borderLeftColor);
    ctx.lineWidth(Math.max(borderLeftWidth, borderTopWidth, borderBottomWidth) * 2);
    if (borderLeftStyle === 'dashed') {
        ctx.dash(borderLeftWidth * 2, { space: borderLeftWidth * 1.2 });
    }
    else if (borderLeftStyle === 'dotted') {
        ctx.dash(borderLeftWidth, { space: borderLeftWidth * 1.2 });
    }
    ctx.stroke();
    ctx.undash();
};
const shouldRenderBorders = (node) => node.box &&
    (node.box.borderTopWidth ||
        node.box.borderRightWidth ||
        node.box.borderBottomWidth ||
        node.box.borderLeftWidth);
const renderBorders = (ctx, node) => {
    if (!node.box)
        return;
    if (!shouldRenderBorders(node))
        return;
    const { width, height, borderTopWidth = 0, borderLeftWidth = 0, borderRightWidth = 0, borderBottomWidth = 0, } = node.box;
    const { opacity = 1, borderTopColor = 'black', borderTopStyle = 'solid', borderLeftColor = 'black', borderLeftStyle = 'solid', borderRightColor = 'black', borderRightStyle = 'solid', borderBottomColor = 'black', borderBottomStyle = 'solid', } = node.style;
    // @ts-expect-error this is always a number due to resolve border radius step
    const borderTopLeftRadius = node.style.borderTopLeftRadius || 0;
    // @ts-expect-error this is always a number due to resolve border radius step
    const borderTopRightRadius = node.style.borderTopRightRadius || 0;
    // @ts-expect-error this is always a number due to resolve border radius step
    const borderBottomLeftRadius = node.style.borderBottomLeftRadius || 0;
    // @ts-expect-error this is always a number due to resolve border radius step
    const borderBottomRightRadius = node.style.borderBottomRightRadius || 0;
    const style = {
        borderTopColor,
        borderTopWidth,
        borderTopStyle,
        borderLeftColor,
        borderLeftWidth,
        borderLeftStyle,
        borderRightColor,
        borderRightWidth,
        borderRightStyle,
        borderBottomColor,
        borderBottomWidth,
        borderBottomStyle};
    const rtr = Math.min(borderTopRightRadius, 0.5 * width, 0.5 * height);
    const rtl = Math.min(borderTopLeftRadius, 0.5 * width, 0.5 * height);
    const rbr = Math.min(borderBottomRightRadius, 0.5 * width, 0.5 * height);
    const rbl = Math.min(borderBottomLeftRadius, 0.5 * width, 0.5 * height);
    ctx.save();
    ctx.strokeOpacity(opacity);
    if (borderTopWidth) {
        ctx.save();
        clipBorderTop(ctx, node.box, style, rtr, rtl);
        fillBorderTop(ctx, node.box, style, rtr, rtl);
        ctx.restore();
    }
    if (borderRightWidth) {
        ctx.save();
        clipBorderRight(ctx, node.box, style, rtr, rbr);
        fillBorderRight(ctx, node.box, style, rtr, rbr);
        ctx.restore();
    }
    if (borderBottomWidth) {
        ctx.save();
        clipBorderBottom(ctx, node.box, style, rbl, rbr);
        fillBorderBottom(ctx, node.box, style, rbl, rbr);
        ctx.restore();
    }
    if (borderLeftWidth) {
        ctx.save();
        clipBorderLeft(ctx, node.box, style, rbl, rtl);
        fillBorderLeft(ctx, node.box, style, rbl, rtl);
        ctx.restore();
    }
    ctx.restore();
};

const drawBackground = (ctx, node) => {
    if (!node.box)
        return;
    const { top, left, width, height } = node.box;
    const color = parseColor(node.style.backgroundColor);
    const nodeOpacity = isNil(node.style?.opacity) ? 1 : node.style.opacity;
    const opacity = Math.min(color.opacity, nodeOpacity);
    ctx
        .fillOpacity(opacity)
        .fillColor(color.value)
        .rect(left, top, width, height)
        .fill();
};
const renderBackground = (ctx, node) => {
    const hasBackground = !!node.box && !!node.style?.backgroundColor;
    if (hasBackground) {
        ctx.save();
        clipNode(ctx, node);
        drawBackground(ctx, node);
        ctx.restore();
    }
};

const isString = (value) => typeof value === 'string';
const isSrcId = (value) => /^#.+/.test(value);
const renderLink = (ctx, node, src) => {
    if (!src || !node.box)
        return;
    const isId = isSrcId(src);
    const method = isId ? 'goTo' : 'link';
    const value = isId ? src.slice(1) : src;
    const { top, left, width, height } = node.box;
    const props = node.props || {};
    const slop = resolveHitSlop('hitSlop' in props ? props.hitSlop : undefined);
    ctx[method](left - slop.left, top - slop.top, width + slop.left + slop.right, height + slop.top + slop.bottom, value);
};
const setLink = (ctx, node) => {
    const props = node.props || {};
    if ('src' in props && isString(props.src))
        return renderLink(ctx, node, props.src);
    if ('href' in props && isString(props.href))
        return renderLink(ctx, node, props.href);
};

const setDestination = (ctx, node) => {
    if (!node.box)
        return;
    if (!node.props)
        return;
    if ('id' in node.props) {
        ctx.addNamedDestination(node.props.id, 'XYZ', null, node.box.top, null);
    }
};

const clean = (options) => {
    const opt = { ...options };
    // We need to ensure the elements are no present if not true
    Object.entries(opt).forEach((pair) => {
        if (!pair[1]) {
            delete opt[pair[0]];
        }
    });
    return opt;
};
const parseCommonFormOptions = (node) => {
    // Common Options
    return {
        required: node.props?.required || false,
        noExport: node.props?.noExport || false,
        readOnly: node.props?.readOnly || false,
        value: node.props?.value || undefined,
        defaultValue: node.props?.defaultValue || undefined,
    };
};
const parseTextInputOptions = (node, fieldSet) => {
    return clean({
        ...parseCommonFormOptions(node),
        parent: fieldSet || undefined,
        align: node.props?.align || 'left',
        multiline: node.props?.multiline || undefined,
        password: node.props?.password || false,
        noSpell: node.props?.noSpell || false,
        format: node.props?.format || undefined,
        fontSize: node.props?.fontSize || undefined,
        MaxLen: node.props?.maxLength || undefined,
    });
};
const parseSelectAndListFieldOptions = (node) => {
    return clean({
        ...parseCommonFormOptions(node),
        sort: node.props?.sort || false,
        edit: node.props?.edit || false,
        multiSelect: node.props?.multiSelect || false,
        noSpell: node.props?.noSpell || false,
        select: node.props?.select || [''],
    });
};
const getAppearance = (ctx, codepoint, width, height) => {
    const appearance = ctx.ref({
        Type: 'XObject',
        Subtype: 'Form',
        BBox: [0, 0, width, height],
        Resources: {
            ProcSet: ['PDF', 'Text', 'ImageB', 'ImageC', 'ImageI'],
            Font: {
                ZaDi: ctx._acroform.fonts.ZaDi,
            },
        },
    });
    appearance.initDeflate();
    appearance.write(`/Tx BMC\nq\n/ZaDi ${height * 0.8} Tf\nBT\n${width * 0.45} ${height / 4} Td (${codepoint}) Tj\nET\nQ\nEMC`);
    appearance.end(null);
    return appearance;
};
const parseCheckboxOptions = (ctx, node, fieldSet) => {
    const { width, height } = node.box || {};
    const onOption = node.props?.onState || 'Yes';
    const offOption = node.props?.offState || 'Off';
    const xMark = node.props?.xMark || false;
    if (!Object.prototype.hasOwnProperty.call(ctx._acroform.fonts, 'ZaDi')) {
        const ref = ctx.ref({
            Type: 'Font',
            Subtype: 'Type1',
            BaseFont: 'ZapfDingbats',
        });
        ctx._acroform.fonts.ZaDi = ref;
        ref.end(null);
    }
    const normalAppearance = {
        [onOption]: getAppearance(ctx, xMark ? '8' : '4', width, height),
        [offOption]: getAppearance(ctx, xMark ? ' ' : '8', width, height),
    };
    return clean({
        ...parseCommonFormOptions(node),
        backgroundColor: node.props?.backgroundColor || undefined,
        borderColor: node.props?.borderColor || undefined,
        parent: fieldSet || undefined,
        value: `/${node.props?.checked === true ? onOption : offOption}`,
        defaultValue: `/${node.props?.checked === true ? onOption : offOption}`,
        AS: node.props?.checked === true ? onOption : offOption,
        AP: { N: normalAppearance, D: normalAppearance },
    });
};

const renderTextInput = (ctx, node, options) => {
    if (!node.box)
        return;
    const { top, left, width, height } = node.box;
    // Element's name
    const name = node.props?.name || '';
    const fieldSetOptions = options.fieldSets?.at(0);
    if (!ctx._root.data.AcroForm) {
        ctx.initForm();
    }
    ctx.formText(name, left, top, width, height, parseTextInputOptions(node, fieldSetOptions));
};

const renderSelect = (ctx, node) => {
    if (!node.box)
        return;
    const { top, left, width, height } = node.box;
    // Element's name
    const name = node.props?.name || '';
    if (!ctx._root.data.AcroForm) {
        ctx.initForm();
    }
    ctx.formCombo(name, left, top, width, height, parseSelectAndListFieldOptions(node));
};

const renderFieldSet = (ctx, node, options) => {
    const name = node.props?.name || '';
    if (!ctx._root.data.AcroForm) {
        ctx.initForm();
    }
    const formField = ctx.formField(name);
    const option = options;
    if (!option.fieldSets) {
        option.fieldSets = [formField];
    }
    else {
        option.fieldSets.push(formField);
    }
};
const cleanUpFieldSet = (_ctx, _node, options) => {
    options.fieldSets.pop();
};

const renderList = (ctx, node) => {
    if (!node.box)
        return;
    const { top, left, width, height } = node.box || {};
    // Element's name
    const name = ('name' in node.props ? node.props.name || '' : '');
    if (!ctx._root.data.AcroForm) {
        ctx.initForm();
    }
    ctx.formList(name, left, top, width, height, parseSelectAndListFieldOptions(node));
};

const renderCheckbox = (ctx, node, options) => {
    if (!node.box)
        return;
    const { top, left, width, height } = node.box;
    // Element's name
    const name = node.props?.name || '';
    const fieldSetOptions = options.fieldSets?.at(0);
    if (!ctx._root.data.AcroForm) {
        ctx.initForm();
    }
    ctx.formCheckbox(name, left, top, width, height, parseCheckboxOptions(ctx, node, fieldSetOptions));
};

const isRecursiveNode = (node) => node.type !== P.Text && node.type !== P.Svg;
const renderChildren = (ctx, node, options) => {
    ctx.save();
    if (node.box) {
        ctx.translate(node.box.left, node.box.top);
    }
    const children = node.children || [];
    const renderChild = (child) => renderNode(ctx, child, options);
    children.forEach(renderChild);
    ctx.restore();
};
const renderFns = {
    [P.Text]: renderText,
    [P.Note]: renderNote,
    [P.Image]: renderImage,
    [P.ImageBackground]: renderImageBackground,
    [P.FieldSet]: renderFieldSet,
    [P.TextInput]: renderTextInput,
    [P.Select]: renderSelect,
    [P.Checkbox]: renderCheckbox,
    [P.List]: renderList,
    [P.Canvas]: renderCanvas,
    [P.Svg]: renderSvg,
    [P.Link]: setLink,
};
const cleanUpFns = {
    [P.FieldSet]: cleanUpFieldSet,
};
const renderNode = (ctx, node, options) => {
    const overflowHidden = node.style?.overflow === 'hidden';
    const shouldRenderChildren = isRecursiveNode(node);
    if (node.type === P.Page)
        renderPage(ctx, node);
    ctx.save();
    if (overflowHidden)
        clipNode(ctx, node);
    applyTransformations(ctx, node);
    renderBackground(ctx, node);
    renderBorders(ctx, node);
    const renderFn = renderFns[node.type];
    if (renderFn)
        renderFn(ctx, node, options);
    if (shouldRenderChildren)
        renderChildren(ctx, node, options);
    const cleanUpFn = cleanUpFns[node.type];
    if (cleanUpFn)
        cleanUpFn(ctx, node, options);
    setDestination(ctx, node);
    renderDebug(ctx, node);
    ctx.restore();
};

const addNodeBookmark = (ctx, node, pageNumber, registry) => {
    if (!node.box)
        return;
    if (!node.props)
        return;
    if ('bookmark' in node.props && node.props.bookmark) {
        const bookmark = node.props.bookmark;
        const { title, parent, expanded, zoom, fit } = bookmark;
        const outline = registry[parent] || ctx.outline;
        const top = bookmark.top || node.box.top;
        const left = bookmark.left || node.box.left;
        const instance = outline.addItem(title, {
            pageNumber,
            expanded,
            top,
            left,
            zoom,
            fit,
        });
        registry[bookmark.ref] = instance;
    }
    if (!node.children)
        return;
    node.children.forEach((child) => addNodeBookmark(ctx, child, pageNumber, registry));
};
const addBookmarks = (ctx, root) => {
    const registry = {};
    const pages = root.children || [];
    pages.forEach((page, i) => {
        addNodeBookmark(ctx, page, i, registry);
    });
};

const render = (ctx, doc) => {
    const pages = doc.children || [];
    const options = { imageCache: new Map(), fieldSets: [] };
    pages.forEach((page) => renderNode(ctx, page, options));
    addBookmarks(ctx, doc);
    ctx.end();
    return ctx;
};

export { render as default };
