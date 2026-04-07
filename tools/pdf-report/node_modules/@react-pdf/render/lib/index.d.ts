/// <reference types="node" />
import { SafeDocumentNode } from '@react-pdf/layout';
import { Font } from '@react-pdf/textkit';
import PDFKitDocument from 'pdfkit';

type PDFFontSource = string | Buffer | Uint8Array | ArrayBuffer | Font;
type Context = typeof PDFKitDocument & {
    _root: any;
    _font: any;
    _imageRegistry: any;
    _acroform: any;
    _fontSize: number;
    openImage: any;
    addNamedDestination: any;
    addPage(options?: any): Context;
    translate(x: number, y: number, options: any): Context;
    font(src: PDFFontSource, size?: number): Context;
    font(src: PDFFontSource, family: string, size?: number): Context;
};

declare const render: (ctx: Context, doc: SafeDocumentNode) => Context;

export { render as default };
