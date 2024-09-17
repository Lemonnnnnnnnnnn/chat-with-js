import { parse_output } from ".";
import { describe, it, expect } from 'bun:test'

describe('parse_output', () => {
    it('parse_output', async () => {
        const res = parse_output("```js\nfunction fibonacci(n) {\n  if (n <= 1) return n;\n  else return fibonacci(n-1) + fibonacci(n-2);\n}\n```")
        expect(res).toBeString();
        expect(res).toBe("function fibonacci(n) {\n  if (n <= 1) return n;\n  else return fibonacci(n-1) + fibonacci(n-2);\n}")
    })
})