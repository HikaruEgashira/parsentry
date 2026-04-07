/// <reference types="node" />
import { Glyph as Glyph$1 } from 'fontkit';
import { Font } from '@react-pdf/font';
export { Font } from '@react-pdf/font';

type Factor = {
    before: number;
    after: number;
    priority?: number;
    unconstrained?: boolean;
};

type Coordinate = {
    x: number;
    y: number;
};
type Rect = {
    x: number;
    y: number;
    width: number;
    height: number;
};
type Container = Rect & {
    truncateMode?: 'ellipsis';
    maxLines?: number;
    excludeRects?: Rect[];
};
type Glyph = Glyph$1;
type Position = {
    xAdvance: number;
    yAdvance: number;
    xOffset: number;
    yOffset: number;
    advanceWidth?: number;
};
type Attachment = {
    x?: number;
    y?: number;
    width: number;
    height: number;
    xOffset?: number;
    yOffset?: number;
    image: Buffer;
};
type Attributes = {
    align?: string;
    alignLastLine?: string;
    attachment?: Attachment;
    backgroundColor?: string;
    bidiLevel?: number;
    bullet?: unknown;
    characterSpacing?: number;
    color?: string;
    direction?: 'rtl' | 'ltr';
    features?: unknown[];
    fill?: boolean;
    font?: Font[];
    fontSize?: number;
    hangingPunctuation?: boolean;
    hyphenationFactor?: number;
    indent?: number;
    justificationFactor?: number;
    lineHeight?: number;
    lineSpacing?: number;
    link?: string;
    margin?: number;
    marginLeft?: number;
    marginRight?: number;
    opacity?: number;
    padding?: number;
    paddingTop?: number;
    paragraphSpacing?: number;
    scale?: number;
    script?: unknown;
    shrinkFactor?: number;
    strike?: boolean;
    strikeColor?: string;
    strikeStyle?: string;
    stroke?: boolean;
    underline?: boolean;
    underlineColor?: string;
    underlineStyle?: string;
    verticalAlign?: string;
    wordSpacing?: number;
    yOffset?: number;
};
type Run = {
    start: number;
    end: number;
    attributes: Attributes;
    glyphIndices?: number[];
    glyphs?: Glyph[];
    positions?: Position[];
    xAdvance?: number;
    height?: number;
    descent?: number;
};
type DecorationLine = {
    rect: Rect;
    opacity: number;
    color: string;
    style: string;
};
type AttributedString = {
    string: string;
    syllables?: string[];
    runs: Run[];
    box?: Rect;
    decorationLines?: DecorationLine[];
    overflowLeft?: number;
    overflowRight?: number;
    xAdvance?: number;
    ascent?: number;
};
type Fragment = {
    string: string;
    attributes?: Attributes;
};
type Paragraph = AttributedString[];
type LayoutOptions = {
    hyphenationCallback?: (word: string | null, fallback: (word: string | null) => string[]) => string[];
    tolerance?: number;
    hyphenationPenalty?: number;
    expandCharFactor?: Factor;
    shrinkCharFactor?: Factor;
    expandWhitespaceFactor?: Factor;
    shrinkWhitespaceFactor?: Factor;
};

declare const bidiEngine: () => (attributedString: AttributedString) => AttributedString;

/**
 * Performs Knuth & Plass line breaking algorithm
 * Fallbacks to best fit algorithm if latter not successful
 *
 * @param options - Layout options
 */
declare const linebreaker: (options: LayoutOptions) => (attributedString: AttributedString, availableWidths: number[]) => AttributedString[];

/**
 * A JustificationEngine is used by a Typesetter to perform line fragment
 * justification. This implementation is based on a description of Apple's
 * justification algorithm from a PDF in the Apple Font Tools package.
 *
 * @param options - Layout options
 */
declare const justification: (options: LayoutOptions) => (line: AttributedString) => AttributedString;

declare const fontSubstitution: () => ({ string, runs }: AttributedString) => AttributedString;

/**
 * A TextDecorationEngine is used by a Typesetter to generate
 * DecorationLines for a line fragment, including underlines
 * and strikes.
 */
declare const textDecoration: () => (line: AttributedString) => AttributedString;

/**
 * Resolves unicode script in runs, grouping equal runs together
 */
declare const scriptItemizer: () => (attributedString: AttributedString) => AttributedString;

declare const wordHyphenation: () => (word: string | null) => string[];

type Engines = {
    bidi: typeof bidiEngine;
    linebreaker: typeof linebreaker;
    justification: typeof justification;
    fontSubstitution: typeof fontSubstitution;
    scriptItemizer: typeof scriptItemizer;
    textDecoration: typeof textDecoration;
    wordHyphenation?: typeof wordHyphenation;
};

/**
 * A LayoutEngine is the main object that performs text layout.
 * It accepts an AttributedString and a Container object
 * to layout text into, and uses several helper objects to perform
 * various layout tasks. These objects can be overridden to customize
 * layout behavior.
 */
declare const layoutEngine: (engines: Engines) => (attributedString: AttributedString, container: Container, options?: LayoutOptions) => Paragraph[];

/**
 * Create attributed string from text fragments
 *
 * @param fragments - Fragments
 * @returns Attributed string
 */
declare const fromFragments: (fragments: Fragment[]) => AttributedString;

export { type Attachment, type AttributedString, type Attributes, type Container, type Coordinate, type DecorationLine, type Fragment, type Glyph, type LayoutOptions, type Paragraph, type Position, type Rect, type Run, bidiEngine as bidi, layoutEngine as default, fontSubstitution, fromFragments, justification, linebreaker, scriptItemizer, textDecoration, wordHyphenation };
