# drf-parser

Parser for display.drf and layermap files

## Installation

```bash
npm install @class-undefined/drf-parser
```

## Usage
### display.drf
```typescript
import wasmURL from "@class-undefined/drf-parser/drf_parser_bg.wasm?url"
import init, { parse_drf } from "@class-undefined/drf-parser"

type ITechColor = {
    name: string
    rgb: [number, number, number] // [R, G, B] 0-255
    blink: boolean
}

type ITechLineStyle = {
    name: string
    width: number
    pattern: number[]
}

type ITechStipple = {
    name: string
    bitmap: number[][]
    row: number
    col: number
}

type ITechPacket = {
    name: string
    outline: string
    stipple: string
    line_style: string
    fill: string
    fill_style: string | null
}

type ITechStyle = {
    name: string
    colors: { [key: string]: ITechColor }
    line_styles: { [key: string]: ITechLineStyle }
    stipples: { [key: string]: ITechStipple }
    packets: { [key: string]: ITechPacket }
}

export const parseDrf = async (content: string): ITechStyle => {
    await init({ module_or_path: wasmURL })
    const drf = parse_drf(content)
    return JSON.parse(drf)
}
```

### layermap
```typescript
import wasmURL from "@class-undefined/drf-parser/drf_parser_bg.wasm?url"
import init, { parse_layermap } from "@class-undefined/drf-parser"

type LayerMap = {
    // key: <layername>#<purpose>
    // value: [streamNumber, dataType]
    // example: M1 drawing 35 0  => M1#drawing: [35, 0]
    [key: string]: [number, number]
}

export const parseLayermap = async (content: string) => {
    await init({ module_or_path: wasmURL })
    const layermap = parse_layermap(content)
    return JSON.parse(layermap)
}


```