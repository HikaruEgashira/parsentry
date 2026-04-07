import * as fontkit from 'fontkit';

type Font = Omit<fontkit.Font, 'type'> & {
    type: 'TTF' | 'WOFF' | 'WOFF2' | 'STANDARD';
    encode?: (string: string) => number[];
};
type FontStyle = 'normal' | 'italic' | 'oblique';
type FontWeight = number | 'thin' | 'hairline' | 'ultralight' | 'extralight' | 'light' | 'normal' | 'medium' | 'semibold' | 'demibold' | 'bold' | 'ultrabold' | 'extrabold' | 'heavy' | 'black';
type FontDescriptor = {
    fontFamily: string;
    fontStyle?: FontStyle;
    fontWeight?: FontWeight;
};
type RemoteOptions = {
    method?: 'GET' | 'HEAD' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
    headers?: Record<string, string>;
    body?: any;
};
type FontSourceOptions = {
    postscriptName?: string;
} & RemoteOptions;
type FontSource$1 = {
    src: string;
    fontStyle?: FontStyle;
    fontWeight?: FontWeight;
} & FontSourceOptions;
type SingleLoad = {
    family: string;
} & FontSource$1;
type BulkLoad = {
    family: string;
    fonts: FontSource$1[];
};
interface EmojiSourceUrl {
    url: string;
    format?: string;
    withVariationSelectors?: boolean;
}
interface EmojiSourceBuilder {
    builder: (code: string) => string;
    withVariationSelectors?: boolean;
}
type EmojiSource = EmojiSourceUrl | EmojiSourceBuilder;
type HyphenationCallback = (word: string) => string[];

declare class FontSource {
    src: string;
    fontFamily: string;
    fontStyle: FontStyle;
    fontWeight: number;
    data: Font | null;
    options: FontSourceOptions;
    loadResultPromise: Promise<void> | null;
    constructor(src: string, fontFamily: string, fontStyle?: FontStyle, fontWeight?: number, options?: FontSourceOptions);
    _load(): Promise<void>;
    load(): Promise<void>;
}

declare class FontFamily {
    family: string;
    sources: FontSource[];
    static create(family: string): FontFamily;
    constructor(family: string);
    register({ src, fontWeight, fontStyle, ...options }: Omit<SingleLoad, 'family'>): void;
    resolve(descriptor: FontDescriptor): FontSource;
}

declare class FontStore {
    fontFamilies: Record<string, FontFamily>;
    emojiSource: EmojiSource | null;
    constructor();
    hyphenationCallback: HyphenationCallback | null;
    register: (data: SingleLoad | BulkLoad) => void;
    registerEmojiSource: (emojiSource: EmojiSource) => void;
    registerHyphenationCallback: (callback: HyphenationCallback) => void;
    getFont: (descriptor: FontDescriptor) => FontSource;
    load: (descriptor: FontDescriptor) => Promise<void>;
    reset: () => void;
    clear: () => void;
    getRegisteredFonts: () => Record<string, FontFamily>;
    getEmojiSource: () => EmojiSource | null;
    getHyphenationCallback: () => HyphenationCallback | null;
    getRegisteredFontFamilies: () => string[];
}
type FontStoreType = FontStore;

export { type BulkLoad, type EmojiSource, type Font, type FontDescriptor, type FontSource$1 as FontSource, type FontSourceOptions, type FontStoreType, type FontStyle, type FontWeight, type HyphenationCallback, type RemoteOptions, type SingleLoad, FontStore as default };
