const { genProof, verify } = require('./');

const [proof, commitment] = genProof(16, 8);

const ok = verify(proof, commitment, 8);
console.log(proof, commitment, ok);
