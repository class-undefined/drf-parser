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

export const parseDrf = async (content: string) => {
    await init({ module_or_path: wasmURL })
    const drf = parse_drf(content)
    return JSON.parse(drf)
}
```

### layermap
```typescript
import wasmURL from "@class-undefined/drf-parser/drf_parser_bg.wasm?url"
import init, { parse_layermap } from "@class-undefined/drf-parser"

export const parseLayermap = async (content: string) => {
    await init({ module_or_path: wasmURL })
    const layermap = parse_layermap(content)
    return JSON.parse(layermap)
}
```