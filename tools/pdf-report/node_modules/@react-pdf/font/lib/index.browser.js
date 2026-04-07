import 'is-url';
import * as fontkit from 'fontkit';
import PDFDocument from '@react-pdf/pdfkit';

// @ts-expect-error ts being silly
const STANDARD_FONTS = [
    'Courier',
    'Courier-Bold',
    'Courier-Oblique',
    'Courier-BoldOblique',
    'Helvetica',
    'Helvetica-Bold',
    'Helvetica-Oblique',
    'Helvetica-BoldOblique',
    'Times-Roman',
    'Times-Bold',
    'Times-Italic',
    'Times-BoldItalic',
];
// Create a shared lightweight document for accessing standard font instances.
// Standard fonts are created once and cached, so this is negligible overhead.
let _sharedDoc = null;
const openStandardFont = (src) => {
    if (!_sharedDoc) {
        _sharedDoc = new PDFDocument({ autoFirstPage: false });
    }
    _sharedDoc.font(src);
    return _sharedDoc._font;
};
class StandardFont {
    name;
    src;
    fullName;
    familyName;
    subfamilyName;
    postscriptName;
    copyright;
    version;
    underlinePosition;
    underlineThickness;
    italicAngle;
    bbox;
    'OS/2';
    hhea;
    numGlyphs;
    characterSet;
    availableFeatures;
    type;
    constructor(src) {
        this.name = src;
        this.fullName = src;
        this.familyName = src;
        this.subfamilyName = src;
        this.type = 'STANDARD';
        this.postscriptName = src;
        this.availableFeatures = [];
        this.copyright = '';
        this.version = 1;
        this.underlinePosition = -100;
        this.underlineThickness = 50;
        this.italicAngle = 0;
        this.bbox = {};
        this['OS/2'] = {};
        this.hhea = {};
        this.numGlyphs = 0;
        this.characterSet = [];
        this.src = openStandardFont(src);
    }
    encode(str) {
        const [encoded, positions] = this.src.encode(str);
        // Soft hyphens (U+00AD) should have zero width for line breaking purposes.
        // Upstream pdfkit maps them to 'hyphen' in AFM data, so we override here.
        for (let i = 0; i < str.length; i++) {
            if (str.charCodeAt(i) === 0x00ad) {
                positions[i].advanceWidth = 0;
            }
        }
        return [encoded, positions];
    }
    layout(str) {
        const [encoded, positions] = this.encode(str);
        const glyphs = encoded.map((g, i) => {
            const glyph = this.getGlyph(parseInt(g, 16));
            glyph.advanceWidth = positions[i].advanceWidth;
            return glyph;
        });
        const advanceWidth = positions.reduce((acc, p) => acc + p.advanceWidth, 0);
        return {
            positions,
            stringIndices: positions.map((_, i) => i),
            glyphs,
            script: 'latin',
            language: 'dflt',
            direction: 'ltr',
            features: {},
            advanceWidth,
            advanceHeight: 0,
            bbox: undefined,
        };
    }
    glyphForCodePoint(codePoint) {
        const glyph = this.getGlyph(codePoint);
        glyph.advanceWidth = 400;
        return glyph;
    }
    getGlyph(id) {
        return {
            id,
            codePoints: [id],
            isLigature: false,
            name: this.src.font.characterToGlyph(id),
            _font: this.src,
            // @ts-expect-error assign proper value
            advanceWidth: undefined,
        };
    }
    hasGlyphForCodePoint(codePoint) {
        return this.src.font.characterToGlyph(codePoint) !== '.notdef';
    }
    // Based on empirical observation
    get ascent() {
        return 900;
    }
    // Based on empirical observation
    get capHeight() {
        switch (this.name) {
            case 'Times-Roman':
            case 'Times-Bold':
            case 'Times-Italic':
            case 'Times-BoldItalic':
                return 650;
            case 'Courier':
            case 'Courier-Bold':
            case 'Courier-Oblique':
            case 'Courier-BoldOblique':
                return 550;
            default:
                return 690;
        }
    }
    // Based on empirical observation
    get xHeight() {
        switch (this.name) {
            case 'Times-Roman':
            case 'Times-Bold':
            case 'Times-Italic':
            case 'Times-BoldItalic':
                return 440;
            case 'Courier':
            case 'Courier-Bold':
            case 'Courier-Oblique':
            case 'Courier-BoldOblique':
                return 390;
            default:
                return 490;
        }
    }
    // Based on empirical observation
    get descent() {
        switch (this.name) {
            case 'Times-Roman':
            case 'Times-Bold':
            case 'Times-Italic':
            case 'Times-BoldItalic':
                return -220;
            case 'Courier':
            case 'Courier-Bold':
            case 'Courier-Oblique':
            case 'Courier-BoldOblique':
                return -230;
            default:
                return -200;
        }
    }
    get lineGap() {
        return 0;
    }
    get unitsPerEm() {
        return 1000;
    }
    stringsForGlyph() {
        throw new Error('Method not implemented.');
    }
    glyphsForString() {
        throw new Error('Method not implemented.');
    }
    widthOfGlyph() {
        throw new Error('Method not implemented.');
    }
    getAvailableFeatures() {
        throw new Error('Method not implemented.');
    }
    createSubset() {
        throw new Error('Method not implemented.');
    }
    getVariation() {
        throw new Error('Method not implemented.');
    }
    getFont() {
        throw new Error('Method not implemented.');
    }
    getName() {
        throw new Error('Method not implemented.');
    }
    setDefaultLanguage() {
        throw new Error('Method not implemented.');
    }
}

const fetchFont = async (src, options) => {
    const response = await fetch(src, options);
    if (!response.ok) {
        throw new Error(`Failed to fetch font from ${src}: ${response.status} ${response.statusText}`);
    }
    const data = await response.arrayBuffer();
    return new Uint8Array(data);
};
const isDataUrl = (dataUrl) => {
    const commaIndex = dataUrl.indexOf(',');
    if (commaIndex === -1)
        return false;
    const header = dataUrl.substring(0, commaIndex);
    const hasDataPrefix = header.startsWith('data:');
    const hasBase64Prefix = header.includes(';base64');
    return hasDataPrefix && hasBase64Prefix;
};
class FontSource {
    src;
    fontFamily;
    fontStyle;
    fontWeight;
    data;
    options;
    loadResultPromise;
    constructor(src, fontFamily, fontStyle, fontWeight, options) {
        this.src = src;
        this.fontFamily = fontFamily;
        this.fontStyle = fontStyle || 'normal';
        this.fontWeight = fontWeight || 400;
        this.data = null;
        this.options = options || {};
        this.loadResultPromise = null;
    }
    async _load() {
        const { postscriptName } = this.options;
        let data = null;
        if (STANDARD_FONTS.includes(this.src)) {
            data = new StandardFont(this.src);
        }
        else if (isDataUrl(this.src)) {
            const raw = this.src.split(',')[1];
            const uint8Array = new Uint8Array(atob(raw)
                .split('')
                .map((c) => c.charCodeAt(0)));
            data = fontkit.create(uint8Array, postscriptName);
        }
        else {
            const { headers, body, method = 'GET' } = this.options;
            const buffer = await fetchFont(this.src, { method, body, headers });
            data = fontkit.create(buffer, postscriptName);
        }
        if (data && 'fonts' in data) {
            throw new Error('Font collection is not supported');
        }
        this.data = data;
    }
    async load() {
        if (this.loadResultPromise === null) {
            this.loadResultPromise = this._load();
        }
        return this.loadResultPromise;
    }
}

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
const resolveFontWeight = (value) => {
    return typeof value === 'string' ? FONT_WEIGHTS[value] : value;
};
const sortByFontWeight = (a, b) => a.fontWeight - b.fontWeight;
class FontFamily {
    family;
    sources;
    static create(family) {
        return new FontFamily(family);
    }
    constructor(family) {
        this.family = family;
        this.sources = [];
    }
    register({ src, fontWeight, fontStyle, ...options }) {
        const numericFontWeight = fontWeight
            ? resolveFontWeight(fontWeight)
            : undefined;
        this.sources.push(new FontSource(src, this.family, fontStyle, numericFontWeight, options));
    }
    resolve(descriptor) {
        const { fontWeight = 400, fontStyle = 'normal' } = descriptor;
        const styleSources = this.sources.filter((s) => s.fontStyle === fontStyle);
        const exactFit = styleSources.find((s) => s.fontWeight === fontWeight);
        if (exactFit)
            return exactFit;
        // Weight resolution. https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight#Fallback_weights
        let font = null;
        const numericFontWeight = resolveFontWeight(fontWeight);
        if (numericFontWeight >= 400 && numericFontWeight <= 500) {
            const leftOffset = styleSources.filter((s) => s.fontWeight <= numericFontWeight);
            const rightOffset = styleSources.filter((s) => s.fontWeight > 500);
            const fit = styleSources.filter((s) => s.fontWeight >= numericFontWeight && s.fontWeight <= 500);
            font = fit[0] || leftOffset[leftOffset.length - 1] || rightOffset[0];
        }
        const lt = styleSources
            .filter((s) => s.fontWeight < numericFontWeight)
            .sort(sortByFontWeight);
        const gt = styleSources
            .filter((s) => s.fontWeight > numericFontWeight)
            .sort(sortByFontWeight);
        if (numericFontWeight < 400) {
            font = lt[lt.length - 1] || gt[0];
        }
        if (numericFontWeight > 500) {
            font = gt[0] || lt[lt.length - 1];
        }
        if (!font) {
            throw new Error(`Could not resolve font for ${this.family}, fontWeight ${fontWeight}, fontStyle ${fontStyle}`);
        }
        return font;
    }
}

class FontStore {
    fontFamilies = {};
    emojiSource = null;
    constructor() {
        this.register({
            family: 'Helvetica',
            fonts: [
                { src: 'Helvetica', fontStyle: 'normal', fontWeight: 400 },
                { src: 'Helvetica-Bold', fontStyle: 'normal', fontWeight: 700 },
                { src: 'Helvetica-Oblique', fontStyle: 'italic', fontWeight: 400 },
                { src: 'Helvetica-BoldOblique', fontStyle: 'italic', fontWeight: 700 },
            ],
        });
        this.register({
            family: 'Courier',
            fonts: [
                { src: 'Courier', fontStyle: 'normal', fontWeight: 400 },
                { src: 'Courier-Bold', fontStyle: 'normal', fontWeight: 700 },
                { src: 'Courier-Oblique', fontStyle: 'italic', fontWeight: 400 },
                { src: 'Courier-BoldOblique', fontStyle: 'italic', fontWeight: 700 },
            ],
        });
        this.register({
            family: 'Times-Roman',
            fonts: [
                { src: 'Times-Roman', fontStyle: 'normal', fontWeight: 400 },
                { src: 'Times-Bold', fontStyle: 'normal', fontWeight: 700 },
                { src: 'Times-Italic', fontStyle: 'italic', fontWeight: 400 },
                { src: 'Times-BoldItalic', fontStyle: 'italic', fontWeight: 700 },
            ],
        });
        // For backwards compatibility
        this.register({
            family: 'Helvetica-Bold',
            src: 'Helvetica-Bold',
        });
        this.register({
            family: 'Helvetica-Oblique',
            src: 'Helvetica-Oblique',
        });
        this.register({
            family: 'Helvetica-BoldOblique',
            src: 'Helvetica-BoldOblique',
        });
        this.register({
            family: 'Courier-Bold',
            src: 'Courier-Bold',
        });
        this.register({
            family: 'Courier-Oblique',
            src: 'Courier-Oblique',
        });
        this.register({
            family: 'Courier-BoldOblique',
            src: 'Courier-BoldOblique',
        });
        this.register({
            family: 'Times-Bold',
            src: 'Times-Bold',
        });
        this.register({
            family: 'Times-Italic',
            src: 'Times-Italic',
        });
        this.register({
            family: 'Times-BoldItalic',
            src: 'Times-BoldItalic',
        });
        // Load default fonts
        this.load({
            fontFamily: 'Helvetica',
            fontStyle: 'normal',
            fontWeight: 400,
        });
        this.load({
            fontFamily: 'Helvetica',
            fontStyle: 'normal',
            fontWeight: 700,
        });
        this.load({
            fontFamily: 'Helvetica',
            fontStyle: 'italic',
            fontWeight: 400,
        });
        this.load({
            fontFamily: 'Helvetica',
            fontStyle: 'italic',
            fontWeight: 700,
        });
    }
    hyphenationCallback = null;
    register = (data) => {
        const { family } = data;
        if (!this.fontFamilies[family]) {
            this.fontFamilies[family] = FontFamily.create(family);
        }
        // Bulk loading
        if ('fonts' in data) {
            for (let i = 0; i < data.fonts.length; i += 1) {
                const { src, fontStyle, fontWeight, ...options } = data.fonts[i];
                this.fontFamilies[family].register({
                    src,
                    fontStyle,
                    fontWeight,
                    ...options,
                });
            }
        }
        else {
            const { src, fontStyle, fontWeight, ...options } = data;
            this.fontFamilies[family].register({
                src,
                fontStyle,
                fontWeight,
                ...options,
            });
        }
    };
    registerEmojiSource = (emojiSource) => {
        this.emojiSource = emojiSource;
    };
    registerHyphenationCallback = (callback) => {
        this.hyphenationCallback = callback;
    };
    getFont = (descriptor) => {
        const { fontFamily } = descriptor;
        if (!this.fontFamilies[fontFamily]) {
            throw new Error(`Font family not registered: ${fontFamily}. Please register it calling Font.register() method.`);
        }
        return this.fontFamilies[fontFamily].resolve(descriptor);
    };
    load = async (descriptor) => {
        const font = this.getFont(descriptor);
        if (font)
            await font.load();
    };
    reset = () => {
        const keys = Object.keys(this.fontFamilies);
        for (let i = 0; i < keys.length; i += 1) {
            const key = keys[i];
            for (let j = 0; j < this.fontFamilies[key].sources.length; j++) {
                const fontSource = this.fontFamilies[key].sources[j];
                fontSource.data = null;
            }
        }
    };
    clear = () => {
        this.fontFamilies = {};
        this.emojiSource = null;
        this.hyphenationCallback = null;
    };
    getRegisteredFonts = () => this.fontFamilies;
    getEmojiSource = () => this.emojiSource;
    getHyphenationCallback = () => this.hyphenationCallback;
    getRegisteredFontFamilies = () => Object.keys(this.fontFamilies);
}

export { FontStore as default };
