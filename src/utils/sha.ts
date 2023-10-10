export async function computeSHA(data: Uint8Array) {
  const buffer = await crypto.subtle.digest('SHA-256', data)

  // Convert the hash to a hexadecimal string
  const hashArray = Array.from(new Uint8Array(buffer))
  const hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('')
  return hashHex
}
