type Container = {
    width: number;
    height: number;
    dpi?: number;
    remBase?: number;
    orientation?: 'landscape' | 'portrait';
};
type Percentage = `${string}%`;
type BorderStyleValue = 'dashed' | 'dotted' | 'solid';
type BorderShorthandStyle = {
    border?: number | string;
    borderTop?: number | string;
    borderRight?: number | string;
    borderBottom?: number | string;
    borderLeft?: number | string;
    borderColor?: string;
    borderRadius?: number | string;
    borderStyle?: BorderStyleValue;
    borderWidth?: number | string;
};
type BorderExpandedStyle = {
    borderTopColor?: string;
    borderTopStyle?: BorderStyleValue;
    borderTopWidth?: number | string;
    borderRightColor?: string;
    borderRightStyle?: BorderStyleValue;
    borderRightWidth?: number | string;
    borderBottomColor?: string;
    borderBottomStyle?: BorderStyleValue;
    borderBottomWidth?: number | string;
    borderLeftColor?: string;
    borderLeftStyle?: BorderStyleValue;
    borderLeftWidth?: number | string;
    borderTopLeftRadius?: number | string;
    borderTopRightRadius?: number | string;
    borderBottomRightRadius?: number | string;
    borderBottomLeftRadius?: number | string;
};
type BorderSafeStyle = BorderExpandedStyle & {
    borderTopWidth?: number;
    borderRightWidth?: number;
    borderBottomWidth?: number;
    borderLeftWidth?: number;
    borderTopLeftRadius?: number | Percentage;
    borderTopRightRadius?: number | Percentage;
    borderBottomRightRadius?: number | Percentage;
    borderBottomLeftRadius?: number | Percentage;
};
type BorderStyle = BorderShorthandStyle & BorderExpandedStyle;
type FlexboxShorthandStyle = {
    flex?: number | string;
};
type AlignContent = 'flex-start' | 'flex-end' | 'center' | 'stretch' | 'space-between' | 'space-around' | 'space-evenly';
type AlignItems = 'flex-start' | 'flex-end' | 'center' | 'stretch' | 'baseline';
type AlignSelf = 'auto' | 'flex-start' | 'flex-end' | 'center' | 'baseline' | 'stretch';
type FlexDirection = 'row' | 'row-reverse' | 'column' | 'column-reverse';
type FlexWrap = 'nowrap' | 'wrap' | 'wrap-reverse';
type JustifyContent = 'flex-start' | 'flex-end' | 'center' | 'space-around' | 'space-between' | 'space-evenly';
type JustifySelf = string;
type FlexboxExpandedStyle = {
    alignContent?: AlignContent;
    alignItems?: AlignItems;
    alignSelf?: AlignSelf;
    flexDirection?: FlexDirection;
    flexWrap?: FlexWrap;
    flexFlow?: number | string;
    flexGrow?: number | string;
    flexShrink?: number | string;
    flexBasis?: number | string;
    justifySelf?: JustifySelf;
    justifyContent?: JustifyContent;
};
type FlexboxSafeStyle = FlexboxExpandedStyle & {
    flexGrow?: number;
    flexShrink?: number;
};
type FlexboxStyle = FlexboxShorthandStyle & FlexboxExpandedStyle;
type GapShorthandStyle = {
    gap?: number | string;
};
type GapExpandedStyle = {
    rowGap?: number | string;
    columnGap?: number | string;
};
type GapSafeStyle = {
    rowGap?: number | string;
    columnGap?: number | string;
};
type GapStyle = GapShorthandStyle & GapExpandedStyle;
type PositionShorthandStyle = {
    objectPosition?: number | string;
};
type PositionExpandedStyle = {
    objectPositionX?: number | string;
    objectPositionY?: number | string;
    objectFit?: string;
};
type PositionSafeStyle = PositionExpandedStyle & {
    objectPositionX?: number;
    objectPositionY?: number;
};
type PositioningStyle = PositionShorthandStyle & PositionExpandedStyle;
type ScaleTransform = {
    operation: 'scale';
    value: [number, number];
};
type TranslateTransform = {
    operation: 'translate';
    value: [number, number];
};
type RotateTransform = {
    operation: 'rotate';
    value: [number, number, number];
};
type SkewTransform = {
    operation: 'skew';
    value: [number, number];
};
type MatrixTransform = {
    operation: 'matrix';
    value: [number, number, number, number, number, number];
};
type Transform = ScaleTransform | TranslateTransform | RotateTransform | SkewTransform | MatrixTransform;
type TransformShorthandStyle = {
    transformOrigin?: number | string;
};
type TransformExpandedStyle = {
    transformOriginX?: number | string;
    transformOriginY?: number | string;
    transform?: string | Transform[];
    gradientTransform?: string | Transform[];
};
type TransformSafeStyle = Omit<TransformExpandedStyle, 'transform'> & {
    transformOriginX?: number | Percentage;
    transformOriginY?: number | Percentage;
    transform?: Transform[];
    gradientTransform?: Transform[];
};
type TransformStyle = TransformShorthandStyle & TransformExpandedStyle;
type Display = 'flex' | 'none';
type Position = 'absolute' | 'relative' | 'static';
type LayoutStyle = {
    aspectRatio?: number | string;
    bottom?: number | string;
    display?: Display;
    left?: number | string;
    position?: Position;
    right?: number | string;
    top?: number | string;
    overflow?: 'hidden';
    zIndex?: number | string;
};
type LayoutExpandedStyle = LayoutStyle;
type LayoutSafeStyle = LayoutExpandedStyle & {
    aspectRatio?: number;
    bottom?: number;
    left?: number;
    right?: number;
    top?: number;
    zIndex?: number;
};
type DimensionStyle = {
    height?: number | string;
    maxHeight?: number | string;
    maxWidth?: number | string;
    minHeight?: number | string;
    minWidth?: number | string;
    width?: number | string;
};
type DimensionExpandedStyle = DimensionStyle;
type DimensionSafeStyle = DimensionExpandedStyle & {
    height?: number | Percentage;
    maxHeight?: number | Percentage;
    maxWidth?: number | Percentage;
    minHeight?: number | Percentage;
    minWidth?: number | Percentage;
    width?: number | Percentage;
};
type ColorStyle = {
    backgroundColor?: string;
    color?: string;
    opacity?: number | string;
};
type ColorExpandedStyle = ColorStyle;
type ColorSafeStyle = {
    backgroundColor?: string;
    color?: string;
    opacity?: number;
};
type FontStyle = 'normal' | 'italic' | 'oblique';
type FontWeight = string | number | 'thin' | 'hairline' | 'ultralight' | 'extralight' | 'light' | 'normal' | 'medium' | 'semibold' | 'demibold' | 'bold' | 'ultrabold' | 'extrabold' | 'heavy' | 'black';
type TextAlign = 'left' | 'right' | 'center' | 'justify';
type TextDecoration = 'line-through' | 'underline' | 'none' | 'line-through underline' | 'underline line-through';
type TextDecorationStyle = 'dashed' | 'dotted' | 'solid' | string;
type TextTransform = 'capitalize' | 'lowercase' | 'uppercase' | 'upperfirst' | 'none';
type VerticalAlign = 'sub' | 'super';
type TextStyle = {
    direction?: 'ltr' | 'rtl';
    fontSize?: number | string;
    fontFamily?: string | string[];
    fontStyle?: FontStyle;
    fontWeight?: FontWeight;
    letterSpacing?: number | string;
    lineHeight?: number | string;
    maxLines?: number | string;
    textAlign?: TextAlign;
    textDecoration?: TextDecoration;
    textDecorationColor?: string;
    textDecorationStyle?: TextDecorationStyle;
    textIndent?: any;
    textOverflow?: 'ellipsis';
    textTransform?: TextTransform;
    verticalAlign?: VerticalAlign;
};
type TextExpandedStyle = TextStyle;
type TextSafeStyle = TextExpandedStyle & {
    fontSize?: number;
    fontWeight?: number;
    letterSpacing?: number;
    lineHeight?: number;
};
type MarginShorthandStyle = {
    margin?: number | string;
    marginHorizontal?: number | string;
    marginVertical?: number | string;
};
type MarginExpandedStyle = {
    marginTop?: number | string;
    marginRight?: number | string;
    marginBottom?: number | string;
    marginLeft?: number | string;
};
type MarginSafeStyle = MarginExpandedStyle & {
    marginTop?: number | Percentage;
    marginRight?: number | Percentage;
    marginBottom?: number | Percentage;
    marginLeft?: number | Percentage;
};
type MarginStyle = MarginShorthandStyle & MarginExpandedStyle;
type PaddingShorthandStyle = {
    padding?: number | string;
    paddingHorizontal?: number | string;
    paddingVertical?: number | string;
};
type PaddingExpandedStyle = {
    paddingTop?: number | string;
    paddingRight?: number | string;
    paddingBottom?: number | string;
    paddingLeft?: number | string;
};
type PaddingSafeStyle = PaddingExpandedStyle & {
    paddingTop?: number | Percentage;
    paddingRight?: number | Percentage;
    paddingBottom?: number | Percentage;
    paddingLeft?: number | Percentage;
};
type PaddingStyle = PaddingShorthandStyle & PaddingExpandedStyle;
interface SvgStyle {
    fill?: string;
    stroke?: string;
    strokeDasharray?: string;
    strokeWidth?: string | number;
    fillOpacity?: string | number;
    fillRule?: 'nonzero' | 'evenodd';
    strokeOpacity?: string | number;
    textAnchor?: 'start' | 'middle' | 'end';
    strokeLinecap?: 'butt' | 'round' | 'square';
    strokeLinejoin?: 'butt' | 'round' | 'square' | 'miter' | 'bevel';
    visibility?: 'visible' | 'hidden' | 'collapse';
    clipPath?: string;
    dominantBaseline?: 'auto' | 'middle' | 'central' | 'hanging' | 'mathematical' | 'text-after-edge' | 'text-before-edge';
}
type SvgExpandedStyle = SvgStyle;
type SvgSafeStyle = SvgStyle & {
    strokeWidth?: number;
    fillOpacity?: number;
    strokeOpacity?: number;
};
type BaseStyle = BorderStyle & ColorStyle & DimensionStyle & FlexboxStyle & GapStyle & LayoutStyle & MarginStyle & PaddingStyle & PositioningStyle & TextStyle & TransformStyle & SvgStyle;
type MediaQueryStyle = {
    [key in `@media${string}`]: BaseStyle;
};
type Style = BaseStyle & MediaQueryStyle;
type StyleKey = keyof BaseStyle;
type ExpandedStyle = BorderExpandedStyle & ColorExpandedStyle & DimensionExpandedStyle & FlexboxExpandedStyle & GapExpandedStyle & LayoutExpandedStyle & MarginExpandedStyle & PaddingExpandedStyle & PositionExpandedStyle & TextExpandedStyle & TransformExpandedStyle & SvgExpandedStyle;
type SafeStyle = BorderSafeStyle & ColorSafeStyle & DimensionSafeStyle & FlexboxSafeStyle & GapSafeStyle & LayoutSafeStyle & MarginSafeStyle & PaddingSafeStyle & PositionSafeStyle & TextSafeStyle & TransformSafeStyle & SvgSafeStyle;

/**
 * Transform given color to hexa
 *
 * @param value - Styles value
 * @returns Transformed value
 */
declare const transformColor: (value: string) => string;

type StyleInput = Style | StyleInput[] | null | undefined;
/**
 * Flattens an array of style objects, into one aggregated style object.
 * Supports nested arrays of styles.
 *
 * @param styles - Style or style array (can be nested)
 * @returns Flattened style object
 */
declare const flatten: (value: StyleInput | StyleInput[], ...args: any[]) => Style;

type StyleParam = Style | null | undefined;
/**
 * Resolves styles
 *
 * @param container
 * @param style - Style
 * @returns Resolved style
 */
declare const resolveStyles: (container: Container, style: StyleParam | StyleParam[]) => SafeStyle;

export { type AlignContent, type AlignItems, type AlignSelf, type BorderExpandedStyle, type BorderSafeStyle, type BorderShorthandStyle, type BorderStyle, type BorderStyleValue, type ColorExpandedStyle, type ColorSafeStyle, type ColorStyle, type Container, type DimensionExpandedStyle, type DimensionSafeStyle, type DimensionStyle, type Display, type ExpandedStyle, type FlexDirection, type FlexWrap, type FlexboxExpandedStyle, type FlexboxSafeStyle, type FlexboxShorthandStyle, type FlexboxStyle, type FontStyle, type FontWeight, type GapExpandedStyle, type GapSafeStyle, type GapShorthandStyle, type GapStyle, type JustifyContent, type JustifySelf, type LayoutExpandedStyle, type LayoutSafeStyle, type LayoutStyle, type MarginExpandedStyle, type MarginSafeStyle, type MarginShorthandStyle, type MarginStyle, type MatrixTransform, type PaddingExpandedStyle, type PaddingSafeStyle, type PaddingShorthandStyle, type PaddingStyle, type Percentage, type Position, type PositionExpandedStyle, type PositionSafeStyle, type PositionShorthandStyle, type PositioningStyle, type RotateTransform, type SafeStyle, type ScaleTransform, type SkewTransform, type Style, type StyleKey, type SvgExpandedStyle, type SvgSafeStyle, type SvgStyle, type TextAlign, type TextDecoration, type TextDecorationStyle, type TextExpandedStyle, type TextSafeStyle, type TextStyle, type TextTransform, type Transform, type TransformExpandedStyle, type TransformSafeStyle, type TransformShorthandStyle, type TransformStyle, type TranslateTransform, type VerticalAlign, resolveStyles as default, flatten, transformColor };
