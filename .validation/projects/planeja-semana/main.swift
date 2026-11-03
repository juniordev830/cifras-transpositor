import Foundation
struct Task: Codable { let id: String; let minutes: Int; let priority: Int }
struct Input: Codable { let capacity: [Int]; let tasks: [Task] }
struct Schedule: Codable { var days: [[String]]; var remaining: [Int]; var backlog: [String] }
enum Problem: Error { case invalidInput }
func plan(_ input: Input) throws -> Schedule {
    guard input.capacity.count == 7, input.capacity.allSatisfy({ (0...1440).contains($0) }), Set(input.tasks.map(\.id)).count == input.tasks.count,
        input.tasks.allSatisfy({ !$0.id.isEmpty && (1...10080).contains($0.minutes) && (1...5).contains($0.priority) }) else { throw Problem.invalidInput }
    var result=Schedule(days:Array(repeating:[],count:7),remaining:input.capacity,backlog:[])
    for task in input.tasks.sorted(by:{ $0.priority != $1.priority ? $0.priority > $1.priority : $0.id < $1.id }) {
        if let day=result.remaining.firstIndex(where: { $0 >= task.minutes }) {
            result.days[day].append(task.id); result.remaining[day] -= task.minutes
        } else { result.backlog.append(task.id) }
    }
    return result
}
do {
    let args=Array(CommandLine.arguments.dropFirst())
    if args == ["--self-test"] {
        let tasks=[Task(id:"a",minutes:45,priority:5),Task(id:"b",minutes:90,priority:1)]
        let result=try plan(Input(capacity:[60,0,0,0,0,0,0],tasks:tasks))
        guard result.days[0]==["a"], result.remaining[0]==15, result.backlog==["b"] else { throw Problem.invalidInput }
        do { _=try plan(Input(capacity:[0],tasks:[])); fatalError("Teste falhou") } catch Problem.invalidInput {}
        print("2 cenários OK")
    } else {
        guard args.count==1 else { throw Problem.invalidInput }
        let input=try JSONDecoder().decode(Input.self,from:Data(contentsOf:URL(fileURLWithPath:args[0])))
        let encoder=JSONEncoder(); encoder.outputFormatting=[.prettyPrinted,.sortedKeys]
        print(String(decoding:try encoder.encode(plan(input)),as:UTF8.self))
    }
} catch { FileHandle.standardError.write(Data("Erro: entrada inválida. Uso: swift main.swift semana.json\n".utf8)); exit(2) }
