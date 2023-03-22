export { decode, encode };
declare function decode(string: string, isUrlEncoded?: boolean): Uint8Array;
declare function encode(bytes: Uint8Array, urlEncode?: boolean, withPad?: boolean): string;
