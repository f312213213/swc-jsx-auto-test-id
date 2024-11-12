# swc-jsx-auto-test-id

A SWC plugin that automatically adds test IDs to React components based on their parent function names.

## Installation 

```bash
npm install --save-dev swc-jsx-auto-test-id
```

## Usage

Add the plugin to your `.swcrc`:

```json
{
    jsc: {
        experimental: {
            plugins: [
              [
                'swc-jsx-auto-test-id',
                {
                  attribute_name: 'data-swc-test-id',
                },
              ],
            ],
        },
    },
}
```

## Examples

### Basic Usage

Input:
```jsx
function TestComponent() {
    return <Card>Test</Card>;
}
```

Output:
```jsx
function TestComponent() {
    return <Card data-swc-test-id="TestComponent">Test</Card>;
}
```

### Features

- Automatically adds test IDs to the outermost element
- Uses parent function name as the test ID
- Preserves existing test IDs
- Configurable attribute name

## Configuration

- `attribute_name` (optional): The attribute name to use for test IDs. Defaults to "data-test-id"

## License

MIT