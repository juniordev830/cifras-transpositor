import Foundation
struct Move: Codable, Equatable { let source: String; let destination: String; let conflict: Bool }
enum Problem: Error { case invalidInput }
func category(_ name: String) -> String {
    switch URL(fileURLWithPath:name).pathExtension.lowercased() {
        case "pdf","docx","txt": return "documentos"
        case "png","jpg","jpeg","webp": return "imagens"
        case "zip","tar","gz": return "compactados"
        case "mp3","wav","flac": return "audio"
        default: return "outros"
    }
}
func preview(_ folder: URL) throws -> [Move] {
    let manager=FileManager.default
    let entries=try manager.contentsOfDirectory(at:folder,includingPropertiesForKeys:[.isRegularFileKey,.isSymbolicLinkKey],options:[.skipsHiddenFiles])
    return try entries.sorted(by:{$0.lastPathComponent<$1.lastPathComponent}).compactMap { file in
        let values=try file.resourceValues(forKeys:[.isRegularFileKey,.isSymbolicLinkKey])
        guard values.isRegularFile==true,values.isSymbolicLink!=true else { return nil }
        let target=folder.appendingPathComponent(category(file.lastPathComponent)).appendingPathComponent(file.lastPathComponent)
        return Move(source:file.path,destination:target.path,conflict:manager.fileExists(atPath:target.path))
    }
}
do {
    let args=Array(CommandLine.arguments.dropFirst())
    if args == ["--self-test"] {
        guard category("FOTO.JPG")=="imagens",category("sem-extensao")=="outros",category("a.pdf")=="documentos" else { throw Problem.invalidInput }
        let dir=FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        try FileManager.default.createDirectory(at:dir,withIntermediateDirectories:true)
        defer { try? FileManager.default.removeItem(at:dir) }
        try Data("teste".utf8).write(to:dir.appendingPathComponent("a.pdf"))
        let moves=try preview(dir); guard moves.count==1,!moves[0].conflict else {throw Problem.invalidInput}
        print("4 cenários OK")
    } else {
        guard args.count==1 else {throw Problem.invalidInput}
        let encoder=JSONEncoder(); encoder.outputFormatting=[.prettyPrinted,.sortedKeys]
        print(String(decoding:try encoder.encode(preview(URL(fileURLWithPath:args[0],isDirectory:true))),as:UTF8.self))
    }
} catch { FileHandle.standardError.write(Data("Erro: pasta inválida ou sem permissão. Uso: swift main.swift PASTA\n".utf8)); exit(2) }
