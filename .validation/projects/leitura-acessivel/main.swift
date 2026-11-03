import Foundation
enum Problem: Error {case invalidInput}
func escape(_ text:String)->String {text.replacingOccurrences(of:"&",with:"&amp;").replacingOccurrences(of:"<",with:"&lt;").replacingOccurrences(of:">",with:"&gt;").replacingOccurrences(of:"\"",with:"&quot;").replacingOccurrences(of:"'",with:"&#39;")}
func render(_ text:String,font:Int) throws -> String {
    guard (16...40).contains(font),text.utf8.count<=2_000_000 else {throw Problem.invalidInput}
    return """
    <!doctype html><html lang="pt-BR"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
    <title>Leitura acessível</title><style>body{margin:0;background:#fff;color:#151515;font: \(font)px/1.8 system-ui}main,nav{max-width:65ch;margin:1.5rem auto;padding:0 1.5rem}pre{white-space:pre-wrap;font:inherit}button{font:inherit;padding:.4em;margin:.3em}:focus-visible{outline:3px solid #2050c0;outline-offset:3px}@media(prefers-color-scheme:dark){body{background:#161616;color:#f2f2f2}}</style>
    <nav aria-label="Ajustes de leitura"><button id="less" type="button" aria-label="Diminuir fonte">A−</button><button id="more" type="button" aria-label="Aumentar fonte">A+</button><a href="#texto">Ir ao texto</a></nav>
    <main id="texto" tabindex="-1"><h1>Leitura</h1><pre>\(escape(text))</pre></main>
    <script>let size=\(font);function adjust(n){size=Math.max(16,Math.min(40,size+n));document.body.style.fontSize=size+'px'}document.getElementById('less').onclick=()=>adjust(-2);document.getElementById('more').onclick=()=>adjust(2);</script></html>
    """
}
do {
    let args=Array(CommandLine.arguments.dropFirst())
    if args == ["--self-test"] {
        let html=try render("<script>alert(1)</script> &",font:22)
        guard html.contains("&lt;script&gt;alert(1)&lt;/script&gt; &amp;"),!html.contains("<script>alert(1)") else {throw Problem.invalidInput}
        do {_=try render("a",font:5);fatalError("Teste falhou")}catch Problem.invalidInput{}
        print("2 cenários OK")
    } else {
        guard args.count==2,let font=Int(args[1]) else {throw Problem.invalidInput}
        print(try render(String(contentsOfFile:args[0],encoding:.utf8),font:font))
    }
} catch {FileHandle.standardError.write(Data("Erro: use swift main.swift texto.txt TAMANHO(16..40) > leitura.html\n".utf8));exit(2)}
