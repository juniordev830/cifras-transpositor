import 'dart:convert';
import 'dart:io';
void expect(bool value) { if (!value) throw StateError('Teste falhou'); }
DateTime date(String value) {
  final parsed=DateTime.parse(value);
  if (!RegExp(r'^\d{4}-\d{2}-\d{2}$').hasMatch(value) || parsed.toIso8601String().substring(0,10)!=value) throw FormatException('Data inválida');
  return DateTime.utc(parsed.year,parsed.month,parsed.day);
}

Map<String,dynamic> summarize(List<dynamic> rows,DateTime start) {
  final end=start.add(const Duration(days:7));
  final totals=<String,Map<String,num>>{'corrida':{'km':0.0,'minutos':0},'pedal':{'km':0.0,'minutos':0}};
  for(final raw in rows) {
    final row=raw as Map<String,dynamic>;
    final kind=row['modalidade']; final km=row['km']; final minutes=row['minutos'];
    if (!totals.containsKey(kind) || km is! num || !km.isFinite || km<=0 || minutes is! int || minutes<=0) throw FormatException('Atividade inválida');
    final day=date(row['data'] as String);
    if(!day.isBefore(start) && day.isBefore(end)) {
      totals[kind]!['km']=totals[kind]!['km']!+km;
      totals[kind]!['minutos']=totals[kind]!['minutos']!+minutes;
    }
  }
  return {'inicio':start.toIso8601String().substring(0,10),'fim_exclusivo':end.toIso8601String().substring(0,10),'totais':totals};
}
void main(List<String> args) {
 try {
  if(args.length==1 && args[0]=='--self-test') {
   final start=date('2026-11-02');
   final out=summarize([{'data':'2026-11-02','modalidade':'corrida','km':5,'minutos':30},{'data':'2026-11-09','modalidade':'pedal','km':20,'minutos':60}],start);
   expect(out['totais']['corrida']['km']==5 && out['totais']['pedal']['km']==0);
   var rejected=false; try {date('2026-02-30');} catch (_) {rejected=true;} expect(rejected);
   print('2 cenários OK'); return;
  }
  if(args.length!=2) throw FormatException('Uso: dart run bin/main.dart INICIO atividades.json');
  final out=summarize(jsonDecode(File(args[1]).readAsStringSync()) as List<dynamic>,date(args[0]));
  print(const JsonEncoder.withIndent('  ').convert(out));
 } catch(e) {stderr.writeln('Erro: $e'); exitCode=2;}
}
