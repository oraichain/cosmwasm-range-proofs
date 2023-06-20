export function genProof(secret: number, n: number): [Uint8Array, Uint8Array];

export function verify(proof: Uint8Array, n: number): boolean;
