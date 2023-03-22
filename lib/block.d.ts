export { deserialize, fromCompactSize, fromBigInt };
declare function deserialize(bytes: Uint8Array): Array<Uint8Array>;
declare function fromCompactSize(bytes: Uint8Array): number;
declare function fromBigInt(bytes: Uint8Array): bigint;
