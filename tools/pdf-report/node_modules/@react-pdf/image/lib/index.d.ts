/// <reference types="node" />
interface Image {
    width: number;
    height: number;
    data: Buffer;
    format: 'jpeg' | 'png';
    key?: string;
}
type ImageFormat = 'jpg' | 'jpeg' | 'png';
type DataImageSrc = {
    data: Buffer;
    format: ImageFormat;
};
type LocalImageSrc = {
    uri: string;
    format?: ImageFormat;
};
type RemoteImageSrc = {
    uri: string;
    method?: 'GET' | 'HEAD' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
    headers?: Record<string, string>;
    format?: ImageFormat;
    body?: any;
    credentials?: 'omit' | 'same-origin' | 'include';
};
type Base64ImageSrc = {
    uri: `data:image${string}`;
};
type ImageSrc = Blob | Buffer | DataImageSrc | LocalImageSrc | RemoteImageSrc | Base64ImageSrc;

declare const resolveImage: (src: ImageSrc, { cache }?: {
    cache?: boolean | undefined;
}) => Promise<Image | null> | null | undefined;

export { type Image, type ImageSrc, resolveImage as default };
