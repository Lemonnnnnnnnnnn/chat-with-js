export function parse_output(text: string) {
    const reg = /```[\s\S]*?\n([\s\S]*?)```/;
    const match = reg.exec(text)
    if (match) {
        return match[1].trim()
    } else {
        return text
    }
}
