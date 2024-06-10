import * as _hex from './hex'
import { sha3 } from 'hash-wasm'

export const publicKeyToOwner = async (publicKey: string): Promise<string> => {
  const publicKeyBytes = _hex.toBytes(publicKey)
  const typeNameBytes = new TextEncoder().encode('PublicKey::')
  const bytes = new Uint8Array([...typeNameBytes, ...publicKeyBytes])
  return await sha3(bytes, 256)
}
