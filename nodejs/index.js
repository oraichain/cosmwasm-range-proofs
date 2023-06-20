const { genProof, verify } = require('./dist');

module.exports = {
  genProof,
  verify: (proof, commitment, n) => {
    return verify(Buffer.from(proof), Buffer.from(commitment), n);
  }
};
