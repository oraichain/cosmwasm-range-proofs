import { SimulateCosmWasmClient } from '@terran-one/cw-simulate';
import { Buffer } from 'buffer';
import { ContractClient, ContractTypes } from './src';
import { genProof } from '../nodejs';

const client = new SimulateCosmWasmClient({
  chainId: 'Oraichain',
  bech32Prefix: 'orai'
});

const sender = 'orai1hz4kkphvt0smw4wd9uusuxjwkp604u7m4akyzv';

(async () => {
  const { contractAddress } = await client.deploy<ContractTypes.InstantiateMsg>(sender, '../contract/artifacts/contract.wasm', { n: 8 }, 'bulletproofs', 'auto');
  const contractClient = new ContractClient(client, sender, contractAddress);
  const [proofRaw, commitment] = genProof(16, 8).map((c) => Buffer.from(c).toString('base64'));
  console.log('proof', proofRaw, 'commitment', commitment);
  const challenge = Buffer.from('random_unique_value').toString('base64');
  const verified = await contractClient.verifyProof({ commitment, proofRaw, challenge });
  console.log(verified);
})();
