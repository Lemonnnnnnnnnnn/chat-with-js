const CODE_WRAPPER = "```"
const PREFIX = "我有一些代码块如下：\n\n"

export function compose(code: string[], prompt: string) {
    return PREFIX + code.map(wrap_code).join("\n\n") + "\n\n" + prompt
}

function wrap_code(code: string) {
    return `${CODE_WRAPPER}\n${code}\n${CODE_WRAPPER}`
}

export function addon_prompt(prompt: string) {
    return `${prompt},output nothing but the code`
}
