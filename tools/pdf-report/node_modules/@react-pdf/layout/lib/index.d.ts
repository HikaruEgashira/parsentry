/// <reference path="../globals.d.ts" />
/// <reference types="node" />
import { Style, SafeStyle, Transform } from '@react-pdf/stylesheet';
import { YogaNode } from 'yoga-layout/load';
import * as React from 'react';
import * as P from '@react-pdf/primitives';
import { SrcSet, Sizes, HitSlop } from '@react-pdf/types';
import { Image } from '@react-pdf/image';
import { HyphenationCallback } from '@react-pdf/font';
import { Paragraph } from '@react-pdf/textkit';

interface LineProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    x1: string | number;
    x2: string | number;
    y1: string | number;
    y2: string | number;
}
interface SafeLineProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    x1: number;
    x2: number;
    y1: number;
    y2: number;
}
type LineNode = {
    type: typeof P.Line;
    props: LineProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafeLineNode = Omit<LineNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafeLineProps;
};

interface PolylineProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    points: string;
}
interface SafePolylineProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    points: string;
}
type PolylineNode = {
    type: typeof P.Polyline;
    props: PolylineProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafePolylineNode = Omit<PolylineNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafePolylineProps;
};

interface PolygonProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    points: string;
}
interface SafePolygonProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    points: string;
}
type PolygonNode = {
    type: typeof P.Polygon;
    props: PolygonProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafePolygonNode = Omit<PolygonNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafePolygonProps;
};

interface PathProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    d: string;
}
interface SafePathProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    d: string;
}
type PathNode = {
    type: typeof P.Path;
    props: PathProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafePathNode = Omit<PathNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafePathProps;
};

interface RectProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    x?: string | number;
    y?: string | number;
    width: string | number;
    height: string | number;
    rx?: string | number;
    ry?: string | number;
}
interface SafeRectProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    x?: number;
    y?: number;
    width: number;
    height: number;
    rx?: number;
    ry?: number;
}
type RectNode = {
    type: typeof P.Rect;
    props: RectProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafeRectNode = Omit<RectNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafeRectProps;
};

interface CircleProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    cx?: string | number;
    cy?: string | number;
    r: string | number;
}
interface SafeCircleProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    cx?: number;
    cy?: number;
    r: number;
}
type CircleNode = {
    type: typeof P.Circle;
    props: CircleProps;
    style?: Style | Style[];
    box?: never;
    origin?: Origin;
    yogaNode?: never;
    children?: never[];
};
type SafeCircleNode = Omit<CircleNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafeCircleProps;
};

interface EllipseProps extends SVGPresentationAttributes {
    style?: SVGPresentationAttributes;
    cx?: string | number;
    cy?: string | number;
    rx: string | number;
    ry: string | number;
}
interface SafeEllipseProps extends SafeSVGPresentationAttributes {
    style?: SafeSVGPresentationAttributes;
    cx?: number;
    cy?: number;
    rx: number;
    ry: number;
}
type EllipseNode = {
    type: typeof P.Ellipse;
    props: EllipseProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafeEllipseNode = Omit<EllipseNode, 'style' | 'props'> & {
    style: SafeStyle;
    props: SafeEllipseProps;
};

interface ClipPathProps {
    id?: string;
}
type ClipPathNode = {
    type: typeof P.ClipPath;
    props: ClipPathProps;
    style: never;
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: (LineNode | PolylineNode | PolygonNode | PathNode | RectNode | CircleNode | EllipseNode)[];
};
type SafeClipPathNode = Omit<ClipPathNode, 'children'> & {
    children?: (SafeLineNode | SafePolylineNode | SafePolygonNode | SafePathNode | SafeRectNode | SafeCircleNode | SafeEllipseNode)[];
};

interface StopProps {
    offset: string | number;
    stopColor: string;
    stopOpacity?: string | number;
}
interface StopSafeProps {
    offset: number;
    stopColor: string;
    stopOpacity?: number;
}
type StopNode = {
    type: typeof P.Stop;
    props: StopProps;
    style?: never;
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: never[];
};
type SafeStopNode = Omit<StopNode, 'props'> & {
    props: StopSafeProps;
};

interface LinearGradientProps {
    id: string;
    x1?: string | number;
    x2?: string | number;
    y1?: string | number;
    y2?: string | number;
    xlinkHref?: string;
    gradientTransform?: string;
    gradientUnits?: 'userSpaceOnUse' | 'objectBoundingBox';
}
interface SafeLinearGradientProps {
    id: string;
    x1?: number;
    x2?: number;
    y1?: number;
    y2?: number;
    xlinkHref?: string;
    gradientTransform?: Transform[];
    gradientUnits?: 'userSpaceOnUse' | 'objectBoundingBox';
}
type LinearGradientNode = {
    type: typeof P.LinearGradient;
    props: LinearGradientProps;
    style?: never;
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: StopNode[];
};
type SafeLinearGradientNode = Omit<LinearGradientNode, 'props' | 'children'> & {
    props: SafeLinearGradientProps;
    children?: SafeStopNode[];
};

interface RadialGradientProps {
    id: string;
    cx?: string | number;
    cy?: string | number;
    fr?: string | number;
    fx?: string | number;
    fy?: string | number;
    r?: string | number;
    xlinkHref?: string;
    gradientTransform?: string;
    gradientUnits?: 'userSpaceOnUse' | 'objectBoundingBox';
}
interface SafeRadialGradientProps {
    id: string;
    cx?: number;
    cy?: number;
    fr?: number;
    fx?: number;
    fy?: number;
    r?: number;
    xlinkHref?: string;
    gradientTransform?: Transform[];
    gradientUnits?: 'userSpaceOnUse' | 'objectBoundingBox';
}
type RadialGradientNode = {
    type: typeof P.RadialGradient;
    props: RadialGradientProps;
    style?: never;
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: StopNode[];
};
type SafeRadialGradientNode = Omit<RadialGradientNode, 'props' | 'children'> & {
    props: SafeRadialGradientProps;
    children?: SafeStopNode[];
};

type YogaInstance = {
    node: {
        create: () => YogaNode;
    };
};
type Box = {
    width: number;
    height: number;
    top: number;
    left: number;
    right: number;
    bottom: number;
    marginTop?: number;
    marginRight?: number;
    marginBottom?: number;
    marginLeft?: number;
    paddingTop?: number;
    paddingRight?: number;
    paddingBottom?: number;
    paddingLeft?: number;
    borderTopWidth?: number;
    borderRightWidth?: number;
    borderBottomWidth?: number;
    borderLeftWidth?: number;
};
type Origin = {
    left: number;
    top: number;
};
interface Bookmark {
    title: string;
    top?: number;
    left?: number;
    zoom?: number;
    fit?: true | false;
    expanded?: true | false;
    parent?: number;
    ref?: number;
}
type DynamicPageProps = {
    pageNumber: number;
    totalPages?: number;
    subPageNumber?: number;
    subPageTotalPages?: number;
};
type RenderProp = (props: DynamicPageProps) => React.ReactNode | null | undefined;
type NodeProps = {
    id?: string;
    /**
     * Render component in all wrapped pages.
     * @see https://react-pdf.org/advanced#fixed-components
     */
    fixed?: boolean;
    /**
     * Force the wrapping algorithm to start a new page when rendering the
     * element.
     * @see https://react-pdf.org/advanced#page-breaks
     */
    break?: boolean;
    /**
     * Hint that no page wrapping should occur between all sibling elements following the element within n points
     * @see https://react-pdf.org/advanced#orphan-&-widow-protection
     */
    minPresenceAhead?: number;
    /**
     * Enables debug mode on page bounding box.
     * @see https://react-pdf.org/advanced#debugging
     */
    debug?: boolean;
    /**
     * Bookmark for the element.
     * @see https://react-pdf.org/advanced#bookmarks
     */
    bookmark?: string | Bookmark;
};
type FillRule = 'nonzero' | 'evenodd';
type TextAnchor = 'start' | 'middle' | 'end';
type StrokeLinecap = 'butt' | 'round' | 'square';
type StrokeLinejoin = 'butt' | 'round' | 'square' | 'miter' | 'bevel';
type Visibility = 'visible' | 'hidden' | 'collapse';
type DominantBaseline = 'auto' | 'middle' | 'central' | 'hanging' | 'mathematical' | 'text-after-edge' | 'text-before-edge';
type SVGPresentationAttributes = {
    fill?: string;
    color?: string;
    stroke?: string;
    transform?: string;
    strokeDasharray?: string;
    opacity?: string | number;
    strokeWidth?: string | number;
    fillOpacity?: string | number;
    fillRule?: FillRule;
    strokeOpacity?: string | number;
    textAnchor?: TextAnchor;
    strokeLinecap?: StrokeLinecap;
    strokeLinejoin?: StrokeLinejoin;
    visibility?: Visibility;
    clipPath?: string;
    dominantBaseline?: DominantBaseline;
};
type SafeSVGPresentationAttributes = {
    fill?: string | SafeLinearGradientNode | SafeRadialGradientNode;
    color?: string;
    stroke?: string;
    transform?: Transform[];
    strokeDasharray?: string;
    opacity?: number;
    strokeWidth?: number;
    fillOpacity?: number;
    fillRule?: FillRule;
    strokeOpacity?: number;
    textAnchor?: TextAnchor;
    strokeLinecap?: StrokeLinecap;
    strokeLinejoin?: StrokeLinejoin;
    visibility?: Visibility;
    clipPath?: SafeClipPathNode;
    dominantBaseline?: DominantBaseline;
};
interface FormCommonProps extends NodeProps {
    name?: string;
    required?: boolean;
    noExport?: boolean;
    readOnly?: boolean;
    value?: number | string;
    defaultValue?: number | string;
}

interface CanvasProps extends NodeProps {
    paint: (painter: any, availableWidth?: number, availableHeight?: number) => null;
}
type CanvasNode = {
    type: typeof P.Canvas;
    props: CanvasProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: never[];
};
type SafeCanvasNode = Omit<CanvasNode, 'style'> & {
    style: SafeStyle;
};

interface CheckboxProps extends FormCommonProps {
    backgroundColor?: string;
    borderColor?: string;
    checked?: boolean;
    onState?: string;
    offState?: string;
    xMark?: boolean;
}
type CheckboxNode = {
    type: typeof P.Checkbox;
    props: CheckboxProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: never[];
};
type SafeCheckboxNode = Omit<CheckboxNode, 'style'> & {
    style: SafeStyle;
};

type DefsNode = {
    type: typeof P.Defs;
    props?: never;
    style?: never;
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: (ClipPathNode | LinearGradientNode | RadialGradientNode)[];
};
type Defs = Record<string, DefsNode['children'][number]>;
type SafeDefsNode = Omit<DefsNode, 'children'> & {
    children?: (SafeClipPathNode | SafeLinearGradientNode | SafeRadialGradientNode)[];
};
type SafeDefs = Record<string, SafeDefsNode['children'][number]>;

type HTTPMethod = 'GET' | 'HEAD' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
type SourceURL = string;
type SourceBuffer = Buffer;
type SourceBlob = Blob;
type SourceDataBuffer = {
    data: Buffer;
    format: 'png' | 'jpg';
};
type SourceURLObject = {
    uri: string;
    method?: HTTPMethod;
    body?: any;
    headers?: any;
    credentials?: 'omit' | 'same-origin' | 'include';
};
type Source = SourceURL | SourceBuffer | SourceBlob | SourceDataBuffer | SourceURLObject | undefined;
type SourceFactory = () => Source;
type SourceAsync = Promise<Source>;
type SourceAsyncFactory = () => Promise<Source>;
type SourceObject = Source | SourceFactory | SourceAsync | SourceAsyncFactory;
interface BaseImageProps extends NodeProps {
    cache?: boolean;
    x?: number;
    y?: number;
    srcSet?: SrcSet;
    sizes?: Sizes;
}
interface ImageWithSrcProp extends BaseImageProps {
    src: SourceObject;
    source?: never;
}
interface ImageWithSourceProp extends BaseImageProps {
    source: SourceObject;
    src?: never;
}
type ImageProps = ImageWithSrcProp | ImageWithSourceProp;
type ImageNode = {
    type: typeof P.Image;
    props: ImageProps;
    image?: Image;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: never[];
};
type SafeImageNode = Omit<ImageNode, 'style'> & {
    style: SafeStyle;
};

type TextInstanceNode = {
    type: typeof P.TextInstance;
    props?: never;
    style?: never;
    box?: never;
    origin?: never;
    children?: never[];
    yogaNode?: never;
    value: string;
};
type SafeTextInstanceNode = TextInstanceNode;

interface TspanProps extends SVGPresentationAttributes {
    x?: string | number;
    y?: string | number;
}
interface SafeTspanProps extends SafeSVGPresentationAttributes {
    x?: number;
    y?: number;
}
type TspanNode = {
    type: typeof P.Tspan;
    props: TspanProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    lines?: Paragraph;
    children?: TextInstanceNode[];
};
type SafeTspanNode = Omit<TspanNode, 'style' | 'props' | 'children'> & {
    style: SafeStyle;
    props: SafeTspanProps;
    children?: SafeTextInstanceNode[];
};

interface TextProps extends NodeProps {
    /**
     * Enable/disable page wrapping for element.
     * @see https://react-pdf.org/components#page-wrapping
     */
    wrap?: boolean;
    render?: RenderProp;
    /**
     * Override the default hyphenation-callback
     * @see https://react-pdf.org/fonts#registerhyphenationcallback
     */
    hyphenationCallback?: HyphenationCallback;
    /**
     * Specifies the minimum number of lines in a text element that must be shown at the bottom of a page or its container.
     * @see https://react-pdf.org/advanced#orphan-&-widow-protection
     */
    orphans?: number;
    /**
     * Specifies the minimum number of lines in a text element that must be shown at the top of a page or its container..
     * @see https://react-pdf.org/advanced#orphan-&-widow-protection
     */
    widows?: number;
    x?: number;
    y?: number;
}
type TextNode = {
    type: typeof P.Text;
    props: TextProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    lines?: Paragraph;
    alignOffset?: number;
    children?: (TextNode | TextInstanceNode | ImageNode | TspanNode)[];
};
type SafeTextNode = Omit<TextNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: (SafeTextNode | SafeTextInstanceNode | SafeImageNode | SafeTspanNode)[];
};

interface LinkProps extends NodeProps {
    /**
     * Enable/disable page wrapping for element.
     * @see https://react-pdf.org/components#page-wrapping
     */
    wrap?: boolean;
    href?: string;
    src?: string;
    hitSlop?: HitSlop;
    render?: RenderProp;
}
type LinkNode = {
    type: typeof P.Link;
    props: LinkProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: (ViewNode | ImageNode | TextNode | TextInstanceNode)[];
};
type SafeLinkNode = Omit<LinkNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: (SafeViewNode | SafeImageNode | SafeTextNode | SafeTextInstanceNode)[];
};

interface TextInputFormatting {
    type: 'date' | 'time' | 'percent' | 'number' | 'zip' | 'zipPlus4' | 'phone' | 'ssn';
    param?: string;
    nDec?: number;
    sepComma?: boolean;
    negStyle?: 'MinusBlack' | 'Red' | 'ParensBlack' | 'ParensRed';
    currency?: string;
    currencyPrepend?: boolean;
}
interface TextInputProps extends FormCommonProps {
    align?: 'left' | 'center' | 'right';
    multiline?: boolean;
    /**
     * The text will be masked (e.g. with asterisks).
     */
    password?: boolean;
    /**
     * If set, text entered in the field is not spell-checked
     */
    noSpell?: boolean;
    format?: TextInputFormatting;
    /**
     * Sets the fontSize (default or 0 means auto sizing)
     */
    fontSize?: number;
    /**
     * Sets the maximum length (characters) of the text in the field
     */
    maxLength?: number;
}
type TextInputNode = {
    type: typeof P.TextInput;
    props: TextInputProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: never[];
};
type SafeTextInputNode = Omit<TextInputNode, 'style'> & {
    style: SafeStyle;
};

interface FieldSetProps extends NodeProps {
    name: string;
}
type FieldSetNode = {
    type: typeof P.FieldSet;
    props: FieldSetProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: (TextNode | ViewNode | TextInputNode)[];
};
type SafeFieldSetNode = Omit<FieldSetNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: (SafeTextNode | SafeViewNode | SafeTextInputNode)[];
};

interface SelectAndListProps extends FormCommonProps {
    sort?: boolean;
    edit?: boolean;
    multiSelect?: boolean;
    noSpell?: boolean;
    select?: string[];
}
type SelectNode = {
    type: typeof P.Select;
    props: SelectAndListProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: never;
    children?: never[];
};
type SafeSelectNode = Omit<SelectNode, 'style'> & {
    style: SafeStyle;
};
type ListNode = {
    type: typeof P.List;
    props: SelectAndListProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: never[];
};
type SafeListNode = Omit<ListNode, 'style'> & {
    style: SafeStyle;
};

type NoteNode = {
    type: typeof P.Note;
    props: NodeProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: TextInstanceNode[];
};
type SafeNoteNode = Omit<NoteNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: SafeTextInstanceNode[];
};

interface ViewProps extends NodeProps {
    id?: string;
    /**
     * Enable/disable page wrapping for element.
     * @see https://react-pdf.org/components#page-wrapping
     */
    wrap?: boolean;
    render?: RenderProp;
}
type ViewNode = {
    type: typeof P.View;
    props: ViewProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: (ViewNode | ImageNode | ImageBackgroundNode | TextNode | LinkNode | CanvasNode | FieldSetNode | TextInputNode | SelectNode | ListNode | CheckboxNode | NoteNode)[];
};
type SafeViewNode = Omit<ViewNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: (SafeViewNode | SafeImageNode | SafeImageBackgroundNode | SafeTextNode | SafeLinkNode | SafeCanvasNode | SafeFieldSetNode | SafeTextInputNode | SafeSelectNode | SafeListNode | SafeCanvasNode | SafeNoteNode)[];
};

interface BaseImageBackgroundProps extends NodeProps {
    cache?: boolean;
    srcSet?: SrcSet;
    sizes?: Sizes;
    imageStyle?: Style;
    wrap?: boolean;
}
interface ImageBackgroundWithSrcProp extends BaseImageBackgroundProps {
    src: SourceObject;
    source?: never;
}
interface ImageBackgroundWithSourceProp extends BaseImageBackgroundProps {
    source: SourceObject;
    src?: never;
}
type ImageBackgroundProps = ImageBackgroundWithSrcProp | ImageBackgroundWithSourceProp;
type ImageBackgroundNode = {
    type: typeof P.ImageBackground;
    props: ImageBackgroundProps;
    image?: Image;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: (ViewNode | ImageNode | TextNode | LinkNode | CanvasNode | FieldSetNode | TextInputNode | SelectNode | ListNode | CheckboxNode | NoteNode)[];
};
type SafeImageBackgroundNode = Omit<ImageBackgroundNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: (SafeViewNode | SafeImageNode | SafeTextNode | SafeLinkNode | SafeCanvasNode | SafeFieldSetNode | SafeTextInputNode | SafeSelectNode | SafeListNode | SafeCheckboxNode | SafeNoteNode)[];
};

type Orientation = 'portrait' | 'landscape';
type StandardPageSize = '4A0' | '2A0' | 'A0' | 'A1' | 'A2' | 'A3' | 'A4' | 'A5' | 'A6' | 'A7' | 'A8' | 'A9' | 'A10' | 'B0' | 'B1' | 'B2' | 'B3' | 'B4' | 'B5' | 'B6' | 'B7' | 'B8' | 'B9' | 'B10' | 'C0' | 'C1' | 'C2' | 'C3' | 'C4' | 'C5' | 'C6' | 'C7' | 'C8' | 'C9' | 'C10' | 'RA0' | 'RA1' | 'RA2' | 'RA3' | 'RA4' | 'SRA0' | 'SRA1' | 'SRA2' | 'SRA3' | 'SRA4' | 'EXECUTIVE' | 'FOLIO' | 'LEGAL' | 'LETTER' | 'TABLOID' | 'ID1';
type StaticSize = number | string;
type PageSize = number | StandardPageSize | [StaticSize] | [StaticSize, StaticSize] | {
    width: StaticSize;
    height?: StaticSize;
};
interface PageProps extends NodeProps {
    /**
     * Enable page wrapping for this page.
     * @see https://react-pdf.org/components#page-wrapping
     */
    wrap?: boolean;
    size?: PageSize;
    orientation?: Orientation;
    dpi?: number;
}
type PageNode = {
    type: typeof P.Page;
    props: PageProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: (ViewNode | ImageNode | ImageBackgroundNode | TextNode | LinkNode | CanvasNode | FieldSetNode | TextInputNode | SelectNode | ListNode | CheckboxNode | NoteNode)[];
};
type SafePageNode = Omit<PageNode, 'style' | 'children'> & {
    style: SafeStyle;
    children?: (SafeViewNode | SafeImageNode | SafeImageBackgroundNode | SafeTextNode | SafeLinkNode | SafeCanvasNode | SafeFieldSetNode | SafeTextInputNode | SafeSelectNode | SafeListNode | SafeCheckboxNode | SafeNoteNode)[];
};

type PDFVersion = '1.3' | '1.4' | '1.5' | '1.6' | '1.7' | '1.7ext3';
type PageLayout = 'singlePage' | 'oneColumn' | 'twoColumnLeft' | 'twoColumnRight' | 'twoPageLeft' | 'twoPageRight';
type PageMode = 'useNone' | 'useOutlines' | 'useThumbs' | 'fullScreen' | 'useOC' | 'useAttachments';
interface OnRenderProps {
    blob?: Blob;
}
type Permissions = {
    printing?: 'lowResolution' | 'highResolution';
    modifying?: boolean;
    copying?: boolean;
    annotating?: boolean;
    fillingForms?: boolean;
    contentAccessibility?: boolean;
    documentAssembly?: boolean;
};
type DocumentProps = {
    bookmark?: never;
    title?: string;
    author?: string;
    subject?: string;
    creator?: string;
    keywords?: string;
    producer?: string;
    language?: string;
    creationDate?: Date;
    modificationDate?: Date;
    pdfVersion?: PDFVersion;
    pageMode?: PageMode;
    pageLayout?: PageLayout;
    ownerPassword?: string;
    userPassword?: string;
    permissions?: Permissions;
    onRender?: (props: OnRenderProps) => any;
};
type DocumentNode = {
    type: typeof P.Document;
    props: DocumentProps;
    box?: never;
    origin?: never;
    style?: Style | Style[];
    yoga?: YogaInstance;
    yogaNode?: never;
    children: PageNode[];
};
type SafeDocumentNode = Omit<DocumentNode, 'style' | 'children'> & {
    style: SafeStyle;
    children: SafePageNode[];
};

interface GProps extends SVGPresentationAttributes {
    style?: Style | Style[];
}
interface SafeGProps extends SafeSVGPresentationAttributes {
    style?: Style;
}
type GNode = {
    type: typeof P.G;
    props: GProps;
    style?: Style | Style[];
    box?: never;
    origin?: never;
    yogaNode?: never;
    children?: (LineNode | PolylineNode | PolygonNode | PathNode | RectNode | CircleNode | EllipseNode | ImageNode | TextNode | TspanNode | GNode)[];
};
type SafeGNode = Omit<GNode, 'style' | 'props' | 'children'> & {
    style: SafeStyle;
    props: SafeGProps;
    children?: (SafeLineNode | SafePolylineNode | SafePolygonNode | SafePathNode | SafeRectNode | SafeCircleNode | SafeEllipseNode | SafeImageNode | SafeTextNode | SafeTspanNode | SafeGNode)[];
};

type Viewbox = {
    minX: number;
    minY: number;
    maxX: number;
    maxY: number;
};
type PreserveAspectRatio = {
    align: 'none' | 'xMinYMin' | 'xMidYMin' | 'xMaxYMin' | 'xMinYMid' | 'xMidYMid' | 'xMaxYMid' | 'xMinYMax' | 'xMidYMax' | 'xMaxYMax';
    meetOrSlice: 'meet' | 'slice';
};
interface SvgProps extends NodeProps, SVGPresentationAttributes {
    width?: string | number;
    height?: string | number;
    viewBox?: string | Viewbox;
    preserveAspectRatio?: string;
}
interface SvgSafeProps extends NodeProps, SafeSVGPresentationAttributes {
    width?: string | number;
    height?: string | number;
    viewBox?: Viewbox;
    preserveAspectRatio?: PreserveAspectRatio;
}
type SvgNode = {
    type: typeof P.Svg;
    props: SvgProps;
    style?: Style | Style[];
    box?: Box;
    origin?: Origin;
    yogaNode?: YogaNode;
    children?: (LineNode | PolylineNode | PolygonNode | PathNode | RectNode | CircleNode | EllipseNode | ImageNode | TextNode | TspanNode | GNode | DefsNode)[];
};
type SafeSvgNode = Omit<SvgNode, 'style' | 'props' | 'children'> & {
    style: SafeStyle;
    props: SvgSafeProps;
    children?: (SafeLineNode | SafePolylineNode | SafePolygonNode | SafePathNode | SafeRectNode | SafeCircleNode | SafeEllipseNode | SafeImageNode | SafeTextNode | SafeTspanNode | SafeGNode | SafeDefsNode)[];
};

type Node = DocumentNode | PageNode | ImageNode | ImageBackgroundNode | SvgNode | CircleNode | ClipPathNode | DefsNode | EllipseNode | GNode | LineNode | LinearGradientNode | PathNode | PolygonNode | PolylineNode | RadialGradientNode | RectNode | StopNode | TspanNode | ViewNode | LinkNode | TextNode | TextInstanceNode | NoteNode | CanvasNode | FieldSetNode | TextInputNode | SelectNode | ListNode | CheckboxNode;
type SafeNode = SafeDocumentNode | SafePageNode | SafeImageNode | SafeImageBackgroundNode | SafeSvgNode | SafeCircleNode | SafeClipPathNode | SafeDefsNode | SafeEllipseNode | SafeGNode | SafeLineNode | SafeLinearGradientNode | SafePathNode | SafePolygonNode | SafePolylineNode | SafeRadialGradientNode | SafeRectNode | SafeStopNode | SafeTspanNode | SafeViewNode | SafeLinkNode | SafeTextNode | SafeTextInstanceNode | SafeNoteNode | SafeCanvasNode | SafeFieldSetNode | SafeTextInputNode | SafeSelectNode | SafeListNode | SafeCheckboxNode;

declare const layout: (value: DocumentNode) => Promise<SafeDocumentNode>;

export { type Bookmark, type Box, type CanvasNode, type CheckboxNode, type CircleNode, type ClipPathNode, type Defs, type DefsNode, type DocumentNode, type DocumentProps, type DominantBaseline, type DynamicPageProps, type EllipseNode, type FieldSetNode, type FillRule, type FormCommonProps, type GNode, type ImageBackgroundNode, type ImageBackgroundProps, type ImageNode, type ImageProps, type LineNode, type LinearGradientNode, type LinkNode, type ListNode, type Node, type NodeProps, type NoteNode, type OnRenderProps, type Orientation, type Origin, type PDFVersion, type PageLayout, type PageMode, type PageNode, type PageSize, type PathNode, type Permissions, type PolygonNode, type PolylineNode, type PreserveAspectRatio, type RadialGradientNode, type RectNode, type RenderProp, type SVGPresentationAttributes, type SafeCanvasNode, type SafeCheckboxNode, type SafeCircleNode, type SafeClipPathNode, type SafeDefs, type SafeDefsNode, type SafeDocumentNode, type SafeEllipseNode, type SafeFieldSetNode, type SafeGNode, type SafeImageBackgroundNode, type SafeImageNode, type SafeLineNode, type SafeLinearGradientNode, type SafeLinkNode, type SafeListNode, type SafeNode, type SafeNoteNode, type SafePageNode, type SafePathNode, type SafePolygonNode, type SafePolylineNode, type SafeRadialGradientNode, type SafeRectNode, type SafeSVGPresentationAttributes, type SafeSelectNode, type SafeStopNode, type SafeSvgNode, type SafeTextInputNode, type SafeTextInstanceNode, type SafeTextNode, type SafeTspanNode, type SafeViewNode, type SelectNode, type SourceObject, type StandardPageSize, type StopNode, type StrokeLinecap, type StrokeLinejoin, type SvgNode, type TextAnchor, type TextInputNode, type TextInstanceNode, type TextNode, type TspanNode, type ViewNode, type Viewbox, type Visibility, type YogaInstance, layout as default };
