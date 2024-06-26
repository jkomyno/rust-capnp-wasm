import * as capnp from '@jkomyno/capnp-ts'

// @deno-types='./capnp/event.capnp.d.ts'
import { Event as EventRoot } from './capnp/event.capnp.js'

// @deno-types='./wasm/event_capnp.d.ts'
import { createEvent, modifyEvent, readEvent, type Event } from './wasm/event_capnp.js'
import { xxd } from './utils/xxd.ts'
import { computeSHA } from './utils/sha.ts'

function createEventFromJS(event: Event): Uint8Array {
  // - use unpacked capnp
  const message = new capnp.Message(undefined, false)
  const root = message.initRoot(EventRoot)

  root.setName(event.name)
  root.setYear(event.year)

  const arrayBuffer = message.toArrayBuffer()
  return new Uint8Array(arrayBuffer)
}

function readEventFromJS(eventAsBinary: Uint8Array) {
  // - use unpacked capnp
  const message = new capnp.Message(eventAsBinary.slice(), false)

  const root = message.getRoot(EventRoot)

  return {
    name: root.getName(),
    year: root.getYear(),
  }
}

// DEMO: try replacing `*EventFromJS` with the `*Event` counterparts (coming from Rust + Wasm)
// and see what happens
async function main() {
  const eventJS = {
    name: 'rustfest',
    year: 2024,
  }

  console.log('eventJS', eventJS, '\n')

  const eventAsBinary = createEventFromJS(eventJS)
  console.log('eventAsBinary')
  console.log(xxd(eventAsBinary), '\n')

  const eventAsPlainJS = readEventFromJS(eventAsBinary)
  console.log('eventAsPlainJS', eventAsPlainJS, '\n')

  const modifiedEventAsBinary = eventAsBinary.slice()
  modifyEvent(modifiedEventAsBinary)
  console.log('modifiedEventAsBinary')
  console.log(xxd(modifiedEventAsBinary), '\n')

  const modifiedEventAsPlainJS = readEvent(modifiedEventAsBinary)
  console.log('modifiedEventAsPlainJS', modifiedEventAsPlainJS, '\n')

  const eventBinaryPath = './bin/event.bin'

  try {
    await Deno.stat(eventBinaryPath)
    console.log(`Found previous binary in ${eventBinaryPath}`)
    const oldEventAsBinary = await Deno.readFile(eventBinaryPath)
    
    const shaOldEventAsBinary = await computeSHA(oldEventAsBinary)
    const shaNewEventAsBinary = await computeSHA(eventAsBinary)
    console.log('SHA-256 match?', shaOldEventAsBinary === shaNewEventAsBinary)
  } catch (_) {
    // File does not exist, so we create it
    console.log(`Writing binary to ${eventBinaryPath}`)
    await Deno.writeFile(eventBinaryPath, eventAsBinary)
  }
}

main()
