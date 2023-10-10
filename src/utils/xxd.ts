import { Buffer } from 'node:buffer'

type XDDOptions = {
  bytesPerLine: number
}

function* _xxd(_data: Uint8Array, xddOptions: Partial<XDDOptions> = {}): Generator<string, void, unknown> {
  // - print `bytesPerLine` bytes per line
  // - each byte printed in 'hex' format occupies 2 characters
  // - add a space every 2 bytes

  const defaultXDDOptions = {
    bytesPerLine: 8,
  }

  const { bytesPerLine } = { ...defaultXDDOptions, ...xddOptions }

  const data = Buffer.from(_data.buffer)

  for (let i = 0; i < data.byteLength; i += bytesPerLine) {
    const chunk = Uint8Array.prototype.slice.call(data, i, i + bytesPerLine)
    const offset = i.toString(16).padStart(8, '0')
    const hexLine = chunk.toString('hex').match(/.{1,2}/g)!.join(' ')
    const asciiLine = chunk.toString('utf-8').replace(/[^\x20-\x7E]/g, '.');

    const bytesPerGroup = 1
    const padLength = bytesPerLine * 2 + (bytesPerLine / bytesPerGroup)
    yield `${offset}: ${hexLine.padEnd(padLength, ' ')}  ${asciiLine}`
  }
}

export function xxd(data: Uint8Array, xddOptions: Partial<XDDOptions> = {}): string {
  const xddGenerator = _xxd(data, xddOptions)
  const xddSnapshot = [...xddGenerator].join('\n')

  return xddSnapshot
}
